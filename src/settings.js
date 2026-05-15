export const STORAGE_KEYS = {
  theme: "vd.theme",
  dir: "vd.dir",
  proxy: "vd.proxy",
  cookiesMode: "vd.cookies_mode",
  cookiesPath: "vd.cookies_path",
  cookiesBrowser: "vd.cookies_browser",
  format: "vd.format",
  filenameTemplate: "vd.filename_template",
  retries: "vd.retries",
  concurrentFragments: "vd.concurrent_fragments",
  autoOpenDir: "vd.auto_open_dir",
  autoPasteClipboard: "vd.auto_paste_clipboard",
};

export const FORMAT_OPTIONS = [
  { label: "最佳画质", value: "best" },
  { label: "最高 1080p", value: "1080p" },
  { label: "最高 720p", value: "720p" },
  { label: "最高 480p", value: "480p" },
  { label: "仅音频 MP3", value: "audio" },
];

export const FILENAME_TEMPLATE_OPTIONS = [
  { label: "标题", value: "title" },
  { label: "标题 + ID", value: "title_id" },
  { label: "作者 + 标题", value: "uploader_title" },
  { label: "日期 + 标题", value: "date_title" },
];

export const COOKIES_MODE_OPTIONS = [
  { label: "不使用", value: "none" },
  { label: "cookies.txt 文件", value: "file" },
  { label: "从浏览器读取", value: "browser" },
];

export const COOKIES_BROWSER_OPTIONS = [
  { label: "Chrome", value: "chrome" },
  { label: "Edge", value: "edge" },
  { label: "Firefox", value: "firefox" },
  { label: "Chromium", value: "chromium" },
  { label: "Brave", value: "brave" },
  { label: "Opera", value: "opera" },
  { label: "Vivaldi", value: "vivaldi" },
  { label: "Safari", value: "safari" },
  { label: "Whale", value: "whale" },
];

export const DEFAULT_DOWNLOAD_OPTIONS = {
  cookiesMode: "none",
  cookiesPath: "",
  cookiesBrowser: "chrome",
  format: "best",
  filenameTemplate: "title",
  retries: 10,
  concurrentFragments: 4,
};

export function readNumberSetting(key, defaultValue, min, max) {
  const value = Number(localStorage.getItem(key));
  if (!Number.isFinite(value)) return defaultValue;
  return Math.min(max, Math.max(min, value));
}

export function readOptionSetting(key, options, defaultValue) {
  const value = localStorage.getItem(key);
  return options.some((option) => option.value === value) ? value : defaultValue;
}
