#!/bin/bash
cargo build --lib --target aarch64-linux-android --features "verbose_log" &
cargo build --release --lib --target aarch64-linux-android --features "verbose_log"