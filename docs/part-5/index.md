---
title: Part 5. Concurrency and async
description: thread, channel, Send/Sync, Future, Tokio runtime을 실전 감각으로 정리한다
part: 5
order: 0
prerequisites:
  - ownership
  - trait basics
---

Rust의 async는 Python의 coroutine이나 Go의 goroutine을 그대로 옮겨온 것이 아니다. `Future`, `Send`, `Sync`, pinning이 얽혀 있기 때문에, 런타임과 trait 제약을 같이 이해해야 실전에서 덜 막힌다.

## 이 파트를 지나면 할 수 있어야 하는 일

- spawned task에 어떤 상태를 넘겨도 되는지 `Send`/`Sync` 관점으로 설명할 수 있다.
- channel, task, lock 중 어떤 경계가 가장 덜 위험한지 상황에 맞게 고를 수 있다.
- cancellation, timeout, graceful shutdown을 ad-hoc if문이 아니라 구조로 설계할 수 있다.
- async 코드에서 blocking, contention, backpressure 냄새를 빨리 찾을 수 있다.
- `Arc<Mutex<T>>`가 필요한 경우와 피해야 하는 경우를 코드 리뷰에서 구분할 수 있다.
- bounded parallelism과 first-response-wins 패턴을 runtime 제어 관점에서 설명할 수 있다.

## 실무에서 반복해서 답해야 하는 질문

- 이 작업은 task로 분리할 가치가 있는가, 아니면 순차 코드가 더 단순한가
- `Arc<Mutex<T>>`가 필요한가, 아니면 ownership transfer나 channel이 더 나은가
- 이 future는 어디에서 poll되고, 어디까지 이동할 수 있으며, 누가 cancellation을 책임지는가
- 이 시스템은 입력이 많아질 때 자연스럽게 느려지는가, 아니면 무제한으로 쌓여서 터지는가

## 이 파트가 깊게 들어갈 주제

- `Future`와 pinning을 문법이 아니라 실행 모델로 이해하기
- `JoinSet`, channel, `select!`를 통해 fan-out, cancellation, race를 설계하는 법
- Tokio runtime 위에서 blocking I/O, CPU 작업, shared state를 분리하는 법
- capacity 제어를 위한 `Semaphore`와 queue 설계
- `Arc<Mutex<T>>`를 쓰기 전에 상태를 분할하거나 ownership을 넘기는 대안들

## 파일럿과 확장 로드맵

<PartRoadmap part-id="concurrency-async" />
