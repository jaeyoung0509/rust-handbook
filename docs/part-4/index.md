---
title: Part 4. Memory and advanced semantics
description: lifetime, smart pointer, iterator, closure를 관계 모델로 정리한다
part: 4
order: 0
prerequisites:
  - ownership
  - borrowing
---

lifetime는 "이 값을 얼마나 오래 살릴까"를 적는 문법이 아니다. 여러 reference 사이의 관계를 compiler가 추론하지 못할 때, 그 관계를 시그니처에 적어주는 표기다.

## 이 파트를 지나면 할 수 있어야 하는 일

- lifetime 에러를 볼 때 annotation만 추가할지, ownership 구조를 바꿀지 판단할 수 있다.
- `Rc`, `Arc`, `RefCell` 같은 도구를 shared ownership과 mutation 제약 관점으로 구분할 수 있다.
- zero-copy parser나 borrowed view API가 왜 빠르면서도 더 까다로운지 설명할 수 있다.
- iterator와 closure가 borrow 관계를 어떻게 끌고 다니는지 읽을 수 있다.
- reference를 담는 struct가 언제 유용하고 언제 유지보수 비용이 되는지 판단할 수 있다.
- "annotation을 더 붙일 문제"와 "ownership 경계를 다시 그릴 문제"를 구분할 수 있다.

## 실무에서 반복해서 답해야 하는 질문

- 이 reference를 오래 끌고 가는 게 정말 필요한가, 아니면 owned data로 바꿔도 되는가
- lifetime annotation 부족 문제인가, 데이터 구조가 자기 책임을 너무 적게 지는 문제인가
- shared ownership이 필요한가, 아니면 owner를 하나로 유지한 채 view만 나누면 되는가
- parser와 iterator가 자기 입력을 그대로 빌려줄 때, 호출자가 그 제약을 감당할 준비가 되어 있는가

## 이 파트가 깊게 들어갈 주제

- 함수, struct, iterator에서 lifetime가 다른 모양으로 드러나는 이유
- `Rc`/`Arc`/`RefCell`/interior mutability를 쓰는 기준과 냄새
- borrow 범위를 줄이는 리팩터링과 zero-copy의 tradeoff
- lifetime가 복잡할수록 owned boundary를 어디에 둘지 다시 설계하는 법

## 파일럿과 확장 로드맵

- [Lifetime 심화](/part-4/lifetimes)
- [Shared ownership](/part-4/shared-ownership)

<PartRoadmap part-id="memory-semantics" />
