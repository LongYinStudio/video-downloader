<script setup>
import { ref } from "vue";
import { Menu as IconMenu, Setting, InfoFilled } from "@element-plus/icons-vue";

const isCollapse = ref(true);
const THEME_KEY = "vd.theme";

function applyTheme(mode) {
  const root = document.documentElement;
  if (!mode || mode === "system") {
    root.removeAttribute("data-theme");
    return;
  }
  root.setAttribute("data-theme", mode);
}

applyTheme(localStorage.getItem(THEME_KEY) || "system");
const handleOpen = (key, keyPath) => {
  console.log(key, keyPath);
};
const handleClose = (key, keyPath) => {
  console.log(key, keyPath);
};
</script>

<template>
  <main class="container">
    <el-menu
      default-active="/"
      class="sideBar"
      :collapse="isCollapse"
      :collapse-transition="true"
      @open="handleOpen"
      @close="handleClose"
      router="true"
    >
      <el-menu-item index="/">
        <el-icon class="menu-icon"><icon-menu /></el-icon>
        <template #title>首页</template>
      </el-menu-item>
      <el-menu-item index="/settings">
        <el-icon class="menu-icon"><setting /></el-icon>
        <template #title>设置</template>
      </el-menu-item>
      <el-menu-item index="/about">
        <el-icon class="menu-icon"><InfoFilled /></el-icon>
        <template #title>关于</template>
      </el-menu-item>
      <el-menu-item class="collapse-item" @click="isCollapse = !isCollapse">
        <el-icon class="collapse-icon">
          <img
            v-if="isCollapse"
            src="./assets/side-open.png"
            alt=""
            class="collapseImage"
          />
          <img
            v-else
            src="./assets/side-close.png"
            alt=""
            class="collapseImage"
          />
        </el-icon>
        <template #title>{{ isCollapse ? "展开" : "收起" }}</template>
      </el-menu-item>
    </el-menu>
    <div id="main">
      <router-view></router-view>
    </div>
  </main>
</template>

<style>
/* CSS 变量定义 */
:root {
  --primary-color: #1976d2;
  --primary-hover: #1565c0;
  --success-color: #52c41a;
  --danger-color: #f56c6c;
  --warning-color: #faad14;

  /* 浅色模式 */
  --bg-primary: #f6f6f6;
  --bg-secondary: #ffffff;
  --bg-tertiary: #e8eaed;

  --text-primary: #0f0f0f;
  --text-secondary: #424242;
  --text-tertiary: #757575;

  --border-color: #d0d0d0;
  --hover-bg: #d3d6db;

  --scrollbar-track: #f1f1f1;
  --scrollbar-thumb: #c1c1c1;
  --scrollbar-thumb-hover: #a8a8a8;

  --shadow-sm: 0 2px 4px rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 8px rgba(0, 0, 0, 0.1);
  --shadow-lg: 0 8px 16px rgba(0, 0, 0, 0.15);
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  :root {
    --bg-primary: #1a1a1a;
    --bg-secondary: #2d2d2d;
    --bg-tertiary: #363636;

    --text-primary: #f6f6f6;
    --text-secondary: #e0e0e0;
    --text-tertiary: #b0b0b0;

    --border-color: #424242;
    --hover-bg: #4a4a4a;

    --scrollbar-track: #2d2d2d;
    --scrollbar-thumb: #555555;
    --scrollbar-thumb-hover: #666666;

    --shadow-sm: 0 2px 4px rgba(0, 0, 0, 0.3);
    --shadow-md: 0 4px 8px rgba(0, 0, 0, 0.4);
    --shadow-lg: 0 8px 16px rgba(0, 0, 0, 0.5);
  }
}

/* 主题强制覆盖（不跟随系统） */
:root[data-theme="light"] {
  --bg-primary: #f6f6f6;
  --bg-secondary: #ffffff;
  --bg-tertiary: #e8eaed;

  --text-primary: #0f0f0f;
  --text-secondary: #424242;
  --text-tertiary: #757575;

  --border-color: #d0d0d0;
  --hover-bg: #d3d6db;

  --scrollbar-track: #f1f1f1;
  --scrollbar-thumb: #c1c1c1;
  --scrollbar-thumb-hover: #a8a8a8;

  --shadow-sm: 0 2px 4px rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 8px rgba(0, 0, 0, 0.1);
  --shadow-lg: 0 8px 16px rgba(0, 0, 0, 0.15);
}

