#!/bin/bash
cargo flamegraph >example.ppm | head -n-1
firefox flamegraph.svg
