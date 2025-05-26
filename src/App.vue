<script setup>
import { ref } from "vue";
import { Menu as IconMenu, Setting, InfoFilled } from "@element-plus/icons-vue";

const isCollapse = ref(true);
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
      @open="handleOpen"
      @close="handleClose"
      router="true"
    >
      <el-menu-item index="/">
        <el-icon><icon-menu /></el-icon>
        <template #title>首页</template>
      </el-menu-item>
      <el-menu-item index="/settings">
        <el-icon><setting /></el-icon>
        <template #title>设置</template>
      </el-menu-item>
      <el-menu-item index="/about">
        <el-icon><InfoFilled /></el-icon>
        <template #title>关于</template>
      </el-menu-item>
      <el-menu-item @click="isCollapse = !isCollapse">
        <el-icon>
          <img
            v-if="isCollapse"
            src="./assets/side-open.png"
            alt=""
            class="collapseIcon"
          />
          <img
            v-else
            src="./assets/side-close.png"
            alt=""
            class="collapseIcon"
          />
        </el-icon>
        <template #title>展开</template>
      </el-menu-item>
    </el-menu>
    <div id="main">
      <router-view></router-view>
    </div>
  </main>
</template>

<style>
* {
  margin: 0;
  padding: 0;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  /* margin: 0; */
  height: 100%;
  display: grid;
  grid-template-columns: 4em minmax(400px, 1fr);
  --el-menu-bg-color: #dee0e2;
  --el-menu-text-color: #303133;
  --el-menu-active-color: #409eff;
}

.sideBar:not(.el-menu--collapse) {
  width: 100px;
  min-height: 400px;
}

.collapseIcon {
  --size: 1.1em;
  width: var(--size);
  height: var(--size);
}

#main {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  /* justify-content: center; */
  text-align: center;
}
</style>
