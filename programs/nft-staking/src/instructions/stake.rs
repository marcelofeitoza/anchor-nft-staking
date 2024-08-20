use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::state::{StakeAccount, StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Stake<'info> {
    /// User executing the operations, mutable because is the fee payer and will have its lamports balance changed
    #[account(mut)]
    pub user: Signer<'info>,

    /// Mint account and its collections
    pub mint: Account<'info, Mint>,
    pub collection: Account<'info, Mint>,

    /// Associated token acocunt in which the NFT will be staken
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    /// metadata and edition are the metadata account and master edition, used to verify if the NFT is part of the collection specified or whether it's frozen or not
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified,
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>, //  MasterEditionAccount tracks the information of the NFT

    /// account holding the configuration details of the staking such as the points per stake, etc.
    pub config: Account<'info, StakeConfig>,

    /// account initialized to store information about the staking such as the owner, NFT mint and last update
    #[account(
        init,
        payer = user,
        space = StakeAccount::INIT_SPACE,
        seeds = [
            b"stake".as_ref(),
            mint.key().as_ref(), 
            config.key().as_ref()
        ],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    /// user accounts storing how much NFTs are staken for the User and points earnt
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Stake<'info> {
    pub fn stake(
        &mut self,
        bumps: &StakeBumps
    ) -> Result<()> {
       // initializes the stake account with the information of the staking being made
        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.mint.key(),
            last_update: Clock::get()?.unix_timestamp, // current time in unix number (i64)
            bump: bumps.stake_account
        });

        // Approves the ata an amount of 1 NFT to be staken
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        approve(cpi_ctx, 1)?;

        // Freezes the account to prevent movementations of the NFT while staked
        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];
        let signer_seeds = &[&seeds[..]];
        
        let delegate = &self.stake_account.to_account_info(); // StakeAccount, the delegate that has control over the NFT
        let token_account = &self.mint_ata.to_account_info(); // ATA that holds the NFT
        let edition = &self.edition.to_account_info(); // Master edition of the NFT, managing its properties
        let mint = &self.mint.to_account_info(); // Mint account of the NFT that manages its creation
        let token_program = &self.token_program.to_account_info(); // SPL Token program that manages operations
        let metadata_program = &self.metadata_program.to_account_info(); // Manages the associated tokens metadata

        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program
            }
        ).invoke_signed(signer_seeds)?; // Freezees the NFT so that it can't be moved while staked and only the delegate can control it

        self.user_account.amount_staked += 1; // increments the amount of NFTs staken by the user

        Ok(())
    }
}
