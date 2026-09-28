<script setup>
import { onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { openUrl } from "@tauri-apps/plugin-opener";
import {
  Refresh,
  Folder,
  Delete,
  Sunny,
  Moon,
  Monitor,
  Tools,
  QuestionFilled,
} from "@element-plus/icons-vue";
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

const themeMode = ref("system");
const defaultDir = ref("");
const defaultProxy = ref("");
const defaultCookiesMode = ref(DEFAULT_DOWNLOAD_OPTIONS.cookiesMode);
const defaultCookiesPath = ref(DEFAULT_DOWNLOAD_OPTIONS.cookiesPath);
const defaultCookiesBrowser = ref(DEFAULT_DOWNLOAD_OPTIONS.cookiesBrowser);
const defaultFormat = ref(DEFAULT_DOWNLOAD_OPTIONS.format);
const defaultFilenameTemplate = ref(DEFAULT_DOWNLOAD_OPTIONS.filenameTemplate);
const defaultRetries = ref(DEFAULT_DOWNLOAD_OPTIONS.retries);
const defaultConcurrentFragments = ref(DEFAULT_DOWNLOAD_OPTIONS.concurrentFragments);
const autoOpenDir = ref(true);
const autoPasteClipboard = ref(false);

const isCheckingEnv = ref(false);
const envInfo = ref({
  ffmpegAvailable: false,
  ffmpegVersion: "",
  ytdlpAvailable: false,
  ytdlpVersion: "",
});

async function detectEnvironment() {
  isCheckingEnv.value = true;
  try {
    const res = await invoke("check_environment");
    envInfo.value = res;
  } catch (err) {
    console.error("Failed to detect environment:", err);
  } finally {
    isCheckingEnv.value = false;
  }
}

function applyTheme(mode) {
  const root = document.documentElement;
  if (!mode || mode === "system") {
    root.removeAttribute("data-theme");
    return;
  }
  root.setAttribute("data-theme", mode);
}

onMounted(() => {
  themeMode.value = localStorage.getItem(STORAGE_KEYS.theme) || "system";
  defaultDir.value = localStorage.getItem(STORAGE_KEYS.dir) || "";
  defaultProxy.value = localStorage.getItem(STORAGE_KEYS.proxy) || "";
  defaultCookiesMode.value = readOptionSetting(
    STORAGE_KEYS.cookiesMode,
    COOKIES_MODE_OPTIONS,
    localStorage.getItem(STORAGE_KEYS.cookiesPath)
      ? "file"
      : DEFAULT_DOWNLOAD_OPTIONS.cookiesMode,
  );
  defaultCookiesPath.value =
    localStorage.getItem(STORAGE_KEYS.cookiesPath) ||
    DEFAULT_DOWNLOAD_OPTIONS.cookiesPath;
  defaultCookiesBrowser.value = readOptionSetting(
    STORAGE_KEYS.cookiesBrowser,
    COOKIES_BROWSER_OPTIONS,
    DEFAULT_DOWNLOAD_OPTIONS.cookiesBrowser,
  );
  defaultFormat.value = readOptionSetting(
    STORAGE_KEYS.format,
    FORMAT_OPTIONS,
    DEFAULT_DOWNLOAD_OPTIONS.format,
  );
  defaultFilenameTemplate.value = readOptionSetting(
    STORAGE_KEYS.filenameTemplate,
    FILENAME_TEMPLATE_OPTIONS,
    DEFAULT_DOWNLOAD_OPTIONS.filenameTemplate,
  );
  defaultRetries.value = readNumberSetting(
    STORAGE_KEYS.retries,
    DEFAULT_DOWNLOAD_OPTIONS.retries,
    0,
    20,
  );
  defaultConcurrentFragments.value = readNumberSetting(
    STORAGE_KEYS.concurrentFragments,
    DEFAULT_DOWNLOAD_OPTIONS.concurrentFragments,
    1,
    16,
  );
  autoOpenDir.value = localStorage.getItem(STORAGE_KEYS.autoOpenDir) !== "false";
  autoPasteClipboard.value =
    localStorage.getItem(STORAGE_KEYS.autoPasteClipboard) === "true";
  applyTheme(themeMode.value);
  detectEnvironment();
});

watch(themeMode, (val) => {
  localStorage.setItem(STORAGE_KEYS.theme, val);
  applyTheme(val);
});

watch(defaultDir, (val) => {
  if (!val) {
    localStorage.removeItem(STORAGE_KEYS.dir);
    return;
  }
  localStorage.setItem(STORAGE_KEYS.dir, val);
});

watch(defaultProxy, (val) => {
  if (!val) {
    localStorage.removeItem(STORAGE_KEYS.proxy);
    return;
  }
  localStorage.setItem(STORAGE_KEYS.proxy, val);
});

watch(defaultCookiesMode, (val) => {
  localStorage.setItem(
    STORAGE_KEYS.cookiesMode,
    val || DEFAULT_DOWNLOAD_OPTIONS.cookiesMode,
  );
});

watch(defaultCookiesPath, (val) => {
  if (!val) {
    localStorage.removeItem(STORAGE_KEYS.cookiesPath);
    return;
  }
  localStorage.setItem(STORAGE_KEYS.cookiesPath, val);
});

watch(defaultCookiesBrowser, (val) => {
  localStorage.setItem(
    STORAGE_KEYS.cookiesBrowser,
    val || DEFAULT_DOWNLOAD_OPTIONS.cookiesBrowser,
  );
});

watch(defaultFormat, (val) => {
  localStorage.setItem(STORAGE_KEYS.format, val || DEFAULT_DOWNLOAD_OPTIONS.format);
});

watch(defaultFilenameTemplate, (val) => {
  localStorage.setItem(
    STORAGE_KEYS.filenameTemplate,
    val || DEFAULT_DOWNLOAD_OPTIONS.filenameTemplate,
  );
});

watch(defaultRetries, (val) => {
  localStorage.setItem(STORAGE_KEYS.retries, String(val));
});

watch(defaultConcurrentFragments, (val) => {
  localStorage.setItem(STORAGE_KEYS.concurrentFragments, String(val));
});

watch(autoOpenDir, (val) => {
  localStorage.setItem(STORAGE_KEYS.autoOpenDir, String(val));
});

watch(autoPasteClipboard, (val) => {
  localStorage.setItem(STORAGE_KEYS.autoPasteClipboard, String(val));
});

async function chooseDir() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "选择目录",
  });

  if (selected) {
    defaultDir.value = selected;
  }
}

