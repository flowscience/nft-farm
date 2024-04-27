This document provides an overview of the repository, details about each contract, instructions for building, testing, and deploying the contracts, and some general information on prerequisites.

# NFT Staking and Fungible Token Rewards Contracts

This repository contains two separate smart contracts designed for the NEAR protocol. The `nft-staking` contract allows users to stake NFTs and earn rewards based on the duration of staking. The `ft-rewards` contract manages the issuance and distribution of fungible tokens (FT) that serve as rewards for staked NFTs.

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust (latest stable version) [[install]](https://www.rust-lang.org/tools/install)
- [`near-cli`](https://github.com/near/near-cli) for interacting with the NEAR network
- [Node.js](https://nodejs.org/en/download/current) and [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm/) (for optional tooling)

## Directory Structure
```
/nft-farm
  ├── Cargo.toml             # Workspace configuration file
  ├── /nft-staking
  │   ├── Cargo.toml         # Cargo configuration for the nft-staking contract
  │   └── /src
  │       └── lib.rs         # Source file for the nft-staking contract
  └── /ft-rewards
      ├── Cargo.toml         # Cargo configuration for the ft-rewards contract
      └── /src
          └── lib.rs         # Source file for the ft-rewards contract
```

Each contract has its own directory and `Cargo.toml` file for Rust package management.

## Contracts

### NFT Staking Contract (`nft-staking`)

The [NFT Staking Contract](https://github.com/flowscience/nft-farm/tree/main/nft-staking) enables users to stake their NFTs and receive rewards based on staking metrics such as duration and quantity of NFTs staked. This contract interacts with a fungible token contract to distribute these rewards.

### Fungible Token Contract (`ft-rewards`)

The [Fungible Token Rewards Contract](https://github.com/flowscience/nft-farm/tree/main/ft-rewards) adheres to the NEP-141 standard and is used to issue and manage the fungible tokens that are distributed as rewards in the NFT staking process.

## Build

### Using the Build Script

[build.sh](https://github.com/flowscience/nft-farm/blob/main/build.sh)

Ensure you have Rust and the wasm32-unknown-unknown target installed:
```bash
rustup target add wasm32-unknown-unknown
```

Make the script executable:
```bash
chmod +x build.sh
```

Run the script:
```bash
./build.sh
```

This script should correctly build your smart contracts and store the compiled WASM files in the specified res directory, simplifying the process of managing builds within a Rust workspace structure.

_The build script automates the following steps:_

1. Creating a Directory for Outputs: The script starts by creating a directory named res inside your nft-farm workspace directory. This is where the compiled WASM files will be stored after building.
2. Building the Contracts: It navigates into your workspace directory and uses cargo build to compile each contract with the wasm32-unknown-unknown target, necessary for deploying Rust code to blockchain environments that support WASM. The --package flag specifies which package to compile, allowing selective compilation within a workspace.
3. Copying the WASM Files: After each build, the script copies the resulting .wasm files to the res directory for easy deployment.

**Usage**

Place [the build.sh script](https://github.com/flowscience/nft-farm/blob/main/build.sh) in the parent directory of nft-farm and run it.

### Building the Contracts Separately

To build the contracts, navigate to the root directory of each contract and run the following commands:

```bash
cd nft-staking
cargo build --target wasm32-unknown-unknown --release

cd ../ft-rewards
cargo build --target wasm32-unknown-unknown --release
```

This will compile each contract to WebAssembly (WASM) suitable for deployment on the NEAR network.

## Testing
Unit tests can be run by navigating to each contract directory and using:

`cargo test`

Ensure all tests pass before deploying the contracts.

## Deployment

- [Testnet deploy instructions](https://github.com/flowscience/nft-farm/blob/main/testnetdeploy.md)
- [Mainnet deploy instructions](https://github.com/flowscience/nft-farm/blob/main/mainnetdeploy.md)

**Quickstart Deploy**
Deploy the contracts to the NEAR testnet using the following commands:

```bash
near deploy --accountId YOUR_ACCOUNT_HERE.testnet --wasmFile ./target/wasm32-unknown-unknown/release/nft_staking.wasm

near deploy --accountId YOUR_ACCOUNT_HERE.testnet --wasmFile ./target/wasm32-unknown-unknown/release/ft_rewards.wasm
```
Replace YOUR_ACCOUNT_HERE with your actual NEAR testnet account ID.

## Initialization
After deployment, initialize each contract by calling its new method using [near-cli](https://docs.near.org/docs/tools/near-cli).

### NFT Staking Contract Initialization Example

```bash
near call YOUR_STAKING_CONTRACT_ACCOUNT.new '{"owner_id": "YOUR_ACCOUNT_HERE.testnet", "nft_contract_id": "NFT_CONTRACT_HERE.testnet", "rewards_contract_id": "REWARDS_CONTRACT_HERE.testnet", "reward_rate": 0.1}' --accountId YOUR_ACCOUNT_HERE.testnet
```
### Fungible Token Contract Initialization Example

```bash
near call YOUR_REWARDS_CONTRACT_ACCOUNT.new '{"owner_id": "YOUR_ACCOUNT_HERE.testnet", "total_supply": "1000000"}' --accountId YOUR_ACCOUNT_HERE.testnet
```

## Contributing
Contributions are welcome. Please submit an issue or fork the repository and submit a pull request with your changes and improvements.

## License
[MIT](https://github.com/flowscience/nft-farm/blob/main/LICENSE)
