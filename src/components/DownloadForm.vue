<script setup>
defineProps({
  url: {
    type: String,
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

const emit = defineEmits(["update:url", "download", "cancel"]);
</script>

<template>
  <el-form id="downloadForm" @submit.prevent="emit('download')">
    <el-input
      id="url-input"
      :model-value="url"
      type="textarea"
      placeholder="请输入视频链接，支持 Bilibili、YouTube 等；多个链接可每行一个"
      size="large"
      clearable
      :disabled="isDownloading"
      show-word-limit
      maxlength="4000"
      :autosize="{ minRows: 2, maxRows: 6 }"
      @update:model-value="emit('update:url', $event)"
    />
    <el-tooltip
      content="支持 YouTube、Bilibili、Vimeo 等主流视频平台"
      placement="top"
      :disabled="url.trim()"
    >
      <div class="download-actions">
        <el-button
          type="primary"
          size="large"
          :loading="isDownloading"
          :disabled="isDownloading || !url.trim()"
          @click="emit('download')"
        >
          {{ isDownloading ? "下载中..." : "开始下载" }}
        </el-button>
        <el-button
          v-if="isDownloading"
          type="danger"
          size="large"
          plain
          :loading="isCancelling"
          @click="emit('cancel')"
        >
          取消
        </el-button>
      </div>
    </el-tooltip>
  </el-form>
</template>

<style scoped>
#downloadForm {
  display: grid;
  grid-template-columns: 1fr auto;
  grid-gap: 1em;
}

#downloadForm > .download-actions {
  min-height: 3.6em;
  font-size: 1em;
}

.download-actions {
  display: flex;
  gap: 0.75em;
  height: 100%;
}

.download-actions .el-button {
  height: 100%;
  min-width: 7.5em;
  margin-left: 0;
}

@media (max-width: 720px) {
  #downloadForm {
    grid-template-columns: 1fr;
  }

  .download-actions {
    width: 100%;
  }

  .download-actions .el-button {
    flex: 1;
    min-width: 0;
  }
}
</style>
