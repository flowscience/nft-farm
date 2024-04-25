Deployment Instructions on NEAR Testnet
Here are step-by-step instructions to deploy your contracts to the NEAR testnet. Make sure you have the NEAR CLI installed and logged in to a NEAR testnet account.

Step 1: Compile the Contracts
Run the build script from your project's root directory:

bash
Copy code
./build.sh
Step 2: Deploy the Contracts
You'll need two separate NEAR accounts for the NFT staking contract and the fungible token rewards contract. These can be created through the NEAR Wallet or CLI if they do not exist.

Deploy NFT Staking Contract
markdown
Copy code
- **Deploy the NFT Staking contract:**
```bash
near deploy --accountId nft-staking.testnet --wasmFile res/nft_staking.wasm
Deploy Fungible Token Rewards Contract
markdown
Copy code
- **Deploy the FT Rewards contract:**
```bash
near deploy --accountId ft-rewards.testnet --wasmFile res/ft_rewards.wasm
Step 3: Initialize the Contracts
Adjust the initialization parameters as needed based on your contract's requirements.

Initialize NFT Staking Contract
bash
Copy code
near call nft-staking.testnet new '{"owner_id": "your-account.testnet", "nft_contract_id": "nft-contract.testnet", "rewards_contract_id": "ft-rewards.testnet", "reward_rate": 0.1}' --accountId your-account.testnet
Initialize Fungible Token Rewards Contract
bash
Copy code
near call ft-rewards.testnet new '{"owner_id": "your-account.testnet", "total_supply": "1000000"}' --accountId your-account.testnet
Testing the Deployed Contracts
After deployment and initialization, you can test the functionality of your contracts by simulating interactions such as staking NFTs, claiming rewards, and transferring fungible tokens.

markdown
Copy code
#### Test Staking an NFT

- **Stake an NFT by calling the staking method:**
```bash
near call nft-staking.testnet stake_nft '{"account_id": "user-account.testnet", "nft_id": "nft1"}' --accountId user-account.testnet
Test Unstaking an NFT
Unstake an NFT by calling the unstaking method:
bash
Copy code
near call nft-staking.testnet unstake_nft '{"account_id": "user-account.testnet", "nft_id": "nft1"}' --accountId user-account.testnet
Test Transferring FT Tokens
Transfer tokens to another account:
bash
Copy code
near call ft-rewards.testnet transfer '{"recipient": "friend-account.testnet", "amount": "100"}' --accountId your-account.testnet
Make sure to adjust account names and parameters based on your actual testnet accounts and contract logic. These commands provide a basic framework to verify that your contracts are functioning as expected on the NEAR testnet.