---
title: Part 2. Ownership-backed fundamentals
description: move, borrow, String/&str, slice, enum/match를 복사 비용과 aliasing 관점으로 정리한다
part: 2
order: 0
prerequisites:
  - Cargo basics
---

이 파트는 Rust 초심자가 가장 많이 막히는 이유를 정면으로 다룬다. ownership은 문법 규칙이 아니라 "이 값에 대한 수정 권한과 정리 책임을 누가 가지는가"를 분명히 하는 계약이다.

## 핵심 질문

- 왜 Rust는 기본값으로 move를 택할까
- 언제 `&T`와 `&mut T`를 써야 할까
- `String`과 `&str`, `Vec<T>`와 `&[T]`는 어떤 API 신호를 주는가

## 파일럿과 확장 로드맵

<PartRoadmap part-id="ownership-fundamentals" />

## 추가 챕터

- [String과 &str, slice를 함께 읽기](/part-2/strings-and-slices)
- [Enum과 match로 상태를 닫아두기](/part-2/enums-and-matching)
