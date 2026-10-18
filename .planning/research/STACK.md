# Stack Research

**Domain:** Windows 10/11 desktop "personal command center": tray-resident multi-service dashboard with a plugin architecture (built-in connectors plus an MCP client), AI morning summary, background monitors, and local-only data
**Project:** Mornfold
**Researched:** 2026-10-18
**Confidence:** HIGH for the framework choice and core versions (all checked against the npm, crates.io and NuGet registries and official docs on 2026-10-18). MEDIUM for the CI/testing and code-signing details. LOW for the X/Twitter pricing figures (from third-party blogs only).

---

## TL;DR (the prescriptive answer)

**Use Tauri 2 (Rust core + WebView2) with a React 19 / TypeScript / Vite / Tailwind 4 / Motion frontend.** All integration logic lives in Rust: MCP client (`rmcp`), Google, LLMs, scheduler, scraping and secrets. The webview is purely the UI. Store data in SQLite through `sqlx`. Keep secrets in Windows Credential Manager through `keyring-core` and `windows-native-keyring-store`. Ship an NSIS installer built on GitHub Actions `windows-2025` runners, using `tauri-action` and `tauri-plugin-updater` with GitHub Releases.

Why Tauri wins for this project:
1. **Windows-only removes Tauri's biggest weakness.** Tauri's usual problem is cross-platform webview inconsistency (WebKit on macOS/Linux). On Windows the webview is WebView2, an evergreen Chromium that ships with Windows 10 (since April 2018) and Windows 11. You get Chromium rendering, the same as Electron, without bundling Chromium.
2. **It is tray-resident all day.** The Rust core keeps running schedulers and monitors with no window open (you can destroy the webview while hidden). Electron always carries Node plus the Chromium browser process.
3. **The installer is small enough to distribute.** NSIS output is around 5-15 MB with the default `downloadBootstrapper` WebView2 mode, which adds 0 MB. Electron installers are around 90-120 MB.
4. **Every required capability has a first-party answer:**
   - Tray: core `tray-icon` feature.
   - Notifications, autostart, single-instance, updater, deep-link, window-state and global-shortcut: official plugins.
   - Mica/Acrylic: built into the window config (`windowEffects`).
   - MCP: the **Rust MCP SDK (`rmcp`) is now Tier 1**, alongside TypeScript, Python, C# and Go, and implements spec `2026-07-28`.
5. **The privacy model fits the Core Value.** API keys and OAuth tokens never enter the webview. Tauri's capability ACL limits what the UI can call.

---

## Framework Decision: Tauri 2 vs Electron vs WinUI 3/.NET

| Criterion | **Tauri 2.12** (recommended) | Electron 44 | WinUI 3 / Windows App SDK 2.5 (.NET) |
|---|---|---|---|
| Glass / Mica / Acrylic | Built in: `windowEffects` in `tauri.conf.json` (Mica, MicaDark/Light, Tabbed, Acrylic, Blur) plus the `window-vibrancy` 0.8 crate. Mica is Win11-only. Acrylic/Blur lag on drag/resize on some builds (documented). | `backgroundMaterial: 'auto'/'none'/'mica'/'acrylic'/'tabbed'` (Win11 22H2+) | Native `MicaBackdrop` / `DesktopAcrylicBackdrop`. Best fidelity. |
| "Linear-grade" UI and animation | Full web stack (React, Tailwind, Motion), rendered in Chromium (WebView2) | Same web stack, Chromium | XAML plus Composition animations. Much more effort for a Linear-style look, and a thin component ecosystem. |
| Tray | Core (`tray-icon` feature) | Core `Tray` | **No built-in tray.** Needs the third-party H.NotifyIcon (2.4.1). |
| Windows toasts | `tauri-plugin-notification` 2.7: works only for installed apps, and actions are mobile-only. Use `tauri-winrt-notification` 0.8.1 directly for click/button callbacks. | Built-in `Notification` with click events. Needs an AppUserModelID set. | `AppNotificationManager`, native and best-in-class |
| Bundle size | ~5-15 MB NSIS (WebView2 already on the OS) | ~90-120 MB (bundled Chromium plus Node) | Small to medium. The Windows App SDK runtime is a separate dependency. |
| Idle RAM (tray, window hidden) | Rust core only, a few tens of MB if the webview is destroyed | Main process (Node plus browser process), ~80-150 MB+ | Low |
| MCP client SDK | `rmcp` 3.5.1: **official, Tier 1**, child-process stdio plus streamable HTTP client plus OAuth | `@modelcontextprotocol/sdk` 1.32.1: official, Tier 1 (the reference SDK) | `ModelContextProtocol` 2.2.0: official, Tier 1 |
| LLM SDKs | `genai` 0.6.5 (native Anthropic, Gemini and OpenRouter adapters, streaming, tools). No official Rust SDKs. | Official `@anthropic-ai/sdk` 0.135, `@google/genai` 2.30, Vercel `ai` 7 | Official `Anthropic` 12.58, `Google.GenAI` 1.26 |
| Google OAuth loopback | `oauth2` 5.0 (PKCE) plus a localhost listener (`tauri-plugin-oauth` 2.1) | `google-auth-library` 11 plus a local server | `GoogleWebAuthorizationBroker` (built-in loopback). Easiest. |
| Background scheduling | tokio tasks in the Rust core, independent of any window | Node timers in the main process | .NET hosted services |
| Secrets (DPAPI / Credential Manager) | `keyring-core` plus `windows-native-keyring-store` (Credential Manager) | `safeStorage` (DPAPI) is built in. `keytar` is archived. | `PasswordVault` / DPAPI, native |
| Build from Linux | Rust/frontend tests and `cargo xwin check` run on Linux. The app runs on Linux (WebKitGTK) for dev. NSIS cross-compile is possible but "a last resort", so build releases on a Windows runner. | Runs identically on Linux. electron-builder can target Windows from Linux. | **Cannot build or run XAML on Linux.** The UI can't be seen or iterated on in the dev environment. |
| Auto-update | `tauri-plugin-updater` 2.14 (signed `latest.json`, GitHub Releases) | `electron-updater` 6.8 | MSIX / Store or a custom solution |
| Verdict | **Best fit:** small, light, secure, all capabilities covered | Viable fallback. Pick it only if you insist on a single-language TS codebase. | Rejected. Linux dev environment plus design-iteration cost. |

