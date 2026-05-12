<script setup>
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, onMounted, onUnmounted, watch } from "vue";
// import { setTheme } from "@tauri-apps/api/app";
import { open } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";
import { downloadDir } from "@tauri-apps/api/path";
import {
  QuestionFilled,
  Loading,
  CircleCheck,
  CircleClose,
  VideoCamera,
} from "@element-plus/icons-vue";
import { version } from "../utils.js";

const url = ref("");
const progress = ref("");
const error = ref("");
const dir = ref("");
const proxy = ref("");
const isDownloading = ref(false);
const isCancelling = ref(false);
const downloadStatus = ref("idle"); // idle, downloading, completed, failed, cancelled
const fileName = ref("");
const downloadSpeed = ref("");

const DIR_KEY = "vd.dir";
const PROXY_KEY = "vd.proxy";
const AUTO_OPEN_DIR_KEY = "vd.auto_open_dir";
const AUTO_PASTE_CLIPBOARD_KEY = "vd.auto_paste_clipboard";
const URL_PATTERN = /^https?:\/\/.+/;

// 表单引用
const formRef = ref(null);

function getErrorMessage(err) {
  if (typeof err === "string") return err;
  return err?.message || String(err);
}

// URL 验证规则
const urlRules = {
  required: true,
  message: "请输入视频链接",
  trigger: "blur",
  pattern: /^https?:\/\/.+/,
  patternMessage: "请输入有效的 URL（以 http:// 或 https:// 开头）",
};

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

  // 验证 URL
  if (!url.value.trim()) {
    error.value = "请输入视频链接";
    downloadStatus.value = "failed";
    return;
  }

  // 验证 URL 格式
  if (!URL_PATTERN.test(url.value.trim())) {
    error.value = "请输入有效的 URL（以 http:// 或 https:// 开头）";
    downloadStatus.value = "failed";
    return;
  }

  // 重置状态
  progress.value = "";
  error.value = "";
  isDownloading.value = true;
  downloadStatus.value = "downloading";

  try {
    const result = await invoke("download", {
      url: url.value,
      dir: dir.value,
      proxy: proxy.value,
    });
    console.log(result);
    downloadStatus.value = "completed";
    const autoOpenDir = localStorage.getItem(AUTO_OPEN_DIR_KEY) !== "false";
    if (autoOpenDir) {
      let targetDir = dir.value;
      if (!targetDir) {
        try {
          targetDir = await downloadDir();
        } catch (dirErr) {
          console.warn("Resolve downloadDir failed:", dirErr);
        }
      }
      try {
        if (targetDir) await openPath(targetDir);
      } catch (openErr) {
        console.warn("Open directory failed:", openErr);
      }
    }
  } catch (err) {
    console.error("Failed to run yt-dlp:", err);
    const message = getErrorMessage(err);
    if (message === "下载已取消") {
      error.value = "";
      downloadStatus.value = "cancelled";
    } else {
      error.value = "下载失败：" + message;
      downloadStatus.value = "failed";
    }
  } finally {
    isDownloading.value = false;
    isCancelling.value = false;
  }
}

async function cancelDownload() {
  if (!isDownloading.value || isCancelling.value) return;

  isCancelling.value = true;
  try {
    await invoke("cancel_download");
  } catch (err) {
    error.value = getErrorMessage(err);
    isCancelling.value = false;
  }
}

// 监听下载进度
let onListenProgress;
let onListenError;

