<script setup lang="ts">
import mermaid from "mermaid/dist/mermaid.core.mjs";
import { computed, onMounted, ref, watch } from "vue";

const props = defineProps<{
  code: string;
}>();

const svg = ref("");
const error = ref("");
const decodedCode = computed(() => decodeURIComponent(props.code));

function hash(value: string) {
  let result = 0;

  for (const character of value) {
    result = (result * 31 + character.charCodeAt(0)) >>> 0;
  }

  return result.toString(16);
}

async function renderDiagram() {
  try {
    error.value = "";
    mermaid.initialize({
      startOnLoad: false,
      securityLevel: "strict",
      theme: "neutral",
    });

    const rendered = await mermaid.render(
      `mermaid-${hash(decodedCode.value)}`,
      decodedCode.value,
    );

    svg.value = rendered.svg;
  } catch (err) {
    svg.value = "";
    error.value = err instanceof Error ? err.message : String(err);
  }
}

onMounted(() => {
  void renderDiagram();
});

watch(decodedCode, () => {
  void renderDiagram();
});
</script>

<template>
  <div class="mermaid-shell">
    <div v-if="error" class="mermaid-shell__error">
      Mermaid rendering failed: {{ error }}
    </div>
    <div v-else class="mermaid-shell__canvas" v-html="svg" />
  </div>
</template>
