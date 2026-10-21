---
phase: 01-windows-kabu-u-ve-platform-altyap-s
plan: 02
subsystem: ui-scaffold-and-i18n
tags: [react, vite, tailwind, biome, vitest, i18n, pnpm, shell-08]
requires: []
provides:
  - package.json + pnpm-workspace.yaml (workspace root, pinned @tauri-apps/cli 2.12.3, packageManager pnpm@12.14.0)
  - ui/ React 19 + Vite 8 + TS 7 + Tailwind 4 app (`pnpm -C ui dev|build|test|lint|typecheck`)
  - ui/src/i18n/tr.json (complete Phase 1 Turkish catalog; Rust embeds it in plan 01-08)
  - ui/src/i18n/index.ts (i18n default export, formatRelativeTr, formatTimeTr)
  - ui/src/test/i18n.test.ts (SHELL-08 guard - key coverage + raw JSX text)
  - ui/src/test/setup.ts (imports i18n, clearMocks() after each test)
affects: [01-08, 01-09, 01-11, 01-12, 01-13, 01-14, 01-15, 01-16, 01-17]
tech-stack:
  added:
    - "react/react-dom 19.3.1, @tauri-apps/api 2.12.4, i18next 26.5.2, react-i18next 17.1.0, @tanstack/react-query 5.104.4, zustand 5.0.15, motion 14.3.0, lucide-react 1.59.0, @fontsource-variable/inter 5.3.0"
    - "dev: vite 8.3.4, @vitejs/plugin-react 6.1.2, typescript 7.0.2, tailwindcss + @tailwindcss/vite 4.3.3, @biomejs/biome 2.5.15, vitest 5.0.5, jsdom 30.1.3, @testing-library/react 16.3.3, @testing-library/dom 10.4.2, @types/react 19.3.0, @types/react-dom 19.3.0"
    - "root dev: @tauri-apps/cli 2.12.3"
  patterns:
    - "Exact version pins in package.json; pnpm-lock.yaml committed; `pnpm install --frozen-lockfile` must pass"
    - "No raw text in JSX: every string goes through t(\"...\") and must exist in tr.json (test-enforced)"
    - "Source scanning in tests uses Vite `import.meta.glob(..., { query: '?raw', eager: true })` instead of node:fs (no @types/node needed)"
    - "Vitest `fsModuleCache: false` so eager globs never go stale"
    - "Biome 2.5: `linter.rules.preset: recommended`, `css.parser.tailwindDirectives: true`, `!!dist` and `!!src/bindings.ts` force-excluded"
key-files:
  created:
    - package.json
    - pnpm-workspace.yaml
    - pnpm-lock.yaml
    - ui/.gitignore
    - ui/package.json
    - ui/index.html
    - ui/vite.config.ts
    - ui/vitest.config.ts
    - ui/tsconfig.json
    - ui/biome.json
    - ui/src/main.tsx
    - ui/src/App.tsx
    - ui/src/styles.css
    - ui/src/vite-env.d.ts
    - ui/src/i18n/index.ts
    - ui/src/i18n/tr.json
    - ui/src/test/setup.ts
    - ui/src/test/i18n.test.ts
  modified: []
decisions:
  - "Turkish is the only and fallback i18n language; i18next initialised synchronously (initAsync: false)"
  - "i18n guard scans sources with import.meta.glob ?raw instead of node:fs, avoiding an unapproved @types/node dependency"
  - "Vitest fsModuleCache disabled: the persistent transform cache keeps eager glob file lists stale"
  - "Biome config uses the 2.5 `preset: recommended` form (the `recommended: true` field is deprecated) and enables Tailwind directive parsing"
  - "pnpm 12.14.0 is selected through the root packageManager field (pnpm 10 auto-switches); corepack 0.34 cannot launch pnpm 12's .mjs entry"
metrics:
  duration: "~12 min"
  completed: 2026-10-21
  tasks: 3
  files: 18
---

# Phase 1 Plan 02: UI scaffold and Turkish i18n Summary

This plan adds the pnpm workspace root, which carries the pinned Tauri CLI, and a React 19 + Vite 8 + TypeScript 7 + Tailwind 4 frontend in `ui/`, set up with Biome and Vitest. It also adds the complete Phase 1 Turkish string catalog (`tr.json`), loaded by i18next with Turkish as the only and fallback language. A guard test fails if a `t("...")` key is missing from the catalog or if raw text appears inside JSX (SHELL-08).