onMounted(() => {
  const savedDir = localStorage.getItem(DIR_KEY);
  if (savedDir) dir.value = savedDir;
  const savedProxy = localStorage.getItem(PROXY_KEY);
  if (savedProxy) proxy.value = savedProxy;
  const autoPaste =
    localStorage.getItem(AUTO_PASTE_CLIPBOARD_KEY) === "true";
  if (autoPaste && !url.value) {
    navigator.clipboard
      ?.readText?.()
      .then((text) => {
        if (text && URL_PATTERN.test(text.trim())) {
          url.value = text.trim();
        }
      })
      .catch(() => {
        // ignore clipboard errors
      });
  }

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

watch(dir, (val) => {
  if (!val) {
    localStorage.removeItem(DIR_KEY);
    return;
  }
  localStorage.setItem(DIR_KEY, val);
});

watch(proxy, (val) => {
  if (!val) {
    localStorage.removeItem(PROXY_KEY);
    return;
  }
  localStorage.setItem(PROXY_KEY, val);
});

onUnmounted(() => {
  // 清理事件监听器
  if (onListenProgress) onListenProgress.then((onListen) => onListen());
  if (onListenError) onListenError.then((onListen) => onListen());
});
</script>

<template>
  <div class="logo-container">
    <img src="../../src-tauri/icons/logo.png" class="logo" alt="logo" />
  </div>
  <h1 class="appname">
    视频下载器
    <el-tag type="primary" effect="dark" round>{{ version }}</el-tag>
  </h1>

  <el-card class="main-card" shadow="hover">
    <!-- 下载状态提示 -->
    <div v-if="downloadStatus !== 'idle'" class="status-bar">
      <el-tag
        v-if="downloadStatus === 'downloading'"
        type="primary"
        effect="plain"
      >
        <el-icon class="is-loading"><Loading /></el-icon>
        下载中...
      </el-tag>
      <el-tag
        v-else-if="downloadStatus === 'completed'"
        type="success"
        effect="plain"
      >
        <el-icon><CircleCheck /></el-icon>
        下载完成
      </el-tag>
      <el-tag
        v-else-if="downloadStatus === 'failed'"
        type="danger"
        effect="plain"
      >
        <el-icon><CircleClose /></el-icon>
        下载失败
      </el-tag>
      <el-tag
        v-else-if="downloadStatus === 'cancelled'"
        type="warning"
        effect="plain"
      >
        <el-icon><CircleClose /></el-icon>
        已取消
      </el-tag>
    </div>

    <el-form id="downloadForm" @submit.prevent="download">
      <el-input
        id="url-input"
        v-model="url"
        placeholder="请输入视频链接，支持 Bilibili、YouTube 等"
        size="large"
        clearable
        :disabled="isDownloading"
        show-word-limit
        maxlength="500"
      >
        <template #prefix>
          <el-icon><VideoCamera /></el-icon>
        </template>
      </el-input>
      <el-tooltip
        content="支持 YouTube、Bilibili、Vimeo 等主流视频平台"
        placement="top"
        :disabled="url.trim()"
      >
        <div class="download-actions">
          <el-button
            type="primary"
            size="large"
            :loading="isDownloading"
            :disabled="isDownloading || !url.trim()"
            @click="download()"
          >
            {{ isDownloading ? "下载中..." : "开始下载" }}
          </el-button>
          <el-button
            v-if="isDownloading"
            type="danger"
            size="large"
            plain
            :loading="isCancelling"
            @click="cancelDownload()"
          >
            取消
          </el-button>
        </div>
      </el-tooltip>
    </el-form>
    <el-progress
      id="progress"
      v-if="progress"
      :text-inside="true"
      :stroke-width="24"
      :percentage="parseFloat(progress)"
      status="success"
    />
    <div v-if="error" class="error-message">
      <el-alert type="error" :closable="true" show-icon @close="error = ''">
        <template #title>
          {{ error }}
        </template>
        <template #default>
          <div class="error-tips">
            请检查：
            <ul>
              <li>URL 是否正确（以 http:// 或 https:// 开头）</li>
              <li>网络连接是否正常</li>
              <li>视频链接是否有效</li>
              <li>代理设置是否正确（如需使用代理）</li>
            </ul>
          </div>
        </template>
      </el-alert>
    </div>
    <el-divider />
    <div class="conf">
      <div class="confItem">
        <el-text class="label" tag="b">保存目录</el-text>
        <div class="confContent">
          <el-input
            id="dir-input"
            v-model="dir"
            placeholder="未选择目录(默认：系统Downloads目录)"
            readonly
            :disabled="isDownloading"
          />
          <el-button type="info" :disabled="isDownloading" @click="chooseDir()"
            >选择目录</el-button
          >
        </div>
      </div>
      <div class="confItem">
        <el-text class="label" tag="b">代理设置</el-text>
        <div class="confContent">
          <el-input
            id="proxy-input"
            v-model="proxy"
            placeholder="可选：http://127.0.0.1:7890"
            :disabled="isDownloading"
          />
          <el-tooltip
            content="支持 HTTP/SOCKS5 代理，如：http://127.0.0.1:7890"
            placement="top"
          >
            <el-icon class="help-icon"><QuestionFilled /></el-icon>
          </el-tooltip>
        </div>
      </div>
    </div>
  </el-card>

  <footer class="footer">
    <p>
      © 2025 by
      <a target="_blank" href="https://github.com/LongYinStudio"
        >LongYinStudio</a
      >
    </p>
  </footer>
</template>

<style scoped>
.logo-container {
  margin-top: 2.5em;
  display: flex;
  justify-content: center;
  align-items: center;
}

.logo {
  height: 7em;
  width: 7em;
  border-radius: 1.5em;
  transition: all 0.4s ease;
  box-shadow: var(--shadow-md);
  object-fit: cover;
}

.logo:hover {
  transform: scale(1.05);
  box-shadow: var(--shadow-lg);
  filter: drop-shadow(0 0 2em #030040);
}

.appname {
  padding: 1em 0;
  font-size: 2em;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5em;
  color: var(--text-primary);
}

h1 {
  text-align: center;
  margin: 0;
}

.main-card {
  width: 90%;
  max-width: 700px;
  margin: 1.5em auto;
  border-radius: 12px;
  background-color: var(--bg-secondary);
  border-color: var(--border-color);
}

.status-bar {
  display: flex;
  justify-content: center;
  margin-bottom: 1em;
}

.status-bar .el-tag {
  font-size: 1em;
  padding: 0.5em 1em;
}

#progress {
  padding: 2.5em 0;
  width: 70%;
  margin: 0 auto;
}

#downloadForm {
  display: grid;
  grid-template-columns: 1fr auto;
  grid-gap: 1em;
}

#downloadForm > * {
  height: 3.6em;
  font-size: 1em;
}

