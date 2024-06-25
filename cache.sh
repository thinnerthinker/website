#!/bin/bash

# Build the default package, continue on failure
nix-build .#aarch64-linux --keep-going --no-out-link || echo "Build failed, attempting to push partial results"

# Find all /nix/store paths related to the current flake
store_paths=$(nix path-info --all)

# Push each path to Cachix
for path in $store_paths; do
    cachix push YOUR_CACHE_NAME $path
done

echo "Partial results pushed to Cachix."
