<script setup>
import { Loading, CircleCheck, CircleClose, Warning, Refresh } from "@element-plus/icons-vue";

defineProps({
  downloadStatus: {
    type: String,
    required: true,
  },
  progress: {
    type: String,
    required: true,
  },
  isDownloading: {
    type: Boolean,
    required: true,
  },
  currentFile: {
    type: String,
    required: true,
  },
  downloadSpeed: {
    type: String,
    required: true,
  },
  eta: {
    type: String,
    required: true,
  },
  queueIndex: {
    type: Number,
    required: true,
  },
  queueTotal: {
    type: Number,
    required: true,
  },
  successCount: {
    type: Number,
    default: 0,
  },
  failedItems: {
    type: Array,
    default: () => [],
  },
  error: {
    type: String,
    required: true,
  },
  errorTips: {
    type: Array,
    required: true,
  },
});

const emit = defineEmits(["clear-error", "retry-failed"]);
</script>

<template>
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
      {{ queueTotal > 1 ? `全部下载完成 (${successCount} 个文件)` : "下载完成" }}
    </el-tag>
    <el-tag
      v-else-if="downloadStatus === 'partial_failed'"
      type="warning"
      effect="plain"
    >
      <el-icon><Warning /></el-icon>
      部分下载失败 (成功 {{ successCount }}，失败 {{ failedItems.length }})
    </el-tag>
    <el-tag v-else-if="downloadStatus === 'failed'" type="danger" effect="plain">
      <el-icon><CircleClose /></el-icon>
      {{ queueTotal > 1 ? `全部下载失败 (${failedItems.length} 个文件)` : "下载失败" }}
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

  <el-progress
    id="progress"
    v-if="progress"
    :text-inside="true"
    :stroke-width="24"
    :percentage="parseFloat(progress)"
    status="success"
  />
  <div
    v-if="isDownloading || currentFile || downloadSpeed || eta"
    class="download-details"
  >
    <div v-if="queueTotal > 1" class="detail-row">
      <span class="detail-label">队列</span>
      <span>{{ queueIndex }} / {{ queueTotal }}</span>
    </div>
    <div v-if="currentFile" class="detail-row">
      <span class="detail-label">文件</span>
      <span class="detail-value">{{ currentFile }}</span>
    </div>
    <div v-if="downloadSpeed || eta" class="detail-grid">
      <div class="detail-row">
        <span class="detail-label">速度</span>
        <span>{{ downloadSpeed || "-" }}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">剩余</span>
        <span>{{ eta || "-" }}</span>
      </div>
    </div>
  </div>
  <div v-if="error" class="error-message">
    <el-alert
      type="error"
      :closable="true"
      show-icon
      @close="emit('clear-error')"
    >
      <template #title>
        {{ error }}
      </template>
      <template #default>
        <div v-if="errorTips.length" class="error-tips">
          请检查：
          <ul>
            <li v-for="tip in errorTips" :key="tip">{{ tip }}</li>
          </ul>
        </div>
      </template>
    </el-alert>
  </div>
  <div v-if="failedItems && failedItems.length > 0" class="failed-summary">
    <div class="failed-header">
      <span class="failed-title">
        <el-icon><Warning /></el-icon>
        下载失败项 ({{ failedItems.length }})
      </span>
      <el-button
        type="warning"
        size="small"
        plain
        :disabled="isDownloading"
        @click="emit('retry-failed')"
      >
        <el-icon><Refresh /></el-icon>
        重试失败项
      </el-button>
    </div>
    <ul class="failed-list">
      <li v-for="(item, idx) in failedItems" :key="idx" class="failed-item">
        <div class="failed-url" :title="item.url">{{ item.url }}</div>
        <div class="failed-reason">{{ item.message }}</div>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.failed-summary {
  margin: 1.2em auto 0;
  width: 90%;
  padding: 0.85em 1em;
  background-color: color-mix(in srgb, var(--danger-color) 8%, var(--bg-secondary));
  border: 1px solid color-mix(in srgb, var(--danger-color) 25%, transparent);
  border-radius: 8px;
  text-align: left;
}

.failed-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.6em;
}

.failed-title {
  display: flex;
  align-items: center;
  gap: 0.4em;
  font-weight: 600;
  color: var(--danger-color);
  font-size: 0.95em;
}

.failed-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 0.5em;
  max-height: 180px;
  overflow-y: auto;
}

.failed-item {
  font-size: 0.88em;
  padding: 0.4em 0.6em;
  background: var(--bg-secondary);
  border-radius: 6px;
  border-left: 3px solid var(--danger-color);
}

.failed-url {
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.failed-reason {
  margin-top: 0.2em;
  color: var(--text-tertiary);
  font-size: 0.82em;
}

@media (max-width: 720px) {
  .failed-summary {
    width: 100%;
  }
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
  padding: 2em 0 1em;
  width: 70%;
  margin: 0 auto;
}

.download-details {
  display: flex;
  flex-direction: column;
  gap: 0.6em;
  margin: 1em auto 0;
  width: 90%;
  color: var(--text-secondary);
  font-size: 0.92em;
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.75em;
}

.detail-row {
  display: flex;
  gap: 0.6em;
  min-width: 0;
}

.detail-label {
  flex: 0 0 auto;
  color: var(--text-tertiary);
}

.detail-value {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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

@media (max-width: 720px) {
  #progress,
  .download-details {
    width: 100%;
  }
}
</style>
