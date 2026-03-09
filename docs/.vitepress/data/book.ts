export type ChapterStatus = "pilot" | "planned";

export interface ChapterMeta {
  id: string;
  title: string;
  description: string;
  route: string;
  prerequisites: string[];
  status: ChapterStatus;
}

export interface PartMeta {
  id: string;
  order: number;
  title: string;
  description: string;
  overview: string;
  accent: string;
  chapters: ChapterMeta[];
  plannedTopics: string[];
}

export const book = {
  title: "Rust Handbook",
  description:
    "Python/Go 개발자가 Rust를 왜 배우고 언제 써야 하는지까지 이해하도록 돕는 deep dive handbook",
  repository: "https://github.com/jaeyoung0509/rust-handbook",
  homepage: "https://jaeyoung0509.github.io/rust-handbook/",
};

export const parts: PartMeta[] = [
  {
    id: "mindset-shift",
    order: 1,
    title: "Part 1. Rust mindset shift",
    description:
      "GC 언어와 Rust의 메모리·동시성 모델 차이를 먼저 맞추고, Cargo와 compiler diagnostics를 읽는 감각을 잡는다.",
    overview: "/part-1/",
    accent: "teal",
    chapters: [
      {
        id: "why-rust-feels-strict",
        title: "왜 Rust는 이렇게 빡빡하게 느껴질까",
        description:
          "runtime 비용을 compile-time contract로 옮긴다는 관점에서 Rust의 엄격함을 설명한다.",
        route: "/part-1/strictness",
        prerequisites: ["Python 또는 Go 실무 경험"],
        status: "pilot",
      },
    ],
    plannedTopics: [
      "왜 Rust가 Python/Go보다 더 많은 제약을 두는가",
      "stack, heap, ownership을 보는 기본 프레임",
      "Cargo workflow와 compiler clinic 읽는 법",
    ],
  },
  {
    id: "ownership-fundamentals",
    order: 2,
    title: "Part 2. Ownership-backed fundamentals",
    description:
      "move, borrow, slice, String/&str, Vec, enum, match를 '복사 비용과 aliasing 제어' 관점으로 배운다.",
    overview: "/part-2/",
    accent: "amber",
    chapters: [
      {
        id: "ownership-intro",
        title: "Ownership 입문: 복사를 줄이고 관계를 드러내기",
        description:
          "borrow가 왜 필요한지, 언제 clone보다 좋은지, mutable borrow가 API 설계에 어떤 신호를 주는지 설명한다.",
        route: "/part-2/ownership",
        prerequisites: ["Cargo basics", "함수와 컬렉션 기본 문법"],
        status: "pilot",
      },
    ],
    plannedTopics: [
      "`String`과 `&str`의 경계",
      "`Vec`, slice, pattern matching으로 데이터 읽기",
      "enum과 `match`로 상태를 닫아두기",
    ],
  },
  {
    id: "api-design",
    order: 3,
    title: "Part 3. API design and abstraction",
    description:
      "struct, module, impl, trait, generics, `Option`/`Result`, `?`, custom error를 API 계약 관점으로 다룬다.",
    overview: "/part-3/",
    accent: "slate",
    chapters: [
      {
        id: "traits-as-contracts",
        title: "Trait와 Generic: capability contract로 읽기",
        description:
          "trait, impl, generic bound를 재사용 문법이 아니라 API 계약으로 이해하도록 돕는다.",
        route: "/part-3/traits",
        prerequisites: ["ownership", "borrowing", "기본 함수 시그니처 읽기"],
        status: "pilot",
      },
    ],
    plannedTopics: [
      "trait가 인터페이스가 아니라 capability contract인 이유",
      "`From`/`Into`, `AsRef`, `Default`로 ergonomics 높이기",
      "에러 타입 설계와 `?` 연산자 읽기",
    ],
  },
  {
    id: "memory-semantics",
    order: 4,
    title: "Part 4. Memory and advanced semantics",
    description:
      "lifetimes, smart pointers, `Rc`/`Arc`, `RefCell`, interior mutability, iterator, closure를 관계 모델로 정리한다.",
    overview: "/part-4/",
    accent: "rose",
    chapters: [
      {
        id: "lifetime-deep-dive",
        title: "Lifetime 심화: 값을 연장하는 문법이 아니라 관계를 서술하는 표기",
        description:
          "함수 시그니처, struct, iterator가 서로 다른 lifetime 문제를 어떻게 드러내는지 단계적으로 다룬다.",
        route: "/part-4/lifetimes",
        prerequisites: ["ownership", "borrowing", "reference semantics"],
        status: "pilot",
      },
    ],
    plannedTopics: [
      "`Rc`/`Arc`와 shared ownership",
      "`RefCell`과 runtime borrow checking",
      "iterator와 closure가 borrow 관계를 전파하는 방식",
    ],
  },
  {
    id: "concurrency-async",
    order: 5,
    title: "Part 5. Concurrency and async",
    description:
      "thread/channel, `Send`/`Sync`, `Future`, pinning mental model, Tokio runtime과 task orchestration을 다룬다.",
    overview: "/part-5/",
    accent: "cyan",
    chapters: [
      {
        id: "tokio-intro",
        title: "Tokio 입문: runtime, task, `Send` 제약을 실전 감각으로 이해하기",
        description:
          "goroutine이나 Python coroutine과 다른 점, task spawn과 channel, `select!` 조합을 Rust trait 관점으로 설명한다.",
        route: "/part-5/tokio",
        prerequisites: ["ownership", "trait basics", "thread와 channel 개념"],
        status: "pilot",
      },
    ],
    plannedTopics: [
      "`Future`와 pinning을 mental model로 이해하기",
      "cancellation, timeout, graceful shutdown",
      "async I/O trait와 stream-like 패턴",
    ],
  },
];

export const documentedRoutes = [
  "/",
  ...parts.flatMap((part) => [
    part.overview,
    ...part.chapters
      .filter((chapter) => chapter.status === "pilot")
      .map((chapter) => chapter.route),
  ]),
];

export function buildSidebar() {
  return parts.map((part) => ({
    text: part.title,
    collapsed: false,
    items: [
      { text: "Overview", link: part.overview },
      ...part.chapters
        .filter((chapter) => chapter.status === "pilot")
        .map((chapter) => ({
          text: chapter.title,
          link: chapter.route,
        })),
    ],
  }));
}

export function findPart(partId: string) {
  return parts.find((part) => part.id === partId);
}

export function routeToSourcePath(route: string) {
  if (route === "/") {
    return "index.md";
  }

  const normalized = route.replace(/^\/|\/$/g, "");
  if (!normalized) {
    return "index.md";
  }

  if (!normalized.includes("/")) {
    return `${normalized}/index.md`;
  }

  return `${normalized}.md`;
}
