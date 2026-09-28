<script setup>
import { computed, onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { openPath } from "@tauri-apps/plugin-opener";
import { ElMessage } from "element-plus";
import {
  Clock,
  Delete,
  Document,
  FolderOpened,
  Link,
  Refresh,
  Search,
  VideoPlay,
} from "@element-plus/icons-vue";
import { clearHistory, getHistory, removeHistoryItem } from "../history.js";

const router = useRouter();
const historyList = ref([]);
const searchQuery = ref("");
const statusFilter = ref("all");

function loadHistory() {
  historyList.value = getHistory();
}

onMounted(() => {
  loadHistory();
});

const filteredList = computed(() => {
  return historyList.value.filter((item) => {
    if (statusFilter.value !== "all" && item.status !== statusFilter.value) {
      return false;
    }
    if (!searchQuery.value.trim()) return true;
    const query = searchQuery.value.trim().toLowerCase();
    return (
      (item.title && item.title.toLowerCase().includes(query)) ||
      (item.url && item.url.toLowerCase().includes(query)) ||
      (item.file && item.file.toLowerCase().includes(query))
    );
  });
});

async function handleOpenFile(filePath) {
  if (!filePath) {
    ElMessage.warning("未记录文件路径");
    return;
  }
  try {
    await openPath(filePath);
  } catch (err) {
    ElMessage.error(`打开文件失败: ${err?.message || err}`);
  }
}

async function handleOpenDir(filePath) {
  if (!filePath) {
    ElMessage.warning("未记录文件路径");
    return;
  }
  try {
    const lastSlash = Math.max(filePath.lastIndexOf("/"), filePath.lastIndexOf("\\"));
    const dirPath = lastSlash > 0 ? filePath.substring(0, lastSlash) : filePath;
    await openPath(dirPath);
  } catch (err) {
    ElMessage.error(`打开目录失败: ${err?.message || err}`);
  }
}

async function handleCopyUrl(url) {
  if (!url) return;
  try {
    await navigator.clipboard.writeText(url);
    ElMessage.success("链接已复制到剪贴板");
  } catch (err) {
    ElMessage.error("复制失败");
  }
}

function handleRedownload(item) {
  router.push({
    path: "/",
    query: { redownloadUrl: item.url },
  });
}

function handleDeleteItem(id) {
  historyList.value = removeHistoryItem(id);
  ElMessage.success("已删除此条记录");
}

function handleClearAll() {
  clearHistory();
  historyList.value = [];
  ElMessage.success("历史记录已清空");
}

function getStatusTag(status) {
  switch (status) {
    case "success":
      return { type: "success", text: "成功" };
    case "failed":
      return { type: "danger", text: "失败" };
    case "cancelled":
      return { type: "warning", text: "已取消" };
    default:
      return { type: "info", text: status || "未知" };
  }
}
</script>

<template>
  <div class="main">
    <div class="page-header">
      <h3 class="title">下载历史</h3>
      <div class="header-actions">
        <el-popconfirm
          title="确定清空所有下载历史记录吗？"
          confirm-button-text="确定清空"
          cancel-button-text="取消"
          confirm-button-type="danger"
          @confirm="handleClearAll"
        >
          <template #reference>
            <el-button
              type="danger"
              plain
              size="default"
              :disabled="historyList.length === 0"
            >
              <el-icon><Delete /></el-icon>
              清空历史
            </el-button>
          </template>
        </el-popconfirm>
      </div>
    </div>

    <el-card class="history-card" shadow="hover">
      <div class="filter-bar">
        <el-input
          v-model="searchQuery"
          placeholder="搜索标题、链接或文件名..."
          prefix-icon="Search"
          clearable
          class="search-input"
        />
        <el-radio-group v-model="statusFilter" size="default">
          <el-radio-button label="all">全部</el-radio-button>
          <el-radio-button label="success">成功</el-radio-button>
          <el-radio-button label="failed">失败</el-radio-button>
        </el-radio-group>
      </div>

      <div v-if="filteredList.length === 0" class="empty-state">
        <el-empty description="暂无历史记录" />
      </div>

      <div v-else class="history-list">
        <article
          v-for="item in filteredList"
          :key="item.id"
          class="history-item"
        >
          <div class="item-main">
            <div class="item-title-row">
              <el-tag
                :type="getStatusTag(item.status).type"
                size="small"
                effect="light"
                round
              >
                {{ getStatusTag(item.status).text }}
              </el-tag>
              <el-tag v-if="item.format" size="small" effect="plain" round>
                {{ item.format }}
              </el-tag>
              <h4 class="item-title" :title="item.title">{{ item.title }}</h4>
            </div>

            <div class="item-meta">
              <span class="meta-time">
                <el-icon><Clock /></el-icon>
                {{ item.dateStr }}
              </span>
              <a
                class="meta-link"
                :href="item.url"
                target="_blank"
                rel="noreferrer"
                :title="item.url"
              >
                <el-icon><Link /></el-icon>
                <span class="url-text">{{ item.url }}</span>
              </a>
            </div>

            <div v-if="item.file" class="item-file" :title="item.file">
              <el-icon><Document /></el-icon>
              <span>{{ item.file }}</span>
            </div>

            <div v-if="item.error && item.status === 'failed'" class="item-error">
              <span>原因：{{ item.error }}</span>
            </div>
          </div>

          <div class="item-actions">
            <el-tooltip v-if="item.status === 'success' && item.file" content="打开文件" placement="top">
              <el-button
                type="primary"
                size="small"
                circle
                @click="handleOpenFile(item.file)"
              >
                <el-icon><VideoPlay /></el-icon>
              </el-button>
            </el-tooltip>
            <el-tooltip v-if="item.file" content="打开所在目录" placement="top">
              <el-button
                size="small"
                circle
                @click="handleOpenDir(item.file)"
              >
                <el-icon><FolderOpened /></el-icon>
              </el-button>
            </el-tooltip>
            <el-tooltip content="复制视频链接" placement="top">
              <el-button
                size="small"
                circle
                @click="handleCopyUrl(item.url)"
              >
                <el-icon><Link /></el-icon>
              </el-button>
            </el-tooltip>
            <el-tooltip content="重新下载" placement="top">
              <el-button
                type="success"
                plain
                size="small"
                circle
                @click="handleRedownload(item)"
              >
                <el-icon><Refresh /></el-icon>
              </el-button>
            </el-tooltip>
            <el-tooltip content="删除此记录" placement="top">
              <el-button
                type="danger"
                plain
                size="small"
                circle
                @click="handleDeleteItem(item.id)"
              >
                <el-icon><Delete /></el-icon>
              </el-button>
            </el-tooltip>
          </div>
        </article>
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.main {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  width: 100%;
  padding-bottom: 2rem;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: min(90%, 860px);
  margin: 0.8em auto 0.6em;
}

.title {
  margin: 0;
  font-size: 1.4em;
  font-weight: 600;
  color: var(--text-primary);
}

.history-card {
  width: min(90%, 860px);
  max-width: 860px;
  margin: 0 auto;
  border-radius: 12px;
  background-color: var(--bg-secondary);
  border-color: var(--border-color);
}

.filter-bar {
  display: flex;
  gap: 1em;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1.2em;
  flex-wrap: wrap;
}

.search-input {
  flex: 1;
  min-width: 200px;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 0.85em;
}

.history-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1em;
  padding: 0.9em 1.1em;
  border-radius: 10px;
  background-color: color-mix(in srgb, var(--bg-primary) 70%, var(--bg-secondary));
  border: 1px solid color-mix(in srgb, var(--border-color) 80%, transparent);
  transition: all 0.2s ease;
}

