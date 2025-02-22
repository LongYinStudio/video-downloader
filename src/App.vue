<script setup>
// import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, onMounted, onUnmounted } from "vue";
// import { setTheme } from "@tauri-apps/api/app";
import { Menu as IconMenu, Setting } from "@element-plus/icons-vue";
import { open } from "@tauri-apps/plugin-dialog";

const url = ref("");
const progress = ref("");
const error = ref("");
const dir = ref("");
const proxy = ref("");

async function chooseDir() {
  const selected = await open({
    directory: true, // 设置为 true 表示选择目录
    multiple: false, // 是否允许多选
    title: "选择目录", // 对话框标题
  });

  if (selected) {
    dir.value = selected;
    console.log("选择的目录:", selected);
  } else {
    console.log("用户取消了选择");
  }
}
async function download() {
  // 测试
  // 国内：https://www.bilibili.com/video/BV1GzfUYmEGE
  // 国外：https://www.youtube.com/watch?v=ObEN8jqJZ7o
  try {
    const result = await invoke("download", {
      url: url.value,
      dir: dir.value,
      proxy: proxy.value,
    });
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
let onListenProgress;
let onListenError;

onMounted(() => {
  // 监听下载进度事件
  onListenProgress = listen("yt-dlp-progress", (event) => {
    const output = event.payload;
    console.log("Download progress:", output);

    // 解析进度信息
    const progressMatch = output.match(/\[download\]\s+(\d+\.?\d*)%/);
    if (progressMatch) {
      // progress.value = `下载进度：${parseFloat(progressMatch[1]).toFixed(2)}%`;
      progress.value = parseFloat(progressMatch[1]).toFixed(2);
    }
    // else {
    //   progress.value = output; // 显示其他输出
    // }
  });

  // 监听错误事件
  onListenError = listen("yt-dlp-error", (event) => {
    console.error("Download error:", event.payload);
    const urlERROR = event.payload.match(/'([^']*)' is not a valid URL\./);
    if (urlERROR) {
      error.value = urlERROR[0] + "请输入正确url地址";
    } else {
      error.value = event.payload;
    }
  });
});

onUnmounted(() => {
  // 清理事件监听器
  if (onListenProgress) onListenProgress.then((onListen) => onListen());
  if (onListenError) onListenError.then((onListen) => onListen());
});
</script>

<template>
  <main class="container">
    <el-menu default-active="2" id="sideBar" popper-effect="dark">
      <el-menu-item index="2">
        <el-icon><icon-menu /></el-icon>
      </el-menu-item>
      <el-menu-item index="4">
        <el-icon>
          <setting />
        </el-icon>
      </el-menu-item>
    </el-menu>
    <div id="main">
      <div>
        <img src="../src-tauri/icons/logo.png" class="logo" alt="logo" />
      </div>
      <h1 class="appname">视频下载器</h1>
      <el-progress
        id="progress"
        v-if="process"
        :text-inside="true"
        :stroke-width="24"
        :percentage="progress"
        status="success"
      />
      <div v-if="error">
        <p style="color: red">下载出错：{{ error }}</p>
      </div>
      <p v-if="dir">下载目录:{{ dir }}</p>
      <button id="dirBtn" @click="chooseDir">选择保存目录</button>
      <div class="proxy">
        <input
          id="proxy-input"
          v-model="proxy"
          placeholder="Enter a proxy..."
        />
        <p>
          代理设置如：[http|socks5://]127.0.0.1:7890，具体地址、端口查看代理软件
        </p>
      </div>
      <form id="download" @submit.prevent="download">
        <input id="url-input" v-model="url" placeholder="Enter a url..." />
        <button type="submit">开始下载</button>
      </form>
      <footer>
        <p>默认下载路径：系统Downloads目录</p>
        <p>
          © 2025 by
          <a href="https://github.com/LongYinStudio">LongYinStudio</a>
        </p>
      </footer>
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

#sideBar {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

#main {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  will-change: filter;
  transition: 0.75s;
  border-radius: 1em;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #030040);
}

.appname {
  padding: 1em 0px;
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

#progress {
  padding: 2em 0px;
  width: 66%;
  margin: 0 auto;
}

#dirBtn {
  margin: 0 auto;
  width: 16em;
}

.proxy {
  margin: 1em;
}

.proxy p {
  margin-top: 6px;
}

form#download {
  margin: 10px 0px;
  display: flex;
  justify-content: center;
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

input {
  padding: 0.8em 1.2em;
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
  /* position: fixed; */
  /* bottom: 10px; */
  width: 100%;
  text-align: center;
  padding: 10px;
  background-color: #f6f6f6;
  color: #0f0f0f;
}
</style>
