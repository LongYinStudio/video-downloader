import { createI18n } from "vue-i18n";
import zhCN from "./locales/zh-CN.js";
import enUS from "./locales/en-US.js";
import ptBR from "./locales/pt-BR.js";
import zhCnElement from "element-plus/es/locale/lang/zh-cn";
import enElement from "element-plus/es/locale/lang/en";
import ptBrElement from "element-plus/es/locale/lang/pt-br";

export const STORAGE_LANG_KEY = "vd.language";

export const SUPPORTED_LANGUAGES = [
  { label: "简体中文", value: "zh-CN" },
  { label: "English", value: "en-US" },
  { label: "Português (Brasil)", value: "pt-BR" },
];

export const elementLocales = {
  "zh-CN": zhCnElement,
  "en-US": enElement,
  "pt-BR": ptBrElement,
};

export function getDefaultLanguage() {
  if (typeof localStorage !== "undefined") {
    const saved = localStorage.getItem(STORAGE_LANG_KEY);
    if (saved && ["zh-CN", "en-US", "pt-BR"].includes(saved)) {
      return saved;
    }
  }
  const navLang =
    typeof navigator !== "undefined"
      ? (navigator.language || "").toLowerCase()
      : "";
  if (navLang.startsWith("zh")) {
    return "zh-CN";
  }
  if (navLang.startsWith("pt")) {
    return "pt-BR";
  }
  if (navLang.startsWith("en")) {
    return "en-US";
  }
  return "en-US";
}

const initialLang = getDefaultLanguage();

const i18n = createI18n({
  legacy: false,
  locale: initialLang,
  fallbackLocale: "en-US",
  messages: {
    "zh-CN": zhCN,
    "en-US": enUS,
    "pt-BR": ptBR,
  },
});

export function setAppLanguage(lang) {
  if (!["zh-CN", "en-US", "pt-BR"].includes(lang)) return;
  i18n.global.locale.value = lang;
  if (typeof localStorage !== "undefined") {
    localStorage.setItem(STORAGE_LANG_KEY, lang);
  }
}

export default i18n;
