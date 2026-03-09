---
layout: home
title: Rust Handbook
description: Python/Go 개발자를 위한 why-first Rust deep dive handbook
hero:
  name: Rust Handbook
  text: Python/Go 개발자를 위한 Rust deep dive ebook
  tagline: 문법을 외우는 대신 ownership, lifetime, trait, Tokio가 왜 필요한지부터 이해하는 handbook
  actions:
    - theme: brand
      text: 파일럿 챕터 읽기
      link: /part-2/ownership
    - theme: alt
      text: 전체 로드맵 보기
      link: /part-1/
features:
  - title: Why-first curriculum
    details: Python/Go에서 익숙한 문제를 먼저 던지고 Rust가 어떤 제약으로 해결하는지 설명한다.
  - title: Runnable examples
    details: 문서에 등장하는 Rust 코드는 별도 Cargo workspace에서 실제로 컴파일하고 테스트한다.
  - title: CI for docs
    details: Mermaid, frontmatter, snippet include, 링크, VitePress build를 한 파이프라인에서 검증한다.
---

## 누구를 위한 책인가

- Python/Go로 이미 도구나 서비스를 만든 적이 있지만 Rust의 ownership, lifetime, async에서 자주 막히는 개발자
- "borrow checker가 짜증난다"는 감각을 넘어 "이 제약이 어떤 사고를 막는가"까지 이해하고 싶은 개발자
- 문서와 예제가 금방 어긋나는 튜토리얼 대신, 실제로 실행되는 코드와 함께 학습하고 싶은 개발자

## 이 책이 다루는 방식

```mermaid
flowchart LR
    A["Python/Go 실전 문제"] --> B["Rust가 막으려는 버그와 비용"]
    B --> C["Mental model 다이어그램"]
    C --> D["Python · Go · Rust 비교 코드"]
    D --> E["Runnable example"]
    E --> F["Compiler clinic + takeaway"]
```

::: tip 학습 원칙
이 handbook은 Rust를 "모든 곳에서 clone 하지 않기 위한 언어"가 아니라 "메모리와 동시성 관계를 타입으로 설계하는 언어"로 설명한다.
:::

## 파일럿 챕터

- [Ownership 입문](/part-2/ownership)
- [Lifetime 심화](/part-4/lifetimes)
- [Tokio 입문](/part-5/tokio)

## 전체 로드맵

<BookRoadmap />
