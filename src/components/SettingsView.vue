<script setup>
import { onMounted, ref, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
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
  <div class="main">
    <h3 class="title">设置</h3>
    <el-card class="settings-card" shadow="hover">
      <section class="section">
        <div class="section-title">主题</div>
        <el-radio-group v-model="themeMode" size="large">
          <el-radio-button label="system">跟随系统</el-radio-button>
          <el-radio-button label="light">浅色</el-radio-button>
          <el-radio-button label="dark">深色</el-radio-button>
        </el-radio-group>
        <p class="section-desc">覆盖系统主题，仅影响应用界面。</p>
      </section>

      <el-divider />

      <section class="section">
        <div class="section-title">下载默认值</div>
        <div class="field">
          <el-text class="label" tag="b">保存目录</el-text>
          <div class="field-content dir-field">
            <el-input
              v-model="defaultDir"
              placeholder="未选择目录(默认：系统Downloads目录)"
              readonly
            />
            <div class="field-actions">
              <el-button type="info" @click="chooseDir()">选择目录</el-button>
              <el-button plain @click="clearDir()">清除</el-button>
            </div>
          </div>
        </div>

        <div class="field">
          <el-text class="label" tag="b">下载格式</el-text>
          <div class="field-content single">
            <el-select v-model="defaultFormat" placeholder="选择下载格式">
              <el-option
                v-for="item in FORMAT_OPTIONS"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </div>
          <p class="hint">指定最高分辨率或仅下载音频；MP3 需要 FFmpeg 支持。</p>
        </div>

        <div class="field">
          <el-text class="label" tag="b">Cookies 来源</el-text>
          <div class="field-content single">
            <el-select
              v-model="defaultCookiesMode"
              placeholder="选择 Cookies 来源"
            >
              <el-option
                v-for="item in COOKIES_MODE_OPTIONS"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </div>
          <div
            v-if="defaultCookiesMode === 'file'"
            class="field-content cookies-field with-top-gap"
          >
            <el-input
              v-model="defaultCookiesPath"
              placeholder="未选择 cookies.txt（Netscape 格式）"
              readonly
            />
            <div class="field-actions">
              <el-button type="info" @click="chooseCookiesFile()">选择文件</el-button>
              <el-button plain @click="clearCookiesFile()">清除</el-button>
            </div>
          </div>
          <div
            v-else-if="defaultCookiesMode === 'browser'"
            class="field-content single with-top-gap"
          >
            <el-select
              v-model="defaultCookiesBrowser"
              placeholder="选择浏览器"
            >
              <el-option
                v-for="item in COOKIES_BROWSER_OPTIONS"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </div>
          <p class="hint" v-if="defaultCookiesMode === 'file'">
            用于下载需要登录、会员或私密权限的视频，建议导出 Netscape 格式的
            cookies.txt。
          </p>
          <p class="hint" v-else-if="defaultCookiesMode === 'browser'">
            直接从已登录浏览器读取 Cookies；如读取失败，先关闭浏览器再重试。
          </p>
          <p class="hint" v-else>不使用 Cookies，适合公开可访问内容。</p>
        </div>

        <div class="field">
          <el-text class="label" tag="b">文件名模板</el-text>
          <div class="field-content single">
            <el-select
              v-model="defaultFilenameTemplate"
              placeholder="选择文件名模板"
            >
              <el-option
                v-for="item in FILENAME_TEMPLATE_OPTIONS"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </div>
          <p class="hint">控制保存文件名的组成方式。</p>
        </div>

        <div class="field">
          <el-text class="label" tag="b">下载参数</el-text>
          <div class="field-content numeric">
            <div class="number-field">
              <el-text size="small">重试次数</el-text>
              <el-input-number
                v-model="defaultRetries"
                :min="0"
                :max="20"
                controls-position="right"
              />
            </div>
            <div class="number-field">
              <el-text size="small">并发片段</el-text>
              <el-input-number
                v-model="defaultConcurrentFragments"
                :min="1"
                :max="16"
                controls-position="right"
              />
            </div>
          </div>
          <p class="hint">重试次数同时用于普通重试和分片重试。</p>
        </div>

        <div class="field">
          <el-text class="label" tag="b">代理设置</el-text>
          <div class="field-content">
            <el-input
              v-model="defaultProxy"
              placeholder="可选：http://127.0.0.1:7890"
            />
          </div>
          <p class="hint">支持 HTTP/SOCKS5 代理配置。</p>
        </div>
      </section>

      <el-divider />

      <section class="section">
        <div class="section-title">其他</div>
        <div class="field inline">
          <el-text class="label" tag="b">下载完成后打开目录</el-text>
          <el-switch v-model="autoOpenDir" />
        </div>
        <div class="field inline">
          <el-text class="label" tag="b">启动时自动粘贴剪贴板 URL</el-text>
          <el-switch v-model="autoPasteClipboard" />
        </div>
        <div class="field inline">
          <el-text class="label" tag="b">重置所有设置</el-text>
          <el-button type="danger" plain @click="resetAll()">重置</el-button>
        </div>
      </section>
    </el-card>
  </div>
</template>

<style scoped>
.main {
  display: flex;
  flex-direction: column;
  align-items: start;
  width: 100%;
}

.main .title {
  margin: 0.8em 0em 0.6em 0.6em;
}

.settings-card {
  width: min(90%, 780px);
  max-width: 780px;
  margin: 0 auto;
  border-radius: 12px;
  background-color: var(--bg-secondary);
  border-color: var(--border-color);
}

.section {
  display: flex;
  flex-direction: column;
  gap: 0.8em;
}

.section-title {
  font-size: 1.05em;
  font-weight: 600;
  color: var(--text-primary);
}

.section-desc,
.hint {
  margin: 0;
  color: var(--text-tertiary);
  font-size: 0.9em;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.5em;
}

.field.inline {
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
}

.field-content {
  display: grid;
  grid-template-columns: 1fr auto auto;
  grid-gap: 0.75em;
  align-items: center;
}

.dir-field {
  grid-template-columns: 1fr auto;
}

.field-actions {
  display: flex;
  gap: 0.75em;
  align-items: center;
}

.field-actions .el-button {
  margin-left: 0;
}

.field-content.single {
  grid-template-columns: 1fr;
}

.field-content.numeric {
  grid-template-columns: repeat(2, minmax(10em, 1fr));
}

.with-top-gap {
  margin-top: 0.5em;
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

.label {
  color: var(--text-primary);
  font-weight: 500;
}

@media (max-width: 800px) {
  .main {
    align-items: stretch;
  }

  .settings-card {
    width: calc(100% - 1.2em);
  }

  .field-content,
  .dir-field,
  .field-content.numeric {
    grid-template-columns: 1fr;
  }

  .field-actions {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .field-actions .el-button {
    width: 100%;
  }

  .field.inline {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0.6em;
  }

  .number-field {
    grid-template-columns: 5em minmax(0, 1fr);
  }
}

@media (max-width: 460px) {
  .field-actions,
  .field.inline {
    grid-template-columns: 1fr;
  }

  .field.inline {
    align-items: start;
  }

  .number-field {
    grid-template-columns: 1fr;
    gap: 0.35em;
  }
}
</style>
