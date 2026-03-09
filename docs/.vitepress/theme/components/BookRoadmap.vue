<script setup lang="ts">
import { withBase } from "vitepress";
import { parts } from "../../data/book";
</script>

<template>
  <section class="book-shell">
    <div class="book-grid">
      <article v-for="part in parts" :key="part.id" class="book-card">
        <p class="book-card__eyebrow">Part {{ part.order }}</p>
        <h3>{{ part.title }}</h3>
        <p>{{ part.description }}</p>
        <p>
          <a class="book-card__link" :href="withBase(part.overview)">Overview 열기</a>
        </p>
        <ul>
          <li v-for="chapter in part.chapters" :key="chapter.id">
            <a :href="withBase(chapter.route)">{{ chapter.title }}</a>
          </li>
          <li v-if="part.chapters.length === 0">파일럿 챕터는 다음 단계에서 추가된다.</li>
        </ul>
        <span
          class="book-card__pill"
          :data-status="part.chapters.length > 0 ? 'pilot' : 'planned'"
        >
          {{ part.chapters.length > 0 ? "Pilot available" : "Roadmap only" }}
        </span>
      </article>
    </div>
  </section>
</template>
