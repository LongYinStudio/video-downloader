import { createRouter, createWebHistory } from "vue-router";
import Home from "../components/HomeView.vue";
import Settings from "../components/SettingsView.vue";
import About from "../components/AboutView.vue";

const routes = [
  { path: "/", component: Home },
  { path: "/settings", component: Settings },
  { path: "/about", component: About },
];

const router = createRouter({
  history: createWebHistory(), // 使用 HTML5 History 模式
  routes,
});

export default router;