**Confidence: HIGH.** Versions and capabilities were checked against registries, the Tauri source (`tauri-utils/src/config.rs` WindowEffect enum), the Tauri docs, the Electron docs (`base-window-options.md`) and the MCP SDK tier page. RAM and bundle numbers are MEDIUM (typical community figures, not benchmarked here).

---

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended | Conf. |
|---|---|---|---|---|
| **Tauri** (`tauri`, `@tauri-apps/cli`, `@tauri-apps/api`) | 2.12.4 / cli 2.12.3 / api 2.12.4 | Desktop shell, IPC, tray, windows, bundler | See the decision above. Enable features `tray-icon` and `image-png`. | HIGH |
| **Rust** (stable) | 1.99.1 (MSRV floor: 1.90, required by tauri-plugin-notification) | Core/backend language | Tauri backend. `rmcp` targets edition 2024. | HIGH |
| **tokio** | 1.53 | Async runtime | Tauri already runs on tokio. Schedulers, HTTP and MCP child processes all share it. | HIGH |
| **React** | 19.3.1 | UI framework | Largest ecosystem for premium UI kits (shadcn, Motion, cmdk, sonner). Agent-friendly. | HIGH |
| **TypeScript** | 7.0.2 (native Go compiler) | Frontend typing | About 10x faster type checks. Pair it with Biome, which does not depend on the TS compiler API. If a tool needs the old API, `@typescript/typescript6` provides a side-by-side `tsc6`. | MEDIUM (TS 7 is new; the public API arrives in 7.1) |
| **Vite** | 8.3.4 | Frontend dev server/bundler | Tauri's default template. Gives instant HMR in the WebView2 window and in a plain browser. | HIGH |
| **Tailwind CSS** | 4.3.3 | Styling | CSS-first config and `backdrop-blur`/`bg-white/5` style utilities make glass layers fast to build. | HIGH |
| **Motion** (formerly Framer Motion) | 14.3.0 (`motion` package, import from `motion/react`) | Animation | Spring physics, layout animations and `AnimatePresence`. This is what makes the UI feel like Apple/Linear. | HIGH |
| **shadcn/ui** (on Base UI) | CLI 4.22.0, `@base-ui/react` 1.9.0 | Accessible component primitives (dialogs, menus, popovers, tooltips) | Copy-in components you restyle fully, with no visual lock-in. Base UI has been the shadcn default since July 2026. Radix (`radix-ui` 1.8.1) is still supported via `-b radix`. | HIGH |
| **SQLite via sqlx** | `sqlx` 0.9.0 (features `sqlite`, `runtime-tokio`, `migrate`, `macros`) | Local DB: cards, monitors, snapshots, diffs, notifications, job state, cache | Async, connection pool, embedded migrations (`sqlx::migrate!`), compile-time-checked queries (use offline mode, `cargo sqlx prepare`, so CI needs no DB). Turn on WAL mode. | HIGH |
| **rmcp** (official Rust MCP SDK) | 3.5.1. Features: `client`, `transport-child-process`, `which-command`, `transport-streamable-http-client-reqwest`, `auth`, `reqwest` | MCP client (stdio servers via `npx`/`uvx`, remote streamable-HTTP servers, OAuth for remote servers) | Tier 1 official SDK. Implements spec `2026-07-28` with legacy negotiation back to `2025-11-25`. `which-command` resolves `npx.cmd` on Windows. | HIGH |
| **keyring-core** plus **windows-native-keyring-store** | 1.0.1 / 1.1.0 | Store API keys and OAuth refresh tokens in Windows Credential Manager (DPAPI-backed) | keyring-rs v4 split the API (`keyring-core`) from the per-OS stores. Its README tells apps to depend on these crates directly rather than the all-in-one `keyring` 4.2. | HIGH |
| **reqwest** | 0.13.5 (rustls) | All outbound HTTP: Google REST, X API, free-API cards, web monitors | The same major version `rmcp` and `genai` use, so one HTTP stack. | HIGH |

### Tauri Plugins (all official, same 2.x train, published 2026-09/10)

| Plugin (Rust crate / npm) | Version | Purpose | Notes |
|---|---|---|---|
| `tauri-plugin-single-instance` | 2.5.4 | Second launch focuses the existing instance | **Register it first**, before the other plugins. Essential for a tray app plus autostart. |
| `tauri-plugin-autostart` | 2.7.0 | Launch at Windows login (minimized to tray) | Pass a `--minimized` arg so autostart launches straight to tray. |
| `tauri-plugin-notification` | 2.7.0 | Simple toasts | Windows: "only works for installed apps". In dev it shows the PowerShell name and icon. Actions are mobile-only. For clickable toasts and buttons, use `tauri-winrt-notification` (below). |
| `tauri-plugin-updater` (+ `tauri-plugin-process` 2.4.0 for relaunch) | 2.14.0 | Auto-update from a GitHub Releases `latest.json` | Updates must be signed (minisign key pair). Keep the private key in GitHub secrets. |
| `tauri-plugin-window-state` | 2.5.0 | Remember window size and position | |
| `tauri-plugin-global-shortcut` | 2.4.1 | Global hotkey to summon Mornfold | Optional, but cheap delight |
| `tauri-plugin-positioner` | 2.4.0 | Anchor a tray flyout window near the tray icon | For a quick-glance "mini" panel |
| `tauri-plugin-opener` | 2.7.2 | Open the system browser (OAuth consent, links) | Replaces the old `shell.open` |
| `tauri-plugin-log` | 2.10.1 | Rotating local log files plus frontend log bridge | Local-only diagnostics (no telemetry) |
| `tauri-plugin-oauth` (FabianLars) | 2.1.0 | Temporary localhost server for the OAuth loopback redirect | Use its Rust API from the core. Alternatively write ~60 lines yourself on `tokio::net::TcpListener` bound to `127.0.0.1:0`. |
| `tauri-plugin-prevent-default` | 6.0.1 | Disable browser-ish shortcuts (F5, Ctrl+P, context menu) in production | Makes the app feel native, not like a website |
| `tauri-plugin-deep-link` | 2.6.2 | Optional: a `mornfold://` scheme for future flows | **Not for Google OAuth.** Google no longer supports custom URI schemes. |

