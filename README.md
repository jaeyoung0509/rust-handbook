# Rust Handbook

Python/Go에 익숙한 개발자가 Rust를 깊게 이해하도록 설계한 VitePress 기반 handbook이다. 이 저장소는 문서 사이트, 실행 가능한 Rust 예제, compile-fail 테스트, Mermaid/Markdown 검증, GitHub Pages 배포를 한 번에 관리한다.

## Principles

- 문법을 나열하지 않는다. Rust가 어떤 버그와 비용을 줄이려고 이런 제약을 두는지 먼저 설명한다.
- lifetime, ownership, trait, async를 "외워서 쓰는 문법"이 아니라 "관계와 제약을 설계하는 언어"로 이해하게 만든다.
- 문서에 보이는 Rust 코드는 모두 별도 Cargo workspace에서 실제로 컴파일하거나 테스트한다.
- Mermaid, snippet include, 내부 링크는 CI에서 자동 검증해서 문서 품질 저하를 빠르게 잡는다.

## Layout

- `docs/`: VitePress handbook
- `docs/.vitepress/`: 사이트 설정, 테마 확장, 책 메타데이터
- `examples/`: 문서에 포함되는 Rust 예제와 compile-fail 테스트
- `scripts/`: frontmatter, Mermaid, 내부 링크와 snippet 검증 스크립트
- `.github/workflows/`: CI와 GitHub Pages 배포

## Commands

```bash
pnpm install
pnpm docs:dev
pnpm verify
```

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Writing Model

각 챕터는 다음 흐름을 따른다.

1. 문제 제기: Python/Go에서 겪는 실전 상황을 먼저 제시한다.
2. 왜 필요한가: Rust 제약이 방지하는 버그와 비용을 설명한다.
3. Mental model: Mermaid 다이어그램으로 메모리/태스크/소유권 흐름을 시각화한다.
4. 비교 설명: Python/Go/Rust code-group으로 감각을 맞춘다.
5. Runnable example: 실제 Cargo workspace 코드 조각을 문서에 포함한다.
6. Compiler clinic: 자주 보는 에러와 읽는 법을 정리한다.
7. 언제 쓰는가 / 피해야 하는가 / takeaway까지 정리한다.

## Quality Gates

- Markdown lint
- frontmatter shape validation
- Mermaid fence parsing
- 내부 링크와 snippet include 경로 확인
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test --workspace`
- VitePress production build
