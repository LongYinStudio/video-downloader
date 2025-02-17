<script setup>
// import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, onMounted, onUnmounted } from "vue";
// import { setTheme } from "@tauri-apps/api/app";

const url = ref("");
const msg = ref("");
const progress = ref("");
const error = ref("");
// const chooseDir = ref("");

// async function dir() {
//   alert(1);
// }
async function download() {
  // 测试 https://www.bilibili.com/video/BV1i7411F7NJ
  // msg.value = await invoke("download", { url: url.value });
  try {
    const result = await invoke("download", { url: url.value });
    // await setTheme("dark");
    console.log(result);
  } catch (error) {
    // console.error("Failed to run Lux:", error);
    console.error("Failed to run yt-dlp:", error);
    error.value = "下载失败：" + err.message;
  }
  // const luxCommand = Command.sidecar("bin/mylux");
  // const output = await luxCommand.execute();
  // console.log(output.stdout);
}

// 监听下载进度
let unlistenProgress;
let unlistenError;

onMounted(() => {
  // 监听下载进度事件
  unlistenProgress = listen("yt-dlp-progress", (event) => {
    const output = event.payload;
    console.log("Download progress:", output);

    // 解析进度信息
    const progressMatch = output.match(/\[download\]\s+(\d+\.?\d*)%/);
    if (progressMatch) {
      progress.value = `下载进度：${parseFloat(progressMatch[1]).toFixed(2)}%`;
    } else {
      progress.value = output; // 显示其他输出
    }
  });

  // 监听错误事件
  unlistenError = listen("yt-dlp-error", (event) => {
    console.error("Download error:", event.payload);
    error.value = "错误：" + event.payload;
  });
});

onUnmounted(() => {
  // 清理事件监听器
  if (unlistenProgress) unlistenProgress.then((unlisten) => unlisten());
  if (unlistenError) unlistenError.then((unlisten) => unlisten());
});
</script>

<template>
  <main class="container">
    <div class="row">
      <img src="../src-tauri/icons/logo.png" class="logo" alt="logo" />
    </div>
    <h1>视频下载器</h1>
    <p>显示信息</p>
    <div v-if="progress">
      <p>{{ progress }}</p>
    </div>
    <div v-if="error">
      <p style="color: red">{{ error }}</p>
    </div>
    <!-- <p>目录:{{ msg }}</p> -->
    <!-- <button @click="dir">选择目录</button> -->
    <form class="row" @submit.prevent="download">
      <input id="url-input" v-model="url" placeholder="Enter a url..." />
      <button type="submit">开始下载</button>
    </form>
  </main>
  <footer>
    <p>默认下载路径：系统Downloads目录</p>
    <p>当前只实现了简单的下载，并没有指定下载路径、代理等等功能</p>
    <p>
      © 2025 by <a href="https://github.com/LongYinStudio">LongYinStudio</a>
    </p>
  </footer>
</template>

<style>
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
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #030040);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#url-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}

footer {
  position: fixed;
  bottom: 10px;
  width: 100%;
  text-align: center;
  padding: 10px;
  background-color: #f6f6f6;
  color: #0f0f0f;
}
</style>
