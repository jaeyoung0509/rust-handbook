# Rust Handbook

Ebook: [https://jaeyoung0509.github.io/rust-handbook/](https://jaeyoung0509.github.io/rust-handbook/)

Rust Handbook is a VitePress-based ebook for developers who already know Python or Go and want a real Rust deep dive. It is not a syntax tour. The goal is to build the mental models, API design instincts, debugging habits, and async/runtime judgment that strong Rust engineers rely on in practice.

The repository combines the documentation site, runnable Rust examples, compile-fail references, Markdown and Mermaid validation, and GitHub Pages deployment.

## Principles

- The handbook explains why Rust introduces constraints before it explains syntax.
- Ownership, lifetimes, traits, errors, and async are taught as tools for designing relationships and contracts, not as rules to memorize.
- Each concept is expected to go beyond surface syntax into tradeoffs, bug classes, performance implications, and review heuristics.
- Rust code shown in the docs lives in a separate Cargo workspace so examples can be compiled and tested locally.
- CI focuses on documentation quality first. Full Rust example validation is available as a local-only workflow.
- Mermaid blocks, snippet imports, and internal links are validated automatically to keep the book coherent as it grows.

## Deep-Dive Scope

The handbook is being written to answer questions such as:

- Why does this Rust constraint exist, and what bug or cost does it prevent?
- When should an API borrow, own, clone, stream, or move work across task boundaries?
- What should a reviewer look for in ownership-heavy, trait-heavy, or async-heavy code?
- How do compile-time contracts change runtime behavior, memory pressure, and operational safety?

That does not mean the book can magically replace years of production experience. It does mean the content is being shaped to compress the path toward senior-level Rust judgment instead of stopping at beginner syntax explanations.

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

Current pilot chapters are being expanded so they also answer deeper questions:

- What design smell usually causes this problem?
- Which tradeoff is being made between ergonomics, allocation, and control?
- What would an experienced reviewer question in this code?
- When should the design be changed instead of fighting the compiler harder?

## Quality Gates

- Markdown lint
- Frontmatter shape validation
- Mermaid fence parsing
- Internal link and snippet import validation
- VitePress production build

If you need full Rust example verification, run `pnpm verify:full` or `cargo test --workspace` locally.
