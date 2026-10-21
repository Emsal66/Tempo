#!/usr/bin/env bash
# Mornfold — Linux dev-container toolchain bootstrap (Phase 1, Wave 0).
#
# Purpose: install everything the Linux dev loop needs so the Rust core, the
# Tauri shell (on WebKitGTK) and the Windows cross-check (`cargo xwin check`)
# all work inside the cloud Linux container. The product itself ships only for
# Windows 11; this script is for the Linux development container only.
#
# Idempotent: every step is skipped when it is already satisfied, so running it
# again is cheap.
#
# Usage:
#   bash scripts/dev-setup.sh           # install whatever is missing
#   bash scripts/dev-setup.sh --check   # only report; exit non-zero if anything is missing
#
# NOTE: Wiring this script into a Claude Code SessionStart hook would require
# changing `.claude/settings.json`. That needs explicit owner approval and is
# deliberately NOT done here. Do not modify `.claude/settings.json` from this
# script or as part of running it.

set -euo pipefail

RUST_TOOLCHAIN="1.99.1"
WIN_TARGET="x86_64-pc-windows-msvc"
CARGO_XWIN_VERSION="0.23.1"
SQLX_CLI_VERSION="0.9.0"
PNPM_SPEC="pnpm@12.14.0"
PNPM_MAJOR="12"
APT_PACKAGES=(
  libwebkit2gtk-4.1-dev
  build-essential
  curl
  wget
  file
  libxdo-dev
  libssl-dev
  libayatana-appindicator3-dev
  librsvg2-dev
)

CHECK_ONLY=0
for arg in "$@"; do
  case "$arg" in
    --check) CHECK_ONLY=1 ;;
    -h | --help)
      sed -n '2,19p' "$0"
      exit 0
      ;;
    *)
      echo "unknown argument: $arg" >&2
      exit 2
      ;;
  esac
done

# Run tool probes outside the repository so rust-toolchain.toml does not
# trigger an implicit toolchain download while we are only checking.
PROBE_DIR="${HOME:-/tmp}"

MISSING=()

log() { printf '\033[1;34m[dev-setup]\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m[dev-setup]\033[0m %s\n' "$*" >&2; }

as_root() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  else
    sudo "$@"
  fi
}

# ---------------------------------------------------------------------------
# (1) apt packages (Tauri Linux deps)
# ---------------------------------------------------------------------------
missing_apt=()
for pkg in "${APT_PACKAGES[@]}"; do
  if ! dpkg -s "$pkg" >/dev/null 2>&1; then
    missing_apt+=("$pkg")
  fi
done
if [ "${#missing_apt[@]}" -gt 0 ]; then
  if [ "$CHECK_ONLY" -eq 1 ]; then
    MISSING+=("apt: ${missing_apt[*]}")
  else
    log "installing apt packages: ${missing_apt[*]}"
    as_root apt-get update
    as_root env DEBIAN_FRONTEND=noninteractive apt-get install -y "${missing_apt[@]}"
  fi
else
  log "apt packages: ok"
fi

# ---------------------------------------------------------------------------
# (2) Rust toolchain + Windows target
# ---------------------------------------------------------------------------
has_toolchain() {
  (cd "$PROBE_DIR" && rustup toolchain list) | grep -q "^${RUST_TOOLCHAIN}-"
}
if ! has_toolchain; then
  if [ "$CHECK_ONLY" -eq 1 ]; then
    MISSING+=("rust toolchain ${RUST_TOOLCHAIN}")
  else
    log "installing Rust ${RUST_TOOLCHAIN}"
    (cd "$PROBE_DIR" && rustup toolchain install "$RUST_TOOLCHAIN" --profile minimal --component clippy,rustfmt)
  fi
else
  log "rust ${RUST_TOOLCHAIN}: ok"
fi

if has_toolchain; then
  if ! (cd "$PROBE_DIR" && rustup target list --installed --toolchain "$RUST_TOOLCHAIN") | grep -qx "$WIN_TARGET"; then
    if [ "$CHECK_ONLY" -eq 1 ]; then
      MISSING+=("rust target ${WIN_TARGET}")
    else
      log "adding target ${WIN_TARGET}"
      (cd "$PROBE_DIR" && rustup target add "$WIN_TARGET" --toolchain "$RUST_TOOLCHAIN")
    fi
  else
    log "target ${WIN_TARGET}: ok"
  fi
elif [ "$CHECK_ONLY" -eq 1 ]; then
  MISSING+=("rust target ${WIN_TARGET}")
fi

# ---------------------------------------------------------------------------
# (3) cargo-xwin (Windows compile checks from Linux)
# ---------------------------------------------------------------------------
if ! (cd "$PROBE_DIR" && cargo xwin --version >/dev/null 2>&1); then
  if [ "$CHECK_ONLY" -eq 1 ]; then
    MISSING+=("cargo-xwin ${CARGO_XWIN_VERSION}")
  else
    log "installing cargo-xwin ${CARGO_XWIN_VERSION}"
    (cd "$PROBE_DIR" && cargo install --locked cargo-xwin --version "$CARGO_XWIN_VERSION")
  fi
