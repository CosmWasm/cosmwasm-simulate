#!/usr/bin/env bash

set -e
cargo +nightly build
target/debug/cosmwasm-simulate -m messages.json
