#!/usr/bin/env bash

set -e
#cargo  build
cargo +nightly build

target/debug/cosmwasm-simulate -w erc20/erc20.wasm -m msg.json

#target/debug/cosmwasm-simulate -w erc20/erc20.wasm,erc21/erc20.wasm -m erc20/msg.json
