import { readFile } from "node:fs/promises";
import path from "node:path";
import fg from "fast-glob";
import { JSDOM } from "jsdom";

const docsRoot = path.resolve("docs");
const files = await fg("**/*.md", {
  cwd: docsRoot,
  absolute: true,
});

const mermaidPattern = /^```mermaid[^\n]*\n([\s\S]*?)^```$/gm;
const errors: string[] = [];

const dom = new JSDOM("<!doctype html><html><body></body></html>");
Object.assign(globalThis, {
  window: dom.window,
  document: dom.window.document,
});
Object.defineProperty(globalThis, "navigator", {
  value: dom.window.navigator,
  configurable: true,
});

const { default: mermaid } = await import("mermaid/dist/mermaid.core.mjs");

mermaid.initialize({
  startOnLoad: false,
  securityLevel: "strict",
});

for (const file of files) {
  const relativePath = path.relative(docsRoot, file);
  const raw = await readFile(file, "utf8");
  const blocks = [...raw.matchAll(mermaidPattern)];

  for (const [index, match] of blocks.entries()) {
    const diagram = match[1].trim();

    try {
      await mermaid.parse(diagram);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      errors.push(`docs/${relativePath} block #${index + 1}: ${message}`);
    }
  }
}

if (errors.length > 0) {
  console.error("Mermaid validation failed:");
  for (const error of errors) {
    console.error(`- ${error}`);
  }
  process.exit(1);
}

console.log(`Validated Mermaid diagrams in ${files.length} markdown files.`);
