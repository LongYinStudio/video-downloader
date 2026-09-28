<script setup>
import { Link, Picture, VideoCamera, CollectionTag, Clock, VideoPlay } from "@element-plus/icons-vue";
import { reactive, watch } from "vue";

const props = defineProps({
  isPreviewing: {
    type: Boolean,
    required: true,
  },
  previewItems: {
    type: Array,
    required: true,
  },
});

const brokenImages = reactive({});

watch(
  () => props.previewItems,
  () => {
    Object.keys(brokenImages).forEach((key) => {
      delete brokenImages[key];
    });
  },
);

function getImageKey(item, index) {
  return `${item.url}-${index}`;
}

function markImageBroken(item, index) {
  brokenImages[getImageKey(item, index)] = true;
}

function canShowImage(item, index) {
  return item.thumbnail && !brokenImages[getImageKey(item, index)];
}

function getPreviewType(item) {
  return item.isPlaylist ? "success" : "primary";
}

function getPreviewLabel(item) {
  return item.isPlaylist ? "播放列表" : "单个视频";
}
</script>

<template>
  <section v-if="isPreviewing || previewItems.length" class="preview-section">
    <div class="preview-header">
      <div class="header-info">
        <h4 class="preview-title">解析结果</h4>
        <span class="preview-desc">下载前确认媒体元数据与条目</span>
      </div>
      <el-tag v-if="isPreviewing" type="primary" size="small" effect="plain" round>
        解析中...
      </el-tag>
      <el-tag v-else type="success" size="small" effect="light" round>
        {{ previewItems.length }} 个链接已就绪
      </el-tag>
    </div>

    <div v-if="isPreviewing" class="preview-skeleton-card">
      <el-skeleton animated :rows="3" />
    </div>

    <div v-else class="preview-cards">
      <article
        v-for="(item, index) in previewItems"
        :key="`${item.url}-${index}`"
        class="preview-card"
      >
        <!-- 视频封面区 (带 16:9 比例与浮动时间徽标) -->
        <div class="media-cover-wrap">
          <img
            v-if="canShowImage(item, index)"
            :src="item.thumbnail"
            alt="cover"
            class="media-cover-img"
            @error="markImageBroken(item, index)"
          />
          <div v-else class="media-placeholder">
            <el-icon><Picture /></el-icon>
            <span>暂无封面</span>
          </div>

          <div v-if="item.duration" class="duration-badge">
            <el-icon><Clock /></el-icon>
            {{ item.duration }}
          </div>
        </div>

        <!-- 详细信息区 -->
        <div class="media-body">
          <div class="media-tags-row">
            <el-tag :type="getPreviewType(item)" size="small" effect="light" round>
              {{ getPreviewLabel(item) }}
            </el-tag>
            <el-tag v-if="item.extractor" size="small" effect="plain" round>
              {{ item.extractor }}
            </el-tag>
            <el-tag v-if="item.entryCount" size="small" type="info" effect="plain" round>
              {{ item.entryCount }} 集
            </el-tag>
          </div>

          <h5 class="media-title" :title="item.title">
            {{ item.title || "未获取到标题" }}
          </h5>

          <div class="media-author-row">
            <span v-if="item.uploader" class="author-item">
              <el-icon><VideoCamera /></el-icon>
              {{ item.uploader }}
            </span>
            <a
              v-if="item.webpageUrl"
              class="webpage-link"
              :href="item.webpageUrl"
              target="_blank"
              rel="noreferrer"
            >
              <el-icon><Link /></el-icon>
              <span>{{ item.webpageUrl }}</span>
            </a>
          </div>

          <!-- 分集预览 -->
          <div v-if="item.entries && item.entries.length" class="playlist-preview">
            <div class="playlist-header">
              <el-icon><CollectionTag /></el-icon>
              <span>分集列表预览 (前 {{ item.entries.length }} 集)</span>
            </div>
            <ul class="playlist-list">
              <li
                v-for="(entry, entryIndex) in item.entries"
                :key="`${entry.url}-${entryIndex}`"
                class="playlist-item"
              >
                <span class="entry-num">{{ entryIndex + 1 }}</span>
                <span class="entry-name" :title="entry.title">{{ entry.title || "未命名" }}</span>
                <span v-if="entry.duration" class="entry-time">{{ entry.duration }}</span>
              </li>
            </ul>
          </div>
        </div>
      </article>
    </div>
  </section>
