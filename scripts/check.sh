#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all --check
cargo clippy -p scholar-core --all-targets -- -D warnings
cargo test -p scholar-core
cargo check -p scholaros
npm run build
npm test
npm run test:ui