### Supporting Libraries: Rust Core

| Library | Version | Purpose | When to Use | Conf. |
|---|---|---|---|---|
| `genai` | 0.6.5 | One API across Anthropic, Gemini and OpenRouter (all native adapters), with streaming and tool calls | AI morning summary, and later AI-over-MCP-tools. **Wrap it behind your own `LlmProvider` trait.** It's pre-1.0 with a single maintainer, so you'll want to be able to swap in hand-rolled reqwest calls. | MEDIUM |
| `oauth2` | 5.0.0 (`default-features = false`) | Google OAuth 2.0 auth code + PKCE (S256), token refresh | Google connector. Its built-in reqwest feature pins reqwest 0.12, so disable defaults and implement its `AsyncHttpClient` over reqwest 0.13, as `rmcp` does. | HIGH |
| `mail-builder` / `mail-parser` | 1.0.0 / 0.11.9 | Build RFC 5322 replies (In-Reply-To/References headers) and parse Gmail `format=raw` bodies | Gmail read and reply. Call the Gmail and Calendar REST APIs directly with reqwest plus serde structs. Don't use the generated `google-gmail1` crate (heavy, coupled to yup-oauth2). | HIGH |
| `croner` | 4.0.1 | Cron-expression parsing (including seconds and `L`/`W`) for user schedules | Scheduler: compute `next_run_at`, persist it in SQLite, and run one tokio dispatcher loop. See the patterns section. | HIGH |
| `scraper` | 0.27.0 | CSS-selector extraction from HTML (html5ever-based) | Website change detection ("watch this element"), custom cards | HIGH |
| `dom_smoothie` | 0.18.2 | Readability-style main-content extraction | Monitoring a whole page without a selector (strips nav and ads noise) | MEDIUM |
| `htmd` | 0.5.6 | HTML to Markdown/text normalization before diffing | Stable diffs (whitespace and attribute noise removed). Also gives LLM-friendly text. | MEDIUM |
| `similar` | 3.2.0 | Text diffing (Myers/Patience), unified diff, inline change ops | Producing "what changed" snippets for notifications and the diff view | HIGH |
| `blake3` | 1.8.7 | Content hashing | Cheap "did anything change?" check before running a diff | HIGH |
| `jsonpath-rust` | 1.0.12 | RFC 9535 JSONPath | Custom free-API cards: "enter URL, pick field" maps to a JSONPath expression | MEDIUM |
| `tauri-winrt-notification` | 0.8.1 | Raw WinRT toasts with `on_activated`, `add_button`, hero/icon images, `on_dismissed` | Clickable notifications ("price dropped", click opens the monitor). Use the bundle identifier as the AppUserModelID. In dev, use `Toast::POWERSHELL_APP_ID`. | HIGH |
| `tauri-specta` + `specta` | 2.0.0-rc.25 (pin with `=`) | Generate TypeScript types **and** typed `invoke`/event wrappers from Rust commands | Keeps the frontend and the Rust IPC contract in sync, which is important when an agent edits both sides. It is a long-running RC but widely used. If it breaks, fall back to `ts-rs` 12.0.1 (types only). | MEDIUM |
| `serde` / `serde_json` | 1.0.229 / 1.0.154 | Serialization | Everywhere | HIGH |
| `thiserror` / `anyhow` | 2.0.21 / 1.0.104 | Typed errors in the core crates / app-level errors | `thiserror` in library crates, `anyhow` at the edges | HIGH |
| `tracing` | 0.1.44 | Structured logging (bridged to tauri-plugin-log) | Everywhere | HIGH |
| `backon` | 1.6.0 | Retry with backoff | Flaky network calls (Google, X, monitors) | HIGH |
| `governor` | 0.10.4 | Rate limiting | Per-host politeness for web monitors. Protects the X API budget. | HIGH |
| `jiff` | 0.2.38 | Timezone-correct datetimes (DST-safe "every day at 08:00 Europe/Istanbul") | Scheduler and calendar math. Better DST semantics than `chrono` for user-facing schedules. | MEDIUM |
| `windows` | 0.62.2 | Win32/WinRT escape hatch (power-resume events `WM_POWERBROADCAST`, DPAPI `CryptProtectData` for large blobs) | Only behind `#[cfg(windows)]` modules | HIGH |
| `url` | 2.5.8 | URL parsing and validation | User-entered monitor and API URLs | HIGH |

### Supporting Libraries: Frontend

