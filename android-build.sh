#!/bin/bash
cargo build --lib --target aarch64-linux-android --features "verbose-log" &
cargo build --release --lib --target aarch64-linux-android --features "verbose-log"