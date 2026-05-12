<script setup>
import { Loading, CircleCheck, CircleClose } from "@element-plus/icons-vue";

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
  error: {
    type: String,
    required: true,
  },
  errorTips: {
    type: Array,
    required: true,
  },
});

const emit = defineEmits(["clear-error"]);
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
      下载完成
    </el-tag>
    <el-tag v-else-if="downloadStatus === 'failed'" type="danger" effect="plain">
      <el-icon><CircleClose /></el-icon>
      下载失败
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
</template>

<style scoped>
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
