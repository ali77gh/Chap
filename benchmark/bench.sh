#!/usr/bin/env bash
# Benchmark runner: benchmark/bench.sh <path-to-chap-binary> [runs]
# Runs each .chp benchmark, prints min/median/max seconds.
set -u

BIN="${1:?usage: bench.sh <chap-binary> [runs]}"
RUNS="${2:-5}"
DIR="$(cd "$(dirname "$0")" && pwd)"

for bench in "$DIR"/*.chp; do
    name=$(basename "$bench")
    times=()
    for _ in $(seq 1 "$RUNS"); do
        t=$( { /usr/bin/time -f "%e" "$BIN" "$bench" > /dev/null; } 2>&1 )
        times+=("$t")
    done
    python3 - "$name" "${times[@]}" <<'EOF'
import sys, statistics
name, ts = sys.argv[1], [float(x) for x in sys.argv[2:]]
print(f"{name:25s} min={min(ts):6.3f}s median={statistics.median(ts):6.3f}s max={max(ts):6.3f}s  runs={ts}")
EOF
done
