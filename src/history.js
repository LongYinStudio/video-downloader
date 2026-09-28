export const HISTORY_STORAGE_KEY = "vd.history";

export function getHistory() {
  try {
    const raw = localStorage.getItem(HISTORY_STORAGE_KEY);
    return raw ? JSON.parse(raw) : [];
  } catch {
    return [];
  }
}

export function saveHistory(items) {
  try {
    localStorage.setItem(HISTORY_STORAGE_KEY, JSON.stringify(items.slice(0, 200)));
  } catch (err) {
    console.error("Failed to save download history:", err);
  }
}

export function addHistoryItem({
  url,
  title,
  file = "",
  status = "success",
  error = "",
  format = "",
}) {
  const history = getHistory();
  const newItem = {
    id: `${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
    timestamp: Date.now(),
    dateStr: new Date().toLocaleString("zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
      hour12: false,
    }),
    url,
    title: title || (file ? file.split(/[/\\]/).pop() : url),
    file,
    status,
    error,
    format,
  };

  history.unshift(newItem);
  saveHistory(history);
  return newItem;
}

export function removeHistoryItem(id) {
  const history = getHistory().filter((item) => item.id !== id);
  saveHistory(history);
  return history;
}

export function clearHistory() {
  localStorage.removeItem(HISTORY_STORAGE_KEY);
}
