use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub points: u32, // points the user has earnt
    pub amount_staked: u8, // amount of NFTs staken
    pub bump: u8, // for generating this PDA
}

impl Space for UserAccount {
    const INIT_SPACE: usize = 8 + 4 + 1 + 1;
}
