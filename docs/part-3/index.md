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
- `Option`, `Result`, `?`는 어떤 실패와 부재를 표현하는가

## 이 파트를 지나면 할 수 있어야 하는 일

- generic을 도입할지, concrete type으로 남길지, trait object로 지울지를 더 근거 있게 결정할 수 있다.
- trait bound를 보고 이 API가 요구하는 capability를 설명할 수 있다.
- `Option`, `Result`, `?`, custom error를 섞을 때 호출자 계약이 어떻게 달라지는지 설계할 수 있다.
- 공개 API ergonomics와 내부 구현 자유도의 균형을 더 의식적으로 잡을 수 있다.

## 실무에서 반복해서 답해야 하는 질문

- 이 추상화는 재사용을 위한 것인가, 아니면 진짜 capability contract를 드러내기 위한 것인가
- `AsRef`, `Into`, `From`, `Display`, `Error` 중 어떤 trait가 호출자 경험을 가장 정직하게 만드는가
- generic이 늘어날수록 API가 좋아지는가, 아니면 경계를 흐리게 만드는가

## 이 파트가 깊게 들어갈 주제

- standard trait를 "문법 편의"가 아니라 library contract로 읽기
- 에러 전파와 conversion을 통해 call graph를 단순하게 유지하는 법
- module visibility와 type surface를 이용해 변경 비용을 줄이는 법

## 파일럿 챕터

- [Trait와 Generic](/part-3/traits)

<PartRoadmap part-id="api-design" />

## Option, Result, and Question Mark

trait가 capability의 계약이라면, `Option`과 `Result`는 제어 흐름의 계약이다. Rust는 "없음"과 "실패"를 같은 값으로 뭉개지 않고 타입으로 분리한다. 그래서 호출자는 `None`과 `Err`를 보고 다른 대응을 할 수 있고, 구현자는 `?`를 써서 실패를 위로 자연스럽게 전달할 수 있다.

## 문제 제기

실무에서 "값이 없을 수 있음"과 "복구 가능한 실패"는 전혀 다른 의미다. Python에서는 `None`과 exception이 섞여 보일 수 있고, Go에서는 `value, ok`와 `value, err` 패턴이 같은 함수 안에서 공존한다. Rust는 여기에 이름을 붙여 `Option`과 `Result`로 분리한다.

## 왜 필요한가

```mermaid
flowchart LR
    A["input string"] --> B["parse_note_line"]
    B --> C["Option: missing data"]
    B --> D["Result: recoverable failure"]
    C --> E["caller handles absence"]
    D --> F["? propagates error"]
```

`Option`은 "없음"을, `Result`는 "실패"를 표현한다. `?`는 이 둘을 호출자에게 넘기는 문법이 아니라, 함수의 에러 계약을 끝까지 지켜주는 전달 장치다.

## Python · Go · Rust 비교

::: code-group
<<< @/snippets/python/error_contracts.py#option-result-flow [Python]
<<< @/snippets/go/error_contracts.go#option-result-flow [Go]
<<< ../../examples/api-contracts/src/lib.rs#find-note [Rust]
:::

Python은 `None`과 exception으로, Go는 `ok`와 `error`로 같은 문제를 다룬다. Rust는 `Option`과 `Result`로 그 차이를 타입으로 분명히 남긴다.

## Runnable example

먼저 absence와 parse failure를 분리하는 핵심 함수들을 본다.

<<< ../../examples/api-contracts/src/lib.rs#find-note [Rust]

<<< ../../examples/api-contracts/src/lib.rs#parse-note-line [Rust]

둘을 합쳐서 `?`로 위임하는 상위 함수는 에러 계약을 더 분명하게 만든다.

<<< ../../examples/api-contracts/src/lib.rs#preview-note-line [Rust]

문서 전체에서 실제로 실행할 수 있는 예제는 다음 binary다.

<<< ../../examples/api-contracts/examples/error_contracts.rs#error-main [Rust]

## Custom error layering

`NoteParseError`는 "입력 문자열이 왜 깨졌는가"를 설명하는 low-level error다. `CatalogError`는 여기에 도메인 경계를 더한 error다. 둘을 분리하면 parser는 재사용하기 쉬워지고, caller는 도메인 수준에서 실패를 다룰 수 있다.

```mermaid
flowchart TD
    A["raw input"] --> B["NoteParseError"]
    B --> C["CatalogError::Parse { input, source }"]
    C --> D["Display shows caller-facing context"]
    C --> E["Error::source keeps cause chain"]
```

핵심은 두 가지다.

- `Display`는 호출자에게 보여 줄 문장이다.
- `source()`는 디버깅과 logging을 위한 cause chain이다.

이 구조는 `?`와 `map_err`를 같이 쓰면 가장 자연스럽다. `?`는 이미 있는 error type을 위로 전달하고, `map_err`는 context가 필요한 boundary에서 new shape를 만든다.

<<< ../../examples/api-contracts/src/lib.rs#catalog-error [Rust]

<<< ../../examples/api-contracts/src/lib.rs#catalog-parse-note-line [Rust]

<<< ../../examples/api-contracts/src/lib.rs#preview-note-line [Rust]

## Error conversion

`From`은 단순한 편의 문법이 아니라, "이 lower-level failure를 이 higher-level failure로 올려도 된다"는 계약이다. 이 책의 예제에서는 tuple conversion을 써서, input context와 parser failure를 함께 domain error로 올린다.

<<< ../../examples/api-contracts/src/lib.rs#catalog-parse-note-line [Rust]

이 패턴을 남발하면 error type이 과도하게 커진다. 하지만 boundary에서 context가 진짜 필요할 때는, 이 정도의 명시성이 오히려 reviewer에게 더 정직하다.

## Compiler clinic

`?`는 아무 Result에나 붙는 만능 문법이 아니다. 바깥 함수의 반환 타입이 `Result` 여야 하고, 안쪽 error를 바깥 error로 바꿀 수 있어야 한다. 이 연결은 보통 `From`을 통해 이뤄진다.

::: warning 흔한 오해
`Option`과 `Result`를 둘 다 `unwrap`으로 풀어버리면 타입이 의도한 정보를 다시 잃는다. 그런 경우에는 Rust를 쓴 보람이 사라진다.
:::

## 언제 쓰는가 / 피해야 하는가

- `Option`: 값이 없을 수 있는 상황이 자연스러울 때
- `Result`: 실패 이유를 호출자에게 보여줘야 할 때
- `?`: 에러 전달이 함수의 기본 제어 흐름일 때
- `panic`/`unwrap`: 불가능한 상태를 즉시 깨뜨리는 목적이 아닐 때는 피하는 편이 낫다
- `map_err`: context를 붙일 수 있는 boundary에서만 쓰고, 내부 로직 전반에 뿌리지는 않는다

## Takeaway

- `Option`은 부재, `Result`는 실패다.
- `?`는 에러를 숨기는 게 아니라 계약을 위로 전달하는 문법이다.
- trait 챕터에서 본 `From`, `Display`, `Error`는 여기서 실제 에러 계약으로 이어진다.
