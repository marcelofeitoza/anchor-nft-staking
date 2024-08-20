use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;

pub use instructions::*;

declare_id!("nftmapxi1xxp8F4TiU3cZ7xxEd1kFcT5aSnpRZaqa3U");

#[program]
pub mod nft_staking {
    use super::*;

    /// Initializes the configuration for the staking
    /// pointer_per_stake: Amount of points each stake values
    /// max_stake: Maximum amount of NFTs to be staken
    /// freeze_period: Mininum time for a staken NFT to be withdrawn
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        point_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .initialize_config(point_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    /// Initializes the user with the given bumps for the PDAs
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    /// Stakes a NFT for the user
    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    /// Unstakes a NFT for the user
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    /// Claims the rewards for the staking
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
