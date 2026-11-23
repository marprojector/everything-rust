#!/usr/bin/env bash
# For every port: copies the crate next to the tree, records ffmpeg's behavior on the inputs
# from inputs.py (and everything-rust's generated edge cases), builds the Rust, replays every
# case and runs gate. Run prepare.sh first. Pass crate names to verify only those.
set -euo pipefail
HERE=$(cd "$(dirname "$0")/.." && pwd)
REPO=$(cd "$HERE/../../.." && pwd)
WORK=${WORK:-$HERE/work}
TREE=$WORK/tree
RA=${RA:-$REPO/target/release/everything-rust}
[ -x "$RA" ] || cargo build --release --manifest-path "$REPO/Cargo.toml"
export CFLAGS="-I$TREE -DHAVE_AV_CONFIG_H"

crates=("$@")
[ ${#crates[@]} -gt 0 ] || crates=(adler32-rs base64-rs mathematics-rs avstring-rs rational-rs \
  parseutils-rs oneshot-crc-rs oneshot-md5-rs oneshot-sha-rs oneshot-murmur3-rs)
for crate in "${crates[@]}"; do
  unit=${crate%-rs}
  unit=${unit#oneshot-}
  dir=$TREE/libavutil/rust/$crate
  rm -rf "$dir"
  mkdir -p "$dir"
  (cd "$HERE/$crate" && tar cf - --exclude target .) | (cd "$dir" && tar xf -)
  python3 - "$dir/everything-rust.json" "$TREE/libavutil" <<'PY'
import json, sys
path, root = sys.argv[1], sys.argv[2]
m = json.load(open(path))
m["root"] = root
json.dump(m, open(path, "w"), indent=2)
PY
  python3 "$HERE/scripts/inputs.py" "$unit" "$dir/inputs.jsonl"
  echo "== $crate"
  "$RA" record "$dir" --inputs "$dir/inputs.jsonl" | grep Recorded | sed 's/ in .*//'
  "$RA" build "$dir" > /dev/null
  "$RA" diff "$dir" | tail -1
  "$RA" gate "$dir" | tail -1
done
