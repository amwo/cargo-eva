#!/bin/sh
cargo build; cp ./target/debug/cargo-eva ~/.cargo/bin/cargo-eva; cargo eva
