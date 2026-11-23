#!/bin/sh
# The batches against the original Java methods. The wrapper classes have the original names, so
# each side runs in its own JVM; equal digests mean equal results and errors.
# Run after `everything-rust build ..`: sh run.sh
set -e
cd "$(dirname "$0")"
out=$(mktemp -d)
javac -d "$out/orig" $(find ../../../../tests/fixtures/java/src/main -name '*.java') Common.java Orig.java
javac -d "$out/rust" $(find ../java -name '*.java') Common.java Rust.java
java -cp "$out/orig" Orig
java -Djava.library.path=../target/release -cp "$out/rust" Rust
