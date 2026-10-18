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
| **Framework** | Rust: `cargo test` + `insta` 1.49 snapshots, `keyring_core::mock::Store`, `wiremock` 0.6.5 (provider key testers), temp SQLite, injected `Clock`. TS: Vitest 5.0.5 + jsdom + @testing-library/react 16.3.3 + `@tauri-apps/api/mocks` |
| **Config file** | none yet — Wave 0 installs `rust-toolchain.toml`, `ui/vitest.config.ts`, `ui/biome.json`; plan 01-10 adds `.github/workflows/ci.yml` and `scripts/ci-wait.sh` |
| **Quick run command** | `cargo test -p mornfold-core && pnpm -C ui exec vitest run` |
| **Full suite command** | `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app && pnpm -C ui exec biome ci . && pnpm -C ui exec tsc --noEmit && pnpm -C ui exec vitest run` (no `SQLX_OFFLINE`: Phase 1 uses runtime-checked static SQL, no `query!` macros — ADR 0002) |
| **Windows gate** | `bash scripts/ci-wait.sh` — pushes the committed HEAD and polls `gh api repos/Emsal66/Tempo/commits/<sha>/check-runs` until `linux` and `windows` are `success` (final automated verify of plans 01-10, 01-12..01-17, and the pre-tag gate in 01-18/01-19) |
| **Estimated runtime** | ~60–120 seconds quick (warm cache); full suite several minutes (xwin); Windows gate 20–75 minutes |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p mornfold-core` (plus `pnpm -C ui exec vitest run` when UI is touched)
- **After every plan wave:** Run the full suite command; implementation plans from wave 3 on end with `bash scripts/ci-wait.sh`
- **Before `/gsd:verify-work`:** Full suite green, Windows CI green on final commit, owner signs off smoke items S-0..S-11
- **Max feedback latency:** 120 seconds (quick run)

---

## Per-Task Verification Map

Task-level map. Task ID = `<plan>-T<task number>`; see each PLAN.md for full acceptance criteria. Every task of every plan has a row.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 01-01-T1 | 01-01 | 1 | (supply chain) | T-01-01-SC | crates incl. reqwest/wiremock approved before install | checkpoint (owner) | — (owner replies "onaylandı") | n/a | ⬜ pending |
| 01-01-T2 | 01-01 | 1 | (Wave 0 env) | T-01-01-04 | settings.json untouched | script | `bash scripts/dev-setup.sh --check` | ❌ W0 | ⬜ pending |
| 01-01-T3 | 01-01 | 1 | SHELL-06, SHELL-07 | T-01-01-01/02/05 | static SQL only; MasterKey Debug redacted; rustls only | unit + xwin | `cargo test -p mornfold-core --test foundation && cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-core` | ❌ W0 | ⬜ pending |
| 01-02-T1 | 01-02 | 1 | (supply chain) | T-01-02-SC | npm packages approved before install | checkpoint (owner) | — (owner replies "onaylandı") | n/a | ⬜ pending |
| 01-02-T2 | 01-02 | 1 | SHELL-08 | T-01-02-01/02/04 | no remote assets; no JS plugin packages; no node_modules committed | build + lint | `pnpm install --frozen-lockfile && pnpm -C ui build && pnpm -C ui exec biome ci . && pnpm -C ui exec tsc --noEmit` | ❌ W0 | ⬜ pending |
| 01-02-T3 | 01-02 | 1 | SHELL-08 | — | N/A | unit (Vitest) | `pnpm -C ui exec vitest run src/test/i18n.test.ts` | ❌ W0 | ⬜ pending |
| 01-03-T1 | 01-03 | 1 | SHELL-09 (D-01), SHELL-07 (D-17) | T-01-03-01 | no key material in ADRs | doc grep | `grep -q "com.mornfold.desktop" docs/decisions/0001-distribution-channel.md && grep -q ProviderTester docs/decisions/0002-phase1-platform-defaults.md` | ❌ | ⬜ pending |
| 01-03-T2 | 01-03 | 1 | SHELL-01..SHELL-09 (manual) | T-01-03-03 | fixed results table | doc grep | `for i in 0 1 2 3 4 5 6 7 8 9 10 11; do grep -q "S-$i" docs/smoke-test-phase1.md \|\| exit 1; done` | ❌ | ⬜ pending |
| 01-03-T3 | 01-03 | 1 | SHELL-05 (D-09), SHELL-02 (D-12) | T-01-03-02 | reproducible assets | script | `python3 -I scripts/gen-ping.py --check src-tauri/resources/sounds/mornfold-ping.wav` | ❌ W0 | ⬜ pending |
| 01-04-T1 | 01-04 | 1 | SHELL-01 | T-01-04-04 | SAC state known; Değerlendirme warning given | checkpoint (owner, S-0) | — | n/a | ⬜ pending |
| 01-04-T2 | 01-04 | 1 | SHELL-09 (D-05) | T-01-04-01/02/03 | key generated on owner PC; 3 backups | checkpoint (owner) | — | n/a | ⬜ pending |
| 01-04-T3 | 01-04 | 1 | SHELL-09 (D-05) | T-01-04-01 | no private key in repo | script | `base64 -d docs/release/updater-pubkey.txt \| head -1 \| grep -q "minisign public key"` | ❌ | ⬜ pending |
| 01-05-T1 | 01-05 | 2 | SHELL-02 (D-13) | — | N/A | unit | `cargo test -p mornfold-core --test pause` | ❌ | ⬜ pending |
| 01-05-T2 | 01-05 | 2 | SHELL-06 | T-01-05-01/02 | panicking job → failed, loop survives; no resume burst; resume marks telafi | unit (fake clock + temp DB) | `cargo test -p mornfold-core --test scheduler` | ❌ | ⬜ pending |
| 01-06-T1 | 01-06 | 2 | SHELL-07 (D-15) | T-01-06-04 | masked hint ≤4 trailing chars (longest prefix only); log redaction | unit | `cargo test -p mornfold-core --test masking` | ❌ | ⬜ pending |
| 01-06-T2 | 01-06 | 2 | SHELL-07 (D-16, D-18) | T-01-06-01..08 | AAD row binding; KCV; no silent key regen; canary absent from DB/WAL/logs; values trimmed before storage (`vault_trims_value`) | unit + integration | `cargo test -p mornfold-core --test vault --test canary` | ❌ | ⬜ pending |
| 01-06-T3 | 01-06 | 2 | SHELL-07 (D-17) | T-01-06-09/10/11 | fixed HTTPS hosts, no redirects, no key in result/log/URL; `#[doc(hidden)]` test constructor refuses non-loopback http (`testers_with_endpoints_rejects_remote_http`); values trimmed (`testers_trims_value`) | unit (wiremock) | `cargo test -p mornfold-core --test testers` | ❌ | ⬜ pending |
| 01-07-T1 | 01-07 | 2 | SHELL-05 (D-08) | T-01-07-01..03 | quick-xml escaping; deep-link allowlist | unit (insta + adversarial) | `cargo test -p mornfold-core --test deeplink --test toast_xml` | ❌ | ⬜ pending |
| 01-07-T2 | 01-07 | 2 | SHELL-05 (D-09, D-10, D-12) | T-01-07-01/04 | act links need single-use token < 7 days; consumption marks read; `source_of` for section routing | unit | `cargo test -p mornfold-core --test notifications` | ❌ | ⬜ pending |
| 01-08-T1 | 01-08 | 2 | SHELL-06, SHELL-08 | T-01-08-01 | N/A | unit | `cargo test -p mornfold-core --test i18n --test relative_time` | ❌ | ⬜ pending |
| 01-08-T2 | 01-08 | 2 | SHELL-02 (D-11..D-13) | T-01-08-03 | unknown menu ids ignored | unit (insta) | `cargo test -p mornfold-core --test tray_model` | ❌ | ⬜ pending |
| 01-09-T1 | 01-09 | 2 | SHELL-03, SHELL-04, SHELL-01 | T-01-09-01/02/03 | argv allowlist; minimal capabilities; strict CSP; noRedirectionBitmap | unit + config assert | `cargo test -p mornfold-core --test startup_args` | ❌ | ⬜ pending |
| 01-09-T2 | 01-09 | 2 | SHELL-02 (D-14), SHELL-04 | T-01-09-04 | no Webview log target | build + bindings drift | `cargo test -p mornfold-app --test bindings && cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app` | ❌ | ⬜ pending |
| 01-10-T1 | 01-10 | 3 | SHELL-01 | T-01-10-01..04/06 | no secrets in push CI; SHA-pinned actions; push trigger has `branches: ['**']` + `tags-ignore: ['v*']` | config grep | `grep -q windows-2025 .github/workflows/ci.yml && grep -Eq "^\s+branches:" .github/workflows/ci.yml && ! grep -q "secrets\." .github/workflows/ci.yml && bash -n scripts/ci-wait.sh` | ❌ W0 | ⬜ pending |
| 01-10-T2 | 01-10 | 3 | SHELL-01 | T-01-10-01..05 | NSIS artifact + startup smoke on windows-2025 | CI | `bash scripts/ci-wait.sh` | ❌ W0 | ⬜ pending |
| 01-11-T1 | 01-11 | 3 | SHELL-08 (D-06) | T-01-11-01 | window permissions only | component (Vitest) | `pnpm -C ui exec vitest run src/test/layout.test.tsx` | ❌ | ⬜ pending |
| 01-11-T2 | 01-11 | 3 | SHELL-03, SHELL-08 | T-01-11-02 | N/A | component (Vitest) | `pnpm -C ui exec vitest run` | ❌ | ⬜ pending |
| 01-12-T1 | 01-12 | 4 | SHELL-07 | T-01-12-02/04/08 | redacting log formatter; CredMan persistence=Local; no reqwest in shell | unit + xwin | `cargo test -p mornfold-app logging_ && cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app --tests` | ❌ | ⬜ pending |
| 01-12-T2 | 01-12 | 4 | SHELL-07 (D-15..D-17) | T-01-12-01/03 | no secret-returning command; secrets_test_saved returns only a result; value never cached | component + contract + CI | `pnpm -C ui exec vitest run src/test/keys.test.tsx src/test/bindings.contract.test.ts && bash scripts/ci-wait.sh` (credman_roundtrip on windows-2025) | ❌ | ⬜ pending |
| 01-13-T1 | 01-13 | 5 | SHELL-06 | T-01-13-01/02/06/07 | power callback send-only; unregister on drop; block_on only in setup + shutdown_blocking thread; initial kick; Error watchdog skipped for one tick after resume/gap | unit + build (xwin) | `cargo test -p mornfold-app health_grace_ && cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app` | ❌ | ⬜ pending |
| 01-13-T2 | 01-13 | 5 | SHELL-02 (D-11..D-14), SHELL-06 | T-01-13-03/04 | unknown menu ids ignored; update item guarded | build + unit | `cargo build -p mornfold-app && cargo test -p mornfold-core tray_model` | ❌ | ⬜ pending |
| 01-13-T3 | 01-13 | 5 | SHELL-02, SHELL-06 | — | display-only `setTimeout` tick, no `setInterval` / IPC in timers (`status_display_tick`) | component + CI | `pnpm -C ui exec vitest run src/test/status.test.tsx && bash scripts/ci-wait.sh` | ❌ | ⬜ pending |
| 01-14-T1 | 01-14 | 6 | SHELL-05 (D-09, D-10, D-12) | T-01-14-02/03/05 | AUMID rule; sound only when Windows accepts; unread cleared on show | build + xwin | `cargo test -p mornfold-app && cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app --tests` | ❌ | ⬜ pending |
| 01-14-T2 | 01-14 | 6 | SHELL-05 (D-08) | T-01-14-01/04/07 | act links need token; URLs never reach a shell; no block_on; section from notification source | unit + CI (toast_loadxml on windows-2025) | `cargo test -p mornfold-app deeplink_section_ && cargo test -p mornfold-core -- deeplink notifications_ && bash scripts/ci-wait.sh` | ❌ | ⬜ pending |
| 01-15-T1 | 01-15 | 7 | SHELL-05 (D-08) | T-01-15-01 | closed Route enum; Rust-side mute refetches queryKeys.sound (`navigate_invalidates_sound`, `status_changed_invalidates_sound`) | component (Vitest) | `pnpm -C ui exec vitest run src/test/navigate.test.tsx src/test/status.test.tsx` | ❌ | ⬜ pending |
| 01-15-T2 | 01-15 | 7 | SHELL-05 (D-09, D-10, D-12), SHELL-08 | T-01-15-02/03 | unknown perSource keys rejected by shell | component + CI | `pnpm -C ui exec vitest run src/test/notifications.test.tsx && bash scripts/ci-wait.sh` | ❌ | ⬜ pending |
| 01-16-T1 | 01-16 | 7 | SHELL-09 | T-01-16-01/02/04 | pubkey pinned; secrets only in tag job; tag==version | script | `node scripts/check-release-config.mjs` | ❌ | ⬜ pending |
| 01-16-T2 | 01-16 | 7 | SHELL-09 (D-02) | T-01-16-03/05/06 | install only on user action; no block_on; clean exit | build + CI | `cargo xwin check --target x86_64-pc-windows-msvc -p mornfold-app && bash scripts/ci-wait.sh` | ❌ | ⬜ pending |
| 01-17-T1 | 01-17 | 8 | SHELL-09 (D-02) | T-01-17-01/02 | install button only when Ready | component (Vitest) | `pnpm -C ui exec vitest run src/test/updates.test.tsx` | ❌ | ⬜ pending |
| 01-17-T2 | 01-17 | 8 | SHELL-09 (D-06) | T-01-17-03 | N/A | component + CI | `pnpm -C ui exec vitest run src/test/status-update.test.tsx && bash scripts/ci-wait.sh` | ❌ | ⬜ pending |
| 01-18-T1 | 01-18 | 9 | SHELL-01, SHELL-09 | T-01-18-01/02/05 | full suite + green CI on tagged commit; draft release | release check | `gh release view v0.1.0 --repo Emsal66/Tempo --json isDraft,assets` | ❌ | ⬜ pending |
| 01-18-T2 | 01-18 | 9 | SHELL-01..SHELL-08 | T-01-18-04 | act link from browser refused (S-6d) | manual (owner, CI artifact) | Smoke S-0..S-9 | n/a | ⬜ pending |
| 01-18-T3 | 01-18 | 9 | SHELL-05 (D-09) | T-01-18-03 | results recorded verbatim | unit + doc grep | `grep -q "Ses stratejisi sonucu: [AB]" docs/smoke-test-phase1.md && cargo test -p mornfold-core --test notifications` | ❌ | ⬜ pending |
| 01-19-T1 | 01-19 | 10 | SHELL-09 | T-01-19-03 | tag==version; draft until owner publishes | release check | `gh release view v0.2.0 --repo Emsal66/Tempo --json assets` | ❌ | ⬜ pending |
| 01-19-T2 | 01-19 | 10 | SHELL-09 (D-02, D-05) | T-01-19-01/02 | signed update verified; backups exist | manual (owner, v0.1.0 release installer) | Smoke S-10, S-11 | n/a | ⬜ pending |
| 01-19-T3 | 01-19 | 10 | SHELL-09 | T-01-19-04 | approval dated | doc grep | `grep -q "nyquist_compliant: true" .planning/phases/01-windows-kabu-u-ve-platform-altyap-s/01-VALIDATION.md` | ❌ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `scripts/dev-setup.sh` — apt Tauri deps (webkit2gtk-4.1 etc.), `x86_64-pc-windows-msvc` target, cargo-xwin, sqlx-cli, pnpm 12
- [ ] Cargo workspace + `rust-toolchain.toml` + `crates/mornfold-core` skeleton with `Clock` trait, test helpers (temp DB, mock keyring) and reqwest/wiremock dependencies
- [ ] `ui/` Vite + React + Vitest + Biome config; `ui/src/i18n/tr.json`; `ui/.gitignore`
- [ ] `.github/workflows/ci.yml` (linux + windows jobs) producing the NSIS artifact, and `scripts/ci-wait.sh` (plan 01-10)
- [ ] `docs/smoke-test-phase1.md` skeleton (S-0..S-11)

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Installs the per-push CI artifact of the tagged commit on Win11; Smart App Control state | SHELL-01 | Needs real Win11 desktop | Smoke S-0, S-1, S-2 |
| X hides to tray; tray click/menu; Çık exits; dot+tooltip badge acceptable | SHELL-02 | CI runners have no interactive desktop | Smoke S-3 |
| Autostart after reboot lands in tray | SHELL-03 | Needs reboot of real PC | Smoke S-4 |
| Second launch focuses existing window | SHELL-04 | Needs installed app | Smoke S-5 |
| Toast name/icon, warm/cold/Notification Center click, buttons, unread dot clears, sound | SHELL-05 | Toast display needs installed app + interactive session | Smoke S-6, S-7 |
| Real sleep/resume catch-up (telafi after 20+ min sleep) | SHELL-06 | Needs real sleep | Smoke S-8 |
| Key persists across restart, masked, D-17 test (unknown skip, invalid warning, saved-key test), one CredMan entry, absent from logs | SHELL-07 | Credential Manager and real provider endpoints on real PC | Smoke S-9 |
| v0.1.0 (release installer) → v0.2.0 in-app update; key backups exist | SHELL-09 | Needs published releases + owner's key custody | Smoke S-10, S-11 |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies (checkpoint tasks are owner-verified by design)
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 120s (quick run; the Windows gate is a per-plan exit check)
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