.download-actions {
  display: flex;
  gap: 0.75em;
  height: 100%;
}

.download-actions .el-button {
  height: 100%;
  min-width: 7.5em;
  margin-left: 0;
}

.error-message {
  margin-top: 1.5em;
}

.error-tips {
  margin-top: 0.5em;
  font-size: 0.9em;
  color: var(--text-secondary);
}

.error-tips ul {
  margin-left: 1.2em;
  margin-top: 0.3em;
}

.error-tips li {
  margin: 0.3em 0;
}

.conf {
  margin-top: 1.5em;
}

.confItem {
  margin-bottom: 1.2em;
}

.conf .label {
  display: block;
  text-align: left;
  margin-bottom: 0.5em;
  font-size: 0.95em;
  color: var(--text-primary);
  font-weight: 500;
}

.confContent {
  display: grid;
  grid-template-columns: 1fr auto;
  grid-gap: 0.75em;
  align-items: center;
}

.help-icon {
  cursor: help;
  font-size: 1.2em;
  color: var(--text-tertiary);
  transition: color 0.3s ease;
}

.help-icon:hover {
  color: var(--primary-color);
}

.footer {
  width: 100%;
  text-align: center;
  padding: 1.5em 1em;
  color: var(--text-tertiary);
  font-size: 0.9em;
}

.footer a {
  color: var(--primary-color);
  text-decoration: none;
  transition: color 0.3s;
}

.footer a:hover {
  color: var(--primary-hover);
  text-decoration: underline;
}

/* 深色模式特定调整 */
@media (prefers-color-scheme: dark) {
  .logo {
    box-shadow: 0 4px 12px rgba(255, 255, 255, 0.1);
  }

  .logo:hover {
    box-shadow: 0 8px 24px rgba(255, 255, 255, 0.15);
  }

  .main-card {
    box-shadow: var(--shadow-lg);
  }
}
</style>
