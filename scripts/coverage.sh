#!/bin/bash
set -eox pipefail

cargo install cargo-llvm-cov

source .env

cargo llvm-cov --hide-instantiations --open --ignore-filename-regex tests.rs
