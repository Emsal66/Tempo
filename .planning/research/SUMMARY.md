# Project Research Summary

**Project:** Mornfold
**Domain:** Local-first Windows 10/11 desktop "personal command center". It runs from the tray, combines many services on one dashboard (Gmail/Calendar, website monitors, X monitors, free-API cards), uses MCP as its plugin model, writes an AI morning briefing with the user's own (BYO) key, and sends background notifications.
**Researched:** 2026-10-18
**Confidence:** MEDIUM-HIGH. Stack versions and platform rules were checked against registries and official docs. Competitor UX, the X billing edge cases and the Electron-to-Tauri translation of the architecture are MEDIUM or below.

## Executive Summary

Mornfold is a tray-resident integration hub with a briefing layer on top. Products in this space (Claude Desktop as an MCP host, Raycast, Electron mail and calendar clients, Distill and changedetection.io) all use the same three-part shape. A thin OS shell owns the windows, tray and toasts. A headless core owns the data, sync, integrations and secrets. A disposable UI renders a fixed set of card types. Every service, whether built in, declarative JSON or an MCP server, sits behind one connector contract. Connectors return normalized "items". A single store dedupes and diffs them and emits events. Notifications, UI refresh and the AI briefing all consume those events and never talk to connectors directly. This matches the Core Value exactly: even if everything else breaks, the elegant shell and the service-connection infrastructure must keep working.

**We recommend Tauri 2 with a Rust core** (STACK.md), not the Electron layout ARCHITECTURE.md assumed. Windows-only targeting removes Tauri's main weakness, because WebView2 is evergreen Chromium on every Windows 10/11 machine. In return the installer drops from about 100 MB to 5-15 MB and idle RAM is far lower for an app that runs all day. Every capability has a first-party crate or plugin, including an official Tier-1 Rust MCP SDK (`rmcp`, spec 2026-07-28). The architecture's boundaries carry over unchanged:
- A `mornfold-core` Rust crate with **no Tauri dependency** owns the DB, scheduler, connectors, MCP, the action pipeline and secrets. About 90% of the code builds and tests on Linux.
- A thin `src-tauri` shell provides the tray, window effects, WinRT toasts and the Credential Manager adapter.
- The React 19 / Tailwind 4 / Motion UI is sandboxed and receives only results, never keys.

Electron 44 stays a documented fallback, chosen only if the owner refuses Rust.

The main risks are external and policy-driven, not technical:
- **Google restricted scopes.** Gmail scopes are restricted. Testing mode kills refresh tokens after 7 days. Distributing to others means verification and possibly CASA, and sending mail content to an LLM provider complicates the "local-only" argument.
- **X API cost.** X is now pay-per-use at $0.005 per post read, and one broad keyword monitor can burn tens of dollars a day.
- **MCP and prompt injection.** A stdio MCP server is arbitrary code running as the user, and every input to the briefing (mail, tweets, web pages, tool output) can be written by an attacker.
- **Linux-dev / Windows-reality gap.** Toasts, the tray, Credential Manager, autostart and Mica only work on a real Windows install.
- **Scope.** "All tracking features in v1" can easily turn into 60% done everywhere.

The mitigations are architectural and cheap if they are in place from Phase 1:
- Windows CI plus an owner smoke-test checklist on the real PC.
- A persistent scheduler with catch-up after sleep.
- A notification pipeline with silent first-sync baselines and throttling.
- A SecretStore that the UI can only write to.
- A no-tools, plain-text AI call with a deterministic template fallback.
- One policy-gated action pipeline with exact previews.
- Consent and hash-pinning for MCP servers.
- Spend caps shared by AI and X.
- Thin vertical slices the owner uses every morning.

## Key Findings

### Recommended Stack

Use Tauri 2.12 (Rust 1.99 / tokio 1.53, WebView2) with a React 19 + TypeScript 7 + Vite 8 + Tailwind 4 + Motion 14 + shadcn/Base UI frontend. All network, secret and integration logic lives in Rust. The webview never calls Google, X or an LLM. Set up a Cargo workspace on day one: `crates/mornfold-core` (pure, testable on Linux), `src-tauri` (thin shell plus `#[cfg(windows)]` adapters) and `ui/`. Build releases on GitHub Actions `windows-2025` with `tauri-action` v1. During development, check Windows-only code on Linux with `cargo xwin check`. See STACK.md for the full version table.

