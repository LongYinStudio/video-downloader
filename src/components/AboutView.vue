<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { version } from "../utils.js";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ElMessage, ElMessageBox } from "element-plus";
import {
  ChatLineRound,
  Download,
  Refresh,
  Link,
  InfoFilled,
  Promotion,
} from "@element-plus/icons-vue";
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

function openGithub() {
  openUrl("https://github.com/LongYinStudio/video-downloader");
}
</script>

<template>
  <div class="about-page">
    <div class="page-container">
      <!-- 页面顶部标题栏 -->
      <header class="page-topbar">
        <div class="topbar-title-group">
          <h2 class="page-title">关于软件</h2>
          <span class="page-subtitle">版本信息、架构说明与开源社区维护</span>
        </div>
      </header>

      <div class="about-stack">
        <!-- 软件核心名片卡片 -->
        <section class="identity-card">
          <div class="identity-left">
            <div class="logo-box">
              <img src="../../src-tauri/icons/logo.png" alt="logo" class="identity-logo" />
            </div>
            <div class="identity-details">
              <div class="name-version-row">
                <h3 class="app-display-name">视频下载器</h3>
                <el-tag size="small" type="primary" effect="light" round class="version-tag">
                  {{ version }}
                </el-tag>
              </div>
              <p class="app-tagline">
                跨平台桌面端音视频批量下载工具，基于 Tauri 2.0 与 yt-dlp 构建。
              </p>
            </div>
          </div>

          <div class="identity-actions">
            <el-button
              type="primary"
              size="default"
              class="action-btn check-btn"
              :loading="isChecking"
              @click="checkUpdate"
            >
              <el-icon><Refresh /></el-icon>
              检查更新
            </el-button>
            <el-button
              size="default"
              plain
              class="action-btn"
              @click="feedback"
            >
              <el-icon><ChatLineRound /></el-icon>
              反馈建议
            </el-button>
          </div>
        </section>

        <!-- 开源与技术栈卡片 -->
        <section class="tech-card">
          <div class="card-header">
            <h4 class="card-title">技术架构与协议</h4>
            <span class="card-desc">本项目完全开源且尊重开源许可</span>
          </div>

          <div class="tech-chips-grid">
            <div class="tech-chip">
              <span class="chip-name">核心框架</span>
              <span class="chip-val">Tauri 2.0 + Rust</span>
            </div>
            <div class="tech-chip">
              <span class="chip-name">前端视图</span>
              <span class="chip-val">Vue 3 + Vite</span>
            </div>
            <div class="tech-chip">
              <span class="chip-name">下载内核</span>
              <span class="chip-val">yt-dlp</span>
            </div>
            <div class="tech-chip">
              <span class="chip-name">媒体转码</span>
              <span class="chip-val">FFmpeg</span>
            </div>
          </div>

          <div class="github-repo-banner">
            <div class="repo-info">
              <el-icon class="repo-icon"><Promotion /></el-icon>
              <div class="repo-texts">
                <span class="repo-title">开源代码仓库</span>
                <span class="repo-url">github.com/LongYinStudio/video-downloader</span>
              </div>
            </div>
            <el-button size="small" type="primary" plain class="repo-btn" @click="openGithub">
              <el-icon><Link /></el-icon>
              访问仓库
            </el-button>
          </div>
        </section>

        <!-- 底部版权声明 -->
        <footer class="about-footer">
          <p>© 2025 - 2026 by LongYinStudio · Released under the MIT License</p>
        </footer>
      </div>
    </div>

    <!-- 更新弹窗 -->
    <el-dialog
      v-model="updateDialogVisible"
      title="发现新版本"
      width="min(90%, 500px)"
      align-center
      class="modern-dialog"
    >
      <div class="update-dialog-content">
        <div class="version-badge-row">
          <el-tag type="info" round size="default">当前: {{ version }}</el-tag>
          <span class="arrow-icon">→</span>
          <el-tag type="success" effect="dark" round size="default">最新: {{ latestRelease.tagName }}</el-tag>
        </div>
        <div class="release-notes-box">
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
.about-page {
  width: 100%;
  min-height: 100%;
  padding: 1.5rem 1.75rem 2.5rem;
}