else
  log "cargo-xwin: ok"
fi

# ---------------------------------------------------------------------------
# (4) sqlx-cli (convenience: `sqlx migrate add`; the build does not need it)
# ---------------------------------------------------------------------------
if ! (cd "$PROBE_DIR" && sqlx --version >/dev/null 2>&1); then
  if [ "$CHECK_ONLY" -eq 1 ]; then
    MISSING+=("sqlx-cli ${SQLX_CLI_VERSION}")
  else
    log "installing sqlx-cli ${SQLX_CLI_VERSION}"
    (cd "$PROBE_DIR" && cargo install --locked sqlx-cli --version "$SQLX_CLI_VERSION" \
      --no-default-features --features sqlite,rustls)
  fi
else
  log "sqlx-cli: ok"
fi

# ---------------------------------------------------------------------------
# (5) pnpm 12 via corepack
# ---------------------------------------------------------------------------
pnpm_major() {
  (cd "$PROBE_DIR" && COREPACK_ENABLE_DOWNLOAD_PROMPT=0 pnpm --version 2>/dev/null) | cut -d. -f1
}
if [ "$(pnpm_major || true)" != "$PNPM_MAJOR" ]; then
  if [ "$CHECK_ONLY" -eq 1 ]; then
    MISSING+=("${PNPM_SPEC}")
  else
    log "activating ${PNPM_SPEC} via corepack"
    corepack enable
    (cd "$PROBE_DIR" && COREPACK_ENABLE_DOWNLOAD_PROMPT=0 corepack prepare "$PNPM_SPEC" --activate)
    if [ "$(pnpm_major || true)" != "$PNPM_MAJOR" ]; then
      # corepack <= 0.34 records pnpm's old `bin/pnpm.cjs` entry point, which
      # pnpm 12 no longer ships, so its shim fails with MODULE_NOT_FOUND. The
      # package corepack downloaded (and hash-verified) is fine: link its own
      # launcher into ~/.local/bin, which precedes the node bin dir on PATH.
      # No additional package is installed.
      cached="${COREPACK_HOME:-$HOME/.cache/node/corepack}/v1/pnpm/${PNPM_SPEC#pnpm@}/pnpm"
      if [ -x "$cached" ] && [ "$(cd "$PROBE_DIR" && "$cached" --version 2>/dev/null | cut -d. -f1)" = "$PNPM_MAJOR" ]; then
        log "corepack shim cannot start pnpm ${PNPM_MAJOR}; linking ${cached} into ~/.local/bin"
        mkdir -p "$HOME/.local/bin"
        ln -sf "$cached" "$HOME/.local/bin/pnpm"
        case ":$PATH:" in
          *":$HOME/.local/bin:"*) ;;
          *) warn "add \$HOME/.local/bin to PATH (before the Node.js bin dir) to use pnpm ${PNPM_MAJOR}" ;;
        esac
      fi
    fi
    if [ "$(pnpm_major || true)" != "$PNPM_MAJOR" ]; then
      warn "pnpm on PATH is still not ${PNPM_MAJOR}.x ($(command -v pnpm || echo none)); check for a shadowing global install"
      MISSING+=("${PNPM_SPEC}")
    fi
  fi
else
  log "pnpm ${PNPM_MAJOR}.x: ok"
fi

# ---------------------------------------------------------------------------
# (6) Summary
# ---------------------------------------------------------------------------
ver() {
  local out
  if out=$(cd "$PROBE_DIR" && "$@" 2>/dev/null | head -n1); then
    printf '%s' "${out:-?}"
  else
    printf 'missing'
  fi
}

echo
printf '%-22s %s\n' "tool" "version"
printf '%-22s %s\n' "----" "-------"
printf '%-22s %s\n' "rustup" "$(ver rustup --version)"
printf '%-22s %s\n' "rustc (pinned)" "$(ver rustc +"$RUST_TOOLCHAIN" --version)"
printf '%-22s %s\n' "windows target" "$(rustup target list --installed --toolchain "$RUST_TOOLCHAIN" 2>/dev/null | grep -x "$WIN_TARGET" || echo missing)"
printf '%-22s %s\n' "cargo-xwin" "$(ver cargo xwin --version)"
printf '%-22s %s\n' "sqlx-cli" "$(ver sqlx --version)"
printf '%-22s %s\n' "node" "$(ver node --version)"
printf '%-22s %s\n' "pnpm" "$(ver pnpm --version)"
printf '%-22s %s\n' "webkit2gtk-4.1" "$(dpkg-query -W -f='${Version}' libwebkit2gtk-4.1-dev 2>/dev/null || echo missing)"
echo

if [ "${#MISSING[@]}" -gt 0 ]; then
  warn "missing:"
  for m in "${MISSING[@]}"; do
    warn "  - $m"
  done
  exit 1
fi

log "all dev tools present"
