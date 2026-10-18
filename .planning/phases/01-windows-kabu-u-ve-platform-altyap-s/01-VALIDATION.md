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

Requirement-level map from RESEARCH.md §Validation Architecture. Task IDs are filled in once PLAN.md files exist.

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | — | SHELL-01 | — | N/A | CI | windows job `pnpm tauri build --bundles nsis` + upload-artifact; check-runs success | ❌ W0 | ⬜ pending |
| TBD | TBD | — | SHELL-02 | — | N/A | unit (insta) | `cargo test -p mornfold-core tray_model` / `pause` | ❌ W0 | ⬜ pending |
| TBD | TBD | — | SHELL-03 | — | N/A | unit | `cargo test -p mornfold-core startup_args` | ❌ W0 | ⬜ pending |
| TBD | TBD | — | SHELL-04 | Second-instance argv injection | argv parsed by allowlist, unknown flags ignored | unit + manual | `cargo test -p mornfold-core deeplink`; Smoke S-5 | ❌ W0 | ⬜ pending |
| TBD | TBD | — | SHELL-05 | Toast XML injection; `mornfold://act/*` abuse | quick-xml escaping; act links need single-use notification token | unit (insta + adversarial) | `cargo test -p mornfold-core toast_xml` / `deeplink`; `cargo test -p mornfold-app toast_loadxml` (Windows CI) | ❌ W0 | ⬜ pending |
| TBD | TBD | — | SHELL-06 | — | panicking job → failed status, loop survives | unit (fake clock + temp DB) | `cargo test -p mornfold-core scheduler` / `relative_time` | ❌ W0 | ⬜ pending |
| TBD | TBD | — | SHELL-07 | Secret exfiltration via UI/logs; row swap; key loss | no read command; AAD binding; KCV; redaction; canary | unit + integration | `cargo test -p mornfold-core vault` / `masking` / `canary_no_plaintext`; `pnpm -C ui vitest run bindings.contract` | ❌ W0 | ⬜ pending |
| TBD | TBD | — | SHELL-08 | — | N/A | unit | `pnpm -C ui vitest run i18n`; `cargo test -p mornfold-core i18n` | ❌ W0 | ⬜ pending |
| TBD | TBD | — | SHELL-09 | Malicious update / CI secret theft | mandatory minisign pubkey; secret only in tag release job | unit/script | `node scripts/check-release-config.mjs` or `cargo test -p mornfold-core release_config` | ❌ W0 | ⬜ pending |

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
