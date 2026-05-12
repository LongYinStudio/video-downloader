<script setup>
import { onMounted, ref, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";

const THEME_KEY = "vd.theme";
const DIR_KEY = "vd.dir";
const PROXY_KEY = "vd.proxy";
const FORMAT_KEY = "vd.format";
const AUTO_OPEN_DIR_KEY = "vd.auto_open_dir";
const AUTO_PASTE_CLIPBOARD_KEY = "vd.auto_paste_clipboard";
const FORMAT_OPTIONS = [
  { label: "最佳画质", value: "best" },
  { label: "最高 1080p", value: "1080p" },
  { label: "最高 720p", value: "720p" },
  { label: "最高 480p", value: "480p" },
  { label: "仅音频 MP3", value: "audio" },
];

const themeMode = ref("system");
const defaultDir = ref("");
const defaultProxy = ref("");
const defaultFormat = ref("best");
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
  themeMode.value = localStorage.getItem(THEME_KEY) || "system";
  defaultDir.value = localStorage.getItem(DIR_KEY) || "";
  defaultProxy.value = localStorage.getItem(PROXY_KEY) || "";
  defaultFormat.value = localStorage.getItem(FORMAT_KEY) || "best";
  autoOpenDir.value = localStorage.getItem(AUTO_OPEN_DIR_KEY) !== "false";
  autoPasteClipboard.value =
    localStorage.getItem(AUTO_PASTE_CLIPBOARD_KEY) === "true";
  applyTheme(themeMode.value);
});

watch(themeMode, (val) => {
  localStorage.setItem(THEME_KEY, val);
  applyTheme(val);
});

watch(defaultDir, (val) => {
  if (!val) {
    localStorage.removeItem(DIR_KEY);
    return;
  }
  localStorage.setItem(DIR_KEY, val);
});

watch(defaultProxy, (val) => {
  if (!val) {
    localStorage.removeItem(PROXY_KEY);
    return;
  }
  localStorage.setItem(PROXY_KEY, val);
});

watch(defaultFormat, (val) => {
  localStorage.setItem(FORMAT_KEY, val || "best");
});

watch(autoOpenDir, (val) => {
  localStorage.setItem(AUTO_OPEN_DIR_KEY, String(val));
});

watch(autoPasteClipboard, (val) => {
  localStorage.setItem(AUTO_PASTE_CLIPBOARD_KEY, String(val));
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

function clearDir() {
  defaultDir.value = "";
}

function resetAll() {
  themeMode.value = "system";
  defaultDir.value = "";
  defaultProxy.value = "";
  defaultFormat.value = "best";
  autoOpenDir.value = true;
  autoPasteClipboard.value = false;
  localStorage.removeItem(THEME_KEY);
  localStorage.removeItem(DIR_KEY);
  localStorage.removeItem(PROXY_KEY);
  localStorage.removeItem(FORMAT_KEY);
  localStorage.removeItem(AUTO_OPEN_DIR_KEY);
  localStorage.removeItem(AUTO_PASTE_CLIPBOARD_KEY);
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
          <div class="field-content">
            <el-input
              v-model="defaultDir"
              placeholder="未选择目录(默认：系统Downloads目录)"
              readonly
            />
            <el-button type="info" @click="chooseDir()">选择目录</el-button>
            <el-button plain @click="clearDir()">清除</el-button>
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
  width: 90%;
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

.field-content.single {
  grid-template-columns: 1fr;
}

.label {
  color: var(--text-primary);
  font-weight: 500;
}

@media (max-width: 800px) {
  .settings-card {
    width: calc(100% - 1.2em);
  }

  .field-content {
    grid-template-columns: 1fr;
  }

  .field.inline {
    flex-direction: column;
    align-items: start;
    gap: 0.6em;
  }
}
</style>
