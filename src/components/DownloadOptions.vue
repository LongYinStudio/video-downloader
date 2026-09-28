<script setup>
import { ref } from "vue";
import {
  ArrowDown,
  Folder,
  FolderOpened,
  Operation,
  QuestionFilled,
  Delete,
  Document,
} from "@element-plus/icons-vue";
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

const isExpanded = ref(false);
</script>

<template>
  <div class="options-container">
    <!-- 核心快速参数栏 (常驻显示) -->
    <div class="quick-options-bar">
      <!-- 格式预设 -->
      <div class="quick-option-item format-item">
        <span class="option-label">格式</span>
        <el-select
          :model-value="formatPreset"
          :disabled="isDownloading"
          size="default"
          class="compact-select"
          @update:model-value="emit('update:formatPreset', $event)"
        >
          <el-option
            v-for="item in FORMAT_OPTIONS"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
      </div>

      <!-- 保存目录 -->
      <div class="quick-option-item dir-item">
        <span class="option-label">目录</span>
        <div class="dir-pill" :title="dir || '默认: 系统 Downloads 目录'" @click="emit('choose-dir')">
          <el-icon class="dir-icon"><Folder /></el-icon>
          <span class="dir-text">{{ dir ? dir.split(/[/\\]/).pop() || dir : "系统下载目录" }}</span>
          <el-button size="small" text class="dir-change-btn" :disabled="isDownloading">
            更改
          </el-button>
        </div>
      </div>

      <!-- 高级参数折叠触发器 -->
      <div class="quick-option-item toggle-item">
        <button
          type="button"
          class="advanced-toggle-btn"
          :class="{ 'is-active': isExpanded }"
          @click="isExpanded = !isExpanded"
        >
          <el-icon class="toggle-icon"><Operation /></el-icon>
          <span>高级参数</span>
          <el-icon class="arrow-icon" :class="{ 'is-rotated': isExpanded }">
            <ArrowDown />
          </el-icon>
        </button>
      </div>
    </div>

    <!-- 智能折叠卡片 (平滑过渡) -->
    <transition name="collapse-slide">
      <div v-show="isExpanded" class="advanced-card">
        <div class="advanced-grid">
          <!-- Cookies 来源 -->
          <div class="setting-cell">
            <div class="cell-header">
              <span class="cell-label">Cookies 登录态</span>
              <el-tooltip content="下载会员、私密或限权内容时使用" placement="top">
                <el-icon class="help-icon"><QuestionFilled /></el-icon>
              </el-tooltip>
            </div>
            <el-select
              :model-value="cookiesMode"
              :disabled="isDownloading"
              size="default"
              class="cell-control"
              @update:model-value="emit('update:cookiesMode', $event)"
            >
              <el-option
                v-for="item in COOKIES_MODE_OPTIONS"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>

            <!-- 浏览器选择 -->
            <div v-if="cookiesMode === 'browser'" class="sub-cell">
              <el-select
                :model-value="cookiesBrowser"
                :disabled="isDownloading"
                size="default"
                class="cell-control"
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
              <span class="sub-hint">支持 Chrome、Edge、Firefox 等</span>
            </div>

            <!-- 文件选择 -->
            <div v-else-if="cookiesMode === 'file'" class="sub-cell">
              <div class="file-picker-row">
                <el-input
                  :model-value="cookiesPath"
                  placeholder="未选择 cookies.txt"
                  readonly
                  size="default"
                  class="file-input"
                  :disabled="isDownloading"
                />
                <el-button size="default" :disabled="isDownloading" @click="emit('choose-cookies')">
                  选择
                </el-button>
                <el-button
                  v-if="cookiesPath"
                  size="default"
                  text
                  class="clear-btn"
                  :disabled="isDownloading"
                  @click="emit('clear-cookies')"
                >
                  <el-icon><Delete /></el-icon>
                </el-button>
              </div>
            </div>
          </div>

          <!-- 文件名模板 -->
          <div class="setting-cell">
            <div class="cell-header">
              <span class="cell-label">文件名模板</span>
              <el-tooltip content="控制保存文件的命名格式" placement="top">
                <el-icon class="help-icon"><QuestionFilled /></el-icon>
              </el-tooltip>
            </div>
            <el-select
              :model-value="filenameTemplate"
              :disabled="isDownloading"
              size="default"
              class="cell-control"
              @update:model-value="emit('update:filenameTemplate', $event)"
            >
              <el-option
                v-for="item in FILENAME_TEMPLATE_OPTIONS"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </div>

          <!-- 下载参数 (并发与重试) -->
          <div class="setting-cell">
            <div class="cell-header">
              <span class="cell-label">性能与重试</span>
            </div>
            <div class="numeric-row">
              <div class="num-item">
                <span class="num-label">重试次数</span>
                <el-input-number
                  :model-value="retries"
                  :min="0"
                  :max="20"
                  :disabled="isDownloading"
                  size="default"
                  controls-position="right"
                  class="num-control"
                  @update:model-value="emit('update:retries', $event)"
                />
              </div>
              <div class="num-item">
                <span class="num-label">并发片段</span>
                <el-input-number
                  :model-value="concurrentFragments"
                  :min="1"
                  :max="16"
                  :disabled="isDownloading"
                  size="default"
                  controls-position="right"
                  class="num-control"
                  @update:model-value="emit('update:concurrentFragments', $event)"
                />
              </div>
            </div>
          </div>

          <!-- 代理设置 -->
          <div class="setting-cell">
            <div class="cell-header">
              <span class="cell-label">网络代理</span>
              <el-tooltip content="支持 HTTP / SOCKS5 代理" placement="top">
                <el-icon class="help-icon"><QuestionFilled /></el-icon>
              </el-tooltip>
            </div>
            <el-input
              :model-value="proxy"
              placeholder="可选: http://127.0.0.1:7890"
              :disabled="isDownloading"
              size="default"
              class="cell-control"
              clearable
              @update:model-value="emit('update:proxy', $event)"
            />
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<style scoped>
.options-container {
  display: flex;
  flex-direction: column;
  gap: 0.65rem;
  margin-top: 0.85rem;
}