:root[data-theme="dark"] {
  --bg-primary: #1a1a1a;
  --bg-secondary: #2d2d2d;
  --bg-tertiary: #363636;

  --text-primary: #f6f6f6;
  --text-secondary: #e0e0e0;
  --text-tertiary: #b0b0b0;

  --border-color: #424242;
  --hover-bg: #4a4a4a;

  --scrollbar-track: #2d2d2d;
  --scrollbar-thumb: #555555;
  --scrollbar-thumb-hover: #666666;

  --shadow-sm: 0 2px 4px rgba(0, 0, 0, 0.3);
  --shadow-md: 0 4px 8px rgba(0, 0, 0, 0.4);
  --shadow-lg: 0 8px 16px rgba(0, 0, 0, 0.5);
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: var(--text-primary);
  background-color: var(--bg-primary);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  height: 100vh;
  width: 100vw;
  display: grid;
  grid-template-columns: auto minmax(400px, 1fr);
  overflow: hidden;
}

.sideBar {
  --el-menu-bg-color: var(--bg-tertiary);
  --el-menu-text-color: var(--text-secondary);
  --el-menu-hover-bg-color: var(--hover-bg);
  --el-menu-active-color: var(--primary-color);
  border-right: 1px solid var(--border-color);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.sideBar:not(.el-menu--collapse) {
  width: 120px;
  min-height: 100vh;
}

.el-menu--collapse {
  width: 64px;
}

/* 收起状态下的icon居中对齐 - 使用更强的选择器 */
.sideBar.el-menu--collapse .el-menu-item {
  padding: 0 !important;
}

.sideBar.el-menu--collapse .el-menu-item .el-icon {
  margin-right: 0 !important;
  margin-left: 0 !important;
}

.sideBar.el-menu--collapse .collapse-item .collapseImage {
  margin: 0 !important;
}

.sideBar.el-menu--collapse .el-menu-item__content {
  padding: 0 !important;
}

/* 收起状态下 tooltip 触发器的默认 padding 导致左偏 */
.sideBar.el-menu--collapse .el-menu-tooltip__trigger {
  padding: 0 !important;
  display: flex !important;
  justify-content: center !important;
  align-items: center !important;
}

.el-menu-item {
  transition: all 0.25s ease;
  border-radius: 8px;
  margin: 4px 8px;
}

.el-menu-item:hover {
  background-color: var(--hover-bg);
  transform: translateX(2px);
}

/* 激活状态使用 Element Plus 的变量和通用选择器 */
.el-menu-item.is-active {
  --el-menu-text-color: #ffffff !important;
  background-color: var(--primary-color) !important;
}

.el-menu-item.is-active * {
  color: #ffffff !important;
}

.el-menu-item.is-active {
  color: #ffffff !important;
}

.menu-icon {
  font-size: 1.3em;
  transition: transform 0.3s ease;
}

.el-menu-item:hover .menu-icon {
  transform: scale(1.1);
}

.collapse-item {
  margin-top: auto;
  margin-bottom: 12px;
  color: var(--text-secondary);
}

.collapse-item:hover {
  background-color: var(--hover-bg);
}

.collapse-item span {
  color: var(--text-secondary);
}

.collapse-item:hover span {
  color: var(--text-primary);
}

.collapse-icon {
  display: flex;
  align-items: center;
  justify-content: center;
}

.collapseImage {
  --size: 1.2em;
  width: var(--size);
  height: var(--size);
  transition: transform 0.3s ease;
}

.collapse-item:hover .collapseImage {
  transform: rotate(180deg);
}

#main {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  text-align: center;
  background: linear-gradient(
    135deg,
    var(--bg-primary) 0%,
    var(--bg-secondary) 100%
  );
}

/* 自定义滚动条 */
#main::-webkit-scrollbar {
  width: 8px;
}

#main::-webkit-scrollbar-track {
  background: var(--scrollbar-track);
}

#main::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 4px;
}

#main::-webkit-scrollbar-thumb:hover {
  background: var(--scrollbar-thumb-hover);
}

/* Element Plus 深色模式适配 */
@media (prefers-color-scheme: dark) {
  .el-card {
    background-color: var(--bg-secondary);
    border-color: var(--border-color);
  }

  .el-input__wrapper {
    background-color: var(--bg-tertiary);
    box-shadow: 0 0 0 1px var(--border-color) inset;
  }

  .el-input__inner {
    color: var(--text-primary);
  }

  .el-input__inner::placeholder {
    color: var(--text-tertiary);
  }

  .el-button--primary {
    background-color: var(--primary-color);
    border-color: var(--primary-color);
  }

  .el-button--primary:hover {
    background-color: var(--primary-hover);
    border-color: var(--primary-hover);
  }

  /* 侧边栏深色模式 */
  .sideBar {
    --el-menu-text-color: var(--text-secondary);
  }

  .el-menu-item.is-active {
    --el-menu-text-color: #ffffff !important;
    color: #ffffff !important;
  }

  .collapse-item {
    color: var(--text-secondary);
  }

  .collapse-item span {
    color: var(--text-secondary);
  }

  .collapse-item:hover {
    background-color: var(--hover-bg);
  }

  .collapse-item:hover span {
    color: var(--text-primary);
  }
}
</style>
