---
title: String과 &str, slice를 함께 읽기
description: String/&str와 slice를 ownership과 view의 차이로 구분해서 이해한다
part: 2
order: 2
prerequisites:
  - ownership
  - borrowing
---

Rust에서 `String`과 `&str`, `Vec<T>`와 `&[T]`는 단순한 타입 쌍이 아니다. 소유권을 넘길 것인지, 읽기용 view만 줄 것인지, API 의도를 어떻게 드러낼 것인지의 문제다.

## 문제 제기

Python이나 Go에서는 문자열과 리스트를 넘길 때 대체로 "읽기 전용처럼 쓰는가"만 생각해도 된다. Rust는 여기에 ownership 경계를 추가해서, 읽기와 소유를 분명히 나눈다.

## 왜 필요한가

```mermaid
flowchart LR
    A["String owned buffer"] --> B["&str borrowed view"]
    C["Vec<T> owned buffer"] --> D["&[T] borrowed slice"]
    B --> E["읽기만 하는 함수"]
    D --> E
```

`String`과 `Vec<T>`는 데이터를 소유한다. `&str`과 `&[T]`는 이미 있는 데이터를 빌려 읽는다. 이 차이를 구분하면 clone을 덜 쓰고, API도 덜 헷갈린다.

## Python · Go · Rust 비교

::: code-group
<<< @/snippets/python/string_slice.py#string-slice-compare [Python]
<<< @/snippets/go/string_slice.go#string-slice-compare [Go]
<<< ../../examples/ownership-playbook/src/lib.rs#normalize-username [Rust]
:::

Python과 Go에서도 같은 의도를 표현할 수 있지만, Rust는 borrowed view를 타입으로 드러내서 함수를 더 정확히 읽게 한다.

## Runnable example

문자열 입력은 `&str`로 받고, 내부에서 필요한 경우만 `String`으로 바꾼다.

<<< ../../examples/ownership-playbook/src/lib.rs#normalize-username [Rust]

슬라이스는 전체 벡터를 복사하지 않고도 패턴 매칭으로 읽을 수 있다.

<<< ../../examples/ownership-playbook/src/lib.rs#describe-score-window [Rust]

이 흐름을 하나의 실행 예제로 보면 더 직관적이다.

<<< ../../examples/ownership-playbook/examples/string_and_slice.rs#string-slice-main [Rust]

## Compiler clinic

`String`을 받는 함수는 호출자에게 ownership을 넘기라고 요구한다. 읽기만 한다면 `&str`이 더 정확한 계약이다.

## 언제 쓰는가 / 피해야 하는가

- `&str`: 입력을 읽기만 하고 ownership은 유지시키고 싶을 때
- `String`: 새 문자열을 만들어 반환하거나 소유권이 필요할 때
- `&[T]`: 벡터 전체를 소모하지 않고 읽기만 할 때
- `Vec<T>`: 호출자에게 ownership을 넘겨야 할 때

## Takeaway

- `String`과 `&str`은 소유와 view의 차이다.
- `Vec<T>`와 `&[T]`도 같은 구조다.
- 타입을 보면 함수가 무엇을 빌려 읽고 무엇을 소유하는지 보인다.