.history-item:hover {
  background-color: color-mix(in srgb, var(--bg-primary) 90%, var(--bg-secondary));
  border-color: var(--border-color);
  box-shadow: var(--shadow-sm);
}

.item-main {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.4em;
  text-align: left;
}

.item-title-row {
  display: flex;
  align-items: center;
  gap: 0.6em;
  min-width: 0;
}

.item-title {
  margin: 0;
  font-size: 1em;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
  min-width: 0;
}

.item-meta {
  display: flex;
  align-items: center;
  gap: 1em;
  font-size: 0.84em;
  color: var(--text-tertiary);
  flex-wrap: wrap;
}

.meta-time,
.meta-link {
  display: inline-flex;
  align-items: center;
  gap: 0.35em;
}

.meta-link {
  color: var(--primary-color);
  text-decoration: none;
  max-width: 320px;
}

.meta-link:hover {
  text-decoration: underline;
}

.url-text {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-file {
  display: inline-flex;
  align-items: center;
  gap: 0.35em;
  font-size: 0.82em;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  background-color: color-mix(in srgb, var(--bg-primary) 85%, var(--bg-tertiary));
  padding: 0.25em 0.5em;
  border-radius: 4px;
}

.item-error {
  font-size: 0.82em;
  color: var(--danger-color);
}

.item-actions {
  display: flex;
  gap: 0.4em;
  flex-shrink: 0;
}

.empty-state {
  padding: 2.5em 0;
}

@media (max-width: 720px) {
  .page-header {
    width: calc(100% - 1.2em);
  }

  .history-card {
    width: calc(100% - 1.2em);
  }

  .filter-bar {
    flex-direction: column;
    align-items: stretch;
  }

  .history-item {
    flex-direction: column;
    align-items: stretch;
  }

  .item-actions {
    justify-content: flex-end;
    padding-top: 0.5em;
    border-top: 1px dashed color-mix(in srgb, var(--border-color) 60%, transparent);
  }

  .meta-link {
    max-width: 100%;
  }
}
</style>
