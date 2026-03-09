import { readFile } from "node:fs/promises";
import path from "node:path";
import fg from "fast-glob";
import GithubSlugger from "github-slugger";
import matter from "gray-matter";
import MarkdownIt from "markdown-it";

const repoRoot = process.cwd();
const docsRoot = path.resolve(repoRoot, "docs");
const markdown = new MarkdownIt();

const files = await fg("**/*.md", {
  cwd: docsRoot,
  absolute: true,
});

const headingCache = new Map<string, Set<string>>();
const errors: string[] = [];

function isExternalLink(link: string) {
  return /^(https?:|mailto:|tel:)/.test(link);
}

function escapeRegex(value: string) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function decodeTarget(target: string) {
  return decodeURI(target).split("?")[0];
}

function candidateMarkdownTargets(baseFile: string, target: string) {
  const targetPath = decodeTarget(target);

  if (targetPath.startsWith("/")) {
    const normalized = targetPath.replace(/^\/+/, "");
    if (normalized === "") {
      return [path.join(docsRoot, "index.md")];
    }

    return [
      path.join(docsRoot, `${normalized}.md`),
      path.join(docsRoot, normalized, "index.md"),
    ];
  }

  if (targetPath.endsWith(".md")) {
    return [path.resolve(path.dirname(baseFile), targetPath)];
  }

  if (path.extname(targetPath) !== "") {
    return [path.resolve(path.dirname(baseFile), targetPath)];
  }

  return [
    path.resolve(path.dirname(baseFile), `${targetPath}.md`),
    path.resolve(path.dirname(baseFile), targetPath, "index.md"),
  ];
}

async function headingsFor(file: string) {
  const cached = headingCache.get(file);
  if (cached) {
    return cached;
  }

  const raw = await readFile(file, "utf8");
  const { content } = matter(raw);
  const tokens = markdown.parse(content, {});
  const slugger = new GithubSlugger();
  const headings = new Set<string>();

  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]?.type !== "heading_open") {
      continue;
    }

    const inline = tokens[index + 1];
    if (inline?.type === "inline") {
      headings.add(slugger.slug(inline.content));
    }
  }

  headingCache.set(file, headings);
  return headings;
}

function resolveSnippetPath(sourceFile: string, snippetPath: string) {
  if (snippetPath.startsWith("@/")) {
    return path.resolve(docsRoot, snippetPath.slice(2));
  }

  if (snippetPath.startsWith("/")) {
    return path.resolve(repoRoot, snippetPath.slice(1));
  }

  return path.resolve(path.dirname(sourceFile), snippetPath);
}

for (const file of files) {
  const relativePath = path.relative(docsRoot, file);
  const raw = await readFile(file, "utf8");
  const { content } = matter(raw);
  const tokens = markdown.parse(content, {});

  for (const token of tokens) {
    if (token.type !== "inline" || !token.children) {
      continue;
    }

    for (const child of token.children) {
      if (child.type !== "link_open") {
        continue;
      }

      const href = child.attrGet("href");
      if (!href || isExternalLink(href)) {
        continue;
      }

      const [targetPath, anchor] = href.split("#");
      let targetFile = file;

      if (targetPath && targetPath !== "") {
        const candidates = candidateMarkdownTargets(file, targetPath);
        targetFile = candidates.find((candidate) => files.includes(candidate)) ?? "";

        if (targetFile === "") {
          const prettyCandidates = candidates
            .map((candidate) => path.relative(repoRoot, candidate))
            .join(", ");
          errors.push(
            `docs/${relativePath}: broken link ${href} (checked: ${prettyCandidates})`,
          );
          continue;
        }
      }

      if (anchor) {
        const headings = await headingsFor(targetFile);
        if (!headings.has(anchor)) {
          const prettyTarget = path.relative(repoRoot, targetFile);
          errors.push(
            `docs/${relativePath}: missing anchor #${anchor} in ${prettyTarget}`,
          );
        }
      }
    }
  }

  const snippetPattern = /^<<<\s+(.+)$/gm;
  for (const match of content.matchAll(snippetPattern)) {
    const original = match[1].trim();
    const withoutTitle = original.replace(/\s+\[[^\]]+\]\s*$/, "");
    const withoutOptions = withoutTitle.split("{")[0].trim();
    const [snippetPath, region] = withoutOptions.split("#");
    const resolvedPath = resolveSnippetPath(file, snippetPath);

    try {
      const snippetContent = await readFile(resolvedPath, "utf8");
      if (region) {
        const regionStart = new RegExp(`#region\\s+${escapeRegex(region)}\\b`);
        const regionEnd = new RegExp(`#endregion\\s+${escapeRegex(region)}\\b`);

        if (!regionStart.test(snippetContent) || !regionEnd.test(snippetContent)) {
          errors.push(
            `docs/${relativePath}: snippet region ${region} not found in ${path.relative(
              repoRoot,
              resolvedPath,
            )}`,
          );
        }
      }
    } catch {
      errors.push(
        `docs/${relativePath}: snippet source not found for ${snippetPath} -> ${path.relative(
          repoRoot,
          resolvedPath,
        )}`,
      );
    }
  }
}

if (errors.length > 0) {
  console.error("Link and snippet validation failed:");
  for (const error of errors) {
    console.error(`- ${error}`);
  }
  process.exit(1);
}

console.log(`Validated links and snippet imports across ${files.length} markdown files.`);
