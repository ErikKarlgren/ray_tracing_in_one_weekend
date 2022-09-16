#!/bin/bash
echo "Compiling..."
cargo build --release
echo "Running..."
time target/release/ray-tracing-in-one-weekend >example.ppm
echo "Showing image..."
gwenview example.ppm 2>/dev/null
