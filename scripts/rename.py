#!/usr/bin/env python3
"""Renames the tool: `python3 scripts/rename.py <new-name>`.

Replaces the working name in every tracked file (crate name, binary, schema ids,
`EVERYTHING_RUST_*` environment variables, docs) and in tracked paths. Review with `git diff`.
"""
import os
import re
import subprocess
import sys

OLD = "everything-rust"

def main() -> None:
    if len(sys.argv) != 2 or not re.fullmatch(r"[a-z][a-z0-9-]*", sys.argv[1]):
        sys.exit("usage: rename.py <new-name>  (lowercase letters, digits, dashes)")
    new = sys.argv[1]
    files = subprocess.check_output(["git", "ls-files"], text=True).split()
    for path in files:
        try:
            with open(path, encoding="utf-8") as f:
                text = f.read()
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        updated = (text.replace(OLD, new)
                       .replace(OLD.upper(), new.upper().replace("-", "_"))
                       .replace(OLD.capitalize(), new.capitalize()))
        if updated != text:
            with open(path, "w", encoding="utf-8") as f:
                f.write(updated)
    for path in files:
        if OLD in path and os.path.exists(path):
            target = path.replace(OLD, new)
            os.makedirs(os.path.dirname(target) or ".", exist_ok=True)
            subprocess.check_call(["git", "mv", path, target])
    print(f"Renamed {OLD} to {new}. Review with `git diff`, then `cargo test`.")

if __name__ == "__main__":
    main()
