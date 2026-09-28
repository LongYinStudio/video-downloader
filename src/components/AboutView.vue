<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { version } from "../utils.js";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ElMessage, ElMessageBox } from "element-plus";
import { ChatLineRound, Download, Refresh } from "@element-plus/icons-vue";
import { STORAGE_KEYS } from "../settings.js";

const isChecking = ref(false);
const updateDialogVisible = ref(false);
const latestRelease = ref({
  tagName: "",
  name: "",
  body: "",
  htmlUrl: "",
});

function compareVersions(v1, v2) {
  const clean1 = (v1 || "").replace(/^v/i, "").split(".").map(Number);
  const clean2 = (v2 || "").replace(/^v/i, "").split(".").map(Number);
  const len = Math.max(clean1.length, clean2.length);
  for (let i = 0; i < len; i++) {
    const num1 = clean1[i] || 0;
    const num2 = clean2[i] || 0;
    if (num1 > num2) return 1;
    if (num1 < num2) return -1;
  }
  return 0;
}

async function checkUpdate() {
  isChecking.value = true;
  try {
    const savedProxy = localStorage.getItem(STORAGE_KEYS.proxy) || "";
    const data = await invoke("check_app_update", {
      proxy: savedProxy || null,
    });

    const latestTag = data.tagName || "";
    latestRelease.value = {
      tagName: latestTag,
      name: data.name || latestTag,
      body: data.body || "暂无版本更新说明",
      htmlUrl:
        data.htmlUrl ||
        "https://github.com/LongYinStudio/video-downloader/releases/latest",
    };

    if (compareVersions(latestTag, version) > 0) {
      updateDialogVisible.value = true;
    } else {
      ElMessage.success(`当前已是最新版本 (${version})`);
    }
  } catch (err) {
    ElMessageBox.confirm(
      `检查更新未成功（${err?.message || err || "网络连接异常"}），是否前往 GitHub Releases 页面查看？`,
      "提示",
      {
        confirmButtonText: "前往查看",
        cancelButtonText: "取消",
        type: "info",
      },
    )
      .then(() => {
        openUrl(
          "https://github.com/LongYinStudio/video-downloader/releases/latest",
        );
      })
      .catch(() => {});
  } finally {
    isChecking.value = false;
  }
}

function goToDownload() {
  openUrl(
    latestRelease.value.htmlUrl ||
      "https://github.com/LongYinStudio/video-downloader/releases/latest",
  );
  updateDialogVisible.value = false;
}

function feedback() {
  openUrl("https://github.com/LongYinStudio/video-downloader/issues/new");
}
</script>

<template>
  <div class="main">
    <h3 class="title">关于软件</h3>

    <el-card class="about-card-wrapper" shadow="hover">
      <div class="about-card">
        <div class="info">
          <img src="../../src-tauri/icons/512x512.png" alt="logo" class="app-icon" />
          <div class="name_version">
            <h4>video-downloader</h4>
            <p>版本: {{ version }}</p>
          </div>
        </div>
        <el-button type="primary" :loading="isChecking" @click="checkUpdate">
          <el-icon><Refresh /></el-icon>
          检查更新
        </el-button>
      </div>
    </el-card>

    <el-card class="about-card-wrapper" shadow="hover" style="margin-top: 1em">
      <div class="about-card">
        <div class="name_version">
          <h4>开源地址</h4>
          <el-link
            target="_blank"
            href="https://github.com/LongYinStudio/video-downloader"
          >
            https://github.com/LongYinStudio/video-downloader
          </el-link>
        </div>
        <el-button type="danger" plain @click="feedback">
          <el-icon><ChatLineRound /></el-icon>
          反馈问题
        </el-button>
      </div>
    </el-card>

    <el-dialog
      v-model="updateDialogVisible"
      title="发现新版本"
      width="min(90%, 520px)"
      align-center
    >
      <div class="update-dialog-content">
        <div class="version-badge-row">
          <el-tag type="info" round>当前版本: {{ version }}</el-tag>
          <span class="arrow-icon">→</span>
          <el-tag type="success" effect="dark" round>最新版本: {{ latestRelease.tagName }}</el-tag>
        </div>
        <div class="release-notes">
          <div class="release-title">{{ latestRelease.name }}</div>
          <pre class="release-body">{{ latestRelease.body }}</pre>
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="updateDialogVisible = false">稍后再说</el-button>
          <el-button type="primary" @click="goToDownload">
            <el-icon><Download /></el-icon>
            前往下载更新
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.main {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  width: 100%;
}

.main .title {
  width: min(90%, 780px);
  margin: 0.8em auto 0.6em;
  text-align: left;
}

.about-card-wrapper {
  width: min(90%, 780px);
  margin: 0 auto;
  border-radius: 12px;
  background-color: var(--bg-secondary);
  border-color: var(--border-color);
}

.about-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1em;
  flex-wrap: wrap;
}

.about-card .info {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.app-icon {
  width: 3.2em;
  height: 3.2em;
  border: 1px solid var(--border-color);
  border-radius: 0.8em;
  box-shadow: var(--shadow-sm);
}

.about-card .name_version {
  margin-left: 1em;
  display: flex;
  flex-direction: column;
  align-items: start;
}

.about-card .name_version h4 {
  margin: 0;
  font-size: 1.1em;
  color: var(--text-primary);
}

.about-card .name_version p {
  margin: 0.25em 0 0;
  color: var(--text-tertiary);
  font-size: 0.85em;
}

.update-dialog-content {
  display: flex;
  flex-direction: column;
  gap: 1em;
  text-align: left;
}

.version-badge-row {
  display: flex;
  align-items: center;
  gap: 0.8em;
}

.arrow-icon {
  color: var(--text-tertiary);
  font-weight: bold;
}

.release-notes {
  background-color: color-mix(in srgb, var(--bg-primary) 70%, var(--bg-secondary));
  padding: 0.9em 1.1em;
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.release-title {
  font-weight: 600;
  margin-bottom: 0.5em;
  color: var(--text-primary);
}

.release-body {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
  font-family: inherit;
  font-size: 0.9em;
  line-height: 1.5;
  color: var(--text-secondary);
  max-height: 220px;
  overflow-y: auto;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.6em;
}

@media (max-width: 600px) {
  .about-card-wrapper,
  .main .title {
    width: calc(100% - 1.2em);
  }

  .about-card {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
