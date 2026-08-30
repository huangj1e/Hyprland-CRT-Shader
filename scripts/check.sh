#!/usr/bin/env bash
set -euo pipefail

root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

bash -n bin/hypr-crt-toggle
python -m py_compile bin/hypr-crt-control
rm -rf bin/__pycache__

if command -v glslangValidator >/dev/null 2>&1; then
    glslangValidator -S frag shaders/crt.frag
else
    echo "warning: glslangValidator is not installed; GLSL validation skipped" >&2
fi

echo "All available checks passed."
