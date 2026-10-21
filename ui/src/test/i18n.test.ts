// SHELL-08 guard: every UI string comes from tr.json through i18next.
// Runs in CI via `pnpm -C ui vitest run i18n`.
import { describe, expect, it } from "vitest";
import i18n, { formatRelativeTr, formatTimeTr } from "../i18n";
import tr from "../i18n/tr.json";

type Catalog = { [key: string]: string | Catalog };

function flatten(node: Catalog, prefix = ""): Map<string, string> {
  const out = new Map<string, string>();
  for (const [key, value] of Object.entries(node)) {
    const path = prefix ? `${prefix}.${key}` : key;
    if (typeof value === "string") {
      out.set(path, value);
    } else {
      for (const [k, v] of flatten(value, path)) out.set(k, v);
    }
  }
  return out;
}

const catalog = flatten(tr as Catalog);

// Source files under ui/src, loaded as raw text by Vite (tests and generated bindings excluded).
const sources = import.meta.glob<string>(
  ["../**/*.{ts,tsx}", "!../**/*.test.{ts,tsx}", "!../test/**", "!../bindings.ts", "!../**/*.d.ts"],
  { query: "?raw", import: "default", eager: true },
);

function stripComments(code: string): string {
  return code.replace(/\/\*[\s\S]*?\*\//g, "").replace(/(^|[^:"'`])\/\/.*$/gm, "$1");
}

const KEY_PATTERN = /(?:\bt\(\s*|i18nKey=\{?\s*)(["'`])([^"'`]+?)\1/g;

function extractKeys(code: string): string[] {
  const keys: string[] = [];
  for (const match of stripComments(code).matchAll(KEY_PATTERN)) {
    const key = match[2];
    // Template literals with interpolation are dynamic keys; they cannot be checked statically.
    if (!key.includes("${")) keys.push(key);
  }
  return keys;
}

// `>` (not part of `=>`), optional whitespace, a Latin/Turkish letter, text without `<`/`{`/`}`,
// then the start of a tag.
const RAW_JSX_TEXT = /(?<![=-])>\s*([A-Za-zÇĞİÖŞÜçğıöşü][^<{}]*)<\/?[A-Za-z]/g;

function findRawJsxText(code: string): string[] {
  return [...stripComments(code).matchAll(RAW_JSX_TEXT)].map((m) => m[1].trim());
}

const REQUIRED_KEYS = [
  "app.name",
  "titlebar.minimize",
  "titlebar.maximize",
  "titlebar.restore",
  "titlebar.close",
  "nav.status",
  "nav.settings",
  "status.title",
  "status.lastCheck",
  "status.never",
  "status.version",
  "status.lastResume",
  "status.catchUp",
  "status.health.ok",
  "status.health.degraded",
  "status.health.error",
  "status.runs.title",
  "status.runs.empty",
  "status.runs.catchUp",
  "status.runs.running",
  "status.runs.ok",
  "status.runs.failed",
  "status.runs.timedOut",
  "job.heartbeat",
  "job.demo_check",
  "job.update_check",
  "pause.menu",
  "pause.oneHour",
  "pause.untilTomorrow",
  "pause.indefinite",
  "pause.resume",
  "pause.status.until",
  "pause.status.indefinite",
  "pause.when.today",
  "pause.when.tomorrow",
  "pause.when.date",
  "tray.open",
  "tray.quit",
  "tray.restartUpdate",
  "tray.unread",
  "tray.error",
  "time.lastCheck.never",
  "time.lastCheck.justNow",
  "time.lastCheck.minutes",
  "time.lastCheck.hours",
  "time.lastCheck.days",
  "toast.test.title",
  "toast.test.body",
  "toast.action.open",
  "toast.action.mute",
  "toast.action.restart",
  "toast.update.title",
  "toast.update.body",
  "settings.title",
  "settings.general.title",
  "settings.autostart.label",
  "settings.autostart.hint",
  "settings.logs.open",
  "settings.notifications.title",
  "settings.notifications.test",
  "settings.notifications.sent",
  "settings.sound.label",
  "settings.sound.hint",
  "settings.sound.strategy.label",
  "settings.sound.strategy.toast",
  "settings.sound.strategy.app",
  "settings.sound.perSource",
  "settings.sound.source.system",
  "settings.sound.source.updates",
  "settings.sound.muted",
  "settings.sound.unmute",
  "settings.keys.title",
  "settings.keys.empty",
  "settings.keys.name",
  "settings.keys.value",
  "settings.keys.add",
  "settings.keys.save",
  "settings.keys.test",
  "settings.keys.change",
  "settings.keys.delete",
  "settings.keys.cancel",
  "settings.keys.updated",
  "settings.keys.notTestable",
  "settings.keys.testing",
  "settings.keys.valid",
  "settings.keys.invalid",
  "settings.keys.unreachable",
  "settings.keys.saveAnyway",
  "settings.keys.saved",
  "settings.keys.deleted",
  "settings.keys.nameInvalid",
  "settings.keys.valueInvalid",
  "vault.keyLost",
  "vault.keyMismatch",
  "vault.unavailable",
  "vault.reset",
  "vault.resetConfirm",
  "settings.updates.title",
  "settings.updates.current",
  "settings.updates.check",
  "settings.updates.checking",
  "settings.updates.downloading",
  "settings.updates.upToDate",
  "settings.updates.ready",
  "settings.updates.install",
  "settings.updates.failed",
  "settings.updates.idle",
  "providers.anthropic",
  "providers.gemini",
  "providers.openRouter",
  "providers.openAi",
  "providers.gitHub",
  "providers.slack",
  "errors.generic",
  "common.yes",
  "common.no",
  "common.on",
  "common.off",
] as const;

describe("i18n runtime", () => {
  it("initialises synchronously with Turkish as the only language", () => {
    expect(i18n.isInitialized).toBe(true);
    expect(i18n.language).toBe("tr");
    expect(i18n.options.fallbackLng).toEqual(["tr"]);
    expect(i18n.t("pause.untilTomorrow")).toBe("Yarına kadar");
  });

  it("interpolates {{count}} values", () => {
    expect(i18n.t("time.lastCheck.minutes", { count: 3 })).toBe("son kontrol: 3 dk önce");
    expect(i18n.t("tray.unread", { count: 4 })).toBe("4 okunmamış");
  });

  it("formats relative times and clock times in tr-TR", () => {
    const now = Date.UTC(2026, 9, 21, 9, 0, 0);
    expect(formatRelativeTr(now - 3 * 60_000, now)).toBe("3 dakika önce");
    expect(formatRelativeTr(now - 2 * 3_600_000, now)).toBe("2 saat önce");
    expect(formatRelativeTr(now - 3 * 86_400_000, now)).toBe("3 gün önce");
    expect(formatTimeTr(now)).toMatch(/^\d{2}:\d{2}$/);
  });
});

describe("tr.json catalog", () => {
  it("contains every Phase 1 key", () => {
    const missing = REQUIRED_KEYS.filter((key) => !catalog.has(key));
    expect(missing).toEqual([]);
  });

  it("has no empty values", () => {
    const empty = [...catalog].filter(([, value]) => value.trim() === "").map(([key]) => key);
    expect(empty).toEqual([]);
  });
});

describe("source guard", () => {
  it("scans the UI sources", () => {
    expect(Object.keys(sources).length).toBeGreaterThan(0);
    expect(Object.keys(sources).some((path) => path.endsWith("App.tsx"))).toBe(true);
  });

  it("detects keys and raw JSX text (self-test)", () => {
    expect(extractKeys(`t("a.b"); i18n.t('c.d', { x }); <Trans i18nKey="e.f" />`)).toEqual([
      "a.b",
      "c.d",
      "e.f",
    ]);
    expect(findRawJsxText(`<p>Merhaba dünya</p>`)).toEqual(["Merhaba dünya"]);
    expect(findRawJsxText(`<p>İptal</p>`)).toEqual(["İptal"]);
    expect(findRawJsxText(`<p>{t("app.name")}</p>`)).toEqual([]);
    expect(findRawJsxText(`const f = () => x < y;`)).toEqual([]);
  });

  it('every t("…") / i18nKey literal exists in tr.json', () => {
    const missing: string[] = [];
    for (const [path, code] of Object.entries(sources)) {
      for (const key of extractKeys(code)) {
        if (!catalog.has(key)) missing.push(`${path}: ${key}`);
      }
    }
    expect(missing).toEqual([]);
  });

  it("no .tsx file contains raw letter text inside JSX", () => {
    const offenders: string[] = [];
    for (const [path, code] of Object.entries(sources)) {
      if (!path.endsWith(".tsx")) continue;
      for (const text of findRawJsxText(code)) offenders.push(`${path}: ${text}`);
    }
    expect(offenders).toEqual([]);
  });
});