| Library | Version | Purpose | When to Use | Conf. |
|---|---|---|---|---|
| `@tanstack/react-query` | 5.104.4 | Cache/async state over Tauri `invoke` | All data reads. Invalidate on Rust-emitted events ("gmail.updated"). | HIGH |
| `zustand` | 5.0.15 | Small UI state (selected card, panels, theme) | Local UI state only. Server/core state goes in React Query. | HIGH |
| `@tanstack/react-router` | 1.170.45 | Type-safe routing (Summary, Inbox, Calendar, Monitors, Connections, Settings) | Main window navigation | MEDIUM |
| `cmdk` | 1.1.1 | Ctrl+K command palette | The signature Linear-style interaction | HIGH |
| `sonner` | 2.0.8 | In-app toasts | In-window feedback (separate from OS toasts) | HIGH |
| `lucide-react` | 1.59.0 | Icons (thin stroke fits the aesthetic) | Everywhere | HIGH |
| `@fontsource-variable/inter` (or `@fontsource-variable/geist`) | 5.3.0 | Bundled variable font (no CDN, works offline) | Inter has full Turkish glyph coverage (ğ ş ı İ ç ö ü). Bundle it locally; never load Google Fonts at runtime (privacy). | HIGH |
| `i18next` + `react-i18next` | 26.5.2 / 17.1.0 | i18n: Turkish first, extensible to others | All UI strings from day one. Use `tr-TR` locale for dates, numbers and casing. | HIGH |
| `date-fns` | 4.4.0 | Date formatting with the `tr` locale | Calendar and summary display | HIGH |
| `@tanstack/react-virtual` | 3.14.21 | Virtualized lists | Long mail, notification history and diff lists | MEDIUM |
| `zod` | 4.6.7 | Runtime validation (forms, custom-API definitions, MCP server config import) | Setup wizard and "add custom API" forms | HIGH |
| `react-markdown` | 10.1.0 | Render AI summary if it contains light formatting | Only if the summary uses Markdown. PROJECT.md says plain text. | LOW (optional) |

### Development Tools

| Tool | Version | Purpose | Notes |
|---|---|---|---|
| Node.js | 24.x LTS (24.22.0) | Frontend toolchain | Node 26 becomes LTS soon. Stay on 24 for now. |
| pnpm | 12.14.0 | Package manager | Fast, strict. Commit the lockfile. |
| Biome | 2.5.15 | Lint and format TS/TSX/JSON/CSS | One tool. Doesn't depend on the TS compiler API, so it works fine with TS 7. |
| Vitest + Testing Library | 5.0.5 / `@testing-library/react` 16.3.3 | Frontend unit and component tests | Use `@tauri-apps/api/mocks` (`mockIPC`, `mockWindows`) to fake the Rust side |
| Playwright | 1.64.1 | (a) Visual and interaction tests of the UI in Chromium on Linux against the Vite dev server with mocked IPC. (b) Optional Windows e2e via `connectOverCDP` to WebView2 | Linux Chromium renders very close to WebView2, so the agent can screenshot and iterate on the design on Linux. On Windows, set `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9222` (Tauri also exposes `additionalBrowserArgs`) and attach (MEDIUM: documented for WebView2, not Tauri-specific). |
| WebdriverIO + `@wdio/tauri-service` + `tauri-driver` | wdio 10.0.2 / service 1.5.0 / tauri-driver 2.1.0 | Official Tauri e2e on the real built binary | `tauri-driver` supports Windows and Linux directly. The wdio service keeps msedgedriver in sync on Windows. Run smoke e2e on the `windows-2025` runner. |
| `cargo-xwin` | 0.23.1 | Type-check and build for `x86_64-pc-windows-msvc` **from Linux** | Run `cargo xwin check --target x86_64-pc-windows-msvc` in the Linux dev loop to catch `#[cfg(windows)]` compile errors before CI. It downloads the MSVC CRT/SDK, so accept the license. |
| `cargo-nextest`, `insta` 1.49, `wiremock` 0.6.5 | — | Rust tests: snapshot tests for diff and summary prompt output, HTTP mocks for Google, X and LLM | All business logic is testable on Linux with no Windows involved |
| `sqlx-cli` | 0.9 | Migrations plus `cargo sqlx prepare` (offline query metadata) | Commit the `.sqlx/` directory so CI builds without a DB |
| GitHub Actions `windows-2025` (= `windows-latest`) | — | Build, test, bundle and release Windows binaries | Pin `windows-2025`, not `-latest`. Use `tauri-apps/tauri-action` (`action-v1.0.0`, has breaking changes from v0) for releases. |

---

## Installation

```bash
# Scaffold (React + TS template), then restructure into a workspace (see patterns)
pnpm create tauri-app@latest mornfold --template react-ts --manager pnpm

# Frontend core
pnpm add react@19 react-dom@19 @tauri-apps/api@2 motion@14 \
  @tanstack/react-query@5 @tanstack/react-router@1 zustand@5 \
  cmdk@1 sonner@2 lucide-react zod@4 i18next react-i18next date-fns@4 \
  @fontsource-variable/inter @tanstack/react-virtual@3

# Tauri plugin JS bindings (only the ones the UI actually calls)
pnpm add @tauri-apps/plugin-notification@2 @tauri-apps/plugin-updater@2 \
  @tauri-apps/plugin-process@2 @tauri-apps/plugin-autostart@2 \
  @tauri-apps/plugin-window-state@2 @tauri-apps/plugin-opener@2 \
  @tauri-apps/plugin-global-shortcut@2 @tauri-apps/plugin-log@2

# UI kit (Base UI default; copies components into src/components/ui)
pnpm dlx shadcn@latest init

# Dev
pnpm add -D typescript@7 vite@8 @vitejs/plugin-react tailwindcss@4 @tailwindcss/vite \
  @tauri-apps/cli@2 @biomejs/biome vitest@5 @testing-library/react @playwright/test \
  webdriverio @wdio/cli @wdio/tauri-service
```

