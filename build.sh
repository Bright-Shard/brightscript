#!/bin/bash

# Build an example, then run cargo bloat on it, and finally get the raw file size
EXAMPLE=testing;

echo "Running cargo bloat...";
cargo bloat --release -n 10 --example $EXAMPLE
echo "";
echo "";
echo "";
echo "Raw file size:";
du -h target/release/examples/$EXAMPLE;