async function chooseCookiesFile() {
  const selected = await open({
    multiple: false,
    title: "选择 Cookies 文件",
  });

  if (typeof selected === "string") {
    defaultCookiesPath.value = selected;
  }
}

function clearDir() {
  defaultDir.value = "";
}

function clearCookiesFile() {
  defaultCookiesPath.value = "";
}

function resetAll() {
  themeMode.value = "system";
  defaultDir.value = "";
  defaultProxy.value = "";
  defaultCookiesMode.value = DEFAULT_DOWNLOAD_OPTIONS.cookiesMode;
  defaultCookiesPath.value = DEFAULT_DOWNLOAD_OPTIONS.cookiesPath;
  defaultCookiesBrowser.value = DEFAULT_DOWNLOAD_OPTIONS.cookiesBrowser;
  defaultFormat.value = DEFAULT_DOWNLOAD_OPTIONS.format;
  defaultFilenameTemplate.value = DEFAULT_DOWNLOAD_OPTIONS.filenameTemplate;
  defaultRetries.value = DEFAULT_DOWNLOAD_OPTIONS.retries;
  defaultConcurrentFragments.value = DEFAULT_DOWNLOAD_OPTIONS.concurrentFragments;
  autoOpenDir.value = true;
  autoPasteClipboard.value = false;
  localStorage.removeItem(STORAGE_KEYS.theme);
  localStorage.removeItem(STORAGE_KEYS.dir);
  localStorage.removeItem(STORAGE_KEYS.proxy);
  localStorage.removeItem(STORAGE_KEYS.cookiesMode);
  localStorage.removeItem(STORAGE_KEYS.cookiesPath);
  localStorage.removeItem(STORAGE_KEYS.cookiesBrowser);
  localStorage.removeItem(STORAGE_KEYS.format);
  localStorage.removeItem(STORAGE_KEYS.filenameTemplate);
  localStorage.removeItem(STORAGE_KEYS.retries);
  localStorage.removeItem(STORAGE_KEYS.concurrentFragments);
  localStorage.removeItem(STORAGE_KEYS.autoOpenDir);
  localStorage.removeItem(STORAGE_KEYS.autoPasteClipboard);
  applyTheme("system");
}
</script>

