<script setup>
import { Link, Picture, VideoCamera, CollectionTag, Clock } from "@element-plus/icons-vue";
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
  return item.isPlaylist ? "success" : "info";
}

function getPreviewLabel(item) {
  return item.isPlaylist ? "播放列表" : "视频";
}
</script>

<template>
  <section v-if="isPreviewing || previewItems.length" class="preview-panel">
    <div class="preview-header">
      <div class="preview-heading">
        <span class="eyebrow">Preview</span>
        <h3>下载前预览</h3>
        <p>先确认标题、作者、时长和条目数量，再开始下载。</p>
      </div>
      <el-tag v-if="isPreviewing" type="primary" effect="dark" round>解析中</el-tag>
      <el-tag v-else type="success" effect="light" round>
        {{ previewItems.length }} 个链接已解析
      </el-tag>
    </div>

    <el-skeleton v-if="isPreviewing" :rows="4" animated />

    <div v-else class="preview-list">
      <article
        v-for="(item, index) in previewItems"
        :key="`${item.url}-${index}`"
        class="preview-card"
      >
        <div class="preview-media">
          <img
            v-if="canShowImage(item, index)"
            :src="item.thumbnail"
            alt=""
            @error="markImageBroken(item, index)"
          />
          <div v-else class="preview-placeholder">
            <el-icon><Picture /></el-icon>
            <span>暂无封面</span>
          </div>
        </div>

        <div class="preview-body">
          <div class="preview-top">
            <div class="preview-tags">
              <el-tag :type="getPreviewType(item)" effect="light" round>
                {{ getPreviewLabel(item) }}
              </el-tag>
              <el-tag v-if="item.extractor" effect="plain" round>
                {{ item.extractor }}
              </el-tag>
            </div>
          </div>

          <h4 class="preview-title">{{ item.title || "未获取到标题" }}</h4>

          <div class="preview-meta">
            <span v-if="item.uploader">
              <el-icon><VideoCamera /></el-icon>
              {{ item.uploader }}
            </span>
            <span v-if="item.duration">
              <el-icon><Clock /></el-icon>
              {{ item.duration }}
            </span>
            <span v-if="item.entryCount">
              <el-icon><CollectionTag /></el-icon>
              {{ item.entryCount }} 个条目
            </span>
          </div>

          <a
            v-if="item.webpageUrl"
            class="preview-link"
            :href="item.webpageUrl"
            target="_blank"
            rel="noreferrer"
          >
            <el-icon><Link /></el-icon>
            <span>{{ item.webpageUrl }}</span>
          </a>

          <div v-if="item.entries?.length" class="preview-entries">
            <div class="entries-title">条目预览</div>
            <ul>
              <li
                v-for="(entry, entryIndex) in item.entries"
                :key="`${entry.url}-${entryIndex}`"
              >
                <span class="entry-index">{{ entryIndex + 1 }}</span>
                <div class="entry-content">
                  <div class="entry-title">{{ entry.title || "未命名条目" }}</div>
                  <div v-if="entry.duration || entry.uploader" class="entry-meta">
                    {{ [entry.uploader, entry.duration].filter(Boolean).join(" · ") }}
                  </div>
                </div>
              </li>
            </ul>
          </div>
        </div>
      </article>
    </div>
  </section>
</template>

<style scoped>
.preview-panel {
  margin: 1.5em 0 0;
  padding: 1.15em;
  border: 1px solid color-mix(in srgb, var(--border-color) 88%, transparent);
  border-radius: 18px;
  background:
    radial-gradient(circle at top right, rgba(25, 118, 210, 0.08), transparent 32%),
    linear-gradient(180deg, var(--bg-secondary), color-mix(in srgb, var(--bg-secondary) 90%, var(--bg-tertiary)));
  box-shadow: var(--shadow-sm);
}

.preview-header {
  display: flex;
  justify-content: space-between;
  gap: 1em;
  align-items: flex-start;
  margin-bottom: 1em;
}

