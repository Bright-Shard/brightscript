#!/bin/bash

# Get the size of compiled binaries
echo $(du -h target/release/examples/testing | cut -f 1)