<template>
  <div class="settings-page">
    <div class="page-container">
      <!-- 页面顶部标题栏 -->
      <header class="page-topbar">
        <div class="topbar-title-group">
          <h2 class="page-title">参数设置</h2>
          <span class="page-subtitle">定制应用外观、下载规则与内核运行环境</span>
        </div>
      </header>

      <div class="settings-stack">
        <!-- 1. 外观主题卡片 -->
        <section class="config-card">
          <div class="card-header">
            <h3 class="card-title">界面外观</h3>
            <span class="card-desc">选择符合使用习惯的主题模式</span>
          </div>
          <div class="theme-options">
            <div
              class="theme-chip"
              :class="{ 'is-selected': themeMode === 'system' }"
              @click="themeMode = 'system'"
            >
              <el-icon><Monitor /></el-icon>
              <span>跟随系统</span>
            </div>
            <div
              class="theme-chip"
              :class="{ 'is-selected': themeMode === 'light' }"
              @click="themeMode = 'light'"
            >
              <el-icon><Sunny /></el-icon>
              <span>浅色模式</span>
            </div>
            <div
              class="theme-chip"
              :class="{ 'is-selected': themeMode === 'dark' }"
              @click="themeMode = 'dark'"
            >
              <el-icon><Moon /></el-icon>
              <span>深色模式</span>
            </div>
          </div>
        </section>

        <!-- 2. 环境健康诊断 -->
        <section class="config-card">
          <div class="card-header-with-action">
            <div>
              <h3 class="card-title">环境健康诊断</h3>
              <span class="card-desc">检测音视频转换与解析内核状态</span>
            </div>
            <el-button
              size="small"
              plain
              class="refresh-btn"
              :loading="isCheckingEnv"
              @click="detectEnvironment"
            >
              <el-icon><Refresh /></el-icon>
              重新检测
            </el-button>
          </div>

          <div class="env-cards-row">
            <div class="env-status-box">
              <div class="env-top">
                <span class="env-label">FFmpeg 转换器</span>
                <el-tag
                  :type="envInfo.ffmpegAvailable ? 'success' : 'danger'"
                  size="small"
                  effect="light"
                  round
                >
                  {{ envInfo.ffmpegAvailable ? '已检测到' : '未检测到' }}
                </el-tag>
              </div>
              <div class="env-info-text">
                <span v-if="envInfo.ffmpegAvailable">版本: {{ envInfo.ffmpegVersion }}</span>
                <span v-else class="env-warn-text">
                  系统 PATH 未找到 ffmpeg，音视频合并与 MP3 转换可能受限。
                </span>
              </div>
              <el-button
                v-if="!envInfo.ffmpegAvailable"
                type="primary"
                link
                size="small"
                class="install-link"
                @click="openUrl('https://ffmpeg.org/download.html')"
              >
                前往安装 FFmpeg →
              </el-button>
            </div>

            <div class="env-status-box">
              <div class="env-top">
                <span class="env-label">yt-dlp 解析内核</span>
                <el-tag
                  :type="envInfo.ytdlpAvailable ? 'success' : 'danger'"
                  size="small"
                  effect="light"
                  round
                >
                  {{ envInfo.ytdlpAvailable ? '就绪' : '异常' }}
                </el-tag>
              </div>
              <div class="env-info-text">
                <span>版本: {{ envInfo.ytdlpVersion || "检测中..." }}</span>
              </div>
            </div>
          </div>
        </section>

        <!-- 3. 下载默认规则 -->
        <section class="config-card">
          <div class="card-header">
            <h3 class="card-title">下载默认值</h3>
            <span class="card-desc">配置启动应用时的默认参数预设</span>
          </div>

          <div class="fields-list">
            <!-- 默认保存目录 -->
            <div class="form-row">
              <div class="row-info">
                <span class="row-label">默认保存目录</span>
                <span class="row-desc">未指定时默认使用系统 Downloads 目录</span>
              </div>
              <div class="row-control-wide">
                <el-input
                  v-model="defaultDir"
                  placeholder="系统 Downloads 目录"
                  readonly
                  size="default"
                >
                  <template #prefix>
                    <el-icon><Folder /></el-icon>
                  </template>
                </el-input>
                <el-button size="default" @click="chooseDir">选择</el-button>
                <el-button v-if="defaultDir" size="default" text class="clear-btn" @click="clearDir">
                  清除
                </el-button>
              </div>
            </div>

            <!-- 默认格式与命名模板 -->
            <div class="form-grid-two">
              <div class="form-col">
                <span class="row-label">默认下载格式</span>
                <el-select v-model="defaultFormat" size="default" class="full-width">
                  <el-option
                    v-for="item in FORMAT_OPTIONS"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value"
                  />
                </el-select>
              </div>
              <div class="form-col">
                <span class="row-label">默认文件名模板</span>
                <el-select v-model="defaultFilenameTemplate" size="default" class="full-width">
                  <el-option
                    v-for="item in FILENAME_TEMPLATE_OPTIONS"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value"
                  />
                </el-select>
              </div>
            </div>

            <!-- Cookies 默认来源 -->
            <div class="form-row">
              <div class="row-info">
                <span class="row-label">Cookies 登录态来源</span>
                <span class="row-desc">用于会员或限制访问内容的认证</span>
              </div>
              <div class="row-control-stack">
                <el-select v-model="defaultCookiesMode" size="default" class="full-width">
                  <el-option
                    v-for="item in COOKIES_MODE_OPTIONS"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value"
                  />
                </el-select>

                <div v-if="defaultCookiesMode === 'browser'" class="sub-row">
                  <el-select
                    v-model="defaultCookiesBrowser"
                    size="default"
                    class="full-width"
                    placeholder="选择目标浏览器"
                  >
                    <el-option
                      v-for="item in COOKIES_BROWSER_OPTIONS"
                      :key="item.value"
                      :label="item.label"
                      :value="item.value"
                    />
                  </el-select>
                </div>

                <div v-else-if="defaultCookiesMode === 'file'" class="sub-row row-control-wide">
                  <el-input
                    v-model="defaultCookiesPath"
                    placeholder="未选择 cookies.txt"
                    readonly
                    size="default"
                  />
                  <el-button size="default" @click="chooseCookiesFile">选择</el-button>
                  <el-button v-if="defaultCookiesPath" size="default" text class="clear-btn" @click="clearCookiesFile">
                    清除
                  </el-button>
                </div>
              </div>
            </div>

            <!-- 重试与分片 -->
            <div class="form-grid-two">
              <div class="form-col">
                <span class="row-label">重试次数 (0-20)</span>
                <el-input-number
                  v-model="defaultRetries"
                  :min="0"
                  :max="20"
                  size="default"
                  controls-position="right"
                  class="full-width"
                />
              </div>
              <div class="form-col">
                <span class="row-label">并发分片数 (1-16)</span>
                <el-input-number
                  v-model="defaultConcurrentFragments"
                  :min="1"
                  :max="16"
                  size="default"
                  controls-position="right"
                  class="full-width"
                />
              </div>
            </div>

            <!-- 网络代理 -->
            <div class="form-row">
              <div class="row-info">
                <span class="row-label">默认网络代理</span>
                <span class="row-desc">支持 HTTP / SOCKS5，例如 http://127.0.0.1:7890</span>
              </div>
              <div class="row-control-wide">
                <el-input
                  v-model="defaultProxy"
                  placeholder="可选: http://127.0.0.1:7890"
                  clearable
                  size="default"
                />
              </div>
            </div>
          </div>
        </section>

        <!-- 4. 偏好与系统 -->
        <section class="config-card">
          <div class="card-header">
            <h3 class="card-title">快捷偏好</h3>
            <span class="card-desc">日常使用体验优化</span>
          </div>

          <div class="switch-list">
            <div class="switch-row">
              <div class="switch-info">
                <span class="switch-title">下载完成后自动打开目标文件夹</span>
                <span class="switch-desc">任务下载成功后直接在系统文件管理器中展示</span>
              </div>
              <el-switch v-model="autoOpenDir" />
            </div>

            <div class="switch-row">
              <div class="switch-info">
                <span class="switch-title">启动时自动粘贴剪贴板中的视频链接</span>
                <span class="switch-desc">启动软件若剪贴板有合规 URL 则自动填充至输入框</span>
              </div>
              <el-switch v-model="autoPasteClipboard" />
            </div>

            <div class="switch-row danger-zone">
              <div class="switch-info">
                <span class="switch-title danger-text">重置所有配置</span>
                <span class="switch-desc">清空所有持久化的下载预设与偏好设置</span>
              </div>
              <el-popconfirm
                title="确定恢复所有设置为出厂默认值吗？"
                confirm-button-text="确定重置"
                cancel-button-text="取消"
                confirm-button-type="danger"
                @confirm="resetAll"
              >
                <template #reference>
                  <el-button type="danger" plain size="small">重置设置</el-button>
                </template>
              </el-popconfirm>
            </div>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
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

