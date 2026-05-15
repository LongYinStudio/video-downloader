<script setup>
import { QuestionFilled } from "@element-plus/icons-vue";
import {
  COOKIES_BROWSER_OPTIONS,
  COOKIES_MODE_OPTIONS,
  FILENAME_TEMPLATE_OPTIONS,
  FORMAT_OPTIONS,
} from "../settings.js";

defineProps({
  dir: {
    type: String,
    required: true,
  },
  cookiesPath: {
    type: String,
    required: true,
  },
  cookiesMode: {
    type: String,
    required: true,
  },
  cookiesBrowser: {
    type: String,
    required: true,
  },
  formatPreset: {
    type: String,
    required: true,
  },
  filenameTemplate: {
    type: String,
    required: true,
  },
  retries: {
    type: Number,
    required: true,
  },
  concurrentFragments: {
    type: Number,
    required: true,
  },
  proxy: {
    type: String,
    required: true,
  },
  isDownloading: {
    type: Boolean,
    required: true,
  },
});

const emit = defineEmits([
  "update:dir",
  "update:cookiesPath",
  "update:cookiesMode",
  "update:cookiesBrowser",
  "update:formatPreset",
  "update:filenameTemplate",
  "update:retries",
  "update:concurrentFragments",
  "update:proxy",
  "choose-dir",
  "choose-cookies",
  "clear-cookies",
]);
</script>

<template>
  <div class="conf">
    <div class="confItem">
      <el-text class="label" tag="b">保存目录</el-text>
      <div class="confContent">
        <el-input
          id="dir-input"
          :model-value="dir"
          placeholder="未选择目录(默认：系统Downloads目录)"
          readonly
          :disabled="isDownloading"
          @update:model-value="emit('update:dir', $event)"
        />
        <el-button type="info" :disabled="isDownloading" @click="emit('choose-dir')"
          >选择目录</el-button
        >
      </div>
    </div>
    <div class="confItem">
      <el-text class="label" tag="b">下载格式</el-text>
      <div class="confContent">
        <el-select
          :model-value="formatPreset"
          placeholder="选择下载格式"
          :disabled="isDownloading"
          @update:model-value="emit('update:formatPreset', $event)"
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
      <el-text class="label" tag="b">Cookies 来源</el-text>
      <div class="confContent">
        <el-select
          :model-value="cookiesMode"
          :disabled="isDownloading"
          placeholder="选择 Cookies 来源"
          @update:model-value="emit('update:cookiesMode', $event)"
        >
          <el-option
            v-for="item in COOKIES_MODE_OPTIONS"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
        <el-tooltip content="需要登录、会员或私密权限内容时使用" placement="top">
          <el-icon class="help-icon"><QuestionFilled /></el-icon>
        </el-tooltip>
      </div>
      <div v-if="cookiesMode === 'file'" class="confContent triple with-top-gap">
        <el-input
          :model-value="cookiesPath"
          placeholder="未选择 cookies.txt（Netscape 格式）"
          readonly
          :disabled="isDownloading"
          @update:model-value="emit('update:cookiesPath', $event)"
        />
        <el-button
          type="info"
          :disabled="isDownloading"
          @click="emit('choose-cookies')"
        >
          选择文件
        </el-button>
        <el-button
          plain
          :disabled="isDownloading || !cookiesPath"
          @click="emit('clear-cookies')"
        >
          清除
        </el-button>
      </div>
      <div v-else-if="cookiesMode === 'browser'" class="confContent with-top-gap">
        <el-select
          :model-value="cookiesBrowser"
          :disabled="isDownloading"
          placeholder="选择浏览器"
          @update:model-value="emit('update:cookiesBrowser', $event)"
        >
          <el-option
            v-for="item in COOKIES_BROWSER_OPTIONS"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
        <el-tooltip
          content="如读取失败，先关闭对应浏览器再重试"
          placement="top"
        >
          <el-icon class="help-icon"><QuestionFilled /></el-icon>
        </el-tooltip>
      </div>
      <el-text size="small" class="inline-hint">
        <template v-if="cookiesMode === 'file'">
          适用于已导出的 cookies.txt，建议使用 Netscape 格式。
        </template>
        <template v-else-if="cookiesMode === 'browser'">
          直接从已登录浏览器读取 Cookies；支持 Chrome、Edge、Firefox 等。
        </template>
        <template v-else>未启用登录态 Cookies。</template>
      </el-text>
    </div>
    <div class="confItem">
      <el-text class="label" tag="b">文件名模板</el-text>
      <div class="confContent">
        <el-select
          :model-value="filenameTemplate"
          placeholder="选择文件名模板"
          :disabled="isDownloading"
          @update:model-value="emit('update:filenameTemplate', $event)"
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
            :model-value="retries"
            :min="0"
            :max="20"
            :disabled="isDownloading"
            controls-position="right"
            @update:model-value="emit('update:retries', $event)"
          />
        </div>
        <div class="number-field">
          <el-text size="small">并发片段</el-text>
          <el-input-number
            :model-value="concurrentFragments"
            :min="1"
            :max="16"
            :disabled="isDownloading"
            controls-position="right"
            @update:model-value="emit('update:concurrentFragments', $event)"
          />
        </div>
      </div>
    </div>
    <div class="confItem">
      <el-text class="label" tag="b">代理设置</el-text>
      <div class="confContent">
        <el-input
          id="proxy-input"
          :model-value="proxy"
          placeholder="可选：http://127.0.0.1:7890"
          :disabled="isDownloading"
          @update:model-value="emit('update:proxy', $event)"
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
</template>

<style scoped>
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

.confContent.triple {
  grid-template-columns: minmax(0, 1fr) auto auto;
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

.help-icon {
  cursor: help;
  font-size: 1.2em;
  color: var(--text-tertiary);
  transition: color 0.3s ease;
}

.help-icon:hover {
  color: var(--primary-color);
}

.inline-hint {
  display: block;
  margin-top: 0.5em;
  color: var(--text-tertiary);
}

@media (max-width: 720px) {
  .confContent,
  .confContent.compact,
  .confContent.triple {
    grid-template-columns: 1fr;
  }

  .number-field {
    grid-template-columns: 5em 1fr;
  }
}
</style>
