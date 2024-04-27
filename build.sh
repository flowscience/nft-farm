#!/bin/bash

# Exit if any command fails
set -e

# Ensure the 'res' directory exists to store the compiled WASM files
mkdir -p res

# Build nft-staking contract
echo "Building nft-staking contract..."
cargo build --package nft-staking --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/nft_staking.wasm res/

# Build ft-rewards contract
echo "Building ft-rewards contract..."
cargo build --package ft-rewards --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/ft_rewards.wasm res/

echo "Build complete. Compiled contracts are now in the 'res' directory."
