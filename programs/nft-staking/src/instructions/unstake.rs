use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{revoke, Mint, Revoke, Token, TokenAccount},
};

use crate::{
    errors::StakeError,
    state::{StakeAccount, StakeConfig, UserAccount},
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    /// User executing the unstake operation, signer of the transaction, mut because will be deducted the transaction fee
    #[account(mut)]
    pub user: Signer<'info>,

    /// Token Mint account
    pub mint: Account<'info, Mint>,

    /// ATA holding the NFT
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    /// Account holding the metadata associated to the NFT
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub metadata: Account<'info, MetadataAccount>,

    /// Master edition of the NFT, important for the NFT related operations
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    /// Staking configurations, holding the pointer earnt by stake, freeze period, etc.
    pub config: Account<'info, StakeConfig>,

    // Account holding the information about the stake, such as the owner of the NFT, last update, bump and seed
    #[account(
        mut,
        close = user,
        seeds = [
            b"stake".as_ref(),
            mint.key().as_ref(),
            config.key().as_ref()
        ],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    /// Holds the information about the user, its points, amount staken, etc.
    #[account(
        mut,
        seeds = [
            b"user",
            user.key().as_ref()
        ],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        // Calculate and update points earnt by the user in the time the NFT was staken
        let time_elapsed =
            ((Clock::get()?.unix_timestamp - self.stake_account.last_update) / 86400) as u32;

        require!(
            time_elapsed >= self.config.freeze_period,
            StakeError::UnstakeLimitUnreached
        );

        self.user_account.points += time_elapsed * (self.config.points_per_stake as u32);

        // Send the NFT back to the user
        // Uses the seeds from the StakeAccount
        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        // Calls the CPI to unfreeze the token account, allowing it to be managed again
        ThawDelegatedAccountCpi::new(
            metadata_program,
            ThawDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        )
        .invoke_signed(signer_seeds)?;

        // Revokes delegation authority, removing control of the program over the NFT
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Revoke {
            source: self.mint_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        revoke(cpi_ctx)?;

        // Updates the user amount in stake, removing the one unstaken in this function
        self.user_account.amount_staked -= 1;

        Ok(())
    }
}
