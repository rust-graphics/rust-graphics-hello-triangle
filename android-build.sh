#!/bin/bash
cargo build --lib --target aarch64-linux-android
cargo build --lib --target x86_64-linux-android
cargo build --release --lib --target aarch64-linux-android
cargo build --release --lib --target x86_64-linux-android