# FT-Rewards Contract

This repository hosts the `ft-rewards` smart contract, which is part of the `nft-farm` project on the NEAR blockchain. The contract manages a fungible token that users can earn as rewards by staking their NFTs via the `nft-staking` contract.

## Features

- **Minting Tokens**: Controlled minting functionality that allows the contract owner to issue new tokens.
- **Token Transfers**: Users can transfer tokens between accounts, facilitating the use of rewards within the ecosystem.
- **Reward Tracking**: Integration with the `nft-staking` contract to track and distribute tokens based on user activities.

## Getting Started

### Prerequisites

- Install [Node.js](https://nodejs.org/) and [Node Package Manager (npm)](https://npmjs.com/).
- NEAR CLI should be installed for interacting with the NEAR blockchain:
  ```bash
  npm install -g near-cli
  ```

### Installation

1. **Clone the Repository**

   Begin by cloning the repository to your local machine and navigate to the `ft-rewards` directory:

   ```bash
   git clone https://github.com/flowscience/nft-farm.git
   cd nft-farm/ft-rewards
   ```

2. **Install Dependencies**

   Install the required Node.js dependencies:

   ```bash
   npm install
   ```

### Deploying

1. **Build the Contract**

   Compile the smart contract to WebAssembly (WASM):

   ```bash
   ./build.sh
   ```

2. **Deploy the Contract**

   Deploy the contract to the NEAR testnet using NEAR CLI:

   ```bash
   near deploy --accountId your-account.testnet --wasmFile res/ft_rewards.wasm
   ```

   Replace `your-account.testnet` with your NEAR testnet account name.

### Usage

- **Initialize the Contract**

  Call the initialization method after deployment:

  ```bash
  near call your-contract.testnet new --accountId your-account.testnet
  ```

- **Mint Tokens**

  Mint new tokens to a specified account:

  ```bash
  near call your-contract.testnet mint '{"recipient": "user.testnet", "amount": "1000"}' --accountId your-account.testnet
  ```

- **Transfer Tokens**

  Transfer tokens from one account to another:

  ```bash
  near call your-contract.testnet transfer '{"sender": "user.testnet", "recipient": "friend.testnet", "amount": "500"}' --accountId your-account.testnet
  ```

## Development

- **Running Tests**

  Test the functionality of the smart contract with:

  ```bash
  cargo test
  ```

## Contributing

Contributions are welcome. Please fork the repository and submit pull requests with any enhancements, bug fixes, or suggestions.

## License

This project is open-sourced under the [MIT License](LICENSE).

---