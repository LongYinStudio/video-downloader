<script setup>
import { computed } from "vue";
import {
  Download,
  Search,
  Close,
  DocumentCopy,
  VideoCamera,
} from "@element-plus/icons-vue";
import { ElMessage } from "element-plus";

const props = defineProps({
  url: {
    type: String,
    required: true,
  },
  isPreviewing: {
    type: Boolean,
    required: true,
  },
  isDownloading: {
    type: Boolean,
    required: true,
  },
  isCancelling: {
    type: Boolean,
    required: true,
  },
});

const emit = defineEmits(["update:url", "preview", "download", "cancel"]);

const urlCount = computed(() => {
  return props.url
    .split(/\s+/)
    .map((item) => item.trim())
    .filter((item) => /^https?:\/\/.+/.test(item)).length;
});

async function pasteClipboard() {
  try {
    const text = await navigator.clipboard.readText();
    if (!text || !text.trim()) {
      ElMessage.warning("剪贴板中无内容");
      return;
    }
    const trimmed = text.trim();
    const newUrl = props.url.trim()
      ? `${props.url.trim()}\n${trimmed}`
      : trimmed;
    emit("update:url", newUrl);
    ElMessage.success("已粘贴剪贴板链接");
  } catch {
    ElMessage.error("无法读取剪贴板，请手动粘贴");
  }
}

function clearUrl() {
  emit("update:url", "");
}
</script>

<template>
  <div class="hero-download-box" :class="{ 'is-focused': isDownloading }">
    <div class="box-header">
      <div class="header-left">
        <el-icon class="header-icon"><VideoCamera /></el-icon>
        <span class="header-title">视频链接</span>
        <el-tag
          v-if="urlCount > 0"
          size="small"
          type="primary"
          effect="light"
          round
          class="count-tag"
        >
          已识别 {{ urlCount }} 个链接
        </el-tag>
      </div>

      <div class="header-tools">
        <el-button
          size="small"
          text
          :disabled="isDownloading"
          class="tool-btn"
          @click="pasteClipboard"
        >
          <el-icon><DocumentCopy /></el-icon>
          粘贴
        </el-button>
        <el-button
          v-if="url.trim()"
          size="small"
          text
          :disabled="isDownloading"
          class="tool-btn danger"
          @click="clearUrl"
        >
          <el-icon><Close /></el-icon>
          清空
        </el-button>
      </div>
    </div>

    <div class="textarea-wrap">
      <el-input
        :model-value="url"
        type="textarea"
        placeholder="支持 Bilibili、YouTube、TikTok、Twitter/X 等，多个链接请每行一个..."
        :disabled="isDownloading"
        :autosize="{ minRows: 2, maxRows: 6 }"
        resize="none"
        class="smart-textarea"
        @update:model-value="emit('update:url', $event)"
      />
    </div>

    <div class="box-footer">
      <div class="footer-hint">
        <span class="platform-hint">支持 1000+ 视频与音频站点</span>
      </div>

      <div class="action-buttons">
        <el-button
          class="action-btn preview-btn"
          :loading="isPreviewing"
          :disabled="isDownloading || isPreviewing || !url.trim()"
          @click="emit('preview')"
        >
          <el-icon v-if="!isPreviewing"><Search /></el-icon>
          <span>{{ isPreviewing ? "解析中..." : "解析信息" }}</span>
        </el-button>

        <el-button
          type="primary"
          class="action-btn download-btn"
          :loading="isDownloading"
          :disabled="isDownloading || isPreviewing || !url.trim()"
          @click="emit('download')"
        >
          <el-icon v-if="!isDownloading"><Download /></el-icon>
          <span>{{ isDownloading ? "下载中..." : "开始下载" }}</span>
        </el-button>

        <el-button
          v-if="isDownloading"
          type="danger"
          plain
          class="action-btn cancel-btn"
          :loading="isCancelling"
          @click="emit('cancel')"
        >
          取消
        </el-button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.hero-download-box {
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: 1.15rem 1.25rem 1rem;
  box-shadow: var(--shadow-sm);
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.hero-download-box:hover {
  border-color: var(--border-hover);
  box-shadow: var(--shadow-md);
}

.hero-download-box:focus-within {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px var(--primary-subtle), var(--shadow-md);
}

/* 顶部工具栏 */
.box-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.header-icon {
  color: var(--primary-color);
  font-size: 1.1rem;
}

.header-title {
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--text-primary);
}

.count-tag {
  font-size: 0.76rem;
  padding: 0 0.5rem;
  height: 22px;
}

.header-tools {
  display: flex;
  gap: 0.25rem;
}

.tool-btn {
  font-size: 0.82rem;
  color: var(--text-tertiary);
  padding: 0.2rem 0.5rem;
}

.tool-btn:hover {
  color: var(--primary-color);
  background-color: var(--primary-subtle);
}

.tool-btn.danger:hover {
  color: var(--danger-color);
  background-color: var(--danger-subtle);
}

/* 文本域 */
.textarea-wrap {
  width: 100%;
}

:deep(.smart-textarea .el-textarea__inner) {
  padding: 0.5rem 0.6rem;
  font-size: 0.95rem;
  line-height: 1.6;
  border: 1px solid transparent;
  background-color: var(--bg-input);
  border-radius: var(--radius-md);
  box-shadow: inset 0 0 0 1px var(--border-color);
  color: var(--text-primary);
  transition: all 0.2s ease;
}

:deep(.smart-textarea .el-textarea__inner:focus) {
  background-color: var(--bg-card);
  box-shadow: inset 0 0 0 1.5px var(--primary-color);
}

/* 底部动作栏 */
.box-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 0.25rem;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.platform-hint {
  font-size: 0.82rem;
  color: var(--text-tertiary);
}

.action-buttons {
  display: flex;
  align-items: center;
  gap: 0.65rem;
}

.action-btn {
  height: 38px;
  padding: 0 1.25rem;
  border-radius: var(--radius-md);
  font-weight: 600;
  font-size: 0.92rem;
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.preview-btn {
  background-color: var(--bg-tertiary);
  border-color: var(--border-color);
  color: var(--text-secondary);
}

.preview-btn:hover:not(:disabled) {
  background-color: var(--border-color);
  color: var(--text-primary);
}

.download-btn {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  border: none;
  box-shadow: 0 2px 8px -1px rgba(37, 99, 235, 0.4);
}

.download-btn:hover:not(:disabled) {
  box-shadow: 0 4px 14px -1px rgba(37, 99, 235, 0.55);
  transform: translateY(-1px);
}

.download-btn:active:not(:disabled) {
  transform: translateY(0);
}

@media (max-width: 640px) {
  .hero-download-box {
    padding: 1rem;
  }

  .box-footer {
    flex-direction: column;
    align-items: stretch;
  }

  .action-buttons {
    width: 100%;
  }

  .action-btn {
    flex: 1;
    justify-content: center;
  }
}
</style>
