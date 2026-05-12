<script setup>
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, onMounted, onUnmounted, watch } from "vue";
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
import {
  DEFAULT_DOWNLOAD_OPTIONS,
  FILENAME_TEMPLATE_OPTIONS,
  FORMAT_OPTIONS,
  STORAGE_KEYS,
  readNumberSetting,
  readOptionSetting,
} from "../settings.js";

const url = ref("");
const progress = ref("");
const error = ref("");
const dir = ref("");
const proxy = ref("");
const formatPreset = ref(DEFAULT_DOWNLOAD_OPTIONS.format);
const filenameTemplate = ref(DEFAULT_DOWNLOAD_OPTIONS.filenameTemplate);
const retries = ref(DEFAULT_DOWNLOAD_OPTIONS.retries);
const concurrentFragments = ref(DEFAULT_DOWNLOAD_OPTIONS.concurrentFragments);
const isDownloading = ref(false);
const isCancelling = ref(false);
const downloadStatus = ref("idle"); // idle, downloading, completed, failed, cancelled

const URL_PATTERN = /^https?:\/\/.+/;

function getErrorMessage(err) {
  if (typeof err === "string") return err;
  return err?.message || String(err);
}

async function chooseDir() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "选择目录",
  });

  if (selected) {
    dir.value = selected;
  }
}
async function download() {
  if (!url.value.trim()) {
    error.value = "请输入视频链接";
    downloadStatus.value = "failed";
    return;
  }

  if (!URL_PATTERN.test(url.value.trim())) {
    error.value = "请输入有效的 URL（以 http:// 或 https:// 开头）";
    downloadStatus.value = "failed";
    return;
  }

  progress.value = "";
  error.value = "";
  isDownloading.value = true;
  downloadStatus.value = "downloading";

  try {
    await invoke("download", {
      url: url.value,
      dir: dir.value,
      proxy: proxy.value,
      formatPreset: formatPreset.value,
      filenameTemplate: filenameTemplate.value,
      retries: retries.value,
      concurrentFragments: concurrentFragments.value,
    });
    downloadStatus.value = "completed";
    const autoOpenDir = localStorage.getItem(STORAGE_KEYS.autoOpenDir) !== "false";
    if (autoOpenDir) {
      let targetDir = dir.value;
      if (!targetDir) {
        try {
          targetDir = await downloadDir();
        } catch (dirErr) {
          error.value = getErrorMessage(dirErr);
        }
      }
      try {
        if (targetDir) await openPath(targetDir);
      } catch (openErr) {
        error.value = getErrorMessage(openErr);
      }
    }
  } catch (err) {
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
  const savedDir = localStorage.getItem(STORAGE_KEYS.dir);
  if (savedDir) dir.value = savedDir;
  const savedProxy = localStorage.getItem(STORAGE_KEYS.proxy);
  if (savedProxy) proxy.value = savedProxy;
  formatPreset.value = readOptionSetting(
    STORAGE_KEYS.format,
    FORMAT_OPTIONS,
    DEFAULT_DOWNLOAD_OPTIONS.format,
  );
  filenameTemplate.value = readOptionSetting(
    STORAGE_KEYS.filenameTemplate,
    FILENAME_TEMPLATE_OPTIONS,
    DEFAULT_DOWNLOAD_OPTIONS.filenameTemplate,
  );
  retries.value = readNumberSetting(
    STORAGE_KEYS.retries,
    DEFAULT_DOWNLOAD_OPTIONS.retries,
    0,
    20,
  );
  concurrentFragments.value = readNumberSetting(
    STORAGE_KEYS.concurrentFragments,
    DEFAULT_DOWNLOAD_OPTIONS.concurrentFragments,
    1,
    16,
  );
  const autoPaste =
    localStorage.getItem(STORAGE_KEYS.autoPasteClipboard) === "true";
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

  onListenProgress = listen("yt-dlp-progress", (event) => {
    const output = event.payload;
    const progressMatch = output.match(/\[download\]\s+(\d+\.?\d*)%/);
    if (progressMatch) {
      progress.value = parseFloat(progressMatch[1]).toFixed(2);
    }
  });

  onListenError = listen("yt-dlp-error", (event) => {
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
    localStorage.removeItem(STORAGE_KEYS.dir);
    return;
  }
  localStorage.setItem(STORAGE_KEYS.dir, val);
});

watch(proxy, (val) => {
  if (!val) {
    localStorage.removeItem(STORAGE_KEYS.proxy);
    return;
  }
  localStorage.setItem(STORAGE_KEYS.proxy, val);
});

watch(formatPreset, (val) => {
  localStorage.setItem(STORAGE_KEYS.format, val || DEFAULT_DOWNLOAD_OPTIONS.format);
});

watch(filenameTemplate, (val) => {
  localStorage.setItem(
    STORAGE_KEYS.filenameTemplate,
    val || DEFAULT_DOWNLOAD_OPTIONS.filenameTemplate,
  );
});

watch(retries, (val) => {
  localStorage.setItem(STORAGE_KEYS.retries, String(val));
});

watch(concurrentFragments, (val) => {
  localStorage.setItem(STORAGE_KEYS.concurrentFragments, String(val));
});

onUnmounted(() => {
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
        <el-text class="label" tag="b">下载格式</el-text>
        <div class="confContent">
          <el-select
            v-model="formatPreset"
            placeholder="选择下载格式"
            :disabled="isDownloading"
          >
            <el-option
              v-for="item in FORMAT_OPTIONS"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
          <el-tooltip
            content="指定最高分辨率或仅下载音频；MP3 需要 FFmpeg 支持"
            placement="top"
          >
            <el-icon class="help-icon"><QuestionFilled /></el-icon>
          </el-tooltip>
        </div>
      </div>
      <div class="confItem">
        <el-text class="label" tag="b">文件名模板</el-text>
        <div class="confContent">
          <el-select
            v-model="filenameTemplate"
            placeholder="选择文件名模板"
            :disabled="isDownloading"
          >
            <el-option
              v-for="item in FILENAME_TEMPLATE_OPTIONS"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
          <el-tooltip content="控制保存文件名的组成方式" placement="top">
            <el-icon class="help-icon"><QuestionFilled /></el-icon>
          </el-tooltip>
        </div>
      </div>
      <div class="confItem">
        <el-text class="label" tag="b">下载参数</el-text>
        <div class="confContent compact">
          <div class="number-field">
            <el-text size="small">重试次数</el-text>
            <el-input-number
              v-model="retries"
              :min="0"
              :max="20"
              :disabled="isDownloading"
              controls-position="right"
            />
          </div>
          <div class="number-field">
            <el-text size="small">并发片段</el-text>
            <el-input-number
              v-model="concurrentFragments"
              :min="1"
              :max="16"
              :disabled="isDownloading"
              controls-position="right"
            />
          </div>
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

.confContent.compact {
  grid-template-columns: repeat(2, minmax(8em, 1fr));
}

.number-field {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 0.5em;
  align-items: center;
}

.number-field .el-input-number {
  width: 100%;
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
