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
  } catch {
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
      return { type: "success", text: "已完成" };
    case "failed":
      return { type: "danger", text: "下载失败" };
    case "cancelled":
      return { type: "warning", text: "已取消" };
    default:
      return { type: "info", text: status || "未知" };
  }
}
</script>

<template>
  <div class="history-page">
    <div class="page-container">
      <!-- 页面顶部标题栏 -->
      <header class="page-topbar">
        <div class="topbar-title-group">
          <h2 class="page-title">下载历史</h2>
          <span class="page-subtitle">查看与管理过去下载的本地文件与记录</span>
        </div>
        <div class="topbar-actions">
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
                size="small"
                class="clear-all-btn"
                :disabled="historyList.length === 0"
              >
                <el-icon><Delete /></el-icon>
                清空历史
              </el-button>
            </template>
          </el-popconfirm>
        </div>
      </header>

      <!-- 搜索与筛选卡片 -->
      <div class="filter-card">
        <el-input
          v-model="searchQuery"
          placeholder="搜索视频标题、链接或本地文件路径..."
          prefix-icon="Search"
          clearable
          size="default"
          class="search-input"
        />
        <div class="filter-pills">
          <div
            class="filter-pill"
            :class="{ active: statusFilter === 'all' }"
            @click="statusFilter = 'all'"
          >
            全部 ({{ historyList.length }})
          </div>
          <div
            class="filter-pill"
            :class="{ active: statusFilter === 'success' }"
            @click="statusFilter = 'success'"
          >
            成功 ({{ historyList.filter(i => i.status === 'success').length }})
          </div>
          <div
            class="filter-pill"
            :class="{ active: statusFilter === 'failed' }"
            @click="statusFilter = 'failed'"
          >
            失败 ({{ historyList.filter(i => i.status === 'failed').length }})
          </div>
        </div>
      </div>

      <!-- 历史记录列表 -->
      <div v-if="filteredList.length === 0" class="empty-card">
        <el-empty description="暂无历史记录" />
      </div>

      <div v-else class="history-cards-stack">
        <article
          v-for="item in filteredList"
          :key="item.id"
          class="history-item-card"
        >
          <div class="item-content">
            <div class="item-title-line">
              <el-tag
                :type="getStatusTag(item.status).type"
                size="small"
                effect="light"
                round
                class="status-badge"
              >
                {{ getStatusTag(item.status).text }}
              </el-tag>
              <el-tag v-if="item.format" size="small" effect="plain" round class="format-badge">
                {{ item.format }}
              </el-tag>
              <h4 class="item-title" :title="item.title">{{ item.title }}</h4>
            </div>

            <div class="item-meta-line">
              <span class="meta-time">
                <el-icon><Clock /></el-icon>
                {{ item.dateStr }}
              </span>
              <a
                class="meta-url-link"
                :href="item.url"
                target="_blank"
                rel="noreferrer"
                :title="item.url"
              >
                <el-icon><Link /></el-icon>
                <span>{{ item.url }}</span>
              </a>
            </div>

            <div v-if="item.file" class="item-file-pill" :title="item.file">
              <el-icon class="file-icon"><Document /></el-icon>
              <span class="file-path">{{ item.file }}</span>
            </div>

            <div v-if="item.error && item.status === 'failed'" class="item-error-box">
              <span>失败原因：{{ item.error }}</span>
            </div>
          </div>

          <div class="item-action-btns">
            <el-tooltip v-if="item.status === 'success' && item.file" content="打开视频文件" placement="top">
              <button class="action-circle-btn primary" @click="handleOpenFile(item.file)">
                <el-icon><VideoPlay /></el-icon>
              </button>
            </el-tooltip>

            <el-tooltip v-if="item.file" content="定位所在文件夹" placement="top">
              <button class="action-circle-btn" @click="handleOpenDir(item.file)">
                <el-icon><FolderOpened /></el-icon>
              </button>
            </el-tooltip>

            <el-tooltip content="复制视频链接" placement="top">
              <button class="action-circle-btn" @click="handleCopyUrl(item.url)">
                <el-icon><Link /></el-icon>
              </button>
            </el-tooltip>

            <el-tooltip content="重新下载" placement="top">
              <button class="action-circle-btn success" @click="handleRedownload(item)">
                <el-icon><Refresh /></el-icon>
              </button>
            </el-tooltip>

            <el-tooltip content="删除记录" placement="top">
              <button class="action-circle-btn danger" @click="handleDeleteItem(item.id)">
                <el-icon><Delete /></el-icon>
              </button>
            </el-tooltip>
          </div>
        </article>
      </div>
    </div>
  </div>
