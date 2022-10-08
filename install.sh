#!/usr/bin/env bash

set -e

# The root of the project.
PRJ_ROOT="${PRJ_ROOT:-$(git rev-parse --show-toplevel)}"

# Whether to build a debug or release version of the project.
PROFILE="${1:-debug}"

if [ $PROFILE != "debug" ] && [ $PROFILE != "release" ]; then
  echo "Invalid profile \"$PROFILE\": profile should either be \"debug\" or \"release\""
  exit 1
fi

case "$OSTYPE" in
  linux*|bsd*|solaris*) OS="linux" ;;
  msys*|cygwin*) OS="windows" ;;
  darwin*) OS="macos" ;;
  *) echo "Unknown OS type \"$OSTYPE\"" && exit 1 ;;
esac

cargo_build() {
  if ! command -v cargo &>/dev/null; then
    echo "Couldn't find cargo in \$PATH, make sure the Rust toolchain is installed"
    return 1
  fi
  profile=$([ $PROFILE == debug ] && echo "" || echo --release)
  cargo build $profile &>/dev/null
  return 0
}

symlink_dll() {
  case "$OS" in
    linux) dll_prefix="lib" && dll_suffix="so" && target_suffix="so" ;;
    macos) dll_prefix="lib" && dll_suffix="dylib" && target_suffix="so" ;;
    windows) dll_prefix="" && dll_suffix="dll" && target_suffix="dll" ;;
  esac

  mkdir -p "$PRJ_ROOT/lua"

  # Link the dll where Neovim can find it.
  ln -sf \
    "$PRJ_ROOT/target/$PROFILE/${dll_prefix}nvim_completion.${dll_suffix}" \
    "$PRJ_ROOT/lua/nvim_completion.${target_suffix}"
}

cargo_build && symlink_dll
