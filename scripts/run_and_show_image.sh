#!/bin/bash
echo "Compiling..."
cargo build --release
#cargo build 
echo "Running..."
time target/release/ray_tracing_in_one_weekend >example.ppm
echo "Showing image..."
gwenview example.ppm 2>/dev/null
