---
title: Part 3. API design and abstraction
description: trait, generics, Option/Result, custom error를 API 계약 관점으로 정리한다
part: 3
order: 0
prerequisites:
  - ownership
---

trait와 generic은 "코드를 재사용하는 문법"이 아니라 capability를 계약으로 고정하는 도구다. 이 파트에서는 어떤 trait가 Rust 생태계의 기본 어휘인지와, `Result`/`?`가 제어 흐름을 어떻게 단순화하는지 확장할 예정이다.

## 핵심 질문

- trait는 언제 인터페이스보다 계약으로 읽는 편이 더 정확할까
- generic 함수에서 정말 중요한 건 타입 변수인가, bound인가
- `Debug`, `Display`, `From`, `AsRef`는 어떤 API ergonomics를 만들어 주는가

## 파일럿 챕터

- [Trait와 Generic](/part-3/traits)

<PartRoadmap part-id="api-design" />
