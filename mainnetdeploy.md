Deployment Instructions
Here are the instructions to deploy your smart contracts to the NEAR testnet (or mainnet, if desired). These instructions assume you have the NEAR CLI installed and configured on your machine.

1. Compile the Contracts
Run the build script from your project root directory:

bash
Copy code
./build.sh
2. Deploy the Contracts
You'll need two NEAR accounts: one for the NFT staking contract and another for the fungible token rewards contract. These can be created via the NEAR Wallet or CLI if they do not exist.

Deploy NFT Staking Contract
markdown
Copy code
- **Login to NEAR CLI (if not already logged in):**
```bash
near login
Deploy the NFT Staking contract:
bash
Copy code
near deploy --accountId nft-staking.your-account.testnet --wasmFile res/nft_staking.wasm
Deploy Fungible Token Rewards Contract
markdown
Copy code
- **Deploy the FT Rewards contract:**
```bash
near deploy --accountId ft-rewards.your-account.testnet --wasmFile res/ft_rewards.wasm
3. Initialize the Contracts
Initialization of the contracts can be specific to the logic you've implemented in the constructors (init methods) of your smart contracts.

Initialize NFT Staking Contract
Replace nft-contract.testnet and ft-rewards.testnet with actual contract names as per your setup.

bash
Copy code
near call nft-staking.your-account.testnet new '{"owner_id": "your-account.testnet", "nft_contract_id": "nft-contract.testnet", "rewards_contract_id": "ft-rewards.testnet", "reward_rate": 0.1}' --accountId your-account.testnet
Initialize Fungible Token Rewards Contract
bash
Copy code
near call ft-rewards.your-account.testnet new '{"owner_id": "your-account.testnet", "total_supply": "1000000"}' --accountId your-account.testnet
Final Steps
After deploying and initializing your contracts, they will be live on the NEAR blockchain and ready to be interacted with using either NEAR CLI commands or custom front-end applications that interface with the NEAR blockchain.