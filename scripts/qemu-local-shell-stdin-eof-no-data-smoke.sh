#!/usr/bin/env bash
set -euo pipefail

script_dir="$(CDPATH= cd "$(dirname "$0")" && pwd)"

exec "$script_dir/qemu-local-shell-runtime-stdin-readiness-smoke.sh" "$@"
