<script setup>
import { ref } from "vue";
import { useRoute } from "vue-router";
import {
  Menu as IconMenu,
  Clock,
  Setting,
  InfoFilled,
  Fold,
  Expand,
  VideoCameraFilled,
} from "@element-plus/icons-vue";
import { STORAGE_KEYS } from "./settings.js";

const route = useRoute();
const isCollapse = ref(false);

function applyTheme(mode) {
  const root = document.documentElement;
  if (!mode || mode === "system") {
    root.removeAttribute("data-theme");
    return;
  }
  root.setAttribute("data-theme", mode);
}

applyTheme(localStorage.getItem(STORAGE_KEYS.theme) || "system");
</script>

<template>
  <main class="app-layout">
    <aside class="app-sidebar" :class="{ 'is-collapsed': isCollapse }">
      <div class="sidebar-brand">
        <div class="brand-logo-wrap">
          <img src="../src-tauri/icons/logo.png" alt="logo" class="brand-logo" />
        </div>
        <div v-show="!isCollapse" class="brand-text">
          <span class="brand-name">视频下载器</span>
          <span class="brand-sub">Downloader</span>
        </div>
      </div>

      <nav class="sidebar-nav">
        <router-link
          to="/"
          class="nav-item"
          :class="{ active: route.path === '/' }"
          :title="isCollapse ? '下载首页' : ''"
        >
          <el-icon class="nav-icon"><VideoCameraFilled /></el-icon>
          <span v-show="!isCollapse" class="nav-label">下载首页</span>
        </router-link>

        <router-link
          to="/history"
          class="nav-item"
          :class="{ active: route.path === '/history' }"
          :title="isCollapse ? '下载历史' : ''"
        >
          <el-icon class="nav-icon"><Clock /></el-icon>
          <span v-show="!isCollapse" class="nav-label">下载历史</span>
        </router-link>

        <router-link
          to="/settings"
          class="nav-item"
          :class="{ active: route.path === '/settings' }"
          :title="isCollapse ? '参数设置' : ''"
        >
          <el-icon class="nav-icon"><Setting /></el-icon>
          <span v-show="!isCollapse" class="nav-label">参数设置</span>
        </router-link>

        <router-link
          to="/about"
          class="nav-item"
          :class="{ active: route.path === '/about' }"
          :title="isCollapse ? '关于软件' : ''"
        >
          <el-icon class="nav-icon"><InfoFilled /></el-icon>
          <span v-show="!isCollapse" class="nav-label">关于软件</span>
        </router-link>
      </nav>

      <div class="sidebar-footer">
        <button
          class="collapse-toggle-btn"
          :title="isCollapse ? '展开侧边栏' : '收起侧边栏'"
          @click="isCollapse = !isCollapse"
        >
          <el-icon class="toggle-icon">
            <component :is="isCollapse ? Expand : Fold" />
          </el-icon>
          <span v-show="!isCollapse" class="toggle-text">收起侧栏</span>
        </button>
      </div>
    </aside>

    <div class="app-main">
      <router-view v-slot="{ Component }">
        <transition name="fade-slide" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </div>
  </main>
</template>

<style>
/* ==========================================================================
   全新现代化设计系统变量 (Arc / Raycast 风格)
   ========================================================================== */
:root {
  --primary-color: #3b82f6;
  --primary-hover: #2563eb;
  --primary-active: #1d4ed8;
  --primary-subtle: rgba(59, 130, 246, 0.1);
  --primary-glow: rgba(59, 130, 246, 0.25);

  --success-color: #10b981;
  --success-subtle: rgba(16, 185, 129, 0.12);
  --warning-color: #f59e0b;
  --warning-subtle: rgba(245, 158, 11, 0.12);
  --danger-color: #ef4444;
  --danger-subtle: rgba(239, 68, 68, 0.12);

  /* 浅色模式 - 纯净微冷灰 */
  --bg-primary: #f8fafc;
  --bg-secondary: #ffffff;
  --bg-tertiary: #f1f5f9;
  --bg-card: #ffffff;
  --bg-input: #f8fafc;

  --border-color: #e2e8f0;
  --border-subtle: #f1f5f9;
  --border-hover: #cbd5e1;

  --text-primary: #0f172a;
  --text-secondary: #475569;
  --text-tertiary: #94a3b8;

  --shadow-sm: 0 1px 2px 0 rgba(15, 23, 42, 0.05);
  --shadow-md: 0 4px 12px -2px rgba(15, 23, 42, 0.08), 0 2px 6px -2px rgba(15, 23, 42, 0.04);
  --shadow-lg: 0 12px 24px -4px rgba(15, 23, 42, 0.1), 0 4px 8px -2px rgba(15, 23, 42, 0.04);
  --shadow-card: 0 4px 20px -2px rgba(15, 23, 42, 0.06);

  --radius-sm: 6px;
  --radius-md: 10px;
  --radius-lg: 16px;
  --radius-xl: 24px;

  /* Element Plus 适配覆盖 */
  --el-color-primary: #3b82f6;
  --el-color-primary-light-3: #60a5fa;
  --el-color-primary-light-7: #bfdbfe;
  --el-color-primary-light-9: #eff6ff;
  --el-color-primary-dark-2: #1d4ed8;
  --el-border-radius-base: 10px;
  --el-border-radius-round: 24px;
}

