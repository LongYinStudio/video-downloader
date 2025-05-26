<script setup>
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, onMounted, onUnmounted } from "vue";
// import { setTheme } from "@tauri-apps/api/app";
import { open } from "@tauri-apps/plugin-dialog";
import { QuestionFilled } from "@element-plus/icons-vue";
import { version } from "../utils.js";

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
  // 国内 测试：https://www.bilibili.com/video/BV1GzfUYmEGE
  // 国外 测试：https://www.youtube.com/watch?v=ObEN8jqJZ7o
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
  <div style="margin-top: 2em">
    <img src="../../src-tauri/icons/logo.png" class="logo" alt="logo" />
  </div>
  <h1 class="appname">
    视频下载器
    <el-tag type="info" effect="dark" round> {{ version }}</el-tag>
  </h1>

  <el-card style="width: 96%; margin: 0 auto">
    <el-form id="downloadForm" @submit.prevent="download">
      <el-input id="url-input" v-model="url" placeholder="Enter a url..." />
      <el-button @click="download()">开始下载</el-button>
    </el-form>
    <el-progress
      id="progress"
      v-if="process"
      :text-inside="true"
      :stroke-width="24"
      :percentage="progress"
      status="success"
    />
    <div v-if="error" style="margin-top: 1em">
      <p style="color: red">下载出错：{{ error }}</p>
    </div>
    <el-divider />
    <div class="conf">
      <div class="confItem">
        <el-text class="label">保存目录:</el-text>
        <div class="confContent">
          <el-input
            id="dir-input"
            v-model="dir"
            placeholder="未选择目录(默认：系统Downloads目录)"
          />
          <el-button @click="chooseDir()">选择目录</el-button>
        </div>
      </div>
      <div class="confItem" style="padding-top: 1em">
        <el-text class="label">代理设置:</el-text>
        <div class="confContent">
          <el-input
            id="proxy-input"
            v-model="proxy"
            placeholder="Enter a proxy..."
          />
          <el-tooltip
            content="举例：[http|socks5://]127.0.0.1:7890，具体地址、端口查看代理软件"
            placement="top"
          >
            <el-icon><QuestionFilled /></el-icon>
          </el-tooltip>
        </div>
      </div>
    </div>
  </el-card>

  <footer>
    <!-- <p>默认下载路径：系统Downloads目录</p> -->
    <p>
      © 2025 by
      <a target="_blank" href="https://github.com/LongYinStudio"
        >LongYinStudio</a
      >
    </p>
  </footer>
</template>

<style scoped>
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
  padding: 0.8em 0px;
}

h1 {
  text-align: center;
}

#progress {
  padding: 2em 0px;
  width: 66%;
  margin: 0 auto;
}

#downloadForm {
  display: grid;
  grid-template-columns: 1fr 10em;
  grid-gap: 1em;
}
#downloadForm > * {
  height: 3.4em;
}

.conf .confItem .label {
  text-align: start;
}

.confContent {
  display: grid;
  grid-template-columns: 20em 4.8em;
  grid-gap: 0.5em;
  align-items: center;
  padding-top: 0.3em;
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
