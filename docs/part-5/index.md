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

## 파일럿과 확장 로드맵

<PartRoadmap part-id="concurrency-async" />