/* 核心快速选项栏 */
.quick-options-bar {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  padding: 0.65rem 0.95rem;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  flex-wrap: wrap;
}

.quick-option-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.option-label {
  font-size: 0.84rem;
  font-weight: 600;
  color: var(--text-tertiary);
  white-space: nowrap;
}

.format-item {
  flex: 0 0 auto;
}

.compact-select {
  width: 135px;
}

.dir-item {
  flex: 1;
  min-width: 180px;
}

.dir-pill {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 0.4rem;
  padding: 0.25rem 0.6rem;
  background-color: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
  min-width: 0;
}

.dir-pill:hover {
  background-color: var(--border-color);
}

.dir-icon {
  color: var(--primary-color);
  font-size: 0.95rem;
  flex-shrink: 0;
}

.dir-text {
  font-size: 0.84rem;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.dir-change-btn {
  padding: 0 0.35rem;
  font-size: 0.78rem;
  color: var(--primary-color);
  height: 20px;
}

.toggle-item {
  margin-left: auto;
}

.advanced-toggle-btn {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  padding: 0.35rem 0.75rem;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  font-size: 0.84rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.advanced-toggle-btn:hover {
  color: var(--primary-color);
  border-color: var(--primary-color);
  background-color: var(--primary-subtle);
}

.advanced-toggle-btn.is-active {
  color: var(--primary-color);
  border-color: var(--primary-color);
  background-color: var(--primary-subtle);
}

.arrow-icon {
  font-size: 0.8rem;
  transition: transform 0.25s ease;
}

.arrow-icon.is-rotated {
  transform: rotate(180deg);
}

/* 高级折叠面板卡片 */
.advanced-card {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: 1.15rem 1.25rem;
  box-shadow: var(--shadow-sm);
}

.advanced-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 1.15rem 1.5rem;
}

.setting-cell {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.cell-header {
  display: flex;
  align-items: center;
  gap: 0.35rem;
}

.cell-label {
  font-size: 0.86rem;
  font-weight: 600;
  color: var(--text-primary);
}

.help-icon {
  font-size: 0.88rem;
  color: var(--text-tertiary);
  cursor: help;
}

.cell-control {
  width: 100%;
}

.sub-cell {
  margin-top: 0.35rem;
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.sub-hint {
  font-size: 0.78rem;
  color: var(--text-tertiary);
}

.file-picker-row {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.file-input {
  flex: 1;
}

.clear-btn {
  color: var(--danger-color);
  padding: 0 0.4rem;
}

.numeric-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.75rem;
}

.num-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.num-label {
  font-size: 0.78rem;
  color: var(--text-tertiary);
}

.num-control {
  width: 100%;
}

/* 平滑过渡动效 */
.collapse-slide-enter-active,
.collapse-slide-leave-active {
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
}

.collapse-slide-enter-from,
.collapse-slide-leave-to {
  opacity: 0;
  transform: translateY(-8px);
  max-height: 0;
  padding-top: 0;
  padding-bottom: 0;
  margin-top: 0;
}

@media (max-width: 640px) {
  .quick-options-bar {
    flex-direction: column;
    align-items: stretch;
  }

  .dir-item {
    width: 100%;
  }

  .toggle-item {
    margin-left: 0;
    width: 100%;
  }

  .advanced-toggle-btn {
    width: 100%;
    justify-content: center;
  }

  .advanced-grid {
    grid-template-columns: 1fr;
    gap: 0.95rem;
  }
}
</style>