## What was built

- **Workspace root:** `package.json` (`mornfold-workspace`, `packageManager: pnpm@12.14.0`, `engines.node >=22.12`, `@tauri-apps/cli` 2.12.3, `tauri` script) and `pnpm-workspace.yaml` (`ui`).
- **ui/ toolchain:** exact-pinned dependencies with no `@tauri-apps/plugin-*` JS packages and no typescript-eslint. The Vite config uses a fixed `127.0.0.1:5173` with strictPort, `clearScreen: false`, `envPrefix` VITE_/TAURI_ENV_ and target chrome120. The Vitest config uses jsdom, a setup file and `restoreMocks`. `tsconfig.json` is strict with bundler resolution. The Biome config excludes `src/bindings.ts` and `dist`.
- **Page shell:** `index.html` has `lang="tr"` and no external URLs. `styles.css` has a transparent `html/body/#root` so Mica shows through (D-06), uses the bundled Inter Variable font, and sets `color-scheme: light dark`. `main.tsx` imports the font, the CSS and `./i18n`, then renders `<App/>` in StrictMode with a QueryClientProvider (`staleTime: 30_000`, `retry: 1`). `App.tsx` is a placeholder that renders `t("app.name")`.
- **i18n:** `tr.json` holds the full key tree from the plan's interfaces block, 118 keys, with literal Turkish characters. `index.ts` exports the i18n instance, `formatRelativeTr` (Intl.RelativeTimeFormat tr-TR: minutes under 1 h, hours under 24 h, then days) and `formatTimeTr` (HH:MM in tr-TR).
- **Guard tests (9 tests in 3 groups):**
  - Runtime: sync init, `lng=tr`, `pause.untilTomorrow`, `{{count}}` interpolation, tr-TR helpers.
  - Catalog: all REQUIRED_KEYS present, no empty values.
  - Source guard: the scan finds App.tsx, a regex self-test, every t()/i18nKey literal exists, and no raw letter text appears in `.tsx` JSX.

## Task outcomes

| Task | Name | Commit | Outcome |
|------|------|--------|---------|
| 1 | Package legitimacy check (checkpoint:human-verify, blocking-human) | n/a | **Approved (`onaylandı`).** The owner delegated the check to the orchestrator, which verified each package on the npm registry on 2026-10-21. vitest: repo github.com/vitest-dev/vitest, maintainers ariperkkio/antfu/hiogawa/oreanno, 5.0.5 is latest, about 77.5M weekly downloads; the slopcheck [SUS] flag was a name-similarity false positive. @types/react and @types/react-dom: DefinitelyTyped, latest 19.3.0. @testing-library/dom: testing-library/dom-testing-library, latest 10.4.2, maintainers include kentcdodds. |
| 2 | pnpm workspace, Vite/React/Tailwind scaffold, Biome and Vitest config | fac332b | Install, build, `biome ci`, `tsc --noEmit` and the frozen-lockfile install all pass |
| 3 | i18next setup, full Phase 1 catalog, guard tests (TDD) | 0022cef (RED), d234db8 (GREEN) | 9/9 tests pass |

## Verification

- `pnpm install --frozen-lockfile`: lockfile up to date
- `pnpm -C ui exec vitest run`: 9 passed
- `pnpm -C ui exec biome ci .`: 15 files checked, no issues
- `pnpm -C ui exec tsc --noEmit`: OK
- `pnpm -C ui build`: `ui/dist/index.html` plus bundled Inter woff2 assets, no remote resources
- `python3 -c "import json;json.load(open('ui/src/i18n/tr.json',encoding='utf-8'))"`: OK
- `git ls-files | grep -E '(^|/)node_modules/|^ui/dist/'`: empty
- Guard probes: temporarily adding a `.tsx` with `t("nope.missing")` and `<span>Merhaba</span>` made both source-guard tests fail, as did adding a new file with raw JSX text. Both pass again after removal.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Minimal i18n bootstrap created in Task 2**
- **Found during:** Task 2
- **Issue:** `main.tsx` must import `./i18n` and `App.tsx` must call `t("app.name")`, but `i18n/index.ts` and `tr.json` belong to Task 3. Without them, Task 2's build and typecheck cannot pass.
- **Fix:** Task 2 committed a minimal `i18n/index.ts` (same init options) and a `tr.json` with only `app.name`. Task 3's RED test then failed against it, and GREEN filled in the full catalog and helpers.
- **Files modified:** ui/src/i18n/index.ts, ui/src/i18n/tr.json
- **Commit:** fac332b

