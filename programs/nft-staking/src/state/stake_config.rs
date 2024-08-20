use anchor_lang::prelude::*;

#[account]
pub struct StakeConfig {
    pub points_per_stake: u8, // amount of points the stake is valued
    pub max_stake: u8, // maximum amount of NFTs to be staken
    pub freeze_period: u32, // period in which the staken NFTs can't be withdrawn
    pub rewards_bump: u8, 
    pub bump: u8
}

impl Space for StakeConfig {
    const INIT_SPACE: usize = 8 + 1 + 1 + 4 + 1 + 1;
}