</template>

<style scoped>
.preview-section {
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: 1.15rem 1.25rem;
  box-shadow: var(--shadow-sm);
  margin-top: 0.85rem;
  text-align: left;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.95rem;
}

.header-info {
  display: flex;
  align-items: baseline;
  gap: 0.65rem;
}

.preview-title {
  margin: 0;
  font-size: 0.96rem;
  font-weight: 700;
  color: var(--text-primary);
}

.preview-desc {
  font-size: 0.8rem;
  color: var(--text-tertiary);
}

.preview-skeleton-card {
  padding: 1rem 0;
}

.preview-cards {
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
}

.preview-card {
  display: flex;
  gap: 1.15rem;
  padding: 0.9rem;
  border-radius: var(--radius-md);
  background-color: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  transition: all 0.2s ease;
}

.preview-card:hover {
  border-color: var(--border-hover);
  box-shadow: var(--shadow-sm);
}

/* 媒体封面 */
.media-cover-wrap {
  width: 170px;
  height: 98px;
  border-radius: var(--radius-md);
  overflow: hidden;
  position: relative;
  background-color: var(--bg-primary);
  flex-shrink: 0;
  box-shadow: inset 0 0 0 1px var(--border-color);
}

.media-cover-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.media-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.3rem;
  color: var(--text-tertiary);
  font-size: 1.2rem;
}

.media-placeholder span {
  font-size: 0.72rem;
}

.duration-badge {
  position: absolute;
  bottom: 6px;
  right: 6px;
  background-color: rgba(0, 0, 0, 0.75);
  backdrop-filter: blur(4px);
  color: #ffffff;
  font-size: 0.72rem;
  font-weight: 600;
  padding: 0.15rem 0.4rem;
  border-radius: 4px;
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

/* 媒体信息主体 */
.media-body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
}

.media-tags-row {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  flex-wrap: wrap;
}

.media-title {
  margin: 0;
  font-size: 0.98rem;
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1.4;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.media-author-row {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  font-size: 0.82rem;
  color: var(--text-secondary);
  flex-wrap: wrap;
}

.author-item {
  display: flex;
  align-items: center;
  gap: 0.3rem;
}

.webpage-link {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  color: var(--primary-color);
  text-decoration: none;
  max-width: 320px;
}

.webpage-link span {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.webpage-link:hover {
  text-decoration: underline;
}

/* 播放列表条目预览 */
.playlist-preview {
  margin-top: 0.4rem;
  padding-top: 0.5rem;
  border-top: 1px dashed var(--border-color);
}

.playlist-header {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--text-tertiary);
  margin-bottom: 0.35rem;
}

.playlist-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.playlist-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.82rem;
  padding: 0.25rem 0.4rem;
  background-color: var(--bg-card);
  border-radius: var(--radius-sm);
}

.entry-num {
  font-size: 0.72rem;
  font-weight: 700;
  color: var(--primary-color);
  background-color: var(--primary-subtle);
  width: 18px;
  height: 18px;
  border-radius: 999px;
  display: grid;
  place-items: center;
  flex-shrink: 0;
}

.entry-name {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--text-primary);
}

.entry-time {
  font-size: 0.75rem;
  color: var(--text-tertiary);
  flex-shrink: 0;
}

@media (max-width: 640px) {
  .preview-card {
    flex-direction: column;
  }

  .media-cover-wrap {
    width: 100%;
    height: 160px;
  }
}
</style>
