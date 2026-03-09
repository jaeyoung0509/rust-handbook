# Rust Handbook

Ebook: [https://jaeyoung0509.github.io/rust-handbook/](https://jaeyoung0509.github.io/rust-handbook/)

Rust Handbook is a VitePress-based ebook for developers who already know Python or Go and want to understand Rust at a deeper level. The repository combines the documentation site, runnable Rust examples, compile-fail references, Markdown and Mermaid validation, and GitHub Pages deployment.

## Principles

- The handbook explains why Rust introduces certain constraints before it explains syntax.
- Ownership, lifetimes, traits, and async are taught as tools for designing relationships and contracts, not as rules to memorize.
- Rust code shown in the docs lives in a separate Cargo workspace so examples can be compiled and tested locally.
- CI focuses on documentation quality first. Full Rust example validation is available as a local-only workflow.
- Mermaid blocks, snippet imports, and internal links are validated automatically to keep the book coherent as it grows.

## Layout

- `docs/`: VitePress handbook content
- `docs/.vitepress/`: site configuration, theme extension, and book metadata
- `examples/`: Rust examples and compile-fail references used by the docs
- `scripts/`: frontmatter, Mermaid, internal link, and snippet validation scripts
- `.github/workflows/`: docs-first CI and GitHub Pages deployment

## Commands

```bash
pnpm install
pnpm docs:dev
pnpm verify
```

```bash
pnpm verify:full
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Verification Policy

- `pnpm verify`: docs-only validation for CI and deployment
- `pnpm verify:full`: docs validation plus full Rust example validation for local use
- `cargo test --workspace`: manual Rust example verification

## Writing Model

Each chapter follows the same structure:

1. Problem framing from a Python or Go perspective
2. Why Rust needs the constraint or abstraction
3. Mental model with Mermaid diagrams
4. Python, Go, and Rust comparison blocks
5. Runnable example pulled from the Cargo workspace
6. Compiler clinic for common diagnostics
7. When to use it, when to avoid it, and a short takeaway

## Quality Gates

- Markdown lint
- Frontmatter shape validation
- Mermaid fence parsing
- Internal link and snippet import validation
- VitePress production build

If you need full Rust example verification, run `pnpm verify:full` or `cargo test --workspace` locally.
