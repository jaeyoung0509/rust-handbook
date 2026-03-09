import { readFile } from "node:fs/promises";
import path from "node:path";
import fg from "fast-glob";
import matter from "gray-matter";
import {
  documentedRoutes,
  parts,
  routeToSourcePath,
} from "../docs/.vitepress/data/book.ts";

const docsRoot = path.resolve("docs");
const errors: string[] = [];

const files = await fg("**/*.md", {
  cwd: docsRoot,
  absolute: true,
});

const existingFiles = new Set(files.map((file) => path.relative(docsRoot, file)));
const partPages = new Set(parts.map((part) => routeToSourcePath(part.overview)));
const chapterPages = new Set(
  parts.flatMap((part) =>
    part.chapters
      .filter((chapter) => chapter.status === "pilot")
      .map((chapter) => routeToSourcePath(chapter.route)),
  ),
);

for (const route of documentedRoutes) {
  const sourcePath = routeToSourcePath(route);
  if (!existingFiles.has(sourcePath)) {
    errors.push(`Missing document for route ${route}: docs/${sourcePath}`);
  }
}

for (const file of files) {
  const relativePath = path.relative(docsRoot, file);
  const raw = await readFile(file, "utf8");
  const { data } = matter(raw);

  if (typeof data.title !== "string" || data.title.trim() === "") {
    errors.push(`docs/${relativePath}: frontmatter.title must be a non-empty string`);
  }

  if (typeof data.description !== "string" || data.description.trim() === "") {
    errors.push(
      `docs/${relativePath}: frontmatter.description must be a non-empty string`,
    );
  }

  if (partPages.has(relativePath) || chapterPages.has(relativePath)) {
    if (typeof data.part !== "number") {
      errors.push(`docs/${relativePath}: frontmatter.part must be a number`);
    }

    if (typeof data.order !== "number") {
      errors.push(`docs/${relativePath}: frontmatter.order must be a number`);
    }

    if (!Array.isArray(data.prerequisites)) {
      errors.push(`docs/${relativePath}: frontmatter.prerequisites must be an array`);
    } else if (data.prerequisites.some((item) => typeof item !== "string")) {
      errors.push(
        `docs/${relativePath}: frontmatter.prerequisites must only contain strings`,
      );
    }
  }
}

if (errors.length > 0) {
  console.error("Frontmatter validation failed:");
  for (const error of errors) {
    console.error(`- ${error}`);
  }
  process.exit(1);
}

console.log(`Validated frontmatter for ${files.length} markdown files.`);