/* 深色模式 - 沉浸式深空蓝黑 */
@media (prefers-color-scheme: dark) {
  :root {
    --bg-primary: #090d16;
    --bg-secondary: #111827;
    --bg-tertiary: #1e293b;
    --bg-card: #111827;
    --bg-input: #0f172a;

    --border-color: #232f48;
    --border-subtle: #1a2336;
    --border-hover: #374768;

    --text-primary: #f8fafc;
    --text-secondary: #cbd5e1;
    --text-tertiary: #64748b;

    --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.4);
    --shadow-md: 0 4px 12px -2px rgba(0, 0, 0, 0.5);
    --shadow-lg: 0 12px 28px -4px rgba(0, 0, 0, 0.6);
    --shadow-card: 0 4px 24px -2px rgba(0, 0, 0, 0.45);
  }
}

/* 强制浅色 */
:root[data-theme="light"] {
  --bg-primary: #f8fafc;
  --bg-secondary: #ffffff;
  --bg-tertiary: #f1f5f9;
  --bg-card: #ffffff;
  --bg-input: #f8fafc;

  --border-color: #e2e8f0;
  --border-subtle: #f1f5f9;
  --border-hover: #cbd5e1;

  --text-primary: #0f172a;
  --text-secondary: #475569;
  --text-tertiary: #94a3b8;

  --shadow-sm: 0 1px 2px 0 rgba(15, 23, 42, 0.05);
  --shadow-md: 0 4px 12px -2px rgba(15, 23, 42, 0.08);
  --shadow-lg: 0 12px 24px -4px rgba(15, 23, 42, 0.1);
  --shadow-card: 0 4px 20px -2px rgba(15, 23, 42, 0.06);
}

/* 强制深色 */
:root[data-theme="dark"] {
  --bg-primary: #090d16;
  --bg-secondary: #111827;
  --bg-tertiary: #1e293b;
  --bg-card: #111827;
  --bg-input: #0f172a;

  --border-color: #232f48;
  --border-subtle: #1a2336;
  --border-hover: #374768;

  --text-primary: #f8fafc;
  --text-secondary: #cbd5e1;
  --text-tertiary: #64748b;

  --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.4);
  --shadow-md: 0 4px 12px -2px rgba(0, 0, 0, 0.5);
  --shadow-lg: 0 12px 28px -4px rgba(0, 0, 0, 0.6);
  --shadow-card: 0 4px 24px -2px rgba(0, 0, 0, 0.45);
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  color: var(--text-primary);
  background-color: var(--bg-primary);
  overflow: hidden;
  user-select: none;
  -webkit-font-smoothing: antialiased;
}

/* 整体两栏布局 */
.app-layout {
  height: 100vh;
  width: 100vw;
  display: flex;
  background-color: var(--bg-primary);
  overflow: hidden;
}

/* 现代化极简侧边栏 */
.app-sidebar {
  width: 180px;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  padding: 1rem 0.75rem;
  transition: width 0.28s cubic-bezier(0.4, 0, 0.2, 1);
  flex-shrink: 0;
  z-index: 10;
}

.app-sidebar.is-collapsed {
  width: 68px;
  padding: 1rem 0.5rem;
}

/* 侧边栏品牌区 */
.sidebar-brand {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.35rem 0.5rem 1.25rem;
  border-bottom: 1px solid var(--border-subtle);
  margin-bottom: 0.75rem;
}

.brand-logo-wrap {
  width: 32px;
  height: 32px;
  border-radius: 9px;
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.15), rgba(99, 102, 241, 0.2));
  display: grid;
  place-items: center;
  flex-shrink: 0;
}

.brand-logo {
  width: 24px;
  height: 24px;
  border-radius: 6px;
  object-fit: cover;
}

.brand-text {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.brand-name {
  font-size: 0.92rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
  white-space: nowrap;
}

.brand-sub {
  font-size: 0.68rem;
  color: var(--text-tertiary);
  font-weight: 500;
  letter-spacing: 0.05em;
  text-transform: uppercase;
}

/* 侧边栏导航链接 */
.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  flex: 1;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.65rem 0.85rem;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 0.9rem;
  font-weight: 500;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  white-space: nowrap;
}

.app-sidebar.is-collapsed .nav-item {
  justify-content: center;
  padding: 0.65rem 0;
}

.nav-icon {
  font-size: 1.15rem;
  flex-shrink: 0;
  transition: transform 0.2s ease;
}

.nav-item:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.nav-item:hover .nav-icon {
  transform: scale(1.08);
}

.nav-item.active {
  background-color: var(--primary-subtle);
  color: var(--primary-color);
  font-weight: 600;
}

.nav-item.active .nav-icon {
  color: var(--primary-color);
}

/* 底部收起按钮 */
.sidebar-footer {
  padding-top: 0.75rem;
  border-top: 1px solid var(--border-subtle);
}

.collapse-toggle-btn {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.55rem 0.85rem;
  background: transparent;
  border: none;
  border-radius: var(--radius-md);
  color: var(--text-tertiary);
  cursor: pointer;
  font-size: 0.82rem;
  transition: all 0.2s ease;
}

.app-sidebar.is-collapsed .collapse-toggle-btn {
  justify-content: center;
  padding: 0.55rem 0;
}

.collapse-toggle-btn:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.toggle-icon {
  font-size: 1.1rem;
  flex-shrink: 0;
}

/* 右侧主视口 */
.app-main {
  flex: 1;
  height: 100vh;
  min-width: 0;
  overflow-y: auto;
  overflow-x: hidden;
  background-color: var(--bg-primary);
}

/* 页面切换平滑过渡 */
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.fade-slide-enter-from {
  opacity: 0;
  transform: translateY(6px);
}

.fade-slide-leave-to {
  opacity: 0;
  transform: translateY(-6px);
}

/* 现代精致滚动条 */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 999px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-tertiary);
}
</style>