```toml
# src-tauri/Cargo.toml (shell crate). Business logic lives in crates/mornfold-core
[dependencies]
tauri = { version = "2.12", features = ["tray-icon", "image-png"] }
tauri-plugin-single-instance = "2.5"
tauri-plugin-autostart = "2.7"
tauri-plugin-notification = "2.7"
tauri-plugin-updater = "2.14"
tauri-plugin-process = "2.4"
tauri-plugin-window-state = "2.5"
tauri-plugin-global-shortcut = "2.4"
tauri-plugin-positioner = { version = "2.4", features = ["tray-icon"] }
tauri-plugin-opener = "2.7"
tauri-plugin-log = "2.10"
tauri-plugin-oauth = "2.1"
tauri-plugin-prevent-default = "6"
tauri-specta = { version = "=2.0.0-rc.25", features = ["derive", "typescript"] }
specta = "=2.0.0-rc.25"

# crates/mornfold-core/Cargo.toml (NO tauri dependency: testable on Linux)
tokio = { version = "1.53", features = ["full"] }
reqwest = { version = "0.13", default-features = false, features = ["rustls", "json", "gzip", "brotli", "stream"] }
sqlx = { version = "0.9", default-features = false, features = ["sqlite", "runtime-tokio", "migrate", "macros", "json"] }
rmcp = { version = "3.5", default-features = false, features = ["client", "transport-child-process", "which-command", "transport-streamable-http-client-reqwest", "auth", "reqwest"] }
genai = "0.6"
oauth2 = { version = "5.0", default-features = false }
keyring-core = "1.0"
croner = "4"
jiff = "0.2"
scraper = "0.27"
dom_smoothie = "0.18"
htmd = "0.5"
similar = "3.2"
blake3 = "1.8"
jsonpath-rust = "1.0"
mail-builder = "1.0"
mail-parser = "0.11"
backon = "1.6"
governor = "0.10"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tracing = "0.1"
url = "2.5"

[target.'cfg(windows)'.dependencies]
windows-native-keyring-store = "1.1"
tauri-winrt-notification = "0.8"
windows = { version = "0.62", features = ["Win32_Security_Cryptography", "Win32_UI_WindowsAndMessaging", "Win32_System_Power"] }

[dev-dependencies]
insta = "1.49"
wiremock = "0.6"
```

(Feature names for `windows` and `reqwest` are indicative. Confirm them during Phase 1 scaffolding.)

