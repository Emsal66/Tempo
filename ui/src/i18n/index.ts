import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import tr from "./tr.json";

// Turkish is the only and fallback language (SHELL-08). tr.json is the single string catalog;
// the Rust core embeds the same file, so keys must stay stable.
// Synchronous init: tests and the first render get strings without waiting.
i18n.use(initReactI18next).init({
  lng: "tr",
  fallbackLng: "tr",
  supportedLngs: ["tr"],
  resources: { tr: { translation: tr } },
  interpolation: { escapeValue: false },
  returnNull: false,
  initAsync: false,
});

const relativeFormat = new Intl.RelativeTimeFormat("tr-TR", { numeric: "auto" });
const timeFormat = new Intl.DateTimeFormat("tr-TR", { hour: "2-digit", minute: "2-digit" });

const MINUTE_MS = 60_000;
const HOUR_MS = 60 * MINUTE_MS;
const DAY_MS = 24 * HOUR_MS;

/** Relative time in Turkish ("3 dakika önce", "2 saat önce", "dün"). */
export function formatRelativeTr(ms: number, nowMs: number): string {
  const diff = ms - nowMs;
  const abs = Math.abs(diff);
  if (abs < HOUR_MS) {
    return relativeFormat.format(Math.trunc(diff / MINUTE_MS), "minute");
  }
  if (abs < DAY_MS) {
    return relativeFormat.format(Math.trunc(diff / HOUR_MS), "hour");
  }
  return relativeFormat.format(Math.trunc(diff / DAY_MS), "day");
}

/** Wall-clock time in Turkish format ("08:05"). */
export function formatTimeTr(ms: number): string {
  return timeFormat.format(ms);
}

export default i18n;
