<script setup>
import { computed, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";
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
  LANGUAGE_OPTIONS,
  STORAGE_KEYS,
  getCookiesModeOptions,
  getFilenameTemplateOptions,
  getFormatOptions,
  readNumberSetting,
  readOptionSetting,
} from "../settings.js";
import { getDefaultLanguage, setAppLanguage } from "../i18n/index.js";

const { t, locale } = useI18n();
const currentLanguage = ref(locale.value);
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

const formatOptions = computed(() => getFormatOptions(t));
const filenameTemplateOptions = computed(() => getFilenameTemplateOptions(t));
const cookiesModeOptions = computed(() => getCookiesModeOptions(t));

const isCheckingEnv = ref(false);
const envInfo = ref({
  ffmpegAvailable: false,
  ffmpegVersion: "",
  ytdlpAvailable: false,
  ytdlpVersion: "",
});

function changeLanguage(lang) {
  currentLanguage.value = lang;
  setAppLanguage(lang);
}

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
  currentLanguage.value = locale.value;
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
    title: t("settings.defaultDir"),
  });

  if (selected) {
    defaultDir.value = selected;
  }
}

async function chooseCookiesFile() {
  const selected = await open({
    multiple: false,
    title: t("options.selectFile"),
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
          <h2 class="page-title">{{ t("settings.title") }}</h2>
          <span class="page-subtitle">{{ t("settings.subtitle") }}</span>
        </div>
      </header>

      <div class="settings-stack">
        <!-- 1. 界面语言卡片 -->
        <section class="config-card">
          <div class="card-header">
            <h3 class="card-title">{{ t("settings.language") }}</h3>
            <span class="card-desc">{{ t("settings.languageDesc") }}</span>
          </div>
          <div class="theme-options">
            <div
              v-for="lang in LANGUAGE_OPTIONS"
              :key="lang.value"
              class="theme-chip"
              :class="{ 'is-selected': currentLanguage === lang.value }"
              @click="changeLanguage(lang.value)"
            >
              <span>{{ lang.label }}</span>
            </div>
          </div>
        </section>

        <!-- 2. 外观主题卡片 -->
        <section class="config-card">
          <div class="card-header">
            <h3 class="card-title">{{ t("settings.appearance") }}</h3>
            <span class="card-desc">{{ t("settings.appearanceDesc") }}</span>
          </div>
          <div class="theme-options">
            <div
              class="theme-chip"
              :class="{ 'is-selected': themeMode === 'system' }"
              @click="themeMode = 'system'"
            >
              <el-icon><Monitor /></el-icon>
              <span>{{ t("settings.themeSystem") }}</span>
            </div>
            <div
              class="theme-chip"
              :class="{ 'is-selected': themeMode === 'light' }"
              @click="themeMode = 'light'"
            >
              <el-icon><Sunny /></el-icon>
              <span>{{ t("settings.themeLight") }}</span>
            </div>
            <div
              class="theme-chip"
              :class="{ 'is-selected': themeMode === 'dark' }"
              @click="themeMode = 'dark'"
            >
              <el-icon><Moon /></el-icon>
              <span>{{ t("settings.themeDark") }}</span>
            </div>
          </div>
        </section>

        <!-- 3. 环境健康诊断 -->
        <section class="config-card">
          <div class="card-header-with-action">
            <div>
              <h3 class="card-title">{{ t("settings.envTitle") }}</h3>
              <span class="card-desc">{{ t("settings.envDesc") }}</span>
            </div>
            <el-button
              size="small"
              plain
              class="refresh-btn"
              :loading="isCheckingEnv"
              @click="detectEnvironment"
            >
              <el-icon><Refresh /></el-icon>
              {{ t("settings.recheck") }}
            </el-button>
          </div>

          <div class="env-cards-row">
            <div class="env-status-box">
              <div class="env-top">
                <span class="env-label">{{ t("settings.ffmpegTitle") }}</span>
                <el-tag
                  :type="envInfo.ffmpegAvailable ? 'success' : 'danger'"
                  size="small"
                  effect="light"
                  round
                >
                  {{ envInfo.ffmpegAvailable ? t("settings.detected") : t("settings.notDetected") }}
                </el-tag>
              </div>
              <div class="env-info-text">
                <span v-if="envInfo.ffmpegAvailable">{{ t("settings.version", { version: envInfo.ffmpegVersion }) }}</span>
                <span v-else class="env-warn-text">
                  {{ t("settings.ffmpegWarning") }}
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
                {{ t("settings.installFfmpeg") }}
              </el-button>
            </div>

            <div class="env-status-box">
              <div class="env-top">
                <span class="env-label">{{ t("settings.ytdlpTitle") }}</span>
                <el-tag
                  :type="envInfo.ytdlpAvailable ? 'success' : 'danger'"
                  size="small"
                  effect="light"
                  round
                >
                  {{ envInfo.ytdlpAvailable ? t("settings.ytdlpReady") : t("settings.ytdlpAbnormal") }}
                </el-tag>
              </div>
              <div class="env-info-text">
                <span>{{ t("settings.version", { version: envInfo.ytdlpVersion || t("settings.checking") }) }}</span>
              </div>
            </div>
          </div>
        </section>

        <!-- 4. 下载默认规则 -->
        <section class="config-card">
          <div class="card-header">
            <h3 class="card-title">{{ t("settings.defaultsTitle") }}</h3>
            <span class="card-desc">{{ t("settings.defaultsDesc") }}</span>
          </div>

          <div class="fields-list">
            <!-- 默认保存目录 -->
            <div class="form-row">
              <div class="row-info">
                <span class="row-label">{{ t("settings.defaultDir") }}</span>
                <span class="row-desc">{{ t("settings.defaultDirDesc") }}</span>
              </div>
              <div class="row-control-wide">
                <el-input
                  v-model="defaultDir"
                  :placeholder="t('settings.defaultDirPlaceholder')"
                  readonly
                  size="default"
                >
                  <template #prefix>
                    <el-icon><Folder /></el-icon>
                  </template>
                </el-input>
                <el-button size="default" @click="chooseDir">{{ t("common.select") }}</el-button>
                <el-button v-if="defaultDir" size="default" text class="clear-btn" @click="clearDir">
                  {{ t("common.clear") }}
                </el-button>
              </div>
            </div>

            <!-- 默认格式与命名模板 -->
            <div class="form-grid-two">
              <div class="form-col">
                <span class="row-label">{{ t("settings.defaultFormat") }}</span>
                <el-select v-model="defaultFormat" size="default" class="full-width">
                  <el-option
                    v-for="item in formatOptions"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value"
                  />
                </el-select>
              </div>
              <div class="form-col">
                <span class="row-label">{{ t("settings.defaultFilename") }}</span>
                <el-select v-model="defaultFilenameTemplate" size="default" class="full-width">
                  <el-option
                    v-for="item in filenameTemplateOptions"
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
                <span class="row-label">{{ t("settings.cookiesSource") }}</span>
                <span class="row-desc">{{ t("settings.cookiesSourceDesc") }}</span>
              </div>
              <div class="row-control-stack">
                <el-select v-model="defaultCookiesMode" size="default" class="full-width">
                  <el-option
                    v-for="item in cookiesModeOptions"
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
                    :placeholder="t('settings.selectBrowserPlaceholder')"
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
                    :placeholder="t('options.noCookiesFile')"
                    readonly
                    size="default"
                  />
                  <el-button size="default" @click="chooseCookiesFile">{{ t("common.select") }}</el-button>
                  <el-button v-if="defaultCookiesPath" size="default" text class="clear-btn" @click="clearCookiesFile">
                    {{ t("common.clear") }}
                  </el-button>
                </div>
              </div>
            </div>

            <!-- 重试与分片 -->
            <div class="form-grid-two">
              <div class="form-col">
                <span class="row-label">{{ t("settings.retriesLabel") }}</span>
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
                <span class="row-label">{{ t("settings.fragmentsLabel") }}</span>
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
                <span class="row-label">{{ t("settings.defaultProxy") }}</span>
                <span class="row-desc">{{ t("settings.defaultProxyDesc") }}</span>
              </div>
              <div class="row-control-wide">
                <el-input
                  v-model="defaultProxy"
                  :placeholder="t('settings.proxyPlaceholder')"
                  clearable
                  size="default"
                />
              </div>
            </div>
          </div>
        </section>

        <!-- 5. 偏好与系统 -->
        <section class="config-card">
          <div class="card-header">
            <h3 class="card-title">{{ t("settings.preferencesTitle") }}</h3>
            <span class="card-desc">{{ t("settings.preferencesDesc") }}</span>
          </div>

          <div class="switch-list">
            <div class="switch-row">
              <div class="switch-info">
                <span class="switch-title">{{ t("settings.autoOpenDir") }}</span>
                <span class="switch-desc">{{ t("settings.autoOpenDirDesc") }}</span>
              </div>
              <el-switch v-model="autoOpenDir" />
            </div>

            <div class="switch-row">
              <div class="switch-info">
                <span class="switch-title">{{ t("settings.autoPaste") }}</span>
                <span class="switch-desc">{{ t("settings.autoPasteDesc") }}</span>
              </div>
              <el-switch v-model="autoPasteClipboard" />
            </div>

            <div class="switch-row danger-zone">
              <div class="switch-info">
                <span class="switch-title danger-text">{{ t("settings.resetTitle") }}</span>
                <span class="switch-desc">{{ t("settings.resetDesc") }}</span>
              </div>
              <el-popconfirm
                :title="t('settings.resetConfirm')"
                :confirm-button-text="t('settings.resetConfirmBtn')"
                :cancel-button-text="t('common.cancel')"
                confirm-button-type="danger"
                @confirm="resetAll"
              >
                <template #reference>
                  <el-button type="danger" plain size="small">{{ t("settings.resetBtn") }}</el-button>
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