.settings-stack {
  display: flex;
  flex-direction: column;
  gap: 1.15rem;
}

/* 配置卡片 */
.config-card {
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: 1.25rem 1.4rem;
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

.card-header-with-action {
  display: flex;
  justify-content: space-between;
  align-items: center;
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

/* 主题分段按钮 */
.theme-options {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 0.75rem;
}

.theme-chip {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
  background-color: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.theme-chip:hover {
  background-color: var(--border-subtle);
  color: var(--text-primary);
}

.theme-chip.is-selected {
  background-color: var(--primary-subtle);
  border-color: var(--primary-color);
  color: var(--primary-color);
  font-weight: 600;
}

/* 环境诊断卡片行 */
.env-cards-row {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 1rem;
}

.env-status-box {
  padding: 0.95rem 1.15rem;
  background-color: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}

.env-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.env-label {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
}

.env-info-text {
  font-size: 0.82rem;
  color: var(--text-secondary);
}

.env-warn-text {
  color: var(--danger-color);
  line-height: 1.4;
}

.install-link {
  align-self: flex-start;
  padding: 0;
  font-size: 0.8rem;
}

/* 字段列表 */
.fields-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1.5rem;
  flex-wrap: wrap;
}

.row-info {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
  flex: 1;
  min-width: 220px;
}

.row-label {
  font-size: 0.88rem;
  font-weight: 600;
  color: var(--text-primary);
}

.row-desc {
  font-size: 0.8rem;
  color: var(--text-tertiary);
}

.row-control-wide {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex: 1.4;
  min-width: 260px;
}

.row-control-stack {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex: 1.4;
  min-width: 260px;
}

.form-grid-two {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.form-col {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.full-width {
  width: 100%;
}

.clear-btn {
  color: var(--danger-color);
}

/* 开关行 */
.switch-list {
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
}

.switch-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.65rem 0.85rem;
  border-radius: var(--radius-md);
  background-color: var(--bg-tertiary);
}

.switch-info {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
}

.switch-title {
  font-size: 0.88rem;
  font-weight: 600;
  color: var(--text-primary);
}

.switch-desc {
  font-size: 0.78rem;
  color: var(--text-tertiary);
}

.danger-text {
  color: var(--danger-color);
}

@media (max-width: 640px) {
  .settings-page {
    padding: 1rem 0.75rem 2rem;
  }

  .theme-options,
  .env-cards-row,
  .form-grid-two {
    grid-template-columns: 1fr;
  }

  .form-row {
    flex-direction: column;
    align-items: stretch;
  }

  .row-control-wide,
  .row-control-stack {
    width: 100%;
  }
}
</style>
