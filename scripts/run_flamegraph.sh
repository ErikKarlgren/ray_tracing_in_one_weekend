#!/bin/bash
echo "Running 'cargo flamegraph'..."
cargo flamegraph >example.ppm 2>/dev/null | head -n-1
echo "Opening 'flamegraph.svg' in firefox..."
firefox flamegraph.svg 2>/dev/null
