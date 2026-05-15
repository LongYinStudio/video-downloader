<script setup>
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ref, onMounted, onUnmounted, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";
import { downloadDir } from "@tauri-apps/api/path";
import { version } from "../utils.js";
import {
  COOKIES_BROWSER_OPTIONS,
  COOKIES_MODE_OPTIONS,
  DEFAULT_DOWNLOAD_OPTIONS,
  FILENAME_TEMPLATE_OPTIONS,
  FORMAT_OPTIONS,
  STORAGE_KEYS,
  readNumberSetting,
  readOptionSetting,
} from "../settings.js";
import DownloadForm from "./DownloadForm.vue";
import DownloadOptions from "./DownloadOptions.vue";
import DownloadProgress from "./DownloadProgress.vue";
import PreviewPanel from "./PreviewPanel.vue";

const url = ref("");
const progress = ref("");
const error = ref("");
const errorTips = ref([]);
const currentFile = ref("");
const downloadSpeed = ref("");
const eta = ref("");
const queueIndex = ref(0);
const queueTotal = ref(0);
const dir = ref("");
const proxy = ref("");
const cookiesMode = ref(DEFAULT_DOWNLOAD_OPTIONS.cookiesMode);
const cookiesPath = ref(DEFAULT_DOWNLOAD_OPTIONS.cookiesPath);
const cookiesBrowser = ref(DEFAULT_DOWNLOAD_OPTIONS.cookiesBrowser);
const formatPreset = ref(DEFAULT_DOWNLOAD_OPTIONS.format);
const filenameTemplate = ref(DEFAULT_DOWNLOAD_OPTIONS.filenameTemplate);
const retries = ref(DEFAULT_DOWNLOAD_OPTIONS.retries);
const concurrentFragments = ref(DEFAULT_DOWNLOAD_OPTIONS.concurrentFragments);
const isPreviewing = ref(false);
const isDownloading = ref(false);
const isCancelling = ref(false);
const downloadStatus = ref("idle"); // idle, downloading, completed, failed, cancelled
const previewItems = ref([]);

const URL_PATTERN = /^https?:\/\/.+/;

function getErrorMessage(err) {
  if (typeof err === "string") return err;
  return err?.message || String(err);
}

function getDownloadError(message) {
  const normalized = message.toLowerCase();
  if (normalized.includes("ffmpeg")) {
    return {
      message: "FFmpeg 不可用，无法完成合并或音频转换",
      tips: ["安装 FFmpeg", "确认 ffmpeg 已加入系统 PATH", "如不需要转换，可改用普通视频格式"],
    };
  }
  if (
    normalized.includes("proxy") ||
    normalized.includes("connection refused") ||
    normalized.includes("failed to establish")
  ) {
    return {
      message: "网络或代理连接失败",
      tips: ["检查网络连接", "确认代理地址和端口可用", "不需要代理时清空代理设置"],
    };
  }
  if (
    normalized.includes("private video") ||
    normalized.includes("sign in") ||
    normalized.includes("login required") ||
    normalized.includes("forbidden")
  ) {
    return {
      message: "视频需要登录或没有访问权限",
      tips: [
        "确认链接可在浏览器中打开",
        "检查视频是否为私密或会员内容",
        "在设置或首页参数中配置 Cookies 来源",
      ],
    };
  }
  if (normalized.includes("cookie")) {
    return {
      message: "Cookies 不可用或读取失败",
      tips: [
        "确认 cookies.txt 为 Netscape 格式，或改用浏览器读取",
        "如使用浏览器读取，先关闭对应浏览器再重试",
        "确认目标浏览器已登录对应站点",
      ],
    };
  }
  if (
    normalized.includes("requested format is not available") ||
    normalized.includes("format is not available")
  ) {
    return {
      message: "当前下载格式在该视频上不可用",
      tips: ["切换为最佳画质", "尝试降低分辨率限制", "如果只需音频，可改为仅音频 MP3"],
    };
  }
  if (
    normalized.includes("unsupported url") ||
    normalized.includes("not a valid url") ||
    normalized.includes("no video formats found")
  ) {
    return {
      message: "视频链接无效或当前站点不受支持",
      tips: ["确认 URL 以 http:// 或 https:// 开头", "检查链接是否完整", "尝试在浏览器中打开该链接"],
    };
  }

  return {
    message,
    tips: ["URL 是否正确", "网络连接是否正常", "视频链接是否有效", "代理设置是否正确"],
  };
}

function setActionError(rawMessage, prefix = "下载失败") {
  const formatted = getDownloadError(rawMessage);
  error.value = `${prefix}：${formatted.message}`;
  errorTips.value = formatted.tips;
}