**2. [Rule 3 - Blocking] Source scan via `import.meta.glob` instead of `node:fs`**
- **Found during:** Task 3
- **Issue:** The plan's tsconfig sets `types: ["vite/client"]` and the approved package list has no `@types/node`, so `node:fs`/`node:path`/`import.meta.dirname` would fail `tsc --noEmit`.
- **Fix:** The test loads `ui/src/**/*.{ts,tsx}` as raw strings through Vite's typed `import.meta.glob` with `?raw`. It excludes tests, `src/test/**`, `bindings.ts` and `*.d.ts`. No new package was installed.
- **Files modified:** ui/src/test/i18n.test.ts
- **Commit:** 0022cef

**3. [Rule 1 - Bug] Vitest fsModuleCache made the guard miss new files**
- **Found during:** Task 3 (guard probe)
- **Issue:** Vitest 5's persistent transform cache (on by default) reused the transformed test module, including the old eager-glob file list. A newly added `.tsx` with violations was not scanned, so the tests passed when they should have failed.
- **Fix:** Set `fsModuleCache: false` in `ui/vitest.config.ts`, with a comment. Re-probed: violations are now detected.
- **Files modified:** ui/vitest.config.ts
- **Commit:** d234db8

**4. [Rule 3 - Blocking] Biome 2.5 config schema**
- **Found during:** Task 2
- **Issue:** `linter.rules.recommended: true` is deprecated in Biome 2.5, and the Tailwind `@theme` at-rule failed to parse.
- **Fix:** Ran `biome migrate --write` (`preset: "recommended"`) and added `css.parser.tailwindDirectives: true`. Used force-exclude `!!dist` / `!!src/bindings.ts`. A probe with a badly formatted `src/bindings.ts` confirmed it is ignored.
- **Files modified:** ui/biome.json
- **Commit:** fac332b

**5. [Environment] pnpm 12 activation**
- `corepack prepare pnpm@12.14.0 --activate` downloaded the package, but corepack 0.34 looks for `bin/pnpm.cjs`, while pnpm 12 ships `bin/pnpm.mjs`. Running `corepack enable` would also have replaced the container's global pnpm, which the parallel agent shares. Instead, the root `packageManager: pnpm@12.14.0` field makes the installed pnpm 10 auto-switch: `pnpm --version` reports 12.14.0 in the repo, and all installs and lockfile writes used 12.14.0. No repo change was needed.

Additional, non-deviation choices: `tsconfig.json` also sets `isolatedModules`, `skipLibCheck`, `noUnused*` and `module: ESNext`. The guard's raw-JSX regex excludes `=>` and requires the closing `<` to start a tag, which avoids false positives on arrow functions and comparisons. This is covered by the regex self-test.

## TDD Gate Compliance

- RED: `0022cef test(01-02): ...` (4 of 9 tests failed: runtime x3, catalog keys x1)
- GREEN: `d234db8 feat(01-02): ...` (9/9 pass)
- REFACTOR: not needed

## Known Stubs

- `ui/src/App.tsx`: placeholder that renders only `t("app.name")`. This is intentional per the plan; plan 01-11 replaces it with the real layout (title bar, Durum / Ayarlar).

## Threat mitigations applied

- T-01-02-SC: owner/orchestrator legitimacy check (Task 1), exact pins, committed lockfile, `--frozen-lockfile` verified.
- T-01-02-01: Inter bundled via @fontsource-variable; `index.html` contains no `https://`.
- T-01-02-02: no `@tauri-apps/plugin-*` packages in `ui/package.json`.
- T-01-02-04: `ui/.gitignore` (node_modules/, dist/), explicit-path staging only, `git ls-files` check is empty. The root `node_modules/` stays untracked until plan 01-01's root `.gitignore` lands.

## Self-Check: PASSED

- All 18 key files exist on disk.
- Commits fac332b, 0022cef and d234db8 exist on the worktree branch.
