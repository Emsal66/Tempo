# Phase 1: Windows Kabuğu ve Platform Altyapısı - Research

**Researched:** 2026-10-18
**Domain:** Tauri 2 desktop shell on Windows 11 (tray, single instance, autostart, WinRT toasts, Credential Manager vault, persistent scheduler, signed updater, CI)
**Confidence:** MEDIUM-HIGH. Crate and plugin APIs were verified against the published crate sources (downloaded and read in this session). Windows-runtime behaviour that cannot be observed from Linux (toast cold-start activation, custom toast audio, focus stealing, Modern Standby resume) is MEDIUM or LOW, and each item is mapped to a smoke-test check.

## Summary

Phase 1 builds the platform skeleton: a Cargo workspace (`crates/mornfold-core` without Tauri, `src-tauri` thin shell, `ui/` React), SQLite plus migrations, an envelope-encrypted secret vault whose master key lives in Windows Credential Manager, a persistent scheduler with sleep/resume catch-up, a tray app (single instance, autostart `--minimized`, close-to-tray), clickable Mornfold-branded toasts, Turkish i18n, and a signed updater fed by GitHub Releases. Every Tauri plugin needed is on the 2.x train at the versions in CLAUDE.md. I confirmed them on crates.io today and read their sources.

Three findings change the plan compared with the project-level research:

1. **`tauri-winrt-notification` 0.8.1 cannot do what D-08 and D-09 need.** It only emits `ms-winsoundevent:` audio. It sets no `launch` attribute and no `activationType`, and it relies on the in-process `Activated` event, which does not fire after the app has exited. Build the toast XML in core and show it from the shell through the `windows` 0.62 crate (`XmlDocument::LoadXml` + `ToastNotificationManager::CreateToastNotifierWithId`). Use **protocol activation** (`mornfold://…`) for the toast body and its buttons. Protocol activation works whether the app is running (single-instance forwards the URL) or closed (Windows launches `Mornfold.exe "mornfold://…"`), with no COM server.
2. **Microsoft documents custom toast audio for `ms-appx:///` and `ms-resource` only, and lists absolute file paths as unsupported.** An unpackaged NSIS app has no `ms-appx`. The plan therefore needs a two-strategy sound design that the owner's Win11 smoke test settles: (A) a `file:///` `src` on `<audio>`, and (B) a silent toast plus the app playing the WAV itself.
3. **Tauri does not expose OS suspend/resume on desktop.** `RunEvent::Suspended`/`Resumed` are mobile-only (verified in the tauri 2.12.4 source). Use `PowerRegisterSuspendResumeNotification` (callback-based, so no window is needed) from the `windows` crate. Back it up with wall-clock-gap detection in the 30-second scheduler tick.

**Primary recommendation:** Put every testable decision in `mornfold-core` as pure functions: toast XML builder, deep-link parser, tray menu model, scheduler catch-up, vault crypto, masking, i18n lookup. Unit-test them on Linux. Keep `src-tauri` as a thin `#[cfg(windows)]` adapter layer, compile-checked with `cargo xwin check`. Get the Windows CI producing the NSIS artifact in the first plan. That artifact is the only way the owner can verify toasts, tray, autostart, Credential Manager and the updater.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Dağıtım ve güncelleme
- **D-01:** Dağıtım kanalı **NSIS + GitHub Releases** (`tauri-plugin-updater`, imzalı `latest.json`). Microsoft Store (MSIX) v2'de yeniden değerlendirilir. Bu, roadmap'teki "dağıtım kanalı kararı yazılı olarak alınır" kriterini karşılar.
- **D-02:** Yeni sürüm bulununca **arka planda indir, sonra kullanıcıya "yeniden başlat" diye sor** — hem Windows bildirimi hem tepsi menüsünde bir öğe olarak. Sessiz kurulum yok.
- **D-03:** Sürümler **bu repoda (emsal66/tempo, herkese açık) GitHub Releases** olarak yayımlanır; ayrı sürüm reposu yok.
- **D-04:** **Her push'ta CI artifact** (NSIS kurulum dosyası) üretilir; **gerçek sürüm yalnızca `v0.x.y` etiketi** ile oluşturulur (tauri-action v1).
- **D-05:** Güncelleyici imza anahtarı (minisign) **şifre yöneticisi + çevrimdışı USB + GitHub Actions secret** olarak yedeklenir. Planlar anahtar üretimi ve yedekleme adımlarını kullanıcıya yaptırılacak manuel adımlar olarak içermeli (anahtar repoya asla girmez).

#### Faz 1 penceresi ve bildirim
- **D-06:** Faz 1 sonunda pencere **sade iskelet**: Mica cam arka plan + özel başlık çubuğu (`decorations: false`, drag region) + Mornfold logosu + sistem durumu (son kontrol zamanı, sürüm, güncelleme durumu) + Ayarlar. Faz 2'de yerine gerçek pano gelir; bu ekran geçicidir, cila minimum ama "çirkin" olmamalı.
- **D-07:** **Elle başlatıldığında pencere açılır**; Windows açılışında otomatik başlatıldığında (`--minimized`) **sadece tepsiye iner**.
- **D-08:** Windows bildirimleri **butonlu**: tıklayınca ilgili ekran açılır + 1–2 hızlı buton (örn. "Aç", "Sessize al"). **Uygulama kapalıyken (cold start) de tıklama çalışmalı** → ToastActivatorCLSID / COM aktivasyon kaydı (NSIS) gerekir; araştırma bayrağı.
- **D-09:** **Özel Mornfold bildirim sesi.** Referans: **Discord'un bildirim sesinin karakteri** — kısa, yumuşak, tanınabilir bir "ping". Discord'un sesi birebir kopyalanmaz (telif); benzer hissi veren **özgün** bir ses dosyası üretilir veya uygun lisanslı bir ses kullanılır. Ses toast'a custom audio olarak bağlanır (installed app gerektirir).
- **D-10:** Ses **her bildirimde çalar**; Ayarlar'dan **tamamen veya servis bazında kapatılabilir**; **sessiz saatlerde çalmaz** (sessiz saatlerin kendisi Faz 3 bildirim motorunda; Faz 1'de ses aç/kapa ayarı ve altyapısı yeterli).

#### Tepsi ve arka plan
- **D-11:** Tepsi ikonuna **sol tık pencereyi açar/gizler**; **sağ tık menü**: Aç / Tümünü duraklat / Çık (Discord gibi). Güncelleme hazırsa menüde "Yeniden başlat ve güncelle" öğesi (D-02).
- **D-12:** Tepsi ikonu **durum gösterir**: okunmamış bildirim sayısı için rozet; duraklatıldığında veya hata olduğunda ikon soluk/farklı görünür. Üzerine gelince ipucunda "son kontrol: X dk önce" (SHELL-06).
- **D-13:** "Tümünü duraklat" **süre seçmeli**: 1 saat / Yarına kadar / Ben açana kadar (Discord "rahatsız etme" menüsü gibi). Süre dolunca otomatik devam.
- **D-14:** Pencerenin X düğmesi **sessizce tepsiye indirir** — bilgi bildirimi yok. Tamamen kapatmak tepsi menüsündeki "Çık" ile.

#### Anahtar kasası deneyimi
- **D-15:** Kayıtlı anahtar arayüzde **maskeli + son 4 hane** görünür (örn. `sk-ant-••••a1b2`); yanında Değiştir / Sil / Test et. Tam değer arayüze asla dönmez (SHELL-07) — maskeli gösterim çekirdekte hesaplanır.
- **D-16:** Değiştirme/silmede **ek doğrulama yok** (Windows Hello istenmez); Windows oturumu açıkken doğrudan yapılır.
- **D-17:** Anahtar girildiğinde **önce test, sonra kaydet**; geçersizse uyar. Test yalnızca sağlayıcısı bilinen anahtar türleri için; genel anahtarlarda test adımı atlanabilir.
- **D-18:** Faz 1'de Ayarlar'da **genel "Anahtarlar" bölümü**: ad + değer ekle / sil / test. Claude / Gemini / OpenRouter alanları Faz 5'te bu kasaya bağlanır. Kasa tasarımı araştırmadaki öneriyi izler: zarf şifreleme (Credential Manager'da ana anahtar + SQLite'ta AES-256-GCM şifreli değerler), `SecretStore` trait arkasında.

### Claude's Discretion
- Geçici iskelet ekranın tam yerleşimi ve tipografisi (D-06 sınırları içinde).
- Bildirim butonlarının tam metinleri ve hangi bildirimde hangi butonların çıkacağı (en fazla 2).
- Tepsi ikonunun rozet/soluk durum görselleri.
- Zamanlayıcı iskeletindeki test işinin ne olduğu (sağlık kontrolü/heartbeat gibi) — uyku telafisini kanıtlamaya yetmeli.
- AEAD crate seçimi ve kasa iç detayları (araştırma bayrağı).

