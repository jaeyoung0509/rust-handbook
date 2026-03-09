<script setup lang="ts">
import { computed } from "vue";
import { withBase } from "vitepress";
import { findPart } from "../../data/book";

const props = defineProps<{
  partId: string;
}>();

const part = computed(() => findPart(props.partId));
</script>

<template>
  <section v-if="part" class="book-shell">
    <div class="book-grid">
      <article
        v-for="chapter in part.chapters"
        :key="chapter.id"
        class="book-card"
      >
        <p class="book-card__eyebrow">Pilot chapter</p>
        <h4>{{ chapter.title }}</h4>
        <p>{{ chapter.description }}</p>
        <p>
          <a class="book-card__link" :href="withBase(chapter.route)">챕터 읽기</a>
        </p>
        <ul>
          <li v-for="item in chapter.prerequisites" :key="item">{{ item }}</li>
        </ul>
        <span class="book-card__pill" data-status="pilot">Pilot</span>
      </article>

      <article
        v-for="topic in part.plannedTopics"
        :key="topic"
        class="book-card"
      >
        <p class="book-card__eyebrow">Planned</p>
        <h4>{{ topic }}</h4>
        <p>파일럿 템플릿이 안정화되면 이 주제로 확장한다.</p>
        <span class="book-card__pill" data-status="planned">Planned</span>
      </article>
    </div>
  </section>
</template>
