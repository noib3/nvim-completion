#!/usr/bin/env sh

set -e

mkdir -p ./lua/deps
cp ./target/debug/libcompleet_client.so ./lua/compleet.so
cp ./target/debug/deps/*.rlib ./lua/deps
cp ./target/debug/compleet-server ./lua/compleet-server