function getUrls() {
  return url.value
    .split(/\s+/)
    .map((item) => item.trim())
    .filter(Boolean);
}

function resetDownloadDetails() {
  progress.value = "";
  currentFile.value = "";
  downloadSpeed.value = "";
  eta.value = "";
}

function clearPreview() {
  previewItems.value = [];
}

function parseProgressOutput(output) {
  const destinationMatch =
    output.match(/\[download\]\s+Destination:\s+(.+)/) ||
    output.match(/\[ExtractAudio\]\s+Destination:\s+(.+)/) ||
    output.match(/\[Merger\]\s+Merging formats into "(.+)"/);
  if (destinationMatch) {
    currentFile.value = destinationMatch[1].trim();
  }

  const alreadyDownloadedMatch = output.match(
    /\[download\]\s+(.+)\s+has already been downloaded/,
  );
  if (alreadyDownloadedMatch) {
    currentFile.value = alreadyDownloadedMatch[1].trim();
    progress.value = "100.00";
  }

  const progressMatch = output.match(/\[download\]\s+(\d+\.?\d*)%/);
  if (progressMatch) {
    progress.value = parseFloat(progressMatch[1]).toFixed(2);
  }

  const speedEtaMatch = output.match(/\bat\s+([^\s]+\/s)\s+ETA\s+([^\s]+)/);
  if (speedEtaMatch) {
    downloadSpeed.value = speedEtaMatch[1];
    eta.value = speedEtaMatch[2];
  }
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

async function chooseCookiesFile() {
  const selected = await open({
    multiple: false,
    title: "选择 Cookies 文件",
  });

  if (typeof selected === "string") {
    cookiesPath.value = selected;
  }
}

function clearCookiesFile() {
  cookiesPath.value = "";
}

async function previewInfo() {
  const urls = getUrls();
  if (!urls.length) {
    error.value = "请输入视频链接";
    return;
  }

  const invalidUrl = urls.find((item) => !URL_PATTERN.test(item));
  if (invalidUrl) {
    error.value = `请输入有效的 URL：${invalidUrl}`;
    return;
  }

  error.value = "";
  errorTips.value = [];
  isPreviewing.value = true;

  try {
    const items = await invoke("get_video_info", {
      options: {
        urls,
        proxy: proxy.value,
        cookiesMode: cookiesMode.value,
        cookiesPath: cookiesPath.value,
        cookiesBrowser: cookiesBrowser.value,
        formatPreset: formatPreset.value,
      },
    });

    previewItems.value = Array.isArray(items) ? items : [];
  } catch (err) {
    clearPreview();
    setActionError(getErrorMessage(err), "预览失败");
  } finally {
    isPreviewing.value = false;
  }
}

async function download() {
  const urls = getUrls();
  if (!urls.length) {
    error.value = "请输入视频链接";
    downloadStatus.value = "failed";
    return;
  }

  const invalidUrl = urls.find((item) => !URL_PATTERN.test(item));
  if (invalidUrl) {
    error.value = `请输入有效的 URL：${invalidUrl}`;
    downloadStatus.value = "failed";
    return;
  }

  resetDownloadDetails();
  error.value = "";
  errorTips.value = [];
  isDownloading.value = true;
  downloadStatus.value = "downloading";
  queueTotal.value = urls.length;

  try {
    for (const [index, item] of urls.entries()) {
      queueIndex.value = index + 1;
      resetDownloadDetails();

      await invoke("download", {
        options: {
          url: item,
          dir: dir.value,
          proxy: proxy.value,
          cookiesMode: cookiesMode.value,
          cookiesPath: cookiesPath.value,
          cookiesBrowser: cookiesBrowser.value,
          formatPreset: formatPreset.value,
          filenameTemplate: filenameTemplate.value,
          retries: retries.value,
          concurrentFragments: concurrentFragments.value,
        },
      });
    }

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
      errorTips.value = [];
      downloadStatus.value = "cancelled";
    } else {
      setActionError(message);
      downloadStatus.value = "failed";
    }
  } finally {
    isDownloading.value = false;
    isCancelling.value = false;
    queueIndex.value = 0;
    queueTotal.value = 0;
  }
}

