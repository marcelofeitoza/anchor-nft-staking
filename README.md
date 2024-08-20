# NFT Staking Program

## Overview

This is an Anchor-based Solana program designed for staking NFTs. It allows users to lock their NFTs to earn rewards based on the duration of the stake. The program includes functionalities for initializing configuration settings, user accounts, staking NFTs, unstaking them, and claiming accrued rewards.

## Features

-   **Configuration Initialization**: Set up global parameters for the staking process, including points per stake and the freeze period.
-   **User Initialization**: Register users in the system and prepare their accounts for staking.
-   **NFT Staking**: Allow users to stake NFTs from specified collections to earn points over time.
-   **NFT Unstaking**: Enable users to retrieve their staked NFTs after the staking period.
-   **Reward Claims**: Users can claim their earned rewards based on the points accumulated through staking.

## Usage

### Commands

-   **Initialize Configuration**: Establish the rules and parameters for the staking process.
-   **Register User**: Set up an account for each user who wants to participate in staking.
-   **Stake NFT**: Lock an NFT into the program to start accumulating points.
-   **Unstake NFT**: Withdraw the NFT once the staking conditions are met.
-   **Claim Rewards**: Convert staked points into SPL tokens or other forms of rewards.

## Code Structure

-   **`lib.rs`**: Contains the core logic for staking, unstaking, and claiming rewards.
-   **`instructions/`**: Holds the instruction modules for various actions within the program.
    -   **`initialize_config.rs`**: Instructions for setting up staking configurations.
    -   **`initialize_user.rs`**: Instructions for registering users.
    -   **`stake.rs`**: Instructions for staking NFTs.
    -   **`unstake.rs`**: Instructions for unstaking NFTs.
    -   **`claim.rs`**: Instructions for claiming rewards.
-   **`state/`**: Manages the state objects representing user accounts, staking configurations, and individual stakes.
    -   **`user_account.rs`**: State of user accounts including points and number of staked NFTs.
    -   **`stake_config.rs`**: Configuration details for the staking mechanism.
    -   **`stake_account.rs`**: State of individual stakes.

### Test Cases

-   **Mint Collection NFT**: Tests the creation of a collection NFT, which is essential for the staking process.
-   **Mint NFT**: Verifies that individual NFTs can be minted and are correctly initialized.
-   **Verify Collection NFT**: Ensures that NFTs belong to the correct collection and meet the criteria for staking.
-   **Initialize Config Account**: Validates the initialization of the staking configuration with the correct parameters.
-   **Initialize User Account**: Ensures that user accounts are set up properly to participate in staking.
-   **Stake NFT**: Confirms that NFTs can be staked correctly and points are accumulated as expected.
-   **Unstake NFT**: Tests the ability of users to unstake NFTs, including proper updates to the user's points and status.
-   **Claim Rewards**: Verifies that rewards based on accumulated points can be claimed and transferred correctly.

### Running Tests

To run the tests, execute the following commands:

```bash
yarn # Install dependencies
anchor build && anchor deploy --provider.cluster devnet && anchor test --provider.cluster devnet # Make sure you have at least 5 SOL in your wallet
```

Example output:

```bash
❯ anchor build && anchor deploy --provider.cluster devnet && anchor test --provider.cluster devnet
   Compiling nft-staking v0.1.0 (~/anchor/nft-staking/programs/nft-staking)
    Finished release [optimized] target(s) in 2.28s
   Compiling nft-staking v0.1.0 (~/anchor/nft-staking/programs/nft-staking)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.81s
     Running unittests src/lib.rs (~/anchor/nft-staking/target/debug/deps/nft_staking-c2c2836b6af134aa)
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: ~/.config/solana/id.json
Deploying program "nft_staking"...
Program path: ~/anchor/nft-staking/target/deploy/nft_staking.so...
Program Id: nftmapxi1xxp8F4TiU3cZ7xxEd1kFcT5aSnpRZaqa3U

Deploy success
    Finished release [optimized] target(s) in 0.24s
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running unittests src/lib.rs (~/anchor/nft-staking/target/debug/deps/nft_staking-c2c2836b6af134aa)
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: ~/.config/solana/id.json
Deploying program "nft_staking"...
Program path: ~/anchor/nft-staking/target/deploy/nft_staking.so...
Program Id: nftmapxi1xxp8F4TiU3cZ7xxEd1kFcT5aSnpRZaqa3U

Deploy success

Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "~/anchor/nft-staking/Anchor.toml"

yarn run v1.22.22
$ ~/anchor/nft-staking/node_modules/.bin/ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'
config PublicKey [PublicKey(J6Xy2WxSAphpmfynGZwqGMKrNGHR8Le1W7hEXSVwMoyF)] {
  _bn: <BN: fe01985a1d8315ac73d6df8638835760dad9dd2f007c29466daff3dfde210776>
}


  nft-staking
Created Collection NFT: AqsV6swPnZxPPbTTQDYyxx6qeZXbi34cANzGHdseg8V2
    ✔ Mint Collection NFT (1772ms)

Created NFT: JA4S6HgjBRqWfLPvNjqd3d7ZQYNb6t2TSDwSEpyVmiaV
    ✔ Mint NFT (706ms)

Collection NFT Verified!
    ✔ Verify Collection NFT (744ms)

Config Account Initialized!
Your transaction signature 42w5DxnnKwzFFKL23JDyT5kYwugsLZUrwrSyr64wjMxSAP3h9Z5kn2NnDMCw4XzNayMStPzc1ZH623G7Wpvrk4R4
    ✔ Initialize Config Account (708ms)

User Account Initialized!
Your transaction signature 5cnZmSWbKnm6cSLU7vyi2vS8E67V6X4frTvX9T7QFKfZKqhYiGMthWzQeLrT15bF7EGrqJKPGW768Ktdy9pXrdqu
    ✔ Initialize User Account (744ms)

NFT Staked!
Your transaction signature 6cbF3rMwWJVqK6vgQeGkYvW4fHwSJAdLtrGwoA6g7SL4jC9SUQRz5vTTSnhKdBbSE13oveQ84LVMV9t3VZU9St8
    ✔ Stake NFT (1454ms)

NFT unstaked!
Your transaction signature sJkV6MPP4pKgkout13SLRhwBTaX6xiNLwMp56dpZwcfWeK67NRJzRCbWfr6vMUBbuB4e3VvQVtCQovtsX8pva6q
user points:  0
    ✔ Unstake NFT (870ms)

Rewards claimed
Your transaction signature 2vsdeZboz8aaMht6725QsEZtds9uZLGkC1MbPRtNaYKAvJC31pJDt6RRRyNgZTiNCJkyG7dJKZUWKszGFQMDRX1Q
user points:  0
    ✔ Claim Rewards (5245ms)


  8 passing (12s)

✨  Done in 13.91s.
```
