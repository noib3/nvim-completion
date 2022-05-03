#!/usr/bin/env bash

set -e

# The root of the project.
PRJ_ROOT="${PRJ_ROOT:-$(git rev-parse --show-toplevel)}"

# Whether to build a debug or release version of the project.
PROFILE="${1:-debug}"

if [ $PROFILE != "debug" ] && [ $PROFILE != "release" ]; then
  echo "Invalid argument \"$PROFILE\": profile should either be \"debug\" or \"release\""
  exit 1
fi

cargo_build() {
  if ! command -v cargo &>/dev/null; then
    echo "Couldn't find cargo in \$PATH, make sure the Rust toolchain is installed"
    return 1
  fi
  profile=$([ $PROFILE == debug ] && echo "" || echo --release)
  # Nightly is needed to compile (rustup toolchain install nightly) until https://github.com/rust-lang/rust/issues/79524 is merged.
  cargo +nightly build $profile &>/dev/null
  return 0
}

copy_stuff() {
  # TODO: extension is `.so` on linux, `.dylib` on macOS and `.dll` on Windows
  library_extension=$(\
    [ -f $PRJ_ROOT/target/$PROFILE/libcompleet_client.so ] \
      && echo so \
      || echo dylib \
  )

  # Place the compiled library where Neovim can find it.
  mkdir -p $PRJ_ROOT/lua
  cp \
    "$PRJ_ROOT/target/$PROFILE/libcompleet_client.$library_extension" \
    $PRJ_ROOT/lua/compleet.so

  # I'm not sure if copying all of the compiled library's dependencies is
  # actually needed.
  mkdir -p $PRJ_ROOT/lua/deps
  cp $PRJ_ROOT/target/$PROFILE/deps/*.rlib $PRJ_ROOT/lua/deps
}

cargo_build && copy_stuff