async function cancelDownload() {
  if (!isDownloading.value || isCancelling.value) return;

  isCancelling.value = true;
  try {
    await invoke("cancel_download");
  } catch (err) {
    setActionError(getErrorMessage(err));
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
  cookiesMode.value = readOptionSetting(
    STORAGE_KEYS.cookiesMode,
    COOKIES_MODE_OPTIONS,
    localStorage.getItem(STORAGE_KEYS.cookiesPath)
      ? "file"
      : DEFAULT_DOWNLOAD_OPTIONS.cookiesMode,
  );
  const savedCookiesPath = localStorage.getItem(STORAGE_KEYS.cookiesPath);
  if (savedCookiesPath) cookiesPath.value = savedCookiesPath;
  cookiesBrowser.value = readOptionSetting(
    STORAGE_KEYS.cookiesBrowser,
    COOKIES_BROWSER_OPTIONS,
    DEFAULT_DOWNLOAD_OPTIONS.cookiesBrowser,
  );
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
    parseProgressOutput(event.payload);
  });

  onListenError = listen("yt-dlp-error", (event) => {
    const urlERROR = event.payload.match(/'([^']*)' is not a valid URL\./);
    if (urlERROR) {
      setActionError(urlERROR[0]);
    } else {
      setActionError(event.payload);
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

watch(cookiesMode, (val) => {
  localStorage.setItem(
    STORAGE_KEYS.cookiesMode,
    val || DEFAULT_DOWNLOAD_OPTIONS.cookiesMode,
  );
});

watch(cookiesPath, (val) => {
  if (!val) {
    localStorage.removeItem(STORAGE_KEYS.cookiesPath);
    return;
  }
  localStorage.setItem(STORAGE_KEYS.cookiesPath, val);
});

watch(cookiesBrowser, (val) => {
  localStorage.setItem(
    STORAGE_KEYS.cookiesBrowser,
    val || DEFAULT_DOWNLOAD_OPTIONS.cookiesBrowser,
  );
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

watch([url, proxy, cookiesMode, cookiesPath, cookiesBrowser, formatPreset], () => {
  if (!isPreviewing.value) {
    clearPreview();
  }
});

onUnmounted(() => {
  if (onListenProgress) onListenProgress.then((onListen) => onListen());
  if (onListenError) onListenError.then((onListen) => onListen());
});
</script>

<template>
  <div class="home-page">
    <div class="logo-container">
      <img src="../../src-tauri/icons/logo.png" class="logo" alt="logo" />
    </div>
    <h1 class="appname">
      视频下载器
      <el-tag type="primary" effect="dark" round>{{ version }}</el-tag>
    </h1>

    <el-card class="main-card" shadow="hover">
      <DownloadProgress
        :download-status="downloadStatus"
        :progress="progress"
        :is-downloading="isDownloading"
        :current-file="currentFile"
        :download-speed="downloadSpeed"
        :eta="eta"
        :queue-index="queueIndex"
        :queue-total="queueTotal"
        :error="error"
        :error-tips="errorTips"
        @clear-error="error = ''"
      />
      <DownloadForm
        v-model:url="url"
        :is-previewing="isPreviewing"
        :is-downloading="isDownloading"
        :is-cancelling="isCancelling"
        @preview="previewInfo"
        @download="download"
        @cancel="cancelDownload"
      />
      <PreviewPanel
        :is-previewing="isPreviewing"
        :preview-items="previewItems"
      />
      <el-divider />
      <DownloadOptions
        v-model:dir="dir"
        v-model:cookies-mode="cookiesMode"
        v-model:cookies-path="cookiesPath"
        v-model:cookies-browser="cookiesBrowser"
        v-model:format-preset="formatPreset"
        v-model:filename-template="filenameTemplate"
        v-model:retries="retries"
        v-model:concurrent-fragments="concurrentFragments"
        v-model:proxy="proxy"
        :is-downloading="isDownloading"
        @choose-dir="chooseDir"
        @choose-cookies="chooseCookiesFile"
        @clear-cookies="clearCookiesFile"
      />
    </el-card>

    <footer class="footer">
      <p>
        © 2025 by
        <a target="_blank" href="https://github.com/LongYinStudio"
          >LongYinStudio</a
        >
      </p>
    </footer>
  </div>
</template>

<style scoped>
.home-page {
  width: 100%;
  min-width: 0;
  padding-bottom: 2rem;
}

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
  width: min(90%, 700px);
  max-width: 700px;
  margin: 1.5em auto;
  border-radius: 12px;
  background-color: var(--bg-secondary);
  border-color: var(--border-color);
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

@media (max-width: 720px) {
  .logo-container {
    margin-top: 1.2em;
  }

  .logo {
    width: 5em;
    height: 5em;
    border-radius: 1.1em;
  }

  .appname {
    padding: 0.8em 0;
    font-size: 1.5em;
  }

  .main-card {
    width: calc(100% - 1em);
    margin: 0.75em auto;
  }

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