.page-container {
  width: 100%;
  max-width: 860px;
  margin: 0 auto;
}

.page-topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.25rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--border-subtle);
}

.topbar-title-group {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.2rem;
}

.page-title {
  margin: 0;
  font-size: 1.35rem;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.01em;
}

.page-subtitle {
  font-size: 0.84rem;
  color: var(--text-tertiary);
}

.about-stack {
  display: flex;
  flex-direction: column;
  gap: 1.15rem;
}

/* 核心名片卡片 */
.identity-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1.5rem;
  padding: 1.5rem 1.75rem;
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  flex-wrap: wrap;
}

.identity-left {
  display: flex;
  align-items: center;
  gap: 1.25rem;
}

.logo-box {
  width: 64px;
  height: 64px;
  border-radius: 16px;
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.15), rgba(99, 102, 241, 0.25));
  display: grid;
  place-items: center;
  flex-shrink: 0;
  box-shadow: var(--shadow-sm);
}

.identity-logo {
  width: 50px;
  height: 50px;
  border-radius: 12px;
  object-fit: cover;
}

.identity-details {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.35rem;
  text-align: left;
}

.name-version-row {
  display: flex;
  align-items: center;
  gap: 0.65rem;
}

.app-display-name {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--text-primary);
}

.version-tag {
  font-weight: 600;
}

.app-tagline {
  margin: 0;
  font-size: 0.86rem;
  color: var(--text-secondary);
  line-height: 1.4;
  max-width: 420px;
}

.identity-actions {
  display: flex;
  align-items: center;
  gap: 0.65rem;
}

.action-btn {
  border-radius: var(--radius-md);
  font-weight: 600;
}

/* 技术架构卡片 */
.tech-card {
  padding: 1.25rem 1.4rem;
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  display: flex;
  flex-direction: column;
  gap: 1rem;
  text-align: left;
}

.card-header {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
}

.card-title {
  margin: 0;
  font-size: 1rem;
  font-weight: 700;
  color: var(--text-primary);
}

.card-desc {
  font-size: 0.82rem;
  color: var(--text-tertiary);
}

.tech-chips-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 0.75rem;
}

.tech-chip {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 0.75rem 0.95rem;
  background-color: var(--bg-tertiary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.chip-name {
  font-size: 0.76rem;
  color: var(--text-tertiary);
}

.chip-val {
  font-size: 0.88rem;
  font-weight: 600;
  color: var(--text-primary);
}

/* GitHub 仓库横幅 */
.github-repo-banner {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.85rem 1rem;
  background-color: var(--bg-tertiary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
  flex-wrap: wrap;
  gap: 0.75rem;
}

.repo-info {
  display: flex;
  align-items: center;
  gap: 0.65rem;
}

.repo-icon {
  font-size: 1.25rem;
  color: var(--primary-color);
}

.repo-texts {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
}

.repo-title {
  font-size: 0.88rem;
  font-weight: 600;
  color: var(--text-primary);
}

.repo-url {
  font-size: 0.78rem;
  color: var(--text-tertiary);
}

.repo-btn {
  border-radius: var(--radius-sm);
}

.about-footer {
  margin-top: 1rem;
  text-align: center;
  font-size: 0.82rem;
  color: var(--text-tertiary);
}

/* 更新弹窗内容 */
.update-dialog-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  text-align: left;
}

.version-badge-row {
  display: flex;
  align-items: center;
  gap: 0.8rem;
}

.arrow-icon {
  color: var(--text-tertiary);
  font-weight: bold;
}

.release-notes-box {
  background-color: var(--bg-tertiary);
  padding: 0.9rem 1.1rem;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.release-title {
  font-weight: 700;
  margin-bottom: 0.5rem;
  color: var(--text-primary);
  font-size: 0.95rem;
}

.release-body {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
  font-family: inherit;
  font-size: 0.86rem;
  line-height: 1.5;
  color: var(--text-secondary);
  max-height: 220px;
  overflow-y: auto;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.6rem;
}

@media (max-width: 640px) {
  .about-page {
    padding: 1rem 0.75rem 2rem;
  }

  .identity-card {
    flex-direction: column;
    align-items: flex-start;
  }

  .identity-actions {
    width: 100%;
  }

  .tech-chips-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>
