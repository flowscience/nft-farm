# NFT Staking Contract

This repository contains the `nft-staking` smart contract designed for use on the NEAR protocol. It allows users to stake their NFTs and earn rewards in the form of fungible tokens based on the duration of the stake. This contract is intended to incentivize NFT holders to engage more deeply with the digital assets they own by rewarding long-term holding.

## Features

- **Staking**: Users can lock their NFTs into the contract to start accumulating rewards.
- **Unstaking**: Users can withdraw their NFTs and claim any accrued rewards.
- **Reward Calculation**: Rewards are calculated based on the number of NFTs staked and the duration of the stake.

## Getting Started

### Prerequisites

- Install [Node.js](https://nodejs.org/) which includes [Node Package Manager](https://www.npmjs.com/).
- Ensure you have NEAR CLI installed by running:
  ```bash
  npm install -g near-cli
  ```

### Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/flowscience/nft-farm.git
   cd nft-farm/nft-staking
   ```

2. **Install Dependencies**

   While in the project directory, install the necessary dependencies:

   ```bash
   npm install
   ```

### Deploying

1. **Build the Contract**

   Compile the smart contract to WebAssembly (WASM) by running:

   ```bash
   ./build.sh
   ```

2. **Deploy the Contract**

   Use the NEAR CLI to deploy the contract to the NEAR testnet:

   ```bash
   near deploy --accountId your-account.testnet --wasmFile res/nft_staking.wasm
   ```

   Replace `your-account.testnet` with your actual testnet account.

### Usage

- **Initialize the Contract**

  After deployment, initialize the contract by calling:

  ```bash
  near call your-contract.testnet new --accountId your-account.testnet
  ```

- **Stake an NFT**

  To stake an NFT:

  ```bash
  near call your-contract.testnet stake_nft '{"nft_id": "1"}' --accountId your-account.testnet
  ```

- **Unstake an NFT**

  To unstake an NFT and collect rewards:

  ```bash
  near call your-contract.testnet unstake_nft '{"nft_id": "1"}' --accountId your-account.testnet
  ```

## Development

- **Running Tests**

  Execute the tests for the smart contract with:

  ```bash
  cargo test
  ```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open issues to improve the documentation, code quality, or add new features.

## License

This project is licensed under the [MIT License](LICENSE).

---