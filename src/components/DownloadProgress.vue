<script setup>
import { Loading, CircleCheck, CircleClose, Warning, Refresh, VideoPlay } from "@element-plus/icons-vue";

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
  <div v-if="downloadStatus !== 'idle'" class="progress-card">
    <!-- 顶部状态指示条 -->
    <div class="status-header">
      <div class="status-indicator">
        <el-tag
          v-if="downloadStatus === 'downloading'"
          type="primary"
          effect="light"
          round
          class="status-tag"
        >
          <el-icon class="is-loading"><Loading /></el-icon>
          正在下载 {{ queueTotal > 1 ? `(${queueIndex}/${queueTotal})` : '' }}
        </el-tag>

        <el-tag
          v-else-if="downloadStatus === 'completed'"
          type="success"
          effect="light"
          round
          class="status-tag"
        >
          <el-icon><CircleCheck /></el-icon>
          {{ queueTotal > 1 ? `全部下载完成 (${successCount} 个文件)` : "下载完成" }}
        </el-tag>

        <el-tag
          v-else-if="downloadStatus === 'partial_failed'"
          type="warning"
          effect="light"
          round
          class="status-tag"
        >
          <el-icon><Warning /></el-icon>
          部分失败 (成功 {{ successCount }}，失败 {{ failedItems.length }})
        </el-tag>

        <el-tag
          v-else-if="downloadStatus === 'failed'"
          type="danger"
          effect="light"
          round
          class="status-tag"
        >
          <el-icon><CircleClose /></el-icon>
          {{ queueTotal > 1 ? `全部下载失败 (${failedItems.length} 个文件)` : "下载失败" }}
        </el-tag>

        <el-tag
          v-else-if="downloadStatus === 'cancelled'"
          type="info"
          effect="light"
          round
          class="status-tag"
        >
          <el-icon><CircleClose /></el-icon>
          下载已取消
        </el-tag>
      </div>

      <!-- 速度与剩余时间 (仅下载中或有数据时展示) -->
      <div v-if="isDownloading && (downloadSpeed || eta)" class="speed-eta-wrap">
        <span v-if="downloadSpeed" class="stat-pill">
          <span class="stat-label">速度</span>
          <span class="stat-value">{{ downloadSpeed }}</span>
        </span>
        <span v-if="eta" class="stat-pill">
          <span class="stat-label">剩余</span>
          <span class="stat-value">{{ eta }}</span>
        </span>
      </div>
    </div>

    <!-- 动态渐变进度条 -->
    <div v-if="progress || isDownloading" class="progress-bar-wrap">
      <el-progress
        :percentage="parseFloat(progress) || 0"
        :stroke-width="10"
        :show-text="false"
        color="linear-gradient(90deg, #3b82f6 0%, #60a5fa 100%)"
        class="custom-progress"
      />
      <span class="progress-number">{{ progress ? `${progress}%` : "0%" }}</span>
    </div>

    <!-- 当前文件信息 -->
    <div v-if="currentFile" class="file-info-row">
      <el-icon class="file-icon"><VideoPlay /></el-icon>
      <span class="file-name" :title="currentFile">{{ currentFile }}</span>
    </div>

    <!-- 异常提示 -->
    <div v-if="error" class="error-notice">
      <el-alert
        type="error"
        :closable="true"
        show-icon
        class="custom-alert"
        @close="emit('clear-error')"
      >
        <template #title>
          <span class="error-title-text">{{ error }}</span>
        </template>
        <template #default>
          <div v-if="errorTips.length" class="error-tips-content">
            <span class="tips-heading">排查建议：</span>
            <ul>
              <li v-for="tip in errorTips" :key="tip">{{ tip }}</li>
            </ul>
          </div>
        </template>
      </el-alert>
    </div>

    <!-- 失败项重试卡片 -->
    <div v-if="failedItems && failedItems.length > 0" class="failed-summary-card">
      <div class="failed-summary-header">
        <div class="failed-title-group">
          <el-icon class="failed-icon"><Warning /></el-icon>
          <span class="failed-heading">失败任务 ({{ failedItems.length }})</span>
        </div>
        <el-button
          type="warning"
          size="small"
          plain
          :disabled="isDownloading"
          class="retry-btn"
          @click="emit('retry-failed')"
        >
          <el-icon><Refresh /></el-icon>
          重试失败项
        </el-button>
      </div>

      <ul class="failed-items-list">
        <li v-for="(item, idx) in failedItems" :key="idx" class="failed-task-item">
          <div class="failed-task-url" :title="item.url">{{ item.url }}</div>
          <div class="failed-task-reason">{{ item.message }}</div>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.progress-card {
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: 1.15rem 1.25rem;
  box-shadow: var(--shadow-sm);
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
  text-align: left;
}

.status-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.status-tag {
  font-size: 0.84rem;
  font-weight: 600;
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  height: 28px;
  padding: 0 0.75rem;
}

.speed-eta-wrap {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.stat-pill {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  background-color: var(--bg-tertiary);
  padding: 0.2rem 0.55rem;
  border-radius: var(--radius-sm);
  font-size: 0.78rem;
}

.stat-label {
  color: var(--text-tertiary);
}

.stat-value {
  color: var(--text-primary);
  font-weight: 600;
}

/* 进度条 */
.progress-bar-wrap {
  display: flex;
  align-items: center;
  gap: 0.85rem;
}

.custom-progress {
  flex: 1;
}

.progress-number {
  font-size: 0.84rem;
  font-weight: 700;
  color: var(--primary-color);
  min-width: 45px;
  text-align: right;
}

/* 文件信息 */
.file-info-row {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  background-color: var(--bg-tertiary);
  padding: 0.35rem 0.65rem;
  border-radius: var(--radius-sm);
  font-size: 0.82rem;
  color: var(--text-secondary);
}

.file-icon {
  color: var(--primary-color);
  flex-shrink: 0;
}

.file-name {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

/* 异常提示 */
.error-notice {
  margin-top: 0.25rem;
}

.custom-alert {
  border-radius: var(--radius-md);
  border: 1px solid var(--danger-subtle);
}

.error-title-text {
  font-size: 0.88rem;
  font-weight: 600;
}

.error-tips-content {
  margin-top: 0.35rem;
  font-size: 0.82rem;
}

.tips-heading {
  font-weight: 600;
}

.error-tips-content ul {
  margin: 0.25rem 0 0 1.25rem;
  padding: 0;
}

.error-tips-content li {
  margin: 0.15rem 0;
}

/* 失败项重试卡片 */
.failed-summary-card {
  padding: 0.85rem 1rem;
  background-color: var(--danger-subtle);
  border: 1px solid color-mix(in srgb, var(--danger-color) 25%, transparent);
  border-radius: var(--radius-md);
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.failed-summary-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.failed-title-group {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.failed-icon {
  color: var(--danger-color);
}

.failed-heading {
  font-size: 0.88rem;
  font-weight: 700;
  color: var(--danger-color);
}

.retry-btn {
  font-size: 0.8rem;
  border-radius: var(--radius-sm);
}

.failed-items-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
  max-height: 160px;
  overflow-y: auto;
}

.failed-task-item {
  background-color: var(--bg-card);
  padding: 0.4rem 0.65rem;
  border-radius: var(--radius-sm);
  border-left: 3px solid var(--danger-color);
  font-size: 0.82rem;
}

.failed-task-url {
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.failed-task-reason {
  margin-top: 0.15rem;
  color: var(--text-tertiary);
  font-size: 0.76rem;
}
</style>
