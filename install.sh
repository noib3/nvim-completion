#!/usr/bin/env sh

set -e

PROFILE="debug"

mkdir -p ./lua/deps

cp ./target/debug/libcompleet_client.so ./lua/compleet.so \
 || cp ./target/debug/libcompleet_client.dylib ./lua/compleet.so

cp ./target/$PROFILE/deps/*.rlib ./lua/deps
cp ./target/$PROFILE/compleet ./lua/compleet
