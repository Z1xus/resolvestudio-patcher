#!/usr/bin/env bash
set -euo pipefail

target="${1:-x86_64-unknown-linux-musl}"
output="${2:-dist}"
root="$(pwd -P)"
cargo_home="${CARGO_HOME:-$HOME/.cargo}"
if command -v cygpath >/dev/null 2>&1; then
  root="$(cygpath -m "$root")"
  cargo_home="$(cygpath -m "$cargo_home")"
fi
flags=("--remap-path-prefix=$root=/src" "--remap-path-prefix=$cargo_home=/cargo")
if [[ "$target" == x86_64-pc-windows-msvc ]]; then
  msvc_bin="$(cygpath -u "${VCToolsInstallDir:?Windows build tools are not initialized}")/bin/Hostx64/x64"
  test -f "$msvc_bin/link.exe"
  export PATH="$msvc_bin:$PATH"
  CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="$(cygpath -m "$msvc_bin/link.exe")"
  export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER
  flags+=(-C target-feature=+crt-static -C link-arg=/Brepro)
elif [[ "$target" != x86_64-unknown-linux-musl ]]; then
  echo 'unsupported target' >&2
  exit 1
fi
printf -v CARGO_ENCODED_RUSTFLAGS '%s\x1f' "${flags[@]}"
export CARGO_ENCODED_RUSTFLAGS="${CARGO_ENCODED_RUSTFLAGS%$'\x1f'}"
unset RUSTFLAGS
export CARGO_INCREMENTAL=0
export TZ=UTC
export LC_ALL=C

cargo build --release --locked --target "$target"
binary="target/$target/release/resolvestudio-patcher"
if [[ "$target" == x86_64-pc-windows-msvc ]]; then
  binary+=.exe
  asset=resolvestudio-patcher-windows-x86_64.exe
else
  asset=resolvestudio-patcher-linux-x86_64
fi
mkdir -p "$output"
cp "$binary" "$output/$asset"
{
  echo "Commit: $(git rev-parse --verify HEAD 2>/dev/null || echo uncommitted)"
  echo "Target: $target"
  rustc -Vv
} > "$output/BUILD.txt"
