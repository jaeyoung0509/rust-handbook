import DefaultTheme from "vitepress/theme";
import type { Theme } from "vitepress";
import MermaidDiagram from "./components/MermaidDiagram.vue";
import BookRoadmap from "./components/BookRoadmap.vue";
import PartRoadmap from "./components/PartRoadmap.vue";
import "./custom.css";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("MermaidDiagram", MermaidDiagram);
    app.component("BookRoadmap", BookRoadmap);
    app.component("PartRoadmap", PartRoadmap);
  },
} satisfies Theme;