.preview-heading h3 {
  margin: 0.15em 0 0;
  font-size: 1.15em;
  color: var(--text-primary);
}

.preview-heading p {
  margin: 0.45em 0 0;
  color: var(--text-tertiary);
  font-size: 0.92em;
}

.eyebrow {
  display: inline-block;
  color: var(--primary-color);
  font-size: 0.76em;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.preview-list {
  display: grid;
  gap: 0.95em;
}

.preview-card {
  display: grid;
  grid-template-columns: 168px minmax(0, 1fr);
  gap: 1em;
  padding: 1em;
  border-radius: 16px;
  border: 1px solid color-mix(in srgb, var(--border-color) 80%, transparent);
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--bg-primary) 76%, var(--bg-secondary)), var(--bg-secondary));
}

.preview-media {
  width: 168px;
  height: 96px;
  border-radius: 14px;
  overflow: hidden;
  background-color: var(--bg-tertiary);
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--border-color) 76%, transparent);
}

.preview-media img,
.preview-placeholder {
  width: 100%;
  height: 100%;
}

.preview-media img {
  display: block;
  object-fit: cover;
}

.preview-placeholder {
  display: grid;
  place-items: center;
  align-content: center;
  gap: 0.35em;
  background:
    linear-gradient(135deg, color-mix(in srgb, var(--bg-tertiary) 92%, white), color-mix(in srgb, var(--bg-primary) 88%, var(--bg-tertiary)));
  color: var(--text-tertiary);
  font-size: 1.35em;
}

.preview-placeholder span {
  font-size: 0.55em;
}

.preview-body {
  min-width: 0;
}

.preview-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75em;
}

.preview-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5em;
}

.preview-title {
  margin: 0.7em 0 0;
  color: var(--text-primary);
  font-size: 1.1em;
  line-height: 1.45;
  text-align: left;
}

.preview-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75em;
  margin-top: 0.7em;
}

.preview-meta span {
  display: inline-flex;
  align-items: center;
  gap: 0.35em;
  padding: 0.38em 0.65em;
  border-radius: 999px;
  background-color: color-mix(in srgb, var(--bg-primary) 84%, var(--bg-tertiary));
  color: var(--text-secondary);
  font-size: 0.88em;
}

.preview-link {
  display: inline-flex;
  align-items: center;
  gap: 0.45em;
  min-width: 0;
  margin-top: 0.8em;
  color: var(--primary-color);
  text-decoration: none;
  font-size: 0.92em;
}

.preview-link span {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.preview-link:hover {
  text-decoration: underline;
}

.preview-entries {
  margin-top: 0.95em;
  padding-top: 0.85em;
  border-top: 1px dashed color-mix(in srgb, var(--border-color) 82%, transparent);
}

.entries-title {
  margin-bottom: 0.6em;
  color: var(--text-secondary);
  font-size: 0.88em;
  font-weight: 700;
  letter-spacing: 0.02em;
}

.preview-entries ul {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 0.55em;
}

.preview-entries li {
  display: grid;
  grid-template-columns: 1.6em minmax(0, 1fr);
  gap: 0.55em;
  align-items: start;
}

.entry-index {
  display: inline-grid;
  place-items: center;
  width: 1.6em;
  height: 1.6em;
  border-radius: 999px;
  background-color: color-mix(in srgb, var(--primary-color) 14%, var(--bg-secondary));
  color: var(--primary-color);
  font-size: 0.78em;
  font-weight: 700;
}

.entry-content {
  min-width: 0;
}

.entry-title {
  color: var(--text-primary);
  line-height: 1.35;
}

.entry-meta {
  margin-top: 0.15em;
  color: var(--text-tertiary);
  font-size: 0.84em;
}

@media (max-width: 720px) {
  .preview-header,
  .preview-card {
    grid-template-columns: 1fr;
  }

  .preview-header {
    display: grid;
  }

  .preview-media {
    width: 100%;
    height: 180px;
  }

  .preview-link {
    width: 100%;
  }

  .preview-link span {
    white-space: normal;
    word-break: break-all;
  }
}
</style>
