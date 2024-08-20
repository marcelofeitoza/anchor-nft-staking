use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::{StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    /// User claiming the rewards, signing, and mutable to pay for the fees
    #[account(mut)]
    pub user: Signer<'info>,

    /// User account holding the points earnt
    #[account(
        mut,
        seeds = [
            b"user",
            user.key().as_ref()
        ],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    /// Mint account used to mint the rewards
    #[account(
        mut,
        seeds = [
            b"rewards",
            config.key().as_ref()
        ],
        bump = config.rewards_bump
    )]
    pub rewards_mint: Account<'info, Mint>,

    /// Stake configuration account
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    /// ATA Account containing in which the rewards will be deposited
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        // Parameters preparation for the CPI, specifying the mint, tokens to be sent, and authority
        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let seeds = &[b"config".as_ref(), &[self.config.bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Mints the reward tokens
        mint_to(
            cpi_context,
            self.user_account.points as u64 * 10_u64.pow(self.rewards_mint.decimals as u32),
        )?;

        // User points are reset
        self.user_account.points = 0;

        Ok(())
    }
}
