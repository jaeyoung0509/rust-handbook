import type Token from "markdown-it/lib/token.mjs";
import { defineConfig } from "vitepress";
import { book, buildSidebar } from "./data/book";

function installMermaidFence(md: {
  renderer: {
    rules: {
      fence?: (
        tokens: Token[],
        idx: number,
        options: unknown,
        env: unknown,
        self: unknown,
      ) => string;
    };
  };
}) {
  const fence = md.renderer.rules.fence;

  md.renderer.rules.fence = (tokens, idx, options, env, self) => {
    const token = tokens[idx];
    if (token.info.trim() === "mermaid") {
      const encoded = encodeURIComponent(token.content);
      return `<MermaidDiagram code="${encoded}" />`;
    }

    return fence ? fence(tokens, idx, options, env, self) : "";
  };
}

export default defineConfig({
  title: book.title,
  description: book.description,
  lang: "ko-KR",
  cleanUrls: true,
  base: "/rust-handbook/",
  lastUpdated: true,
  head: [
    ["meta", { name: "theme-color", content: "#0f766e" }],
    ["meta", { property: "og:type", content: "website" }],
    ["meta", { property: "og:title", content: book.title }],
    ["meta", { property: "og:description", content: book.description }],
  ],
  themeConfig: {
    logo: "/logo.svg",
    nav: [
      { text: "Overview", link: "/" },
      { text: "Roadmap", link: "/part-1/" },
      { text: "GitHub", link: book.repository },
    ],
    sidebar: buildSidebar(),
    socialLinks: [{ icon: "github", link: book.repository }],
    search: {
      provider: "local",
    },
    outline: [2, 3],
    editLink: {
      pattern: `${book.repository}/edit/main/docs/:path`,
      text: "이 페이지 개선하기",
    },
    footer: {
      message: "Rust를 오래 기억하기 위한 why-first handbook",
      copyright: "MIT Licensed",
    },
    docFooter: {
      prev: "이전",
      next: "다음",
    },
  },
  markdown: {
    config(md) {
      installMermaidFence(md);
    },
  },
});
