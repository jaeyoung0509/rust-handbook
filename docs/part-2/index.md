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

## 이 파트를 지나면 할 수 있어야 하는 일

- 함수 인자와 반환 타입을 설계할 때 borrowed view와 owned value를 구분할 수 있다.
- 불필요한 `clone`, 과한 `&mut`, 애매한 ownership 이전 같은 냄새를 코드에서 찾을 수 있다.
- 컬렉션과 문자열 API를 만들 때 allocation, aliasing, mutation scope를 함께 판단할 수 있다.
- `split_at_mut` 같은 API가 왜 필요한지 설명하고, clone 대신 안전한 borrow splitting을 고를 수 있다.
- enum과 `match`로 상태 공간을 닫아 두는 것이 왜 유지보수와 리뷰에 유리한지 설명할 수 있다.

## 실무에서 반복해서 답해야 하는 질문

- 이 함수는 데이터를 소비하는가, 잠깐 읽는가, 직접 수정하는가
- borrow checker가 막는 이유가 일시적인 문법 문제인가, 데이터 흐름 설계 문제인가
- 지금 clone은 의도적인 경계 설정인가, 아니면 설계 미루기인가

## 이 파트가 깊게 들어갈 주제

- `String`/`&str`, `Vec<T>`/`&[T]`, iterator borrowing을 한 흐름으로 읽기
- mutation scope를 줄여 side effect를 명확하게 드러내는 API 설계
- disjoint mutable access를 `split_*` 계열로 증명하는 법
- 값의 상태를 enum으로 닫아서 invalid state를 줄이는 패턴

## 파일럿과 확장 로드맵

<PartRoadmap part-id="ownership-fundamentals" />

## 추가 챕터

- [String과 &str, slice를 함께 읽기](/part-2/strings-and-slices)
- [Enum과 match로 상태를 닫아두기](/part-2/enums-and-matching)
