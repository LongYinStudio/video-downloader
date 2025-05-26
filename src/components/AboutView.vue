<script setup>
import { version } from "../utils.js";
import { openUrl } from "@tauri-apps/plugin-opener";
function feedback() {
  openUrl("https://github.com/LongYinStudio/video-downloader/issues/new");
}
</script>
<template>
  <div class="main">
    <h3 class="title">关于软件</h3>
    <el-card style="width: 96%; margin: 0 auto">
      <div class="about-card">
        <div class="info">
          <img src="../../src-tauri/icons/512x512.png" alt="" />
          <div class="name_version">
            <h4>video-downloader</h4>
            <p>版本: {{ version }}</p>
          </div>
        </div>
        <el-button type="primary">检查更新</el-button>
      </div>
    </el-card>
    <el-card style="width: 96%; margin: 0 auto; margin-top: 1em">
      <div class="about-card">
        <div class="name_version">
          <h4>开源地址</h4>
          <el-link
            target="_blank"
            href="https://github.com/LongYinStudio/video-downloader"
            >https://github.com/LongYinStudio/video-downloader</el-link
          >
        </div>
        <el-button type="danger" plain @click="feedback()">反馈问题</el-button>
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.main {
  display: flex;
  flex-direction: column;
  align-items: start;
}

.main .title {
  margin: 0.8em 0em 0.6em 0.6em;
}

.main .about-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.about-card .info {
  display: flex;
  flex-direction: row;
}

.about-card .info img {
  --imgSize: 3em;
  width: var(--imgSize);
  height: var(--imgSize);
  border: 1px solid #505050;
  border-radius: 0.6em;
}

.about-card .name_version {
  margin-left: 1em;
  display: flex;
  flex-direction: column;
  align-items: start;
}

.about-card .name_version p {
  color: #aaa;
  font-size: 0.8em;
}
</style>
