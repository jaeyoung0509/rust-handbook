---
title: 왜 Rust는 이렇게 빡빡하게 느껴질까
description: Rust의 엄격함을 runtime 비용을 compile-time contract로 옮기는 관점에서 설명한다
part: 1
order: 1
prerequisites:
  - Python 또는 Go 실무 경험
---

Rust를 처음 접하면 "왜 이렇게까지 컴파일러가 간섭하지?"라는 감각이 먼저 든다. 하지만 Rust의 엄격함은 취향 문제가 아니라, runtime에서 뒤늦게 드러나는 비용과 버그를 compile time으로 끌어오는 전략에 가깝다.

## 문제 제기

Python이나 Go에서는 기능을 빠르게 붙이는 동안 ownership, aliasing, mutation scope를 깊게 드러내지 않아도 되는 경우가 많다. 대신 그 비용은 GC pressure, shared mutable state, hidden copies, 또는 디버깅 복잡도로 나중에 다시 나타난다.

## 왜 필요한가

```mermaid
flowchart LR
    A["runtime convenience"] --> B["hidden copies / aliasing / shared state"]
    B --> C["debugging and latency cost later"]
    D["Rust compile-time contract"] --> E["ownership / borrowing / trait bounds"]
    E --> F["more explicit APIs now"]
    F --> G["fewer surprises later"]
```

Rust가 strict하게 느껴지는 건 "자유를 빼앗기 때문"이 아니라, 시스템 비용을 더 일찍 보게 만들기 때문이다.

## Python · Go · Rust 비교

::: code-group
<<< @/snippets/python/runtime_tradeoff.py#hidden-runtime-cost [Python]
<<< @/snippets/go/runtime_tradeoff.go#hidden-runtime-cost [Go]
<<< ../../examples/ownership-playbook/src/lib.rs#promote-title [Rust]
:::

Python과 Go에서도 같은 의도를 표현할 수는 있다. 하지만 Rust는 "이 함수가 ownership을 가져가는가, 빌려 쓰는가"를 더 강하게 시그니처에 남긴다.

## Runtime cost를 compile-time contract로 옮겨 읽기

### 1. Stack vs heap

중요한 건 단순한 속도 비교가 아니다. 값이 어디에 놓이는가보다 "누가 이 값을 소유하고 언제 정리하는가"를 먼저 봐야 한다.

### 2. Ownership

ownership은 메모리 최적화 기법이 아니라 정리 책임과 mutation 권한을 한눈에 드러내는 계약이다.

### 3. Compiler diagnostics

컴파일 에러는 막연한 실패 메시지가 아니라 "지금 API 설계가 어떤 관계를 숨기고 있는가"를 알려주는 피드백이다.

## Runnable example

기존 ownership 예제만 봐도 Rust가 왜 strict하게 보이는지 감이 온다. `&str` 입력은 ownership을 빼앗지 않고, `String` 반환은 새 값을 만든다는 사실을 시그니처에서 바로 보여 준다.

<<< ../../examples/ownership-playbook/src/lib.rs#promote-title [Rust]

읽기 전용 데이터 접근도 같은 식이다. slice를 받는 함수는 값 전체를 복사하지 않고도 필요한 계산을 수행한다.

<<< ../../examples/ownership-playbook/src/lib.rs#borrowed-slice [Rust]

## Compiler clinic

strict함이 가장 선명하게 보이는 지점은 move 이후 재사용을 막을 때다.

<<< ../../examples/ui-harness/tests/ui/use_after_move.rs#use-after-move [Rust]

이 메시지는 "문법이 틀렸다"보다 "ownership 경계를 다시 생각하라"는 피드백에 가깝다.

::: tip 읽는 법
컴파일러 에러를 볼 때는 먼저 "이 값의 owner가 누구였나", "여기서 ownership 이전이 정말 필요했나"를 질문하면 된다.
:::

## 언제 이 관점이 중요한가

- 큰 버퍼나 문자열을 여러 계층에서 재사용할 때
- concurrency와 async에서 shared state를 다룰 때
- API를 만들며 "borrow로 충분한가, owned value가 필요한가"를 판단할 때

## Takeaway

- Rust의 strict함은 비용을 없애는 게 아니라 비용이 보이는 시점을 앞당긴다.
- ownership과 borrowing은 컴파일러를 위한 규칙이 아니라 API 계약을 명시하는 도구다.
- compiler diagnostics를 읽는 감각이 생기면 Rust는 훨씬 덜 답답해진다.
