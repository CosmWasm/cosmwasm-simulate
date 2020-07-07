#!/usr/bin/env bash

set -e
cargo +nightly build
target/debug/cosmwasm-simulate -m messages-dns.json
#target/debug/cosmwasm-simulate -m messages-erc20.json