</template>

<style scoped>
.history-page {
  width: 100%;
  min-height: 100%;
  padding: 1.5rem 1.75rem 2.5rem;
}

.page-container {
  width: 100%;
  max-width: 860px;
  margin: 0 auto;
}

.page-topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.25rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--border-subtle);
}

.topbar-title-group {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.2rem;
}

.page-title {
  margin: 0;
  font-size: 1.35rem;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.01em;
}

.page-subtitle {
  font-size: 0.84rem;
  color: var(--text-tertiary);
}

.clear-all-btn {
  border-radius: var(--radius-sm);
}

/* 过滤栏卡片 */
.filter-card {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  padding: 0.85rem 1rem;
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  margin-bottom: 1rem;
  flex-wrap: wrap;
}

.search-input {
  flex: 1;
  min-width: 220px;
}

.filter-pills {
  display: flex;
  gap: 0.35rem;
  background-color: var(--bg-tertiary);
  padding: 0.25rem;
  border-radius: var(--radius-md);
}

.filter-pill {
  font-size: 0.82rem;
  font-weight: 500;
  padding: 0.3rem 0.75rem;
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.filter-pill:hover {
  color: var(--text-primary);
}

.filter-pill.active {
  background-color: var(--bg-card);
  color: var(--primary-color);
  font-weight: 600;
  box-shadow: var(--shadow-sm);
}

/* 历史卡片流 */
.history-cards-stack {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.history-item-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1.25rem;
  padding: 1rem 1.25rem;
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.history-item-card:hover {
  border-color: var(--border-hover);
  box-shadow: var(--shadow-md);
  transform: translateY(-1px);
}

.item-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.45rem;
  text-align: left;
}

.item-title-line {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  min-width: 0;
}

.status-badge {
  font-size: 0.74rem;
  font-weight: 600;
  flex-shrink: 0;
}

.format-badge {
  font-size: 0.74rem;
  flex-shrink: 0;
}

.item-title {
  margin: 0;
  font-size: 0.98rem;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.item-meta-line {
  display: flex;
  align-items: center;
  gap: 1rem;
  font-size: 0.8rem;
  color: var(--text-tertiary);
  flex-wrap: wrap;
}

.meta-time,
.meta-url-link {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
}

.meta-url-link {
  color: var(--primary-color);
  text-decoration: none;
  max-width: 380px;
}

.meta-url-link span {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.meta-url-link:hover {
  text-decoration: underline;
}

.item-file-pill {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  font-size: 0.78rem;
  color: var(--text-secondary);
  background-color: var(--bg-tertiary);
  padding: 0.2rem 0.55rem;
  border-radius: var(--radius-sm);
  align-self: flex-start;
  max-width: 100%;
}

.file-icon {
  color: var(--primary-color);
  flex-shrink: 0;
}

.file-path {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-error-box {
  font-size: 0.78rem;
  color: var(--danger-color);
}

/* 操作图标按钮群 */
.item-action-btns {
  display: flex;
  align-items: center;
  gap: 0.45rem;
  flex-shrink: 0;
}

.action-circle-btn {
  width: 32px;
  height: 32px;
  border-radius: 999px;
  border: 1px solid var(--border-color);
  background-color: var(--bg-primary);
  color: var(--text-secondary);
  display: grid;
  place-items: center;
  cursor: pointer;
  font-size: 0.95rem;
  transition: all 0.2s ease;
}

.action-circle-btn:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  border-color: var(--border-hover);
}

.action-circle-btn.primary {
  color: var(--primary-color);
  border-color: var(--primary-subtle);
  background-color: var(--primary-subtle);
}

.action-circle-btn.primary:hover {
  background-color: var(--primary-color);
  color: #ffffff;
}

.action-circle-btn.success:hover {
  background-color: var(--success-subtle);
  color: var(--success-color);
  border-color: var(--success-color);
}

.action-circle-btn.danger:hover {
  background-color: var(--danger-subtle);
  color: var(--danger-color);
  border-color: var(--danger-color);
}

.empty-card {
  padding: 3.5rem 0;
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
}

@media (max-width: 640px) {
  .history-page {
    padding: 1rem 0.75rem 2rem;
  }

  .filter-card {
    flex-direction: column;
    align-items: stretch;
  }

  .filter-pills {
    width: 100%;
    justify-content: space-around;
  }

  .history-item-card {
    flex-direction: column;
    align-items: stretch;
  }

  .item-action-btns {
    justify-content: flex-end;
    padding-top: 0.6rem;
    border-top: 1px dashed var(--border-color);
  }
}
</style>
