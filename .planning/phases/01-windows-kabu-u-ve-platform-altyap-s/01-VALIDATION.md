---
phase: 1
slug: windows-kabu-u-ve-platform-altyap-s
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-10-18
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust: `cargo test` + `insta` 1.49 snapshots, `keyring_core::mock::Store`, temp SQLite, injected `Clock`. TS: Vitest 5.0.5 + jsdom + @testing-library/react 16.3.3 + `@tauri-apps/api/mocks` |
| **Config file** | none yet — Wave 0 installs `rust-toolchain.toml`, `ui/vitest.config.ts`, `biome.json`, `.github/workflows/ci.yml` |
| **Quick run command** | `cargo test -p mornfold-core && pnpm -C ui vitest run` |
| **Full suite command** | `cargo fmt --check && cargo clippy --workspace -- -D warnings && SQLX_OFFLINE=true cargo test --workspace && cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app && pnpm -C ui biome ci && pnpm -C ui tsc --noEmit && pnpm -C ui vitest run` |
| **Windows gate** | CI `windows-2025` job green on the commit (`gh api repos/Emsal66/Tempo/commits/<sha>/check-runs`) |
| **Estimated runtime** | ~60–120 seconds quick (warm cache); full suite several minutes (xwin) |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p mornfold-core` (plus `pnpm -C ui vitest run` when UI is touched)
- **After every plan wave:** Run the full suite command, push, and confirm Windows CI green via check-runs
- **Before `/gsd:verify-work`:** Full suite green, Windows CI green on final commit, owner signs off smoke items S-0..S-11
- **Max feedback latency:** 120 seconds (quick run)

---

## Per-Task Verification Map

Task-level map. Task ID = `<plan>-T<task number>`; see each PLAN.md for full acceptance criteria.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 01-01-T2 | 01-01 | 1 | (Wave 0 env) | T-01-01-04 | settings.json untouched | script | `bash scripts/dev-setup.sh --check` | ❌ W0 | ⬜ pending |
| 01-01-T3 | 01-01 | 1 | SHELL-06, SHELL-07 | T-01-01-01/02 | static SQL only; MasterKey Debug redacted | unit | `cargo test -p mornfold-core --test foundation` | ❌ W0 | ⬜ pending |
| 01-02-T3 | 01-02 | 1 | SHELL-08 | — | N/A | unit (Vitest) | `pnpm -C ui exec vitest run src/test/i18n.test.ts` | ❌ W0 | ⬜ pending |
| 01-03-T3 | 01-03 | 1 | SHELL-05 (D-09), SHELL-02 (D-12) | T-01-03-02 | reproducible assets | script | `python3 -I scripts/gen-ping.py --check src-tauri/resources/sounds/mornfold-ping.wav` | ❌ W0 | ⬜ pending |
| 01-04-T3 | 01-04 | 1 | SHELL-09 (D-05) | T-01-04-01 | no private key in repo | script | `base64 -d docs/release/updater-pubkey.txt \| head -1 \| grep -q "minisign public key"` | ❌ | ⬜ pending |
| 01-05-T1 | 01-05 | 2 | SHELL-02 (D-13) | — | N/A | unit | `cargo test -p mornfold-core --test pause` | ❌ | ⬜ pending |
| 01-05-T2 | 01-05 | 2 | SHELL-06 | T-01-05-01/02 | panicking job → failed, loop survives; no resume burst | unit (fake clock + temp DB) | `cargo test -p mornfold-core --test scheduler` | ❌ | ⬜ pending |
| 01-06-T1 | 01-06 | 2 | SHELL-07 (D-15) | T-01-06-04 | masked hint ≤4 trailing chars; log redaction | unit | `cargo test -p mornfold-core --test masking` | ❌ | ⬜ pending |
| 01-06-T2 | 01-06 | 2 | SHELL-07 (D-16..D-18) | T-01-06-01..06 | AAD row binding; KCV; no silent key regen; canary absent from DB/WAL/logs | unit + integration | `cargo test -p mornfold-core --test vault --test canary` | ❌ | ⬜ pending |
| 01-07-T1 | 01-07 | 2 | SHELL-05 (D-08) | T-01-07-01..03 | quick-xml escaping; deep-link allowlist | unit (insta + adversarial) | `cargo test -p mornfold-core --test deeplink --test toast_xml` | ❌ | ⬜ pending |
| 01-07-T2 | 01-07 | 2 | SHELL-05 (D-09, D-10) | T-01-07-01/04 | act links need single-use token < 7 days | unit | `cargo test -p mornfold-core --test notifications` | ❌ | ⬜ pending |
| 01-08-T1 | 01-08 | 2 | SHELL-06, SHELL-08 | T-01-08-01 | N/A | unit | `cargo test -p mornfold-core --test i18n --test relative_time` | ❌ | ⬜ pending |
| 01-08-T2 | 01-08 | 2 | SHELL-02 (D-11..D-13) | T-01-08-03 | unknown menu ids ignored | unit (insta) | `cargo test -p mornfold-core --test tray_model` | ❌ | ⬜ pending |
| 01-09-T1 | 01-09 | 2 | SHELL-03, SHELL-04, SHELL-01 | T-01-09-01/02/03 | argv allowlist; minimal capabilities; strict CSP | unit + config assert | `cargo test -p mornfold-core --test startup_args` | ❌ | ⬜ pending |
| 01-09-T2 | 01-09 | 2 | SHELL-02 (D-14), SHELL-04 | T-01-09-04 | no Webview log target | build + bindings drift | `cargo test -p mornfold-app --test bindings && cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app` | ❌ | ⬜ pending |
| 01-10-T2 | 01-10 | 3 | SHELL-01 | T-01-10-01..04 | no secrets in push CI; SHA-pinned actions | CI | check-runs `linux` + `windows` = success (NSIS artifact + startup smoke) | ❌ W0 | ⬜ pending |
| 01-11-T2 | 01-11 | 3 | SHELL-03, SHELL-08 | T-01-11-01 | N/A | component (Vitest) | `pnpm -C ui exec vitest run` | ❌ | ⬜ pending |
| 01-12-T1 | 01-12 | 4 | SHELL-07 | T-01-12-02/04 | redacting log formatter; CredMan persistence=Local | unit + Windows CI | `cargo test -p mornfold-app logging_` (+ `credman_roundtrip` on windows-2025) | ❌ | ⬜ pending |
| 01-12-T2 | 01-12 | 4 | SHELL-07 (D-15..D-17) | T-01-12-01/03 | no secret-returning command; value never cached | unit (Vitest) | `pnpm -C ui exec vitest run src/test/keys.test.tsx src/test/bindings.contract.test.ts` | ❌ | ⬜ pending |
| 01-13-T1 | 01-13 | 5 | SHELL-06 | T-01-13-01/02 | power callback send-only; unregister on drop | build (xwin) | `cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app` | ❌ | ⬜ pending |
| 01-13-T3 | 01-13 | 5 | SHELL-02, SHELL-06 | — | N/A | component (Vitest) | `pnpm -C ui exec vitest run src/test/status.test.tsx` | ❌ | ⬜ pending |
| 01-14-T1 | 01-14 | 6 | SHELL-05 | T-01-14-02/03 | adversarial XML loads with exactly 2 actions in WinRT | Windows CI unit | `cargo test -p mornfold-app --test toast_loadxml` (windows-2025) | ❌ | ⬜ pending |
| 01-14-T3 | 01-14 | 6 | SHELL-05 (D-08..D-10) | T-01-14-01 | N/A | component (Vitest) | `pnpm -C ui exec vitest run src/test/notifications.test.tsx src/test/navigate.test.tsx` | ❌ | ⬜ pending |
| 01-15-T1 | 01-15 | 7 | SHELL-09 | T-01-15-01/02/04 | pubkey pinned; secrets only in tag job; tag==version | script | `node scripts/check-release-config.mjs` | ❌ | ⬜ pending |
| 01-15-T3 | 01-15 | 7 | SHELL-09 (D-02) | T-01-15-03 | install only on user action | component (Vitest) | `pnpm -C ui exec vitest run src/test/updates.test.tsx` | ❌ | ⬜ pending |
| 01-16-T2 | 01-16 | 8 | SHELL-01..SHELL-08 | T-01-16-04 | act link from browser refused (S-6d) | manual | Smoke S-0..S-9 | ❌ | ⬜ pending |
| 01-17-T2 | 01-17 | 9 | SHELL-09 (D-05) | T-01-17-01/02 | signed update verified; backups exist | manual | Smoke S-10, S-11 | ❌ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `scripts/dev-setup.sh` — apt Tauri deps (webkit2gtk-4.1 etc.), `x86_64-pc-windows-msvc` target, cargo-xwin, sqlx-cli, pnpm 12
- [ ] Cargo workspace + `rust-toolchain.toml` + `crates/mornfold-core` skeleton with `Clock` trait and test helpers (temp DB, mock keyring)
- [ ] `ui/` Vite + React + Vitest + Biome config; `ui/src/i18n/tr.json`
- [ ] `.github/workflows/ci.yml` (linux + windows jobs) producing the NSIS artifact
- [ ] `docs/smoke-test-phase1.md` skeleton (S-0..S-11)

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Installs on Win11; Smart App Control state | SHELL-01 | Needs real Win11 desktop | Smoke S-0, S-1, S-2 |
| X hides to tray; tray click/menu; Çık exits | SHELL-02 | CI runners have no interactive desktop | Smoke S-3 |
| Autostart after reboot lands in tray | SHELL-03 | Needs reboot of real PC | Smoke S-4 |
| Second launch focuses existing window | SHELL-04 | Needs installed app | Smoke S-5 |
| Toast name/icon, warm/cold/Notification Center click, buttons, sound | SHELL-05 | Toast display needs installed app + interactive session | Smoke S-6, S-7 |
| Real sleep/resume catch-up | SHELL-06 | Needs real sleep | Smoke S-8 |
| Key persists across restart, masked, one CredMan entry, absent from logs | SHELL-07 | Credential Manager on real PC | Smoke S-9 |
| v0.1.0 → v0.2.0 in-app update; key backups exist | SHELL-09 | Needs published releases + owner's key custody | Smoke S-10, S-11 |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 120s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
