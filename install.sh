#!/usr/bin/env sh

set -e

PRJ_ROOT="${PRJ_ROOT:-$PWD}"
PROFILE="debug"

mkdir -p ./lua/deps

cp $PRJ_ROOT/target/debug/libcompleet_client.so $PRJ_ROOT/lua/compleet.so \
 || cp $PRJ_ROOT/target/debug/libcompleet_client.dylib $PRJ_ROOT/lua/compleet.so

cp $PRJ_ROOT/target/$PROFILE/deps/*.rlib ./lua/deps
# cp $PRJ_ROOT/target/$PROFILE/compleet ./lua/compleet