**Core technologies:**
- **Tauri 2.12 + official plugins** (single-instance, autostart, updater, window-state, log, opener, oauth, positioner): shell, tray, IPC and bundling. Mica comes from `windowEffects`. It is small, light and has a capability ACL.
- **`rmcp` 3.5 (official Rust MCP SDK, Tier 1)**: MCP client over stdio child processes and streamable HTTP, with OAuth. It speaks 2026-07-28 and negotiates down to 2025-11-25.
- **`sqlx` 0.9 on SQLite (WAL)**: the single-writer local DB with embedded migrations and offline-checked queries. Never mix it with `rusqlite` or `tauri-plugin-sql`.
- **`keyring-core` + `windows-native-keyring-store`**: Credential Manager, protected by DPAPI and unlocked by the Windows login. No master password.
- **`genai` 0.6 behind our own `LlmProvider` trait**: Anthropic, Gemini and OpenRouter. It is pre-1.0 with a single maintainer, so the trait keeps a roughly 300-line hand-rolled reqwest fallback possible.
- **`oauth2` 5.0 (PKCE) + loopback listener**: Google desktop OAuth. Disable its default features so it uses reqwest 0.13.
- **Scheduler and monitor crates**: `croner` + `jiff` (timezone-safe) + SQLite jobs + one tokio loop for scheduling. `governor` and `backon` for rate limits and retries. `scraper`, `htmd`, `dom_smoothie`, `similar` and `blake3` for website diffing. `jsonpath-rust` for API cards.
- **`tauri-winrt-notification`**: clickable toasts with buttons. The plugin's actions are mobile-only.
- **`tauri-specta` (pinned RC)**: generates typed TypeScript wrappers for commands and events, keeping the UI and core in sync.
- **Frontend**: TanStack Query for core data (invalidated by core events), zustand for UI-only state, cmdk palette, i18next (`tr-TR`), a locally bundled Inter variable font, and Biome.

**Version and compatibility notes:** use one reqwest major version (0.13) across `rmcp`, `genai` and our own code. Pin `specta`/`tauri-specta` with `=`. TypeScript 7 has no legacy compiler API until 7.1, so use Biome rather than typescript-eslint. Keep `tauri` and `@tauri-apps/api` on the same minor version.

### Expected Features

PROJECT.md requires every tracking feature in v1, so "table stakes" here means the minimum depth at which each feature stops feeling broken.