---

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|---|---|---|
| Tauri 2 | **Electron 44** (+ electron-vite 6, electron-builder 26.15, electron-updater 6.8) | If the team refuses Rust and wants one TypeScript codebase using the official JS SDKs (Anthropic, `@google/genai`, `googleapis`, Vercel AI SDK, MCP TS SDK). It's a real option: `backgroundMaterial: 'mica'`, `safeStorage` (DPAPI) and `Tray` are all built in. Costs: ~100 MB installer, higher idle RAM for an always-on tray app, Node-in-main-process security surface. |
| Tauri 2 | WinUI 3 / Windows App SDK 2.5 + .NET (MCP C# SDK 2.2 Tier 1, `Anthropic` 12.58, `Google.GenAI` 1.26, Google APIs .NET) | If development moved to a Windows machine and native Fluent look mattered more than a custom Linear look. Rejected here: XAML can't be built or run on the Linux dev box, there's no built-in tray, and custom premium design costs much more. |
| Tauri 2 | Wails (Go, WebView2) | The Go MCP SDK is also Tier 1, but Wails v3 has been in long alpha and its plugin ecosystem is far smaller than Tauri's (LOW confidence on current v3 status). |
| `rmcp` in Rust | MCP TS SDK in a Node sidecar | Only if a needed MCP feature is missing in `rmcp`. A sidecar adds 40-80 MB and a second runtime. Both SDKs are Tier 1, so this should not be needed. |
| `genai` | `rig-core` 0.44 (agent framework, has `rig-rmcp` integration) | If the AI summary grows into a multi-step agent that calls MCP tools. Rig gives agent loops; genai is a thinner client. Revisit at the "AI over MCP tools" phase. |
| `genai` | Hand-rolled reqwest clients (Anthropic Messages, Gemini `generateContent`, OpenRouter OpenAI-compatible) | If `genai` churns or lags a provider feature. Around 300 lines including SSE streaming. The `LlmProvider` trait makes this a drop-in swap. |
| `sqlx` | `rusqlite` 0.40 (`bundled`) + `rusqlite_migration` 2.6 | If you prefer synchronous, simpler code and want SQLCipher (`bundled-sqlcipher`) for an encrypted DB. **Never use both.** They link different `libsqlite3-sys` versions (`links = "sqlite3"` conflict). |
| Own scheduler (croner + SQLite + tokio) | `tokio-cron-scheduler` 0.15 / `apalis` 1.0 | tokio-cron-scheduler is in-memory and doesn't handle missed runs after sleep. apalis is a full job queue (overkill for a single-user desktop app). |
| `keyring-core` (Credential Manager) | DPAPI `CryptProtectData` via `windows` crate, storing ciphertext in SQLite | For secrets larger than Credential Manager's blob limit (~2.5 KB, CRED_MAX_CREDENTIAL_BLOB_SIZE = 5×512 bytes), e.g. a full MCP OAuth token set. Keep both behind one `SecretStore` trait. |
| `tauri-plugin-stronghold` 2.4 | — | Don't. It's a password-protected vault, so the user would need a master password. Credential Manager is what the project requires and is unlocked by Windows login. |
| Base UI (shadcn default) | Radix (`shadcn init -b radix`) | If you need a component that is Radix-only or rely on Radix-specific community blocks |
| NSIS installer | MSI (WiX v3) | Only for enterprise/GPO deployment. MSI can only be built on Windows, and NSIS supports per-user install without admin, which suits consumer distribution. |
| GitHub Releases updater | CrabNebula Cloud (Tauri-partnered CDN/updater) | If you want download analytics and a managed update CDN. Note that analytics conflict with the "nothing leaves the PC" ethos. |

---

## What NOT to Use

| Avoid | Why | Use Instead |
|---|---|---|
| Calling LLM, Google or X APIs **from the webview** (JS SDKs with `dangerouslyAllowBrowser`, or `tauri-plugin-http` fetch) | Puts API keys and OAuth tokens into the renderer's memory and DevTools, breaking the privacy promise. It also needs CORS workarounds, and monitors stop running when the window is closed. | All network and secret handling in the Rust core. The UI gets results through commands, events and channels. |
| `tauri-plugin-sql` (frontend-issued SQL) | It puts raw SQL access in the UI layer. Background jobs in Rust also need the DB, and running two access paths creates locking and migration chaos. | `sqlx` owned by the Rust core, exposed through typed commands |
| `tauri-plugin-store` for anything sensitive | It writes plain JSON on disk | Credential Manager (secrets). Use SQLite for settings, or `tauri-plugin-store` only for trivial UI preferences. |
| `keytar` | Archived by Atom/GitHub. Native module churn. | Electron route: `safeStorage`, or `@napi-rs/keyring` 2.1. Tauri route: `keyring-core`. |
| The all-in-one `keyring` 4.x crate in the app | It pulls in many stores. Its maintainers tell apps to depend on `keyring-core` plus specific stores. | `keyring-core` + `windows-native-keyring-store` |
| `google-gmail1` / `google-calendar3` generated crates + `yup-oauth2` | Huge generated APIs, tied to yup-oauth2's flow and token storage (they want to write token files to disk) | reqwest + small serde models for the ~8 endpoints you need, plus `oauth2` with PKCE |
| Custom URI scheme / OOB redirects for Google OAuth | Google says "Custom URI schemes are no longer supported due to the risk of app impersonation". OOB is also discontinued. | Loopback `http://127.0.0.1:<random port>` with PKCE S256. This is Google's recommended desktop flow. |
| Treating the Google client secret as secret | Google says desktop clients "cannot keep secrets confidential". PKCE is the security. | Ship the client ID (and the secret as non-confidential). Rely on PKCE. Plan for Google verification and restricted-scope review (Gmail). |
| Scraping x.com or Nitter instances for X monitoring | Violates X ToS. Nitter-style frontends are dead or unstable. Breaks often. | Official X API v2 using the user's own key. X is pay-per-use by default; one source puts reads at about $0.005 per post returned (LOW confidence, third-party blogs). Put it behind an `XSource` trait so alternatives can be added later. |
| Bundling headless Chromium/Puppeteer for JS-rendered monitor pages | Adds 150+ MB, which defeats Tauri's size advantage | A hidden, incognito Tauri `WebviewWindow` (WebView2 is already on the machine) that loads the page and runs an extraction script. For heavier needs, `chromiumoxide` 0.9 driving the user's installed Edge via CDP. |
| Relying on `tokio::time::sleep` for "08:00 every day" | Monotonic timers drift or fire late across Windows sleep/hibernate. A PC that was off at 08:00 never runs the job. | Persist `next_run_at` (wall clock, tz-aware), tick every ~30 s, also run on `WM_POWERBROADCAST` resume and at app start, and catch up on missed runs |
| Spawning MCP stdio servers with default process flags on Windows | Each `npx`/`uvx` child flashes a console window, and `npx` is actually `npx.cmd` | `rmcp` `which-command` feature plus `CREATE_NO_WINDOW` (0x08000000) via `tokio::process::Command::creation_flags` on Windows |
| `window-vibrancy`'s `apply_acrylic` / `apply_blur` as the primary effect | Documented lag when dragging or resizing (Acrylic on Win10 1903+ / Win11 22000, Blur on Win11 22621). These use an undocumented API. | Mica (Win11) as the primary backdrop. On Win10, a tasteful solid or gradient fallback (Win10 reached end of support on 2025-10-14). |
| Loading fonts or icons from CDNs at runtime | Breaks offline use and leaks usage to third parties | Bundle `@fontsource-variable/*` and SVG icons |
| `typescript-eslint` stack on TS 7 (for now) | TS 7 doesn't ship the legacy compiler API (expected in 7.1), so tools built on it are in flux | Biome 2.5. If you need type-aware lint rules, add `@typescript/typescript6` side by side. |
| Azure Artifact Signing (Trusted Signing) **as an individual in Turkey** | Individual eligibility is US/Canada only. Organizations are allowed in a longer country list that doesn't include Turkey. | An OV code-signing certificate from a CA with cloud HSM signing (e.g. Certum, SSL.com eSigner), called from Tauri's `bundle.windows.signCommand`. Or SignPath Foundation if the project becomes open source. (MEDIUM: verify CA options at distribution time.) |

---

## Stack Patterns by Variant

**Workspace layout (do this from day one):**
- `crates/mornfold-core`: domain logic with **no Tauri dependency**. Contains connectors (Google, X, LLM, free-API), the MCP client manager, scheduler, monitor and diff engine, the DB layer, and the `SecretStore` and `Notifier` traits.
- `src-tauri`: thin shell. Commands, events, tray, window effects, plugins, and the Windows implementations of `Notifier` (WinRT) and `SecretStore` (Credential Manager).
- `ui/` (or `src/`): React app.
- Why: about 90% of the code then builds and tests on Linux with `cargo test`. Windows-only code is small, `#[cfg(windows)]`, and checked with `cargo xwin check` locally and by CI on `windows-2025`.

**Plugin model (built-in plus MCP plus declarative APIs):**
- Define one `Connector` trait in core with `id`, `capabilities`, `poll()`, `actions()` and `card_schema()`. Built-ins (Google, X, WebMonitor, LLM) implement it in Rust.
- MCP servers are wrapped by an `McpConnector` adapter that maps MCP tools and resources onto the same card/action model.
- "Free API" cards are **declarative JSON definitions**: URL, headers, a secret-ref, a JSONPath field map, a refresh cron and a display template. They're stored in SQLite and run by a generic `HttpCardConnector`.
- Why: this meets "add a service without touching core code" (via MCP config or a JSON definition) without a Wasm/JS plugin runtime. That's an anti-feature for v1: security and complexity.

**Streaming the AI summary:**
- Use `tauri::ipc::Channel<T>` to stream tokens from Rust to React, so the summary "types in" with a Motion fade.
- Cache the finished summary in SQLite, so opening the app at 09:00 shows it instantly.

**Glass and Mica rendering:**
- Set `"transparent": true` and `windowEffects: { effects: ["mica"] }` on the main window. Use `decorations: false` with a custom title bar (`data-tauri-drag-region`) for the Linear look. Keep the root `<body>` background transparent.
- Layer content panels with semi-opaque tints (`bg-white/[0.04]`, `backdrop-blur-xl`) inside the page.
- On Win10, detect via the OS build and switch to a solid or gradient theme.

**If the user is on Windows 10:**
- No Mica (Win11 only). Avoid Acrylic because of the documented lag. Use a CSS-only "glass" look over a static gradient.

**If JS-rendered pages must be monitored:**
- Open a hidden `WebviewWindow` with `incognito(true)`, a unique label, and **no capabilities** (remote origins get no IPC access by default). Wait for load, then evaluate the extraction script.
- Run the tasks through a queue, max 1-2 concurrent hidden webviews, to bound RAM.

**If distribution goes public:**
- The Google Gmail scopes (`gmail.readonly`, `gmail.send`/`modify`) are **restricted**. Beyond 100 test users this requires Google verification plus an annual third-party security assessment (CASA). Alternative: "bring your own Google Cloud OAuth client" in the setup wizard. Flag for the distribution phase.

---

## Version Compatibility

| Package A | Compatible With | Notes |
|---|---|---|
| `tauri` 2.12.x | All `tauri-plugin-*` 2.x and `@tauri-apps/api` 2.12 | Keep the Rust crate and npm `@tauri-apps/api` on the same minor version. The CLI warns on mismatch. |
| `tauri-plugin-notification` 2.7 | Rust ≥ 1.90 | Stated in the plugin docs |
| `rmcp` 3.5 | `reqwest` 0.13, `oauth2` 5.0 (no default features), tokio 1, edition 2024 / Rust ≥ 1.88 | Matches our reqwest major version. Supports MCP spec `2026-07-28` with negotiation down to `2025-11-25`. |
| `genai` 0.6.5 | `reqwest` 0.13 | A v0.7.0-rc exists. Stay on 0.6 until 0.7 is stable. |
| `oauth2` 5.0.0 | Built-in client uses `reqwest` 0.12 | Disable default features and adapt reqwest 0.13, or accept a duplicate reqwest in the tree |
| `tauri-plugin-http` 2.8 | `reqwest` 0.12 | Another reason not to use it. HTTP lives in core on 0.13. |
| `sqlx` 0.9 (`libsqlite3-sys` 0.37) | — | **Do not add `rusqlite` or `tauri-plugin-sql`** alongside it (they conflict on the `sqlite3` links key or drag in a second sqlx) |
| `specta`/`tauri-specta` 2.0.0-rc.25 | tauri 2.x | Pin exact versions (`=`). RC releases can break between versions. |
| TypeScript 7.0 | Vite 8, Vitest 5, Biome 2.5 (none use the TS API for transpile) | Tools that need the TS 6 API: add `@typescript/typescript6` |
| Tailwind 4.3 | Vite 8 via `@tailwindcss/vite` | No `tailwind.config.js` (CSS-first `@theme`) |
| shadcn CLI 4.22 | Tailwind 4, React 19, Base UI 1.9 (default) or Radix 1.8 | The library choice is stored in `components.json` `style` (e.g. `base-vega`) |
| WebView2 Runtime | Win10 (April 2018+) and Win11, preinstalled and evergreen | Use the `downloadBootstrapper` (default, +0 MB) or `embedBootstrapper` (+1.8 MB) install mode |
| `tauri-apps/tauri-action` | `action-v1.0.0` | v1 has breaking changes from v0. Follow the v1 README for inputs. |

---

## Building and Testing Windows Binaries from a Linux Dev Environment

| Layer | Where | How |
|---|---|---|
| Core logic (connectors, diffing, scheduler, DB, MCP manager) | Linux, every change | `cargo nextest run -p mornfold-core`, with `wiremock` for HTTP, `insta` snapshots, in-memory SQLite, and a mock `SecretStore`/`Notifier` |
| Windows-only Rust (`#[cfg(windows)]`) | Linux, pre-push | `cargo xwin check --target x86_64-pc-windows-msvc` (compile check only) |
| UI design and interaction | Linux | `pnpm dev` in a browser with `mockIPC` fixtures, plus Playwright screenshots in Chromium (very close to WebView2). Vitest for components. |
| Full app smoke run on Linux | Linux (optional) | `pnpm tauri dev` (WebKitGTK, needs `libwebkit2gtk-4.1-dev`). Mica, toasts and Credential Manager are no-ops here. |
| Windows build, tests and bundle | GitHub Actions `windows-2025` | `cargo test --workspace` (including Credential Manager round-trip tests), `pnpm tauri build` (NSIS), WebdriverIO + `tauri-driver` smoke e2e (app launches, tray exists, main view renders), upload the installer as an artifact |
| Release | GitHub Actions `windows-2025` on tag | `tauri-action` v1: builds, signs (via `signCommand`), creates a GitHub Release with the `.exe` + `.sig` + `latest.json` for the updater |
| Things CI **cannot** verify | User's real Win11 PC (manual UAT checklist) | Mica appearance, toast display and click activation (Server runners have no interactive desktop session), tray behaviour, autostart after reboot, SmartScreen prompt, updater end-to-end, sleep/resume catch-up |

Cache `~/.cargo` and `target/` (e.g. `Swatinem/rust-cache`) on the Windows runner. Cold Tauri release builds take 10+ minutes.

---

## Sources

**Registries (versions verified 2026-10-18, HIGH):**
- crates.io API: tauri 2.12.4, tauri-build 2.7.3, all tauri-plugin-* versions listed, rmcp 3.5.1, keyring 4.2.0 / keyring-core 1.0.1 / windows-native-keyring-store 1.1.0, sqlx 0.9.0, rusqlite 0.40.2, reqwest 0.13.5, tokio 1.53.3, genai 0.6.5, rig-core 0.44.0, oauth2 5.0.0, scraper 0.27.0, similar 3.2.0, htmd 0.5.6, dom_smoothie 0.18.2, croner 4.0.1, jiff 0.2.38, jsonpath-rust 1.0.12, mail-builder 1.0.0, mail-parser 0.11.9, tauri-winrt-notification 0.8.1, window-vibrancy 0.8.1, specta/tauri-specta 2.0.0-rc.25, ts-rs 12.0.1, cargo-xwin 0.23.1, tauri-driver 2.1.0, windows 0.62.2. Dependency checks: oauth2 → reqwest ^0.12, genai → reqwest ^0.13, tauri-plugin-http → reqwest ^0.12.16, sqlx → libsqlite3-sys ^0.37.
- npm registry: @tauri-apps/cli 2.12.3, @tauri-apps/api 2.12.4, react 19.3.1, vite 8.3.4, tailwindcss 4.3.3, motion 14.3.0, typescript 7.0.2, @base-ui/react 1.9.0, radix-ui 1.8.1, shadcn 4.22.0, @tanstack/react-query 5.104.4, zustand 5.0.15, electron 44.9.0, electron-builder 26.15.3, electron-updater 6.8.9, @modelcontextprotocol/sdk 1.32.1, @anthropic-ai/sdk 0.135.0, @google/genai 2.30.0, ai 7.0.143, vitest 5.0.5, @playwright/test 1.64.1, @biomejs/biome 2.5.15, @wdio/tauri-service 1.5.0, webdriverio 10.0.2, pnpm 12.14.0.
- NuGet: Microsoft.WindowsAppSDK 2.5.1, ModelContextProtocol 2.2.0, Anthropic 12.58.0, Google.GenAI 1.26.1, H.NotifyIcon.WinUI 2.4.1.
- static.rust-lang.org: Rust stable 1.99.1 (2026-10-13). nodejs.org: Node 24.22.0 LTS.

**Official docs and sources (HIGH):**
- https://modelcontextprotocol.io/docs/sdk: SDK tiers (TypeScript, Python, C#, Go, Rust, Ruby are Tier 1)
- https://github.com/modelcontextprotocol/rust-sdk (README + `crates/rmcp/Cargo.toml`): spec `2026-07-28`, client lifecycle modes, transport and auth feature flags
- https://github.com/tauri-apps/tauri `crates/tauri-utils/src/config.rs`: WindowEffect enum (Mica, MicaDark/Light, Acrylic, Blur, Tabbed)
- https://github.com/tauri-apps/window-vibrancy `src/lib.rs`: Mica is Win11-only. Acrylic/Blur performance warnings.
- https://v2.tauri.app/plugin/notification/: Windows "only works for installed apps", PowerShell identity in dev, Actions API mobile-only
- https://v2.tauri.app/distribute/windows-installer/: NSIS cross-compile from Linux "last resort", MSI Windows-only, WebView2 install modes and sizes
- https://v2.tauri.app/develop/tests/webdriver/: tauri-driver supports Windows and Linux. The WebdriverIO service syncs msedgedriver.
- https://github.com/tauri-apps/winrt-notification: `on_activated`, `add_button`, `on_dismissed`, AUMID guidance
- https://github.com/open-source-cooperative/keyring-rs: v4 split into keyring-core plus per-platform stores
- https://github.com/jeremychone/rust-genai: native `anthropic`, `gemini`, `open_router` adapters
- https://github.com/FabianLars/tauri-plugin-oauth: localhost redirect server for Google-style OAuth
- https://developers.google.com/identity/protocols/oauth2/native-app: loopback recommended, PKCE required, custom schemes and OOB unsupported, desktop secret not confidential
- https://github.com/electron/electron `docs/api/structures/base-window-options.md`: `backgroundMaterial` values
- https://github.com/actions/runner-images: `windows-latest` = Windows Server 2025, `windows-2025` label
- https://github.com/tauri-apps/tauri-action/releases: `action-v1.0.0` latest
- https://playwright.dev/docs/webview2: `connectOverCDP` via `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`
- https://ui.shadcn.com/docs/changelog/2026-07-base-ui-default: Base UI default, Radix still supported

**Secondary (MEDIUM/LOW):**
- https://learn.microsoft.com/azure/artifact-signing/quickstart and https://azure.microsoft.com/en-us/products/artifact-signing: individual signing US/Canada only, $9.99/month Basic (MEDIUM)
- https://devblogs.microsoft.com/typescript/ and https://infoq.com/news/2026/08/typescript-7-released/: TS 7 native, `@typescript/typescript6` side-by-side, API in 7.1 (MEDIUM)
- https://techcrunch.com/2025/10/21/x-is-testing-a-pay-per-use-pricing-model-for-its-api/, https://opentweet.io/x-api-pricing, https://zernio.com/blog/twitter-api-pricing: X pay-per-use default, about $0.005/post read, free tier closed Feb 2026 (LOW: third-party, verify in the X Developer Console)
- Electron vs Tauri RAM and installer sizes: community-typical figures, not benchmarked here (MEDIUM)

---
*Stack research for: Windows desktop personal command center (Mornfold)*
*Researched: 2026-10-18*