### Deferred Ideas (OUT OF SCOPE)
- Microsoft Store (MSIX) kanalı — v2'de yeniden değerlendirilecek (D-01).
- Sessiz saatler kuralının kendisi — Faz 3 bildirim motoru (Faz 1'de yalnızca ses aç/kapa).
</user_constraints>

**Note on D-08:** D-08 assumes cold-start clicks need a "ToastActivatorCLSID / COM activation registration". This research finds a simpler mechanism, protocol activation, that meets the same user-facing goal: the click works while the app is closed. The full COM activator stays a documented fallback (Tier 2 below) if the smoke test shows protocol activation failing. The decision's intent, "cold start click must work", is unchanged.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SHELL-01 | NSIS installer for Windows 11 | Tauri NSIS bundler 2.10.3 (`bundle.targets: ["nsis"]`, `installMode: currentUser`, `languages: ["Turkish"]`); CI on `windows-2025` builds on every push and uploads the `.exe` (Pattern 9, CI examples) |
| SHELL-02 | Close → tray; tray menu Aç / Tümünü duraklat / Çık | Tauri 2.12 `TrayIconBuilder` + `on_tray_icon_event` (Click with `button_state`), `CloseRequested` + `prevent_close()` + `hide()`; dynamic menu rebuilt from a pure core `TrayModel` (Pattern 4) |
| SHELL-03 | Autostart at login, minimized to tray, user-toggleable | `tauri-plugin-autostart` 2.7.0 (auto-launch 0.6: HKCU `Run` key, honours `StartupApproved`) with `.arg("--minimized")`; main window `visible:false`; window-state with `VISIBLE` flag removed (Pattern 3, Pitfall 4) |
| SHELL-04 | Second launch focuses existing window | `tauri-plugin-single-instance` 2.5.4 registered **first**, feature `deep-link`; callback shows/unminimizes/focuses (Pattern 3, Pitfall 6) |
| SHELL-05 | Mornfold-branded toasts, click opens the relevant screen | Tauri NSIS already sets `System.AppUserModel.ID = identifier` on the Start Menu shortcut (verified in installer.nsi/utils.nsh); toast XML with `activationType="protocol"` + `mornfold://` scheme registered by NSIS from `plugins.deep-link.desktop.schemes`; custom audio strategies A/B (Pattern 5) |
| SHELL-06 | Missed checks after sleep run once, in order; tray "son kontrol: X dk önce" | Core scheduler: `jobs` table, wall-clock `next_run_at`, 30 s tick, coalesced catch-up ordered by `next_run_at`; `PowerRegisterSuspendResumeNotification` resume trigger + wall-clock-gap fallback; tooltip refreshed on `TrayIconEvent::Enter` + every 60 s (Pattern 6) |
| SHELL-07 | Secrets in a vault tied to Credential Manager; never readable from UI; never in logs | Envelope encryption: 32-byte master key in Credential Manager (`keyring-core` 1.0.1 + `windows-native-keyring-store` 1.1.0, `persistence=Local`), values AES-256-GCM (`aes-gcm` 0.11.1) in SQLite with AAD; write-only IPC; masking in core; log redaction + canary test (Pattern 7) |
| SHELL-08 | Turkish UI via i18n | `i18next` 26.5.2 + `react-i18next` 17.1.0, single `tr.json` source shared with Rust (tray/toast strings via `include_str!`), key-coverage tests (Pattern 8) |
| SHELL-09 | In-app signed update | `tauri-plugin-updater` 2.14.0 driven from Rust: `check()` → `download()` in background → tray/toast "Yeniden başlat ve güncelle" → `install(bytes)`; `tauri-action@v1` with `uploadUpdaterJson`; minisign key generated and backed up by the owner (Pattern 9) |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- **Platform:** Windows 11 desktop only. Mica is Win11-only. Windows 10 is out of scope (REQUIREMENTS "Out of Scope"), so no Win10 fallback work is needed beyond not crashing.
- **Privacy:** data and keys are local only. No server, no telemetry, no CDN fonts/icons at runtime (bundle `@fontsource-variable/inter`).
- **Design:** Apple/Linear aesthetic. Mica as the primary backdrop. **Never Acrylic/Blur on the resizable main window** (documented lag).
- **Architecture:** `crates/mornfold-core` has **no Tauri dependency**. `src-tauri` is a thin shell with `#[cfg(windows)]` adapters. The UI never calls external APIs and never holds secrets.
- **Forbidden:** `tauri-plugin-sql`; `tauri-plugin-store` for anything sensitive; `keytar`; the all-in-one `keyring` 4.x crate; `rusqlite` alongside `sqlx`; `tauri-plugin-http` (reqwest 0.12); `tauri-plugin-stronghold`; `tokio::time::sleep` for wall-clock schedules; typescript-eslint on TS 7 (use Biome); loading fonts/icons from CDNs; `window-vibrancy` Acrylic/Blur as the primary effect.
- **Required tooling:** pnpm (lockfile committed), Biome 2.5, Vitest 5 + Testing Library + `@tauri-apps/api/mocks`, Playwright for UI screenshots on Linux, `cargo xwin check --target x86_64-pc-windows-msvc` for Windows-only Rust, sqlx offline mode (`.sqlx/` committed), GitHub Actions pinned to `windows-2025` (not `-latest`), `tauri-apps/tauri-action` v1.
- **Versions:** keep `tauri` crate and `@tauri-apps/api` on the same minor (2.12). Pin `specta`/`tauri-specta` with `=`. One reqwest major (0.13) for any HTTP work.
- **GSD workflow:** file changes go through GSD commands. Do not change `.claude/settings.json` hooks or permissions without explicit user approval.
- **i18n:** all UI strings from i18n with Turkish first. Use the `tr-TR` locale for dates, numbers and casing (`İ`/`ı`).

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Single instance, tray, window show/hide, autostart registration | Shell (`src-tauri`) | Core (pure `TrayModel`, arg parsing) | OS integration is a shell job. Menu contents and states are pure data, testable in core. |
| Toast XML content, deep-link grammar, notification tokens | Core | Shell (WinRT display) | XML building and URL parsing are pure and security-sensitive (escaping and allowlist), so they need Linux tests. Displaying is Windows-only. |
| Toast display, audio playback fallback, AUMID checks | Shell `#[cfg(windows)]` | — | WinRT/Win32 only |
| Power suspend/resume detection | Shell `#[cfg(windows)]` | Core (gap detector) | Win32 callback in the shell. Core receives a `Resumed` signal and also detects clock gaps itself. |
| Scheduler (jobs, catch-up, pause state, heartbeat) | Core (tokio task) | DB | Never in webview JS (throttled when hidden) |
| Secret vault crypto, masking, persistence | Core | Shell (Credential Manager `KeyProvider` impl) | Crypto and masking are pure. Only the master-key fetch touches Windows. |
| Settings, job state, vault rows | Database (SQLite via sqlx, single writer in core) | — | Single owner. The UI goes through commands. |
| Updater check/download/install | Shell (plugin is Tauri-bound) | Core (schedules `update_check` job, holds "update ready" state) | `tauri-plugin-updater` needs `AppHandle`. The scheduler drives its timing. |
| UI skeleton, Settings forms, i18n rendering | Browser/Client (React in WebView2) | — | Receives only typed results. No secrets after submit. |
| Turkish strings for tray/toasts | Core (lookup over shared `tr.json`) | UI (same file via i18next) | One source of truth for both runtimes |
| Build, sign updater artifacts, release | CI (`windows-2025`) | Owner (key custody) | NSIS signing is unsupported when cross-compiling. The private key never lives on the dev box. |

## Standard Stack

### Core (Rust)
| Library | Version (verified crates.io 2026-10-18) | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `tauri` | 2.12.4 (MSRV 1.90), features `tray-icon`, `image-png`, `specta` | Shell, IPC, tray | Locked stack. The `specta` feature is required by tauri-specta. |
| `tauri-build` | 2.7.3 | build.rs, app manifest/ACL | Standard |
| `tauri-plugin-single-instance` | 2.5.4, feature `deep-link` | Single instance + forwards protocol URLs to the deep-link plugin | Official. Must be registered first. |
| `tauri-plugin-deep-link` | 2.6.2 | `mornfold://` scheme; NSIS writes the `HKCU\Software\Classes\mornfold` handler | Official. The bundler inserts registry writes from `plugins.deep-link.desktop.schemes`. |
| `tauri-plugin-autostart` | 2.7.0 | HKCU Run key (auto-launch 0.6) | Official. Honours `StartupApproved` (user disabling in Task Manager). |
| `tauri-plugin-updater` | 2.14.0 | Signed updates; `check`/`download`/`install` separately | Official. Signature check is mandatory. |
| `tauri-plugin-process` | 2.4.0 | `restart()` helper (optional) | Official |
| `tauri-plugin-window-state` | 2.5.0 | Window size/position persistence | Official. Configure without the `VISIBLE` flag. |
| `tauri-plugin-log` | 2.10.1 | Rotating log files (`LogDir`, `KeepSome(n)`, `max_file_size`, `format`, `level_for`) | Official |
| `tauri-plugin-opener` | 2.7.2 | Open links/folders (e.g., "Log klasörünü aç") | Official |
| `tauri-plugin-prevent-default` | 6.0.1 | Disable F5/Ctrl+P/context menu in release | Locked stack |
| `tauri-specta` / `specta` / `specta-typescript` | `=2.0.0-rc.25` / `=2.0.0-rc.25` / `0.0.12` | Typed commands + events → `ui/src/bindings.ts` | Locked stack. Depends on `tauri = "2"` with feature `specta`. |
| `sqlx` | 0.9.0 (MSRV 1.94), `default-features=false`, features `runtime-tokio`, `sqlite-bundled`, `migrate`, `macros` | SQLite (WAL), embedded migrations, offline macros | Locked stack. Avoid the umbrella `sqlite` feature, which also enables `sqlite-load-extension`. |
| `keyring-core` | 1.0.1 | Credential Manager API (`Entry::new`, `set_secret`, `get_secret`, `delete_credential`), `mock` store for tests | Locked stack |
| `windows-native-keyring-store` | 1.1.0 (MSRV 1.88), `cfg(windows)` | Windows Credential Manager backend | Locked stack |
| `aes-gcm` | 0.11.1 (RustCrypto, MSRV 1.85), features default + `zeroize` | AES-256-GCM for vault values | **Recommended AEAD (Claude's discretion).** Pure Rust, no C toolchain, `getrandom` key/nonce generation built in. |
| `zeroize` | 1.9.1 | Wipe plaintext/key buffers | Standard companion |
| `secrecy` | 0.10.3 | `SecretString`/`SecretBox` with redacted `Debug` | Prevents accidental logging of secrets |
| `base64` | 0.23.1 | Encode master key for Credential Manager (`set_secret` takes bytes, so base64 is optional) | Standard |
| `croner` | 4.0.1, `default-features=false`, features `jiff` | Cron parsing (only needed if Phase 1 adds a cron job; heartbeat/update use intervals) | Locked stack. Use the jiff backend, not chrono. |
| `jiff` | 0.2.38 (default features bundle tzdb on Windows) | Wall-clock time, "Yarına kadar" midnight math, relative time | Locked stack |
| `tokio` | 1.53.3 | Runtime | Locked stack |
| `tracing` | 0.1.44, feature `log` | Logging in core. Records flow to `log`, then tauri-plugin-log when no tracing subscriber is installed. | Locked stack |
| `thiserror` / `anyhow` | 2.0.21 / 1.0.104 | Errors | Locked stack |
| `serde` / `serde_json` | 1.0.229 / 1.0.154 | Serialization; parse `tr.json` in Rust | Locked stack |
| `quick-xml` | 0.42.0 | Build toast XML **with escaping** in core | Avoids hand-escaping. Safe-by-construction XML. |
| `ulid` | 3.0.0 | IDs for notifications/jobs/secrets | Sortable, compact |
| `url` | 2.5.8 | Parse `mornfold://` deep links | Standard |
| `windows` | 0.62.2 (same major as tauri 2.12), `cfg(windows)`; features `Data_Xml_Dom`, `UI_Notifications`, `Win32_System_Power`, `Win32_UI_WindowsAndMessaging`, `Win32_UI_Shell`, `Win32_Media_Audio`, `Win32_System_Com`, `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation` | Toasts, power callback, `PlaySoundW`, `SHQueryUserNotificationState`, shortcut property store | Verified function signatures in the 0.62.2 source |
| `windows-registry` | 0.100.0 (optional) | Light-taskbar detection for tray icon variant; AUMID registry fallback | Small, Microsoft-maintained |

### Supporting (dev/test)
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `insta` | 1.49.0 | Snapshot tests (toast XML, tray model, bindings) | Core tests |
| `tempfile` | 3.27.0 | Temp DB files in tests | Core tests |
| `tokio` `test-util` feature | 1.53 | Paused time in scheduler tests | Core tests (prefer an injected `Clock` trait for wall-clock tests) |

### Frontend (npm, verified registry 2026-10-18)
| Package | Version | Purpose |
|---------|---------|---------|
| `react` / `react-dom` | 19.3.1 | UI |
| `@tauri-apps/api` | 2.12.4 | `invoke`, window controls, `mocks` (`mockIPC(cb, { shouldMockEvents: true })`, `mockWindows`, `clearMocks`) |
| `i18next` / `react-i18next` | 26.5.2 / 17.1.0 | i18n |
| `@tanstack/react-query` | 5.104.4 | Core data cache, invalidated by events |
| `zustand` | 5.0.15 | UI-only state |
| `motion` | 14.3.0 | Minimal transitions in the skeleton screen |
| `lucide-react` | 1.59.0 | Icons |
| `@fontsource-variable/inter` | 5.3.0 | Bundled font with Turkish glyphs |
| `tailwindcss` / `@tailwindcss/vite` | 4.3.3 | Styling |
| dev: `@tauri-apps/cli` 2.12.3, `vite` 8.3.4, `@vitejs/plugin-react` 6.1.2, `typescript` 7.0.2, `@biomejs/biome` 2.5.15, `vitest` 5.0.5, `jsdom` 30.1.3, `@testing-library/react` 16.3.3, `@playwright/test` 1.64.1 | — | Toolchain |

**Do not add JS plugin packages** (`@tauri-apps/plugin-updater`, `-autostart`, `-notification`) in Phase 1. Drive these features through typed app commands. This keeps the webview's capability surface minimal, so it cannot call `updater:install` or `autostart:enable` directly, and it removes JS/Rust plugin version-mismatch warnings.

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Own WinRT toast code (`windows` crate) | `tauri-winrt-notification` 0.8.1 | The crate cannot set `launch`/`activationType`/custom `<audio src>` (verified in its `lib.rs`). It is fine for foreground-only toasts but fails D-08 cold start and D-09 custom audio. |
| `tauri-plugin-notification` 2.7 | — | Same limitation. Actions are mobile-only. Not used for Windows toasts. |
| Protocol activation | Full COM activator (`INotificationActivationCallback`, `CoRegisterClassObject`, `HKCU\Software\Classes\CLSID\{GUID}\LocalServer32`, `AppUserModelId\{AUMID}\CustomActivator`) as the Windows App SDK does for unpackaged apps | COM is in-process and supports text inputs on toasts, but costs about 200 lines of unsafe COM, apartment/threading hazards with the tao main thread, and interaction with single-instance. **Tier 2 fallback only.** |
| `aes-gcm` 0.11 | `chacha20poly1305` 0.11.0, `ring` 0.17.14, `aws-lc-rs` 1.18.2 | ChaCha is equally fine. `ring`/`aws-lc-rs` add C/asm toolchain requirements that complicate `cargo xwin` builds. AES-GCM matches D-18's wording. |
| Envelope (master key in CredMan) | DPAPI `CryptProtectData` per value | Locked by D-18. DPAPI stays a possible `KeyProvider` fallback behind the same trait. |
| `PowerRegisterSuspendResumeNotification` | Subclassing the main window to catch `WM_POWERBROADCAST` | Subclassing breaks if the window is destroyed (memory-saver mode) and is fragile with tao. Message-only windows do not receive broadcast messages. |

**Installation (Wave 0):**
```bash
# Linux container prerequisites (Tauri Linux deps so src-tauri compiles/tests on Linux)
sudo apt-get update && sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
rustup target add x86_64-pc-windows-msvc
cargo install --locked cargo-xwin            # 0.23.1
cargo install --locked sqlx-cli --no-default-features --features sqlite,rustls   # 0.9.0
# pnpm 12 via corepack (container has pnpm 10.28 / node 22.22)
corepack enable && corepack prepare pnpm@12.14.0 --activate
```

## Package Legitimacy Audit

slopcheck was installed and run (`slopcheck scan`) on a scratch Cargo.toml (33 crates) and package.json (22 packages) listing every package above.

| Package | Registry | Age | Downloads | Source Repo | slopcheck | Disposition |
|---------|----------|-----|-----------|-------------|-----------|-------------|
| tauri, tauri-build, tauri-plugin-* (single-instance, autostart, updater, process, window-state, log, opener, deep-link) | crates.io | 4+ yrs | 4.5M–37M total | github.com/tauri-apps | [OK] | Approved |
| tauri-plugin-prevent-default | crates.io | 2+ yrs | 290k | github.com/ferreira-tb/tauri-plugin-prevent-default | [OK] | Approved |
| tauri-specta / specta / specta-typescript | crates.io | 2+ yrs | 1.5M / 3.0M / 1.7M | github.com/specta-rs | [OK] | Approved (pin `=`) |
| keyring-core / windows-native-keyring-store | crates.io | 1+ yr | 4.8M / 2.7M | github.com/open-source-cooperative | [OK] | Approved |
| sqlx, tokio, serde, serde_json, thiserror, anyhow, tracing, tracing-subscriber, base64, insta, jiff, croner | crates.io | many yrs | 10M–2B | official orgs | [OK] | Approved |
| aes-gcm, zeroize, secrecy | crates.io | many yrs | 181M / 779M / 181M | github.com/RustCrypto, iqlusioninc | [OK] | Approved |
| windows, windows-registry | crates.io | many yrs | 366M / 144M | github.com/microsoft/windows-rs | [OK] | Approved |
| quick-xml, ulid, url, tempfile | crates.io | many yrs | high | official repos | not scanned (added after scan; long-established) | Approved [ASSUMED legit: well known] |
| react, react-dom, @tauri-apps/api, @tauri-apps/cli, i18next, react-i18next, @tanstack/react-query, zustand, motion, lucide-react, @fontsource-variable/inter, vite, @vitejs/plugin-react, typescript, tailwindcss, @tailwindcss/vite, @biomejs/biome, jsdom, @testing-library/react, @playwright/test | npm | many yrs | high | official | [OK] | Approved |
| vitest | npm | since 2021-12 | 76.5M/wk | github.com/vitest-dev/vitest | [SUS] (name similarity to `vite`) | **False positive.** Official Vite test runner, locked in CLAUDE.md. Flagged per protocol: planner may add a lightweight human-verify note. |

`npm view <pkg> scripts.postinstall` returned empty for `@tauri-apps/cli` and `@biomejs/biome` (they ship platform binaries through optionalDependencies).

**Packages removed due to [SLOP]:** none
**Packages flagged [SUS]:** `vitest` (false positive, see above)

## Architecture Patterns

### System Architecture Diagram

```
                    ┌────────────── Windows 11 ───────────────────────────────────────────┐
 Login (HKCU Run) ──►  Mornfold.exe --minimized                                           │
 Start menu/2nd run ►  Mornfold.exe ──► [single-instance] ─(already running?)─► forward argv ─┐
 Toast click ──────►  Mornfold.exe "mornfold://open/…?n=<id>"  (protocol activation)        │ │
 Power suspend/resume ─► PowerRegisterSuspendResumeNotification callback ──┐                 │ │
                    └──────────────────────────────────────────────────────┼─────────────────┘ │
                                                                           ▼                   ▼
 ┌──────────────────────────── src-tauri (thin shell) ─────────────────────────────────────────────┐
 │ setup(): parse args (--minimized? deep link?) → open DB → unlock vault (KeyProvider=CredMan)     │
 │          → start core runtime → build tray from TrayModel → show window unless --minimized       │
 │ deep-link handler ─► core::deeplink::parse (allowlist) ─► Navigate event / local action w/ token │
 │ Notifier impl: core::toast_xml(spec) → XmlDocument.LoadXml → ToastNotifier(AUMID).Show           │
 │ Tray: left-click Up → toggle window; right-click → menu (Aç / Duraklat ▸ / Yeniden başlat… / Çık)│
 │ Updater: on `update_check` job → check() → download() → state UpdateReady → tray item + toast    │
 └───────┬───────────────────────────────▲───────────────────────────────────┬──────────────────────┘
         │ typed commands (tauri-specta) │ typed events (StatusChanged, Navigate, UpdateReady)
         ▼                               │                                   ▼
 ┌──────────────── ui/ (React, WebView2, Mica) ───────────┐   ┌──────── mornfold-core (no Tauri) ────────┐
 │ TitleBar(drag) · Durum (son kontrol, sürüm, güncelleme) │   │ Scheduler loop (30s tick + Resumed signal)│
 │ Ayarlar: Otomatik başlat · Ses · Anahtarlar · Test bild.│   │   due jobs ordered by next_run_at, once   │
 │ i18next(tr) · React Query (invalidate on events)        │   │ Vault: AES-256-GCM(AAD) ◄ master key      │
 └─────────────────────────────────────────────────────────┘   │ Settings/pause state · TrayModel · i18n   │
                                                               │ toast_xml · deeplink · masking            │
                                                               └────────────────┬──────────────────────────┘
                                                                                ▼
                                                    SQLite (WAL) %LOCALAPPDATA%\<identifier>\mornfold.db
                                                    Credential Manager: generic cred "Mornfold master key" (Local)
```

### Recommended Project Structure
```
Cargo.toml                    # [workspace] members = ["crates/mornfold-core", "src-tauri"]; resolver = "3"
rust-toolchain.toml           # channel = "1.99.1", targets = ["x86_64-pc-windows-msvc"], components = ["clippy","rustfmt"]
.sqlx/                        # committed offline query data
crates/mornfold-core/
  migrations/0001_init.sql    # settings, jobs, job_runs, vault_meta, secrets, notifications
  src/{lib.rs, db.rs, settings.rs, clock.rs, scheduler/, vault/, notify/{toast_xml.rs, deeplink.rs, spec.rs}, tray_model.rs, i18n.rs, masking.rs, platform.rs (traits: KeyProvider, Notifier, PowerEvents)}
src-tauri/
  Cargo.toml  build.rs  tauri.conf.json  tauri.release.conf.json (createUpdaterArtifacts: true)
  capabilities/main.json      # minimal: core window ops for custom title bar + app commands
  windows/hooks.nsh           # NSIS_HOOK_POSTINSTALL (stub ToastActivatorCLSID, optional) / PREUNINSTALL
  icons/ (app + tray variants: normal, dot, paused, error; light/dark)
  resources/sounds/mornfold-ping.wav
  src/{main.rs, lib.rs (run()), commands.rs, tray.rs, window.rs, updater.rs, win/{toast.rs, power.rs, keyring.rs, sound.rs}}
ui/
  src/{main.tsx, bindings.ts (generated), i18n/{index.ts, tr.json}, routes/, components/, lib/}
  tests/ (vitest), e2e/ (playwright, mockIPC)
scripts/gen-ping.py           # stdlib-only synthesis of the original notification sound
docs/decisions/0001-distribution-channel.md
docs/smoke-test-phase1.md     # owner's Win11 checklist
.github/workflows/{ci.yml, release.yml}
```

### Pattern 1: Thin shell, pure core (testability split)
**What:** Anything with logic (tray menu contents, toast XML, deep-link parsing, scheduler decisions, masking, i18n lookups) is a pure function or struct in `mornfold-core`. `src-tauri` only translates between Tauri/Win32 objects and those values.
**When:** Always in this phase. It is the only way to test on Linux, since Windows behaviour is checked only by CI compilation plus the owner's smoke test.
**Platform traits in core:**
```rust
// crates/mornfold-core/src/platform.rs
pub trait KeyProvider: Send + Sync {            // master key source
    fn load_or_create(&self) -> Result<MasterKey, VaultError>; // MasterKey wraps Zeroizing<[u8;32]>
    fn delete(&self) -> Result<(), VaultError>;
}
pub trait Notifier: Send + Sync {
    fn show(&self, spec: &ToastSpec) -> Result<(), NotifyError>; // spec → toast_xml() inside impl
}
```

### Pattern 2: Startup sequence and plugin order
```rust
// src-tauri/src/lib.rs  (sketch; API names verified in tauri 2.12.4 / plugin sources)
pub fn run() {
    let specta = make_specta_builder(); // tauri_specta::Builder::<tauri::Wry>::new().commands(collect_commands![...]).events(collect_events![...])
    tauri::Builder::default()
        // 1) single-instance FIRST (plugins run in registration order)
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            // deep-link feature already forwarded argv to the deep-link plugin
            if !argv.iter().any(|a| a.starts_with("mornfold://")) { window::show_and_focus(app); }
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_autostart::Builder::new().arg("--minimized").app_name("Mornfold").build())
        .plugin(tauri_plugin_window_state::Builder::new()
            .with_state_flags(tauri_plugin_window_state::StateFlags::all()
                - tauri_plugin_window_state::StateFlags::VISIBLE).build())
        .plugin(tauri_plugin_log::Builder::new() /* LogDir, KeepSome(5), max_file_size, redacting format(), level_for("tauri", Info) */ .build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_prevent_default::init())
        .invoke_handler(specta.invoke_handler())
        .setup(move |app| {
            specta.mount_events(app);
            let minimized = std::env::args().any(|a| a == "--minimized");
            // open DB, run migrations, unlock vault, spawn scheduler, build tray, register power callback,
            // app.deep_link().on_open_url(...), check deep_link().get_current() for cold-start URL
            if !minimized { /* show after UI signals ready (ui_ready command) to avoid a flash */ }
            Ok(())
        })
        .on_window_event(|w, e| if let tauri::WindowEvent::CloseRequested { api, .. } = e {
            api.prevent_close(); let _ = w.hide();   // D-14: silent
        })
        .build(tauri::generate_context!()).expect("build")
        .run(|app, ev| if let tauri::RunEvent::ExitRequested { code: None, api, .. } = ev {
            api.prevent_exit();                     // keep running in tray if a window ever gets destroyed
        });
}
```
`tauri.conf.json` main window: `"visible": false, "decorations": false, "transparent": true, "windowEffects": { "effects": ["mica"] }, "noRedirectionBitmap": true, "shadow": true`. Window effects **require** `transparent: true` (tauri-utils doc comment). `shadow: true` gives undecorated windows rounded corners on Win11.

### Pattern 3: Tray — left-click toggle, right-click dynamic menu
- `TrayIconBuilder::with_id("main").icon(..).tooltip(..).show_menu_on_left_click(false).menu(&menu).on_tray_icon_event(..).on_menu_event(..)`.
- `TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. }` → toggle. **Handle only `Up`.** Click fires for both Down and Up.
- Toggle rule: clicking the tray icon takes focus away from the window first, so "is focused" is always false at click time. Track `last_blur_at`. If the window is visible and lost focus less than ~300 ms ago, or is focused, hide it; otherwise show + unminimize + focus.
- **Dynamic menu = rebuild from `core::TrayModel`** and call `tray.set_menu(Some(menu))` whenever state changes (pause, update ready, health). Model fields: `paused: Option<PauseUntil>`, `update_ready: Option<Version>`, `last_check: Option<Timestamp>`, `unread: u32`, `health: Ok|Degraded|Error`. Menu: `Aç` · `Son kontrol: 3 dk önce` (disabled info line, as PITFALLS recommends) · `Tümünü duraklat ▸ {1 saat, Yarına kadar, Ben açana kadar}` or `Devam et` when paused · `Yeniden başlat ve güncelle` (only when ready) · separator · `Çık`.
- Tooltip: `set_tooltip(Some(format!(...)))` on `TrayIconEvent::Enter` (fresh relative time on hover) plus a 60 s refresh. Windows tooltips are limited to 127 chars.
- Icon state: `set_icon(Some(Image::from_bytes(include_bytes!(..))?))` with pre-rendered 32×32 PNG variants: normal, unread (small dot), paused (desaturated/dimmed), error. A count does not fit legibly at 16 px, so put the count in the tooltip ("3 okunmamış") and show a dot on the icon. Pick light/dark variants by reading `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize\SystemUsesLightTheme` (Claude's discretion).

### Pattern 4: Pause-all with durations (D-13)
Store `pause_until` in `settings` as `{"kind":"for","until_ms":…}`, `{"kind":"until_tomorrow","until_ms":…}` or `{"kind":"indefinite"}`. The scheduler skips network/check jobs while paused but still runs the `heartbeat`/`resume_watch` bookkeeping. On each tick, if `until_ms <= now`, clear the pause and emit `StatusChanged`. "Yarına kadar" = next local midnight via jiff (`Zoned::now().tomorrow()?.start_of_day()?`), shown explicitly in the menu ("Yarın 00:00'a kadar duraklatıldı"). See Open Question 2.

### Pattern 5: Toasts — protocol activation, AUMID, sound (D-08/D-09/D-10)
**Identity (verified):** Tauri's NSIS template calls `SetLnkAppUserModelId` on the Start Menu shortcut (`$SMPROGRAMS\Mornfold.lnk`) with `${BUNDLEID}` = `identifier`. Toasts sent with `CreateToastNotifierWithId(identifier)` therefore show "Mornfold" and the exe icon **only in the installed build**. Dev builds under `target\debug|release` must use the PowerShell AUMID (`{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe`), which is the same rule `tauri-plugin-notification` uses.

**Activation (Tier 1, recommended):**
```xml
<toast launch="mornfold://open/settings?n=01J…" activationType="protocol">
  <visual><binding template="ToastGeneric"><text>Mornfold</text><text>Test bildirimi</text></binding></visual>
  <audio src="file:///C:/Users/…/AppData/Local/Mornfold/resources/sounds/mornfold-ping.wav"/>  <!-- strategy A -->
  <actions>
    <action content="Aç" activationType="protocol" arguments="mornfold://open/settings?n=01J…"/>
    <action content="Sessize al" activationType="protocol" arguments="mornfold://act/mute?n=01J…&amp;for=1h"/>
  </actions>
</toast>
```
- Warm: Windows launches a second `Mornfold.exe "mornfold://…"`. Single-instance forwards argv, the deep-link plugin emits `on_open_url`, and the shell validates and navigates.
- Cold: Windows launches `Mornfold.exe "mornfold://…"`. In `setup`, `app.deep_link().get_current()` returns the URL. Route after the UI signals ready.
- The `mornfold` scheme is registered by NSIS when `"plugins": { "deep-link": { "desktop": { "schemes": ["mornfold"] } } }` is set (verified in bundler `installer.nsi`: `WriteRegStr SHCTX "Software\Classes\mornfold\shell\open\command" "" "\"$INSTDIR\Mornfold.exe\" \"%1\""`).
- **Optional hardening (stub CLSID):** Microsoft's activation-options table lists "No COM / Stub CLSID" as supporting visuals, actions (protocol) and activation while the EXE is not running, but not inputs or in-process activation. If the smoke test shows toasts not persisting in or not activating from Notification Center, add `PKEY_AppUserModel_ToastActivatorCLSID` (`{9F4C2855-9F79-4B39-A8D0-E1D42DE1D5F3},26`, verified constant in windows 0.62) with a fixed app GUID to the Start Menu shortcut. The app can do this itself at startup (IShellLinkW + IPropertyStore), or an NSIS `NSIS_HOOK_POSTINSTALL` can.
- **Tier 2 fallback (only if Tier 1 fails on the owner's PC):** a full COM activator, mirroring the Windows App SDK for unpackaged apps (read in `microsoft/WindowsAppSDK` `AppNotificationUtility.cpp`): `HKCU\Software\Classes\CLSID\{GUID}\LocalServer32 = "\"<exe>\" ----AppNotificationActivated:"`, `HKCU\Software\Classes\AppUserModelId\{AUMID}` with `DisplayName`, `IconUri`, `CustomActivator={GUID}`, and `CoRegisterClassObject(.., CLSCTX_LOCAL_SERVER, REGCLS_MULTIPLEUSE | REGCLS_AGILE)`.

**Deep-link security:** any website or local process can open `mornfold://…`, and the single-instance channel also accepts argv from any local process. The parser in core:
- accepts only the `mornfold` scheme with an allowlisted host/path (`open/{home|settings|status}`, `act/{mute|update}`);
- caps the length;
- rejects unknown query keys.

Navigation needs no token. **Any `act/*` link requires `n=<notification id>`** that exists in `notifications` with `consumed_at IS NULL` and was created less than 7 days ago. It is consumed on use. A link from a web page cannot mute or trigger an update. This is consistent with ACT-05: a notification click never performs an external action.

**Sound (D-09/D-10):** Microsoft's custom-audio doc (updated 2025-07) lists supported sources as `ms-appx:///` and `ms-resource` only, and **absolute paths (C:/) as unsupported**. Third-party libraries (`windows-toasts`, BurntToast) nevertheless pass local paths and `file://` URIs. Behaviour for unpackaged apps is therefore **unverified**. Implement both strategies behind one setting:
- **A (try first):** `<audio src="file:///…/mornfold-ping.wav"/>`.
- **B (fallback):** `<audio silent="true"/>` plus `PlaySoundW(path, None, SND_FILENAME | SND_ASYNC | SND_NODEFAULT)`, played only if all of these hold:
  - the user's sound setting is on;
  - `ToastNotifier::Setting()` is `Enabled`;
  - `SHQueryUserNotificationState()` is `QUNS_ACCEPTS_NOTIFICATIONS`. This mapping to Win11 Do Not Disturb is unverified (LOW).

The smoke test decides the default. The global toggle and a per-source map (`sound.enabled`, `sound.per_source{}`) live in `settings`. With sound off, the toast is `silent="true"`. Ship a 16-bit PCM WAV under 1 s as a Tauri resource (`bundle.resources`), resolved via `app.path().resolve("sounds/mornfold-ping.wav", BaseDirectory::Resource)`.

### Pattern 6: Persistent scheduler with "once, in order" catch-up
```
jobs(id TEXT PK, kind TEXT, interval_sec INT, next_run_at INT /*unix ms*/, last_run_at INT, last_status TEXT,
     last_error TEXT, consecutive_failures INT DEFAULT 0, enabled INT DEFAULT 1, pausable INT DEFAULT 1)
job_runs(id TEXT PK, job_id TEXT, scheduled_for INT, started_at INT, finished_at INT, status TEXT, catch_up INT)

loop {
  select! { _ = sleep(30s) => {}, _ = resume_rx.recv() => {}, _ = kick_rx.recv() => {} , _ = shutdown => break }
  let now = clock.now();                       // wall clock (jiff Timestamp / SystemTime), NOT Instant
  let gap = now - last_tick;  if gap > 90s { mark_resume(now) }   // fallback when no power event
  let due = SELECT * FROM jobs WHERE enabled AND next_run_at <= now AND !(paused AND pausable) ORDER BY next_run_at, id;
  for job in due {                              // sequential => "sırayla"
      let catch_up = now - job.next_run_at > job.interval;   // >=1 period missed => coalesced single run
      run with timeout(job.timeout) in spawned task; JoinError::is_panic() => status=failed
      UPDATE jobs SET last_run_at=now, next_run_at=now + interval (+ jitter later), ... ;  // from NOW, never old+interval
      INSERT job_runs(..., catch_up)
  }
  last_tick = now; emit StatusChanged{last_check: heartbeat.last_run_at}
}
```
- Phase 1 jobs (Claude's discretion): `heartbeat` (every 5 min, writes `last_check_at` and proves the loop is alive; this drives "son kontrol: X dk önce") and `update_check` (every 6 h plus at startup, non-pausable). A third test job such as `demo_check` every 15 min, which logs and writes a `job_runs` row, makes catch-up visible in the status screen ("Kaçırılan kontroller: 2 iş, sırayla çalıştı").
- Power: `PowerRegisterSuspendResumeNotification(DEVICE_NOTIFY_CALLBACK, &DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS{ Callback, Context }, &mut handle)` (Win8+, Powrprof). The callback receives `PBT_APMSUSPEND` (4), `PBT_APMRESUMESUSPEND` (7) and `PBT_APMRESUMEAUTOMATIC` (18). It runs on a system thread, so do only `tx.try_send(PowerEvent::Resumed)` inside it. Unregister with `PowerUnregisterSuspendResumeNotification` on exit.
- Crash isolation: each job runs in its own `tokio::spawn` with `tokio::time::timeout`. Set `panic = "unwind"` in all profiles. A watchdog marks health `Error` (dimmed tray icon) if `heartbeat.last_run_at` is older than 3× its interval while the machine was awake.

### Pattern 7: Vault (envelope encryption) — D-15..D-18
```
vault_meta(key_version INT PK, kcv BLOB NOT NULL, created_at INT)     -- key check value
secrets(id TEXT PK, name TEXT UNIQUE NOT NULL, ciphertext BLOB NOT NULL, nonce BLOB NOT NULL,
        key_version INT NOT NULL, masked_hint TEXT NOT NULL, created_at INT, updated_at INT)
```
- Master key: 32 random bytes (`Key::<Aes256Gcm>::generate()`), stored with `keyring_core::Entry::new_with_modifiers("Mornfold", "vault-master-key-v1", {"persistence":"Local"})` then `set_secret(&key)`. The default persistence for this store is **Enterprise**, which roams with domain profiles. `Local` keeps the key on this machine.
- KCV: on first creation, store `AES-GCM(key, nonce, b"mornfold-kcv", aad="kcv")`. On startup:
  - If the key exists and the KCV decrypts, unlock.
  - If the key exists and the KCV fails, enter state `VaultKeyMismatch`.
  - If the key is missing but secrets exist, enter state `VaultKeyLost`. **Never silently generate a new key over existing ciphertext.** Show "Kasa anahtarı bulunamadı; kayıtlı anahtarları yeniden girmeniz gerekiyor" with a reset action.
- Encrypt: a fresh 96-bit random nonce per write, with AAD = `b"mornfold:secret:v1:" + id + ":" + key_version`. This binds ciphertext to its row and prevents row swapping.
- IPC is write-only:
  - `secrets_list() -> Vec<SecretMeta{id,name,masked_hint,updated_at}>`
  - `secrets_set(name, value) -> SecretMeta`
  - `secrets_delete(id)`
  - `secrets_test(kind, value) -> TestResult` (D-17; Phase 1 has no known providers, so it returns `NotTestable` for generic keys)
  - **No `get` command exists.** A test asserts that `bindings.ts` exports no command that returns a secret value.
- Masking (in core, stored as `masked_hint` at write time): keep a known prefix (`sk-ant-`, `sk-or-v1-`, `sk-or-`, `sk-`, `AIza`, `ghp_`, `xoxb-`), then `••••`, then the last 4 characters. If the value is shorter than 12 chars, show `••••` plus the last 2. Never reveal more than 4 trailing characters.
- Secrets in memory: wrap in `SecretString` immediately after deserialization and zeroize buffers. The UI clears its input state after submit, uses `type="password" autoComplete="off" spellCheck={false}`, and never caches the value in React Query or zustand.
- Linux dev `KeyProvider`: `keyring_core::mock::Store` for tests. For `tauri dev` on Linux, a dev-only file key under the app data dir with a loud warning, `#[cfg(not(windows))]`. The product is Windows-only, so this never ships.
- Accepted risk (document): DPAPI and Credential Manager protect against other users and offline disk theft, **not** against malware running as the same user.

### Pattern 8: i18n single source
`ui/src/i18n/tr.json` (nested keys) is loaded by i18next (`lng: "tr"`, `fallbackLng: "tr"`, `supportedLngs: ["tr"]`, `interpolation.escapeValue: false`, resources bundled). Rust includes the same file with `include_str!("../../../ui/src/i18n/tr.json")` into `core::i18n::Catalog` (flattened `HashMap<String,String>`, `{{var}}` interpolation) for tray, toast and tooltip strings. Tests:
- (Rust) every key the shell uses exists;
- (Vitest) every `t("…")` key in `ui/src` exists in `tr.json`;
- a heuristic test fails on raw letter text inside JSX.

Relative time ("3 dk önce") is computed in Rust for the tooltip using catalog templates, and in the UI with `Intl.RelativeTimeFormat("tr-TR")`.

### Pattern 9: Updater + CI/release
- `tauri.conf.json`:
  - `"bundle": { "targets": ["nsis"], "createUpdaterArtifacts": false, "windows": { "nsis": { "installMode": "currentUser", "languages": ["Turkish"], "installerHooks": "./windows/hooks.nsh" } }, "resources": { "resources/sounds/*": "sounds/" } }`
  - `"plugins": { "updater": { "pubkey": "<owner-provided>", "endpoints": ["https://github.com/Emsal66/Tempo/releases/latest/download/latest.json"], "windows": { "installMode": "passive" } }, "deep-link": { "desktop": { "schemes": ["mornfold"] } } }`
- `tauri.release.conf.json` = `{ "bundle": { "createUpdaterArtifacts": true } }`. Pass it only in the release workflow (`tauri build --config src-tauri/tauri.release.conf.json`). Push builds then never need the private key. (MEDIUM: [ASSUMED] that the CLI only demands `TAURI_SIGNING_PRIVATE_KEY` when updater artifacts are created. Verify in the first CI run.)
- Rust flow:
  - `update_check` job calls `app.updater()?.check().await?`.
  - If `Some(update)`, it calls `update.download(|_, _| {}, || {}).await?` (bytes are verified), keeps `(update, bytes)` in shell state, sets `TrayModel.update_ready`, and shows a toast "Mornfold v0.2.0 hazır — yeniden başlatınca kurulacak" with button "Yeniden başlat" (`mornfold://act/update?n=…`).
  - On user confirmation, call `update.install(bytes)`. **On Windows this launches NSIS `/UPDATE` and calls `std::process::exit(0)`**, so register `Builder::on_before_exit(|| { flush logs; close DB pool; single_instance::destroy })`.
- `ci.yml` (on push + pull_request; `permissions: contents: read`):
  - Job `linux`: ubuntu-24.04. apt deps, `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace` (with `SQLX_OFFLINE=true`), `cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app`, `pnpm biome ci`, `pnpm tsc --noEmit`, `pnpm vitest run`, and a bindings-drift check (`git diff --exit-code ui/src/bindings.ts`).
  - Job `windows`: `runs-on: windows-2025`. setup-node 24, pnpm action, rust stable, `Swatinem/rust-cache`, `pnpm install --frozen-lockfile`, `cargo test --workspace` (includes `#[cfg(windows)]` Credential Manager round-trip and toast-XML LoadXml tests), `pnpm tauri build --bundles nsis`, `actions/upload-artifact` of `target/release/bundle/nsis/*.exe`.
- `release.yml` (on `push: tags: ['v*.*.*']`; `permissions: contents: write`): verify the tag equals the app version, then run `tauri-apps/tauri-action@v1` with `tagName: ${{ github.ref_name }}`, `releaseName: 'Mornfold ${{ github.ref_name }}'`, `releaseDraft: true`, `prerelease: false`, `uploadUpdaterJson: true`, `args: --config src-tauri/tauri.release.conf.json --bundles nsis`. Env: `GITHUB_TOKEN`, `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`. Pin third-party actions by commit SHA. Input names come from the tauri-action README (`dev` branch): `uploadUpdaterJson` (default true) and `uploadWorkflowArtifacts`; `includeUpdaterJson` is not in v1.
- `releases/latest/download/latest.json` resolves only to the latest **published, non-prerelease** release. With `releaseDraft: true`, the owner publishes after a quick install check, which is a deliberate safety gate.
- Owner manual steps (D-05, `checkpoint:human-action`):
  1. On the owner's PC: `npx @tauri-apps/cli@2.12.3 signer generate -w %USERPROFILE%\.tauri\mornfold.key` with a strong password.
  2. Store the password in a password manager. Copy `mornfold.key` (+ `.pub`) to an offline USB.
  3. Add GitHub secrets `TAURI_SIGNING_PRIVATE_KEY` (file **content**) and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.
  4. Paste the **public** key into the chat so the executor can commit it in `tauri.conf.json`. Never generate the key in the cloud container (PITFALLS 7).

### Anti-Patterns to Avoid
- **Showing the main window in `setup` unconditionally, or leaving window-state's `VISIBLE` flag on.** Autostart then flashes a window, which violates D-07.
- **Using `tauri-winrt-notification` `on_activated` as the click path.** The in-process `Activated` event does not fire after the app has exited, and is reported not to fire from Notification Center either.
- **Building toast XML with `format!`.** Any future user/remote text (monitor titles, mail subjects) could inject `<action activationType="protocol" …>`. Use quick-xml's writer with escaping.
- **Recomputing `next_run_at = old_next + interval` after sleep.** This produces a burst of N runs. Always `now + interval`.
- **Driving schedules from `tokio::time::sleep(until 08:00)` or from webview timers.**
- **Granting the webview `updater:default`, `autostart:default`, `deep-link:default` or log `Webview` target.** Expose narrow typed commands instead. Never forward logs to the webview, since DevTools could show them.
- **Changing `identifier` after the first public build.** AUMID, `%LOCALAPPDATA%\<identifier>` (DB, WebView2 profile), Credential Manager naming and the updater all key off it. Pick once. Avoid a `.app` suffix, which the Tauri CLI warns about. Recommended: `com.mornfold.desktop`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Single-instance detection + argv forwarding | Named mutex + WM_COPYDATA | `tauri-plugin-single-instance` (+`deep-link` feature) | Edge cases on exit; already integrated with deep links |
| Protocol registration | Manual registry writes | `plugins.deep-link.desktop.schemes` (NSIS writes it, uninstall removes it) | Uninstall cleanup handled |
| Autostart | Run-key code | `tauri-plugin-autostart` (auto-launch 0.6 handles `StartupApproved`) | Task Manager "disabled" state honoured |
| Update download/verify/install | Custom downloader + minisign check | `tauri-plugin-updater` | Signature verification, NSIS `/UPDATE` args, relaunch |
| AEAD | Custom crypto, CBC+HMAC | `aes-gcm` 0.11 | Audited RustCrypto implementation |
| Credential Manager FFI | `CredWriteW` bindings | `keyring-core` + `windows-native-keyring-store` | Error mapping, persistence modifiers, mock store for tests |
| XML escaping for toasts | `replace('<', "&lt;")` | `quick-xml` writer | Attribute vs text escaping, control characters |
| Log rotation | Own file writer | `tauri-plugin-log` (`RotationStrategy::KeepSome`) | Standard |
| Typed IPC | Hand-written TS types | `tauri-specta` export | Drift-proof; CI checks the diff |
| Relative time in TS | Custom "dk önce" | `Intl.RelativeTimeFormat('tr-TR')` | Locale-correct |

**Key insight:** in this phase the hard parts are OS contracts (AUMID, protocol handlers, Run keys, power callbacks, NSIS update args) that are invisible from Linux. Every hand-rolled replacement is another thing that can only be debugged on the owner's PC.

## Common Pitfalls

### Pitfall 1: Toasts silently missing or showing "PowerShell"
**What goes wrong:** No toast appears, or it shows the wrong name or icon.
**Why:** The AUMID has no Start Menu shortcut (dev build, portable exe), or the code passes the identifier while running from `target\`.
**Avoid:** Use the plugin's rule (PowerShell AUMID when the exe dir ends with `\target\debug|release`). Test toasts only from the NSIS-installed build. The smoke test reads the toast header.
**Warning signs:** Toasts work on CI-built installs but not in dev, or the reverse.

### Pitfall 2: Unsigned build blocked outright by Smart App Control
**What goes wrong:** If Smart App Control is **On** on the owner's Win11, unsigned executables are blocked with no "Run anyway" (PITFALLS 7, HIGH). The NSIS artifact cannot be tested at all.
**Avoid:** Make the first smoke-checklist item "Ayarlar → Gizlilik ve güvenlik → Windows Güvenliği → Uygulama ve tarayıcı denetimi → Akıllı Uygulama Denetimi durumu". If it is On, signing (v2 DIST-01) becomes a blocker. Escalate to the owner immediately. SAC cannot be re-enabled once turned off without a reset, so the owner must decide knowingly.

### Pitfall 3: Custom toast sound plays the default sound or nothing
**Why:** Unpackaged apps cannot use `ms-appx`, and Microsoft lists absolute paths as unsupported.
**Avoid:** Strategy A/B switch (Pattern 5) plus a smoke-test item. Keep the WAV short (under 1 s), 16-bit PCM.

### Pitfall 4: Autostart flashes the window or opens it
**Avoid:** Main window `visible: false` in config. Show only when `--minimized` is absent. window-state flags without `VISIBLE`. The updater relaunch passes the original args through NSIS `/ARGS`, so a `--minimized` session restarts minimized. That is acceptable, but after a user-confirmed "Yeniden başlat ve güncelle", show the window on the next start: persist a `show_on_next_start` flag before `install()`.

### Pitfall 5: Missed-run burst or silent scheduler death after sleep
**Avoid:** Pattern 6 (coalesce, `now + interval`, sequential). Use the wall-clock gap fallback because power callbacks are reported as unreliable on some Modern Standby machines (MEDIUM). A watchdog dims the tray icon when the heartbeat is stale.

### Pitfall 6: Second launch or toast click doesn't bring the window to the front
**Why:** Windows foreground-lock rules. The first instance calls `set_focus()` without having received input.
**Avoid:** `unminimize()` → `show()` → `set_focus()`. If the smoke test shows only a taskbar flash, apply the documented workaround `set_always_on_top(true)` → `set_focus()` → `set_always_on_top(false)` (LOW confidence it is needed).

### Pitfall 7: `update.install()` kills the process without cleanup
**Why:** On Windows the plugin calls `std::process::exit(0)` after launching NSIS (verified in the updater 2.14.0 source). `RunEvent::Exit` never fires.
**Avoid:** `on_before_exit` hook: close the sqlx pool (WAL checkpoint), flush logs, `tauri_plugin_single_instance::destroy(app)`.

### Pitfall 8: Updater key lost or living only on the cloud box
**Avoid:** D-05 manual steps. A smoke-checklist item confirms the owner can locate both backups. The release workflow fails fast if the secret is missing.

### Pitfall 9: Secrets leaking into logs, the DB file or the WebView profile
**Avoid:** No `Debug` on secret types (`SecretString`). The log `format()` closure applies regex redaction for known prefixes and `Authorization:` headers as defence in depth. **Canary test:** set a secret `sk-ant-CANARY…` through the command path, then grep the log dir, the SQLite file (WAL too) and the generated bindings for the plaintext. Expect zero hits. Do not enable tauri's `tracing` feature, which instruments IPC.

### Pitfall 10: sqlx 0.9 `SqlSafeStr`
**What goes wrong:** All `query*()` functions take `impl SqlSafeStr` (only `&'static str` or `AssertSqlSafe(..)`), so dynamic SQL strings fail to compile (sqlx 0.9 CHANGELOG).
**Avoid:** Static SQL plus binds only, which is the right default anyway. Prefer `query!`/`query_as!` macros with committed `.sqlx/`.

### Pitfall 11: Version drift between `tauri` crate and `@tauri-apps/api` / CLI
**Avoid:** Pin `tauri = "2.12.4"`, `@tauri-apps/api` 2.12.4, `@tauri-apps/cli` 2.12.3. Add no JS plugin packages.

### Pitfall 12: Tray Click events double-firing
**Avoid:** Handle `MouseButtonState::Up` only. Use `show_menu_on_left_click(false)`; `menu_on_left_click` also exists and is the older name.

### Pitfall 13: Credential Manager entry roaming or ambiguous names
**Avoid:** `persistence=Local` modifier. Use a fixed service/user pair. The store's default target name is `user.service` with a `.` delimiter, so keep names free of `.` or set an explicit `target`. Do not touch the same entry from multiple threads concurrently (store README warning). Serialize vault key access behind one `OnceCell`.

## Code Examples

### Toast XML in core (escaped, testable on Linux)
```rust
// crates/mornfold-core/src/notify/toast_xml.rs — sketch; quick-xml Writer escapes text/attrs
pub struct ToastSpec { pub id: String, pub title: String, pub body: String, pub launch: DeepLink,
                       pub buttons: Vec<(String, DeepLink)> /* max 2 */, pub sound: Sound }
pub enum Sound { Custom(String /* file:/// URI */), Silent, Default }
pub fn toast_xml(s: &ToastSpec) -> String {
    use quick_xml::{Writer, events::{Event, BytesStart, BytesText, BytesEnd}};
    let mut w = Writer::new(Vec::new());
    let mut toast = BytesStart::new("toast");
    toast.push_attribute(("launch", s.launch.as_str()));
    toast.push_attribute(("activationType", "protocol"));
    w.write_event(Event::Start(toast)).unwrap();
    // <visual><binding template="ToastGeneric"><text>title</text><text>body</text></binding></visual>
    // <audio .../> per Sound; <actions><action content=.. activationType="protocol" arguments=../></actions>
    w.write_event(Event::End(BytesEnd::new("toast"))).unwrap();
    String::from_utf8(w.into_inner()).unwrap()
}
// tests: insta snapshot; adversarial title "</text><action ...>" must appear escaped.
```

### Showing it on Windows
```rust
// src-tauri/src/win/toast.rs (cfg(windows)) — windows 0.62 APIs verified in crate source
use windows::{core::HSTRING, Data::Xml::Dom::XmlDocument, UI::Notifications::{ToastNotification, ToastNotificationManager}};
pub fn show(aumid: &str, xml: &str) -> windows::core::Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(&HSTRING::from(xml))?;
    let toast = ToastNotification::CreateToastNotification(&doc)?;
    ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(aumid))?.Show(&toast)
}
```

### Power resume callback
```rust
// src-tauri/src/win/power.rs (cfg(windows)) — signatures from windows 0.62.2
use windows::Win32::{Foundation::HANDLE, System::Power::*, UI::WindowsAndMessaging::*};
unsafe extern "system" fn cb(ctx: *const core::ffi::c_void, kind: u32, _s: *const core::ffi::c_void) -> u32 {
    let tx = unsafe { &*(ctx as *const tokio::sync::mpsc::UnboundedSender<PowerEvent>) };
    match kind { PBT_APMSUSPEND => { let _ = tx.send(PowerEvent::Suspend); }
                 PBT_APMRESUMEAUTOMATIC | PBT_APMRESUMESUSPEND => { let _ = tx.send(PowerEvent::Resumed); }
                 _ => {} }
    0
}
pub fn register(tx: tokio::sync::mpsc::UnboundedSender<PowerEvent>) -> windows::core::Result<*mut core::ffi::c_void> {
    let ctx = Box::into_raw(Box::new(tx)) as *mut core::ffi::c_void;   // freed on unregister
    let mut params = DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS { Callback: Some(cb), Context: ctx };
    let mut handle = core::ptr::null_mut();
    let err = unsafe { PowerRegisterSuspendResumeNotification(DEVICE_NOTIFY_CALLBACK,
                  HANDLE(&mut params as *mut _ as _), &mut handle) };
    err.ok()?; Ok(handle)
}
// NOTE: DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS must outlive the registration -> keep it boxed too.
```

### Vault encrypt/decrypt (aes-gcm 0.11 API per crate README)
```rust
use aes_gcm::{aead::{Aead, Generate, Key, KeyInit, Payload}, Aes256Gcm, Nonce};
fn seal(key: &Key<Aes256Gcm>, id: &str, kv: u32, plain: &[u8]) -> Result<(Vec<u8>, Vec<u8>), aes_gcm::Error> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::generate();                       // 96-bit, unique per write
    let aad = format!("mornfold:secret:v1:{id}:{kv}");
    let ct = cipher.encrypt(&nonce, Payload { msg: plain, aad: aad.as_bytes() })?;
    Ok((ct, nonce.to_vec()))
}
// Verify exact 0.11 type paths at compile time (aead 0.6 moved generation to the `Generate` trait).
```

### Credential Manager master key
```rust
// src-tauri/src/win/keyring.rs (cfg(windows)) — API from keyring-core 1.0.1 / windows-native-keyring-store 1.1.0
keyring_core::set_default_store(windows_native_keyring_store::Store::new()?);
let entry = keyring_core::Entry::new_with_modifiers("Mornfold", "vault-master-key-v1",
                 &std::collections::HashMap::from([("persistence", "Local")]))?;
match entry.get_secret() {
    Ok(bytes) => /* 32 bytes → MasterKey */,
    Err(keyring_core::Error::NoEntry) => { /* only if no secrets rows exist: generate + set_secret */ }
    Err(e) => /* VaultUnavailable */,
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `keyring` 3.x with compile-time store features | `keyring-core` 1.x + per-OS store crates, `set_default_store` at startup | keyring v4 split (2025-2026) | Explicit store setup; mock store for tests |
| sqlx 0.8 `query(&String)` | sqlx 0.9 `impl SqlSafeStr` | sqlx 0.9.0 (2026-05) | Dynamic SQL needs `AssertSqlSafe` |
| aes-gcm 0.10 (`OsRng` + `generate_nonce`) | aes-gcm 0.11 / aead 0.6 (`Generate` trait, `getrandom` default feature) | 2026-06 | Older examples won't compile |
| Win32 toast docs (C++ WRL "send-local-toast-desktop") | Windows App SDK `AppNotificationManager` docs; legacy pages 404 | 2025-2026 | Unpackaged COM details now best read from the WindowsAppSDK source |
| tauri-action `includeUpdaterJson` (v0) | `uploadUpdaterJson`, `uploadWorkflowArtifacts` (v1) | action v1 | Use v1 input names |

**Deprecated/outdated:** `TrayIconBuilder::menu_on_left_click` (use `show_menu_on_left_click`); `tauri-plugin-notification`'s `windows7-compat` notify.

## Runtime State Inventory

Not a rename/refactor phase, but one item has rename-like permanence: **`identifier`**. Once v0.1 is installed, these live outside git:
- the AUMID on `Mornfold.lnk`;
- `HKCU\…\Run\Mornfold`;
- `HKCU\Software\Classes\mornfold`;
- `%LOCALAPPDATA%\<identifier>\` (DB, logs, WebView2 profile);
- the Credential Manager generic credential;
- installed updater pubkey trust.

Decide `identifier`, `productName` ("Mornfold") and the credential service/user names in Plan 1 and treat them as frozen.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Toast `activationType="protocol"` (body + buttons) launches the registered `mornfold://` handler for an unpackaged NSIS app both when running and when closed, without a COM activator | Pattern 5 | D-08 cold start fails → implement stub CLSID, then Tier 2 COM activator |
| A2 | `<audio src="file:///…wav">` works for unpackaged toasts on current Win11 (Microsoft docs say absolute paths are unsupported) | Pattern 5 | Strategy B becomes default (silent toast + `PlaySoundW`) |
| A3 | `SHQueryUserNotificationState` reflects Win11 Do Not Disturb/Focus | Pattern 5 | Strategy B could play sound while DND is on; mitigate by user toggle |
| A4 | Tauri CLI requires the signing private key only when `createUpdaterArtifacts` is true | Pattern 9 | Push CI fails without secrets → set secrets before the first push, or keep the override approach |
| A5 | `PowerRegisterSuspendResumeNotification` callbacks arrive reliably on the owner's (possibly Modern Standby) machine | Pattern 6 | Mitigated by the wall-clock-gap fallback |
| A6 | Credential Manager writes work on GitHub `windows-2025` runners for `#[cfg(windows)]` tests | Validation | Mark those tests `#[ignore]` on CI and rely on the smoke test |
| A7 | `tracing` with feature `log` routes events into tauri-plugin-log when no tracing subscriber is installed | Standard Stack | Logs missing → add `tracing-log`/a subscriber layer |
| A8 | Pre-rendered dot badge is acceptable for D-12 "okunmamış sayısı için rozet" (count shown in tooltip) | Pattern 3 | Owner may want a numeral → render 1–9/9+ variants |
| A9 | "Yarına kadar" means next local midnight | Pattern 4 | Owner may expect morning (e.g., 08:00) → trivial constant change |
| A10 | `Swatinem/rust-cache`, `pnpm/action-setup`, `actions/setup-node`, `dtolnay/rust-toolchain` are the actions to pin | CI | Low; standard community actions |
| A11 | quick-xml/ulid/url/tempfile are legitimate (not scanned by slopcheck, long-established) | Audit | Very low |

## Open Questions

1. **Does Smart App Control block the unsigned installer on the owner's PC?**
   - Known: SAC blocks unsigned code outright when On.
   - Unclear: the owner's SAC state.
   - Recommendation: the first smoke-checklist item, asked before Plan 1 ships its artifact.
2. **"Yarına kadar" semantics (D-13).** Recommend next local midnight, with the exact resume time shown in the tray (A9). Planner may confirm with the owner in a checkpoint.
3. **Which toast sound strategy becomes the default?** Decided by smoke test item S-7 (A2/A3).
4. **Is the stub ToastActivatorCLSID needed for Notification Center persistence?** Ship without it. Add it if smoke item S-6 (click from Notification Center after the app has exited) fails.
5. **Version source of truth.** Recommend that `src-tauri/Cargo.toml` `package.version` is the single source (omit `version` in `tauri.conf.json`). The release workflow asserts `v${version} == tag`. `package.json` version is ignored.

## Environment Availability

| Dependency | Required By | Available (container) | Version | Fallback |
|------------|------------|-----------|---------|----------|
| rustc/cargo/rustup | all Rust | ✓ | 1.97.0 (stable 1.99.1 exists; rustup can fetch, static.rust-lang.org reachable) | Pin `rust-toolchain.toml` to 1.99.1 |
| `x86_64-pc-windows-msvc` target | cargo xwin check | ✗ | — | `rustup target add` (Wave 0) |
| cargo-xwin | Windows compile check | ✗ | — | `cargo install --locked cargo-xwin` (MSVC CRT download host reachable: aka.ms → download.visualstudio.microsoft.com 200) |
| clang-cl / lld-link / llvm-rc / llvm-lib | cargo-xwin, tauri-winres | ✓ | LLVM 18 | — |
| sqlx-cli | `cargo sqlx prepare` | ✗ | — | `cargo install --locked sqlx-cli --no-default-features --features sqlite,rustls` |
| cargo-nextest | faster tests | ✗ | — | plain `cargo test` |
| libwebkit2gtk-4.1-dev + Tauri Linux deps | compiling/testing `src-tauri` on Linux, bindings export test | ✗ (apt reachable; candidate 2.52.6) | — | `apt-get install` (Wave 0) |
| Node.js | UI toolchain | ✓ | 22.22.0 (stack says 24; all packages' engines accept 22.12+) | CI uses Node 24 |
| pnpm | UI toolchain | ✓ (10.28.0) | stack: 12.14.0 | `corepack prepare pnpm@12.14.0 --activate` + `packageManager` field |
| Playwright Chromium | UI screenshots | ✗ | — | `pnpm exec playwright install chromium` (CDN reachability unverified; optional in Phase 1) |
| python3 (stdlib `wave`, `math`) | generating the original ping WAV | ✓ | 3.13.16 | — |
| gh CLI | push, read check-runs | ✓ 2.89.0 | `repos/Emsal66/Tempo/commits/<sha>/check-runs` works; **`/actions/*` endpoints are blocked by the proxy (403)**, so artifacts cannot be downloaded from the container | Owner downloads the artifact from the GitHub UI |
| Windows 11 machine | toasts, tray, CredMan, autostart, sleep, updater | ✗ (owner's PC only) | — | Smoke-test checklist |
| docker | — | ✓ 29.8.2 | not needed | — |

**Missing with no fallback:** none blocking. The Windows-runtime behaviours are covered by CI plus owner UAT.
**Missing with fallback:** cargo-xwin, sqlx-cli, Windows target, webkit deps, pnpm 12, Playwright browsers. All are Wave 0 installs. Recommend an idempotent `scripts/dev-setup.sh`. Wiring it into a SessionStart hook changes `.claude/settings.json` and needs explicit owner approval.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework (Rust) | `cargo test` (built-in) + `insta` 1.49 snapshots; `keyring_core::mock::Store`; in-memory/tempfile SQLite; injected `Clock` |
| Framework (TS) | Vitest 5.0.5 + jsdom 30 + @testing-library/react 16.3.3 + `@tauri-apps/api/mocks` |
| Config files | none yet → Wave 0: `ui/vitest.config.ts`, `biome.json`, `rust-toolchain.toml`, `.github/workflows/ci.yml` |
| Quick run command | `cargo test -p mornfold-core && pnpm -C ui vitest run` |
| Full suite command | `cargo fmt --check && cargo clippy --workspace -- -D warnings && SQLX_OFFLINE=true cargo test --workspace && cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app && pnpm -C ui biome ci && pnpm -C ui tsc --noEmit && pnpm -C ui vitest run` |
| Windows gate | CI `windows-2025` job green (poll `gh api repos/Emsal66/Tempo/commits/<sha>/check-runs`) |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SHELL-01 | NSIS artifact produced on push | CI | windows job `pnpm tauri build --bundles nsis` + upload-artifact; check-runs = success | ❌ Wave 0 (`ci.yml`) |
| SHELL-01 | Installs on Win11 | manual | Smoke S-1/S-2 | ❌ `docs/smoke-test-phase1.md` |
| SHELL-02 | Tray menu model per state (normal/paused/update-ready) with Turkish labels | unit (insta) | `cargo test -p mornfold-core tray_model` | ❌ Wave 0 |
| SHELL-02 | Pause durations resolve correctly (1h, midnight, indefinite) and auto-resume | unit | `cargo test -p mornfold-core pause` | ❌ |
| SHELL-02 | Close → tray, Çık exits | manual | Smoke S-3 | ❌ |
| SHELL-03 | `--minimized` arg parsing → start hidden; autostart command toggles | unit + manual | `cargo test -p mornfold-core startup_args`; Smoke S-4 (reboot) | ❌ |
| SHELL-04 | Second launch focuses | manual | Smoke S-5 | ❌ |
| SHELL-05 | Toast XML: protocol launch, ≤2 buttons, escaping, audio modes | unit (insta + adversarial) | `cargo test -p mornfold-core toast_xml` | ❌ |
| SHELL-05 | XML loads in WinRT `XmlDocument` | Windows CI unit | `cargo test -p mornfold-app toast_loadxml` (`#[cfg(windows)]` test; executes only in the windows CI job) | ❌ |
| SHELL-05 | Deep-link parser allowlist + `act/*` token rules | unit | `cargo test -p mornfold-core deeplink` | ❌ |
| SHELL-05 | Toast shows "Mornfold"+icon; click warm/cold/Notification Center opens screen; sound | manual | Smoke S-6, S-7 | ❌ |
| SHELL-06 | Catch-up: after a simulated 8 h gap each overdue job runs exactly once, in `next_run_at` order, `next_run_at = now+interval`, `catch_up=1` | unit (fake clock + temp DB) | `cargo test -p mornfold-core scheduler::catch_up` | ❌ |
| SHELL-06 | Wall-clock gap detector fires resume; panicking job → failed status, loop survives | unit | `cargo test -p mornfold-core scheduler` | ❌ |
| SHELL-06 | "son kontrol: X dk önce" formatting (0, 1, 59, 60+ min) | unit | `cargo test -p mornfold-core relative_time` | ❌ |
| SHELL-06 | Real sleep/resume on PC | manual | Smoke S-8 | ❌ |
| SHELL-07 | Vault round-trip, AAD binding (row swap fails), wrong key fails, KCV mismatch/lost-key states | unit | `cargo test -p mornfold-core vault` | ❌ |
| SHELL-07 | Masking rules | unit | `cargo test -p mornfold-core masking` | ❌ |
| SHELL-07 | No secret-returning command in bindings | unit (TS) | `pnpm -C ui vitest run bindings.contract` | ❌ |
| SHELL-07 | Canary secret absent from logs/DB/WAL | integration | `cargo test -p mornfold-core canary_no_plaintext` (+ shell log formatter test) | ❌ |
| SHELL-07 | Credential Manager round-trip | Windows CI | cfg(windows) test in `mornfold-app` | ❌ |
| SHELL-07 | Persists across restart on real PC; not visible in UI | manual | Smoke S-9 | ❌ |
| SHELL-08 | All `t()` keys exist; no raw JSX text; Rust shell keys exist | unit | `pnpm -C ui vitest run i18n`; `cargo test -p mornfold-core i18n` | ❌ |
| SHELL-09 | Config contains pubkey + endpoint; release workflow passes `--config` override; tag==version check script | unit/script | `cargo test -p mornfold-core release_config` or `node scripts/check-release-config.mjs` | ❌ |
| SHELL-09 | v0.1.0 → v0.2.0 in-app update | manual | Smoke S-10 | ❌ |

### Owner smoke-test checklist (input for `docs/smoke-test-phase1.md`)
- **S-0.** Smart App Control state. If it is On, stop and escalate.
- **S-1.** Download the CI artifact. Install (SmartScreen: "Ek bilgi → Yine de çalıştır"). The installer UI is Turkish.
- **S-2.** Window opens with Mica and a custom title bar. All text is Turkish.
- **S-3.** X hides to tray silently. Left-click toggles. Right-click menu shows Aç / Tümünü duraklat ▸ / Çık. Çık exits (Task Manager shows no process).
- **S-4.** Enable "Windows açılışında başlat". Reboot. The app is in the tray, no window. Disable it, reboot, and confirm it does not start.
- **S-5.** Launch from the Start menu again. The existing window comes to the front and there is one process.
- **S-6.** Click "Test bildirimi gönder". The toast header shows "Mornfold" and the icon. Then:
  - (a) click the body while the app runs → Settings opens;
  - (b) Çık, then click the toast in Notification Center → the app starts on Settings;
  - (c) "Sessize al" button → sound is disabled for 1 h;
  - (d) open `mornfold://act/mute` from a browser → refused or ignored.
- **S-7.** Sound: is the Mornfold ping heard (strategy A)? If not, switch to B and retest. With Do Not Disturb on, no sound plays.
- **S-8.** Note "son kontrol". Sleep the PC for over 20 min, then wake it. Within about 30 s the tooltip and status screen show a fresh check, and the job-runs list shows each overdue job once, in order, marked catch-up.
- **S-9.** Add key "test" = `sk-test-…a1b2`. It shows as `sk-••••a1b2`. Restart, and it still shows. Credential Manager (Windows Credentials → Generic) has exactly one Mornfold entry. Search the log files for the key and find nothing.
- **S-10.** Install the v0.1.0 release. Publish v0.2.0. Within about 6 h, or via "Güncellemeleri denetle", the tray shows "Yeniden başlat ve güncelle" and a toast appears. Accept: the app restarts as v0.2.0 (status screen) and keys/settings are kept.
- **S-11.** Updater key backups: the password-manager entry and the USB file both exist.

### Sampling Rate
- **Per task commit:** `cargo test -p mornfold-core` (plus `pnpm -C ui vitest run` when UI is touched)
- **Per wave merge:** full suite command plus push, and Windows CI green via check-runs
- **Phase gate:** full suite green, Windows CI green on the final commit, and the owner signs off S-0..S-11 before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `scripts/dev-setup.sh` covering apt deps, Windows target, cargo-xwin, sqlx-cli, pnpm 12
- [ ] Cargo workspace + `rust-toolchain.toml` + `crates/mornfold-core` skeleton with a `Clock` trait and test helpers (temp DB, mock keyring)
- [ ] `ui/` Vite + React + Vitest + Biome config; `ui/src/i18n/tr.json`
- [ ] `.github/workflows/ci.yml` (linux + windows jobs) producing the NSIS artifact
- [ ] `docs/smoke-test-phase1.md` skeleton (S-0..S-11)

## Security Domain

Security enforcement is enabled (ASVS L1).

### Applicable ASVS Categories
| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Local single-user app; Windows login is the trust boundary |
| V3 Session Management | no | — |
| V4 Access Control | yes | Tauri capabilities: the webview gets only window-chrome permissions plus typed app commands. No plugin permissions for updater/autostart/deep-link/log. Write-only secret IPC. Deep-link `act/*` requires a single-use notification token. |
| V5 Input Validation | yes | serde-typed command args with length limits; `url`-based deep-link allowlist; quick-xml escaping for toast XML; secret name charset/length validation |
| V6 Cryptography | yes | `aes-gcm` 0.11 AES-256-GCM, OS RNG nonces/keys, AAD binding, key in Credential Manager (DPAPI), zeroize. No custom crypto. |
| V7 Error Handling & Logging | yes | Redacting log formatter, `SecretString`, no logs forwarded to the webview, panic hook logs without payloads, rotating local logs only |
| V8 Data Protection | yes | No plaintext secrets in SQLite/WAL/logs/WebView storage (canary test); DB in `%LOCALAPPDATA%` (non-roaming) |
| V10 Malicious Code / Supply chain | yes | Lockfiles committed, slopcheck audit, pinned `=` RC crates, CI actions pinned by SHA, signed updates (minisign, mandatory) over HTTPS |
| V14 Configuration | yes | Strict CSP in `app.security.csp`; devtools not enabled in release; `prevent-default` in release; CI `permissions: contents: read`, release job only `contents: write`; `pull_request` (never `pull_request_target`) |

### Known Threat Patterns for this stack
| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Malicious web page or local process opens `mornfold://act/...` | Tampering / Elevation | Allowlist parser; actions need an unconsumed notification id; navigation-only otherwise |
| Second-instance argv injection via single-instance channel | Tampering | Same parser; ignore unknown flags; never pass argv to a shell |
| Toast XML injection (future remote text) | Tampering / Spoofing | quick-xml writer escaping + adversarial snapshot tests |
| Secret exfiltration through UI/DevTools | Info disclosure | No `get` command; masked hint computed in core; release builds without devtools; no Webview log target |
| Secrets in logs/crash output | Info disclosure | `SecretString`, redaction regex, canary test |
| Ciphertext row swap / replay | Tampering | AAD = secret id + key version |
| Master key lost or replaced | Denial of service / Integrity | KCV check; explicit `VaultKeyLost` state; never silently regenerate over existing rows |
| Malicious update / MITM | Tampering | Mandatory minisign verification, HTTPS endpoint, private key offline + GH secret only |
| CI secret theft via PRs or compromised action | Info disclosure | Secrets only in the tag-triggered release job; SHA-pinned actions; least-privilege `permissions` |
| Same-user malware reads Credential Manager | Info disclosure | **Accepted risk**, documented (DPAPI scope) |
| Unsigned binary tampering on distribution | Tampering | Accepted for owner-only v1 (unsigned NSIS); signing is v2 DIST-01; SmartScreen/SAC behaviour documented |

## Sources

### Primary (HIGH confidence)
- crates.io API and **downloaded crate sources** (read in this session): tauri 2.12.4 (`tray/mod.rs` TrayIconEvent/builder; RunEvent Suspended/Resumed mobile-only; `specta` feature), tauri-utils 2.10.2 (`windowEffects` requires transparent, `shadow`, `noRedirectionBitmap`), tauri-bundler 2.10.3 (`installer.nsi`/`utils.nsh`: `SetLnkAppUserModelId` with BUNDLEID, deep-link registry writes, Run-key cleanup, NSIS hooks, Turkish.nsh), tauri-winrt-notification 0.8.1 (`lib.rs`: audio `ms-winsoundevent` only, no launch/activationType), tauri-plugin-notification 2.7.0 (`desktop.rs` installed-app AUMID rule), tauri-plugin-single-instance 2.5.4, tauri-plugin-deep-link 2.6.2, tauri-plugin-autostart 2.7.0 + auto-launch 0.6.0 (`StartupApproved`), tauri-plugin-updater 2.14.0 (`download`/`install`/`on_before_exit`/`process::exit`), tauri-plugin-window-state 2.5.0 (`VISIBLE` flag default), tauri-plugin-log 2.10.1, tauri-specta 2.0.0-rc.25, keyring-core 1.0.1 (`mock` store, `Entry` API), windows-native-keyring-store 1.1.0 (persistence modifier, default Enterprise, threading warning), sqlx 0.9.0 (CHANGELOG `SqlSafeStr`, features, MSRV 1.94), aes-gcm 0.11.1 (README usage, features), croner 4.0.1 (jiff feature), windows 0.62.2 (`PowerRegisterSuspendResumeNotification`, `PBT_*`, `SHQueryUserNotificationState`, `PlaySoundW`, `PKEY_AppUserModel_ToastActivatorCLSID`, `XmlDocument::LoadXml`, `CreateToastNotifierWithId`)
- https://learn.microsoft.com/en-us/windows/win32/api/powerbase/nf-powerbase-powerregistersuspendresumenotification
- https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/custom-audio-on-toasts (→ app-notifications-custom-audio; supported sources ms-appx/ms-resource; absolute paths unsupported)
- https://github.com/MicrosoftDocs/winapps-winrt-api/blob/docs/microsoft.windows.appnotifications.builder/appnotificationbuilder_setaudiouri_1722288022.md
- https://learn.microsoft.com/en-us/previous-versions/windows/desktop/win32_tile_badge_notif/respond-to-toast-activations (COM activator, shortcut AUMID + CLSID)
- microsoft/WindowsAppSDK `dev/AppNotifications/AppNotificationUtility.cpp` and `AppNotificationManager.cpp` (unpackaged registry layout: AppUserModelId DisplayName/IconUri/CustomActivator, CLSID LocalServer32, `----AppNotificationActivated:`)
- https://v2.tauri.app/plugin/updater/ (signer generate, env vars, config keys, latest.json, capability names)
- https://v2.tauri.app/distribute/windows-installer/ (cross-compile caveats, NSIS hooks, install modes, WebView2 modes)
- tauri-action README (`dev` branch): v1 inputs and outputs
- npm registry (versions, engines, postinstall), static.rust-lang.org (Rust 1.99.1), slopcheck scan results

### Secondary (MEDIUM confidence)
- Microsoft "Activating toast notifications from desktop apps" options table (via search snippets; the page now 404s): "No COM / Stub CLSID" supports visuals + actions, all options activatable while the EXE isn't running
- windows-toasts (Python) `register_hkey_aumid.py` and `toast_audio` source: HKCU AppUserModelId DisplayName/IconUri; `file://` audio URIs used in practice
- BurntToast `New-BTAudio` (local path audio)
- Microsoft archived blog comments on Win32 toast activation (in-process `Activated` not firing from Action Center)

### Tertiary (LOW confidence)
- Behaviour of `file:///` toast audio for unpackaged apps on Win11 (conflicting sources): smoke-test
- `SHQueryUserNotificationState` vs Win11 Do Not Disturb: smoke-test
- Foreground-lock workaround need: smoke-test

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH. Every version was checked on the registry today, and the key APIs were read in the crate sources.
- Architecture: MEDIUM-HIGH. The patterns are standard. The protocol-activation choice for D-08 is MEDIUM until the smoke test runs.
- Pitfalls: MEDIUM-HIGH. Most are verified in source (updater exit, window-state VISIBLE, tray Click Up/Down, sqlx SqlSafeStr). Windows runtime behaviours are flagged.

**Research date:** 2026-10-18
**Valid until:** 2026-11-17 (Tauri plugins release often; re-check versions if planning slips past a month)