**Must have (table stakes):**
- A card contract (loading, empty, error and stale states, last-updated time, refresh) plus a section grid (reorder, S/M/L sizes). Every connector renders through it, so build it first.
- A deterministic "Günün özeti" (today's summary) home that renders without AI. The AI narrative sits on top of it.
- A Ctrl+K command palette with an action registry that connectors add to. Keyboard shortcuts. Turkish UI with i18n from day one. Light/dark themes with Mica and a "reduce transparency" fallback.
- A Connections page: health badge (Connected / Re-auth / Error / Rate-limited / Paused), account identity and scopes, last and next sync, test connection, and disconnect that also purges the connector's data and keys. A global "pause all".
- Gmail: important/unread card, sanitized thread view with remote images blocked, reply with correct threading and UTF-8, archive/read/star, undo-send, and a toast for important mail. Sync by polling `history.list`, not Pub/Sub.
- Calendar: merged agenda, next meeting with a Join button, create event, reminder toast. Sync with `syncToken`.
- AI briefing: three BYO-key providers, model picker, scheduled generation with catch-up, Turkish narrative, streaming, estimated cost per run (exact for OpenRouter via `usage.cost`), history, a report of what data was sent, and a deterministic fallback.
- MCP: add stdio and remote servers, paste a `mcpServers` JSON block, per-tool enable and consent, process supervision with logs.
- Website monitor: URL plus CSS selector, interval with a floor, static fetch (with JS render if feasible), word diff, history, ignore rules, conditions (text appears, number below a value or down by a %), and distinct error states (unreachable, selector missing, blocked).
- X: account watchlist plus keyword/query monitors, filters, a feed card, and mandatory spend estimates and caps.
- API cards: a custom-definition wizard (URL, auth, test, click-to-pick a JSON field, a fixed set of visualizations) plus a catalog of about 10 entries **shipped as the same definitions**.
- Background: close-to-tray, start at login, toasts that deep-link, a notification center, per-source settings, quiet hours, batching, and sleep/resume catch-up.

**Should have (differentiators):**
- A **grounded briefing**: every bullet cites item IDs, so it is clickable and claims can be checked. It is cheap if designed in from the start. P1.
- **MCP-backed cards** (MCP tool or resource, mapped by the shared extractor, shown as a card). No mainstream MCP client turns MCP into dashboard data. P1, minimal version.
- Suggested-action buttons in the briefing (they run only on click). Layered prompt customization (presets plus a "things I care about" list).
- v1.x: visual element picker, AI "only notify me if…" filter and change summaries, API threshold alerts and sparklines, Turkish natural-language quick-add, AI reply drafts, digest mode and evening briefing, local LLM through an OpenAI-compatible URL, global summon hotkey, `.mcpb` install and a connector gallery, multiple Google accounts in the UI (the data model supports N accounts from v1).

**Defer (v2+):** a chat agent with tool use, a full calendar grid with drag-to-reschedule, login-required monitoring and browser steps, screenshot diffs, outbound alert channels, API chaining, MCP Apps/Tasks, multiple dashboard pages.

**Explicit anti-features:**
- Native or JS plugin loading. Plugin-provided HTML/CSS. User-written HTML templates.
- A free-form pixel canvas. YAML-first configuration.
- Embedded web apps (Rambox style).
- Autonomous tool use. Auto-running MCP tools.
- Gmail Pub/Sub push.
- Scraping X or using Nitter.
- Captcha bypassing. Intervals in seconds.
- A Windows service that runs while the user is logged out.

### Architecture Approach

This section translates ARCHITECTURE.md onto Tauri 2. The design keeps a headless core, a thin shell and a disposable UI. In Electron those were separate processes. In Tauri the **core is a Rust crate running in the app process on tokio**. The UI is a WebView2 page restricted by the Tauri capability ACL. Third-party code (MCP stdio servers) and rendering of untrusted pages stay outside the trusted boundary. The core is the only owner and writer of SQLite. The UI holds only cached queries (TanStack Query, invalidated by pushed events), so it is safe to destroy the webview when the window has been hidden for a long time and keep only the core running.

| ARCHITECTURE.md (Electron) | Mornfold on Tauri 2 |
|---|---|
| Main process (tray, windows, toasts, power, login item, updater) | `src-tauri` shell: tray-icon, window effects, `tauri-plugin-autostart`/`updater`/`single-instance`, `WM_POWERBROADCAST` via the `windows` crate, `Notifier` implemented with `tauri-winrt-notification` |
| Core `utilityProcess` + supervisor restart | `mornfold-core` crate in-process. Every job, connector call and MCP session runs in its own tokio task with a timeout. Keep `panic = "unwind"` and treat a panicked task as a failed job. **No separate crash-isolation process**, so the task-level discipline matters more. |
| MessagePort RPC + zod contracts | Tauri commands, events and `ipc::Channel<T>` (for streaming tokens), with types generated by `tauri-specta`. Validate inputs in the Rust command handlers. |
| better-sqlite3 | `sqlx` SQLite (WAL), single writer in the core |
| `@napi-rs/keyring` + `safeStorage` fallback | `SecretStore` trait. Recommendation: envelope encryption, with a 256-bit master key in Credential Manager via `keyring-core` and AES-256-GCM ciphertext in SQLite. This avoids Credential Manager's 2560-byte blob limit for OAuth token sets. The AEAD crate is not version-checked yet. A DPAPI `CryptProtectData` blob is the alternative implementation behind the same trait. |
| `@modelcontextprotocol/client` v2 | `rmcp` (child-process + streamable HTTP + auth). On Windows, use `which-command` (resolves `npx.cmd`), `CREATE_NO_WINDOW`, and a Job Object with kill-on-close |
| AI SDK `generateObject` | `genai` behind `LlmProvider`, with structured JSON output validated in Rust (serde) |
| RenderFetch (hidden BrowserWindow in Main) | Hidden incognito `WebviewWindow` with **no capabilities**, created by the shell when the core asks through a `PageRenderer` trait. Queue of 1-2 at a time. |
| Sandboxed iframe + DOMPurify for email HTML | The same idea in the WebView2 UI: sanitize (in Rust or with DOMPurify), render in a `sandbox` iframe without scripts or same-origin, use a strict CSP, block remote images by default |

**Major components (all in `mornfold-core` unless noted):**
1. **Item Store + Domain Event Bus.** A unified `items` table keyed by (connection, feed, externalId) with a content hash. Upsert, dedupe and diff happen here, then `ItemCreated`/`Changed`/`Removed` events go out on a tokio broadcast channel. The first sync and any full resync are a **baseline** with no notifications.
2. **Persistent Scheduler.** A `jobs` table with wall-clock `next_run_at` and one dispatcher loop. Adds jitter, exponential backoff, `Retry-After` handling and per-host budgets (`governor`). On resume it runs each overdue job once, staggered. Auth failures park the connection in `auth_required`. The digest runs at a cron time or on "first unlock after T".
3. **Connector Host with three trust tiers behind one `Connector` trait.**
   - **Built-in** (Gmail, Calendar, Web Watch, X): first-party Rust.
   - **Declarative** (catalog plus user API definitions): JSON interpreted by `HttpCardConnector`. No code runs.
   - **MCP** (`McpConnector` adapter): untrusted, and every write is confirmed.
   - Connectors only get a scoped `ConnectorContext`: rate-limited HTTP, secrets for their own namespace, a token provider, a redacting logger and cancellation.
4. **Shared Extraction + Condition Engine.** JSONPath with no script evaluation, plus field mapping, turns arbitrary JSON into items or card fields. It is used by API cards, the catalog and MCP cards. A small comparison grammar (never `eval`) handles website triggers, API thresholds and X filters, and fires on threshold *crossings*.
5. **Action Pipeline (Policy Gate → Confirm → Executor).**
   - Every action carries an effect class: `read` / `local` / `external_write` / `outgoing` / `destructive`.
   - For outgoing or destructive effects the core issues an HMAC confirmation token bound to the exact input shown in the preview.
   - Sends get an 8-second undo window. Execution uses idempotency keys and writes an audit log.
   - MCP tool annotations are untrusted. Any tool not marked read-only defaults to `outgoing`.
   - **No action ever runs from a scheduled job, a notification click or AI output in v1.**
6. **Notification Engine.** The core decides: rules, dedupe, per-source batching ("5 yeni e-posta"), quiet hours, priority, deep-link payloads. The shell displays through `Notifier`.
7. **Digest / AI Pipeline.**
   - Steps: collect, rank by deterministic signals, compact (strip quoted text and signatures, per-item cap, token cap, item IDs as references), wrap in untrusted-content delimiters, call the LLM with **no tools**, validate the structured output (`{headline, sections[{bullets[{text, itemRefs}]}]}`), store, render as plain text.
   - On no key, an error or a timeout, it falls back to a template digest built from the same ranked list.
8. **OAuth Service.** Loopback `127.0.0.1:<random>` plus PKCE S256 in the system browser for Google and for MCP HTTP servers. Validates `state` and `iss`. Tokens are written to the SecretStore.
9. **UI (React, sandboxed).** A fixed card library (list, agenda, value + sparkline, timeline, diff, digest), a shared slide-over detail panel, a confirmation sheet, onboarding, and settings forms generated from connector schemas. Sources map data into cards and never supply UI.

### Critical Pitfalls

1. **Linux-dev / Windows-reality gap.** Toasts silently fail without an installed Start Menu shortcut carrying an AppUserModelID. Acrylic lags on resize, and Mica is Windows 11 only. Tauri signing of cross-compiled builds is unsupported. Avoid it this way: in Phase 1, CI on `windows-2025` builds the real NSIS installer on every push, and the owner runs a smoke checklist on their PC (tray, toast from the *installed* build, autostart after reboot, Credential Manager round-trip, sleep/resume). Mica is primary on Windows 11, with a solid or gradient fallback on Windows 10. Never use Acrylic on the resizable main window.
2. **Google OAuth wall at distribution time.** Testing mode means 7-day refresh tokens. Unverified apps are capped at 100 users. Restricted-scope verification plus annual CASA, and the transfer of mail to an LLM, plausibly counts as "through third-party servers". Avoid it this way:
   - Owner v1 runs on a personal GCP project published "In production", unverified.
   - Design for a **bring-your-own OAuth client** from day one, with no hard-coded client ID.
   - Use loopback plus PKCE, minimal scopes, and an explicit, prominent consent step before mail goes to an AI provider.
   - Handle `invalid_grant` with a one-click re-auth card.
   - Make verification a go/no-go decision in the distribution phase.
3. **Prompt injection into the briefing and actions.** Precedents include Gemini-in-Gmail hidden text and EchoLeak. Avoid it this way: the summary call gets no tools and no MCP. Strip hidden HTML (display:none, zero font size, text colored like the background), mark content as untrusted with delimiters, render output as plain text with no images or auto-links, cite a source for every bullet, require a human click for every side effect, and keep a red-team fixture suite (hidden-text email, instruction tweet, malicious page).
4. **MCP as a code-execution and exfiltration hole, plus Windows process hygiene.** Avoid it this way:
   - On add, show the full command untruncated and highlight risky patterns.
   - Hash-pin each server's tool list. On change, disable the server and show a diff. New tools arrive disabled.
   - Set per-tool permission levels. Allow only http(s) URLs and open them through the OS API, never a shell.
   - Do not implement Sampling or Roots.
   - Spawn children into a Job Object with kill-on-close and `CREATE_NO_WINDOW`, capture stderr per server, supervise with backoff.
   - Detect missing Node.js or Python for `npx`/`uvx` servers, and prefer remote servers for non-developers.
5. **X costs and legality.** Avoid it this way: official API only, under the user's own developer app. Use `since_id`, cache handle→ID lookups ($0.01 each), and request minimal fields. Show a cost estimate before a monitor is saved, cap results per poll, and enforce an in-app daily/monthly cap on top of the X console limit. Default keyword monitors to a 15-30 minute interval and toasts to digests. Put X behind an `XSource` trait and build it **last** among the monitors.
6. **The scheduler stops silently, and notifications spam.** Avoid it this way: the scheduler runs only in the Rust core, never in webview JS. Persist cursors, hashes and `last_run`. Catch up on resume once and staggered. Use single-instance plus autostart that matches the distribution channel. Show a tray health line ("son kontrol: 3 dk önce"). Baseline silently, normalize website text, and enforce a toast budget (warn at more than ~10 a day while dogfooding).
7. **Signing and channel found out at the end, plus "everything in v1" that never ships.** Decide the distribution channel early (see Gaps). Generate the updater minisign key in Phase 1 and keep two offline backups plus a CI secret. Ship thin vertical slices the owner uses daily from the Google phase onward, with no more than two connectors in progress at once. Agree a cut line up front: X keyword search and the public catalog are deferred first. The framework, Google and the briefing are never deferred.

## Implications for Roadmap

### How the four orderings were reconciled

| Topic | ARCHITECTURE.md | PITFALLS.md | FEATURES.md | Resolution |
|---|---|---|---|---|
| Foundation | Shell + Windows CI first, then the data core | P1 combines CI, shell, tray, notifications, secrets and scheduler | Vault, storage, scheduler and connector framework first | **Agreed.** Phase 1 combines all of them, including the scheduler and notification primitives. |
| Design system | Phase 2, in parallel with the data core, using fixtures | Warns against polish before real data | Card contract before any connector UI | Phase 2 covers the **card contract and design system on fixtures, time-boxed**. Real data arrives in Phase 3. |
| HTTP-API engine and catalog | Early (phase 3), to prove the pipeline without OAuth | Late (P5) | The catalog equals shipped definitions, and the mapper is shared with MCP | **Early (Phase 3).** It is the cheapest end-to-end proof of scheduler → connector → store → event → card → toast, it delivers "add a service without code" immediately, and the extraction and condition engines it creates are reused by MCP, website and X. |
| AI briefing | Late (phase 8), after at least two real sources | Early (P3), right after Google, for the wow moment and dogfooding | Deterministic home first, AI as an overlay | **Right after Google read (Phase 5).** Calendar and Gmail plus API cards are enough real sources, the deterministic home already exists, and building the read-only AI before any write path fixes the "no tools, no actions" rule first. The prompt is re-tuned as later sources plug in. |
| Action pipeline | Its own phase before any write | Consent model defined with the connector framework | — | **Phase 6**, before Gmail reply, Calendar create and MCP actions. The effect classes are *defined* in Phase 3 so every connector declares them. |
| Website vs MCP vs X | Web and X together (7), MCP in parallel (6) | Website P4 → MCP P6 → X P7 (X last) | — | **Website → MCP → X.** Website is self-contained and high value. MCP needs OAuth, the extractor and the action pipeline. X goes last because of cost and policy volatility, and it inherits the spend tracker and condition engine. |
| Distribution | Final phase | Final phase, with the channel decided early | Onboarding is P1 in priority but only matters for others | **Final phase**, but the **channel decision and updater key belong to Phase 1**. The owner configures through Settings until then. |

### Phase 1: Foundation and Windows Pipeline
**Rationale:** Every later feature depends on it, and it covers the riskiest platform unknowns (toasts, tray, Credential Manager, Mica, autostart), which can't be verified on Linux. This is the Core Value's "shell plus infrastructure".
**Delivers:**
- Workspace layout: `mornfold-core`, `src-tauri`, `ui`.
- GitHub Actions `windows-2025` building the NSIS installer on every push, plus the owner smoke-test checklist.
- Tray with Open / Pause all / Quit. Close-to-tray, single-instance, autostart `--minimized`.
- A WinRT toast from the installed build with a correct AUMID and click activation.
- SQLite + migrations + WAL.
- `SecretStore` (envelope encryption with the master key in Credential Manager) with log redaction.
- A persistent scheduler skeleton with `WM_POWERBROADCAST` resume catch-up.
- i18n scaffolding (`tr`). A Mica window spike with a Windows 10 fallback.
- Updater key generation and backup.
- A **written distribution-channel decision**.
**Addresses:** the background, vault, storage and scheduler table stakes.
**Avoids:** Pitfalls 6, 7 (key and channel), 8 and 12 (secrets).

### Phase 2: Design System and Card Library (on fixtures)
**Rationale:** The aesthetic is the product's differentiator, and the card contract has to exist before any connector UI. Use fixtures with `mockIPC` so the agent can iterate visually on Linux with Playwright screenshots. Time-box it so polish doesn't outrun real data.
**Delivers:** design tokens, glass surfaces and Motion transitions; the card contract (loading, empty, error, stale, last-updated, refresh); the fixed card types (list, agenda, value + sparkline, timeline, diff, digest); the section grid with S/M/L and reorder; the shared slide-over detail panel; the cmdk palette shell; the custom title bar; light/dark and "reduce transparency"; the Turkish empty states.
**Uses:** React 19, Tailwind 4, Motion, shadcn/Base UI, TanStack Query, tauri-specta-typed mocks.
**Avoids:** retrofitting card states later, and a free-form canvas.

### Phase 3: Connector Framework, Notification Pipeline and Declarative API Cards
**Rationale:** Proves the whole pipeline (schedule → sync → upsert/diff → event → card → toast) with no OAuth involved. It builds the shared extraction and condition engines, the Connections page and the health model that every later connector reuses. It also puts real data on the owner's screen early.
**Delivers:**
- The `Connector` trait plus the manifest, with effect classes declared per action.
- `ConnectorContext`, the Item Store and baseline mode, and the domain event bus.
- The Notification Engine: rules, batching, quiet hours, a notification center, and a tray unread badge.
- The Connections page.
- `HttpCardConnector` with the custom-API wizard (test request, JSON tree with click-to-pick, auth stored as secret placeholders) and a starter catalog of about 10 Turkish-relevant entries, shipped as definitions.
- The global "pause all".
**Implements:** Architecture patterns 1, 2, 3, 6 and 7.
**Avoids:** Pitfall 9 (baseline and throttling exist before any monitor) and webview-side fetching.

### Phase 4: Google Connector (read) and the Deterministic "Günün özeti" Home
**Rationale:** The first high-value built-in, and it starts the Google verification clock. The deterministic home becomes the briefing's fallback and its data source.
**Delivers:**
- The OAuth service (loopback plus PKCE, a BYO client-ID override, a personal GCP project "In production").
- Gmail via `history.list` with a 404 → bounded full-sync fallback, `metadata` for list views, and a sanitized thread view in a sandboxed iframe.
- Calendar via `syncToken` with `singleEvents`, all-day and timezone handling.
- Next meeting with Join, an important-mail toast, a meeting-reminder toast, and an `invalid_grant` re-auth card.
- The deterministic Today home combining Calendar, Gmail and API cards.
- An N-account data model.
**Avoids:** Pitfall 2 (Testing mode, hard-coded client ID, over-broad scopes).

### Phase 5: AI Briefing
**Rationale:** The "wow" moment and the reason to open the app each morning. It needs real data (Phase 4) and the deterministic fallback, and it must ship before any write capability so the no-tools rule is the default.
**Delivers:**
- The `LlmProvider` trait over `genai` (Claude, Gemini, OpenRouter) with key test and model picker.
- The pipeline: collect, rank, compact, delimit, structured output, validation, and **grounded item-reference bullets**.
- Streaming through `Channel<T>`, schedule plus first-unlock catch-up, regenerate, history.
- The **shared spend tracker** (OpenRouter `usage.cost`, plus an editable local price table for Anthropic and Gemini) and monthly caps.
- Consent before mail content is sent, the data-sent report, a free-tier Gemini training notice, an optional redaction mode (email, phone, IBAN, TCKN), prompt presets plus a "things I care about" list.
- The red-team injection fixtures.
**Avoids:** Pitfalls 3 and 12.

### Phase 6: Action Pipeline, Gmail Reply and Calendar Create
**Rationale:** The safety infrastructure is a hard prerequisite for any write, whether built-in or MCP. Built-ins come first because their effects are known.
**Delivers:**
- The Policy Gate (validation, effect resolution, preview) and HMAC confirmation tokens.
- The confirmation sheet showing exact recipients, body and time zone.
- An 8-second undo window, idempotency keys, the audit log and a targeted re-sync.
- Gmail reply and reply-all (threadId, In-Reply-To, References, RFC 2047 UTF-8 subjects via `mail-builder`) plus archive, read and star.
- Calendar create (solo events are `external_write`, events with invitees are `outgoing`).
- Palette actions. Suggested-action buttons in the briefing (click only).
- Optional compose-new.
**Avoids:** auto-actions from notifications or AI, and unthreaded or garbled Turkish replies.

### Phase 7: Website Change Detection
**Rationale:** Self-contained and high value, and it reuses the condition engine, scheduler, notifications and the diff card.
**Delivers:**
- A two-tier fetcher: HTTP with ETag and an honest User-Agent by default, plus a "render" option using a hidden, capability-less, incognito `WebviewWindow`, with a queue of at most 1-2.
- A CSS selector or readability extraction, normalization (`htmd`, number and date ignore rules), a `blake3` change gate, a `similar` word diff with history and a retention cap.
- Conditions. Bot-wall and challenge detection marks the monitor "blocked" instead of reporting a false diff.
- Per-domain minimum intervals, a 5-minute floor, robots.txt respected.
- A "test now: what changed?" preview before saving.
**Avoids:** Pitfalls 9 and 10, and IPC-enabled webviews loading remote pages.

### Phase 8: MCP Client and MCP-Backed Cards
**Rationale:** The extensibility promise. It depends on the connector contract and extractor (Phase 3), OAuth (Phase 4) and the action pipeline (Phase 6). Placing it after two real connector shapes keeps the trait from being generalized too early.
**Delivers:**
- `rmcp` stdio and streamable HTTP connections with OAuth (tokens stored in the SecretStore by issuer), protocol negotiation tested against 2025-era and 2026-07-28 servers.
- Paste or import of `mcpServers` JSON.
- The consent dialog with the full command, tool-list hash-pinning and a re-approval diff, per-tool enable and effect overrides.
- Job Object, `CREATE_NO_WINDOW`, stderr logs, backoff restart, idle shutdown, and Node/Python prerequisite detection.
- The MCP-backed card wizard (read-only tool or resource, fixed arguments, interval, extractor mapping).
- MCP actions through the policy gate.
- `mcp:<connId>/<tool>` namespacing.
**Avoids:** Pitfalls 4 and 5. Sampling, Roots, Logging and MCP Apps are explicitly left out.

### Phase 9: X/Twitter Monitoring
**Rationale:** The most volatile external dependency (pricing changed twice in 2025-2026), with the highest cost hazard. Built last, it inherits the spend tracker, condition engine, digest batching and connector health model.
**Delivers:** an `XSource` trait with an official API v2 implementation (BYO bearer token); an account watchlist (cached handle→ID); keyword/query monitors with operator guidance; `since_id`, minimal fields and a per-poll result cap; filters pushed into the query; a feed card; a cost estimate before saving, per-monitor and global caps and spend to date; toasts for watched accounts and digest-only for keywords; a graceful "X not configured" state.
**Avoids:** Pitfall 1. No scraper, Nitter or reseller backend in the core.

### Phase 10: Onboarding and Distribution Hardening
**Rationale:** It gates *other* users, not the owner. Signing, the channel and Google verification are policy work with long lead times, already decided or prepared earlier.
**Delivers:**
- The first-run wizard: language, AI key test, Google with a **BYO GCP project walkthrough** and screenshots of the unverified-app screen, starter cards, notification and autostart preferences. Every step can be skipped and resumed from Settings.
- Signing through the chosen channel.
- An end-to-end updater test (v0.1 → v0.2 with the key restored from backup).
- Clean-VM installs on Windows 11 with Smart App Control and on Windows 10.
- The Google verification/CASA go/no-go, written down. A privacy policy.
**Avoids:** Pitfalls 2 and 7.

### Phase Ordering Rationale

- **Core Value first.** Phases 1-3 deliver exactly "zarif kabuk + servis bağlama altyapısı", and it already works with zero external accounts thanks to the declarative API cards.
- **Shared engines are built once, by their first consumer.**
  - The extractor and condition engine come from API cards (Phase 3). MCP cards, website triggers and X filters reuse them.
  - The spend tracker comes from AI (Phase 5). X reuses it.
  - The action pipeline (Phase 6) serves Gmail, Calendar, briefing buttons and MCP.
  - The notification pipeline (Phase 3) exists before any monitor.
- **Dogfooding from Phase 4 onward.** Each phase ends with an installed build the owner uses in the morning. The briefing arrives in Phase 5, not at the end.
- **Risk-weighted tail.** The externally volatile or policy-gated items (X, Google verification, signing) come last and sit behind traits, so a policy change costs one connector, not the architecture.
- **Possible parallelism:** Phase 2 can overlap Phase 1. Read-only MCP feeds (Phase 8) could start after Phase 4 if needed. Keep the rule of no more than two connectors in progress at once.

### Research Flags

Phases likely to need `/gsd:plan-phase --research-phase <N>`:
- **Phase 1:** the cold-start toast activation path for Tauri on Windows (`tauri-winrt-notification` `on_activated` works while the app is running; activation when it isn't running needs a ToastActivatorCLSID/COM registration via NSIS). Setting the AUMID from the NSIS installer. The MSIX feasibility for Tauri if the Store channel is picked. The choice of AEAD crate for the vault. This phase needs **Windows verification** more than desk research.
- **Phase 4:** the Google scope set and verification path (whether `gmail.modify` covers send or `gmail.send` must be added; the read-only calendar-list scope for merging calendars), the 7-day Testing-mode trap, and the BYO-client UX.
- **Phase 5:** `genai` structured-output support per provider (or the hand-rolled fallback), a current per-model price table, Gemini paid vs unpaid terms, and OpenRouter ZDR flags.
- **Phase 7:** driving a hidden capability-less `WebviewWindow` from the core (lifecycle, script evaluation, isolation), plus heuristics for detecting challenge pages.
- **Phase 8:** `rmcp` compatibility with servers older than 2025-11-25, which many community servers are; Windows `.cmd` spawning and Job Objects with `rmcp`'s child-process transport; remote OAuth (CIMD vs DCR, RFC 8707 `resource`).
- **Phase 9:** re-verify docs.x.com pricing and rate limits at phase start, whether zero-result polls are billed, and list endpoint costs.
- **Phase 10:** the signing CA (Certum or SSL.com cloud HSM vs SignPath if open source), Store certification requirements, Google's written position on LLM transfer under restricted scopes.

Phases with standard patterns (skip the research phase):
- **Phase 2:** the React/Tailwind/Motion/shadcn design-system work is well documented. Validate visually with Playwright.
- **Phase 3:** the connector trait, upsert-diff, scheduler and JSONPath engine are established patterns, fully specified in ARCHITECTURE.md. Only the catalog APIs' terms need a quick check.
- **Phase 6:** the policy-gated command pipeline and MIME building are well understood.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Versions verified against crates.io, npm and NuGet on 2026-10-18. Tauri, Google OAuth and MCP SDK tier facts come from official docs. MEDIUM for `genai` longevity, TS 7 tooling maturity, the `tauri-specta` RC and the signing CA options. |
| Features | MEDIUM | X pricing, MCP spec, Google scope rules, OpenRouter usage, Distill, changedetection.io, Glance and Homepage come from official sources. Competitor UX (Raycast, Sunsama, Akiflow, Station, Notion Calendar) comes from training data and comparison articles. |
| Architecture | MEDIUM-HIGH | The patterns (headless core, item model, upsert-diff, policy gate, three trust tiers, deterministic fallback) are well sourced and stack-agnostic. The **Tauri translation is our inference**, because the source document was written for Electron. In particular, losing process-level crash isolation for the core, and driving a hidden webview from the core, need to be proven in Phases 1 and 7. |
| Pitfalls | MEDIUM-HIGH | Google OAuth, X pricing and rate limits, the MCP security spec, SmartScreen and Artifact Signing geography, and the window-vibrancy limits come from 2026 official docs. Nitter status, X ToS damages, CASA costs and Modern Standby resume reliability are MEDIUM/LOW. |

**Overall confidence:** MEDIUM-HIGH. The build direction is clear. The remaining uncertainty is mostly external policy (Google, X, signing), not engineering.

### Gaps to Address

- **Distribution channel (open decision, settle by the end of Phase 1):**
  - **(a) Microsoft Store MSIX.** Free, available worldwide, no SmartScreen warnings because Microsoft re-signs. But updates go through the Store (no `tauri-plugin-updater`), autostart goes through the MSIX StartupTask, and it needs certification and a privacy policy. Tauri's MSIX support is LOW confidence and must be verified.
  - **(b) NSIS plus an OV code-signing certificate on a cloud HSM** (Certum, SSL.com eSigner; roughly $150-300 a year; 460-day maximum validity, so yearly renewal) plus `tauri-plugin-updater` on GitHub Releases. SmartScreen reputation builds over weeks.
  - Azure Artifact Signing is **not available to an individual in Turkey**.
  - Owner-only builds stay unsigned NSIS either way.
  - The decision affects autostart and updater plumbing. The rest of the architecture is unaffected.
- **Google distribution path:** whether sending mail content to an LLM makes a "local" app subject to CASA is unresolved. Build the BYO-client path regardless, and get Google's written position before paying an assessor (Phase 10).
- **Gmail scope set:** FEATURES.md proposes `gmail.modify` + `gmail.send`, which the archive/read/star quick actions need. PITFALLS.md proposes `gmail.readonly` + `gmail.send`. Recommendation: `gmail.modify` *instead of* `readonly`, since both are restricted and modify is required for triage, plus `gmail.send` unless modify is confirmed to cover sending, plus `calendar.events`. Verify in Phase 4.
- **Vault implementation detail:** ARCHITECTURE.md recommends envelope encryption, STACK.md recommends `keyring-core` directly with a DPAPI-blob fallback. Both fit behind `SecretStore`. Envelope encryption is the recommendation because of the 2560-byte limit and cleaner purge on disconnect. Pick and version-check the AEAD crate in Phase 1.
- **Crash isolation in Tauri:** without a separate core process, a blocking call or panic in core code could stall the app. Enforce per-task timeouts, no blocking I/O on the async runtime (use `spawn_blocking`), panic-to-failed-job handling, and a watchdog health line in the tray. Review in Phase 1.
- **Windows 10 support level:** Windows 10 reached end of support on 2025-10-14. Confirm whether v1 targets Windows 10 at all, or only provides the solid-background fallback.
- **X billing details:** whether zero-result polls are billed and what list endpoints cost are unverified (LOW). Re-check at Phase 9.
- **MCP prerequisites for non-developers:** most stdio servers assume `npx`/`uvx`. Decide in Phase 8 between detection plus guidance and favoring remote servers. Bundling a Node runtime would undercut Tauri's size advantage.
- **`genai` risk:** pre-1.0 with a single maintainer. The `LlmProvider` trait is mandatory, and structured-output support should be verified in Phase 5.

## Sources

### Primary (HIGH confidence)
- crates.io / npm / NuGet registries (2026-10-18): all pinned versions in STACK.md, including tauri 2.12.4, rmcp 3.5.1, sqlx 0.9.0, keyring-core 1.0.1, genai 0.6.5, oauth2 5.0.0, react 19.3.1, vite 8.3.4, tailwindcss 4.3.3, motion 14.3.0
- https://modelcontextprotocol.io/docs/sdk, /specification/latest, /specification/2026-07-28/changelog, security best practices: SDK tiers, stateless redesign, deprecations, untrusted annotations, consent requirements
- https://github.com/modelcontextprotocol/rust-sdk: rmcp spec support and transport/auth features
- https://v2.tauri.app (notification, windows-installer, updater, webdriver): installed-app toast caveat, cross-compile limits, mandatory updater signatures
- https://github.com/tauri-apps/window-vibrancy, tauri `config.rs`: Mica is Windows 11 only, Acrylic/Blur lag
- https://developers.google.com/identity/protocols/oauth2 (+ /native-app, restricted-scope verification, user data policy), Gmail sync guide and quotas: loopback plus PKCE, 7-day Testing tokens, 100-user cap, CASA, Limited Use, historyId 404
- https://docs.x.com/x-api/getting-started/pricing and rate limits: pay-per-use, $0.005 per post, $0.010 per user, 3M monthly cap
- https://learn.microsoft.com code-signing options and SmartScreen reputation, Modern Standby: Artifact Signing geography, EV no longer bypasses, Store MSIX, Smart App Control
- https://openrouter.ai/docs (usage accounting, ZDR); https://ai.google.dev/gemini-api/terms: cost in responses, free-tier training use
- https://www.electronjs.org/docs (utilityProcess, safeStorage, Notification): source of the original Electron architecture assumptions

### Secondary (MEDIUM confidence)
- MCP auth changes (CIMD, RFC 8707) via ssojet and den.dev; MCP TS SDK v2 version-negotiation guides
- Credential Manager 2560-byte limit (Docker PR #474, wincred, Chilkat)
- Tool poisoning and rug pulls (arXiv 2506.02040, CSA, CyberArk, OWASP MCP Top 10); Windows MCP orphan-process reports
- Gemini-in-Gmail injection (0DIN), EchoLeak CVE-2025-32711
- CA/B Forum 460-day code-signing validity (DigiCert); TS 7 release coverage
- Distill, Visualping, changedetection.io, Glance and Homepage docs for monitor and API-card UX
- rclone's personal-OAuth-client pattern

### Tertiary (LOW confidence)
- X third-party pricing blogs, legacy tier status, vendor X data APIs; xAI `x_search` pricing (sources conflict)
- Nitter shutdown timeline (Aug-Sep 2026); X ToS liquidated-damages interpretation
- CASA assessor costs; Tauri MSIX support status
- Competitor UX (Raycast, Sunsama, Akiflow, Station/Rambox/Shift, Notion Calendar, Superhuman) from training knowledge
- Electron vs Tauri RAM and installer sizes (community figures, not benchmarked)

---
*Research completed: 2026-10-18*
*Ready for roadmap: yes*
