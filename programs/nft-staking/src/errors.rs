use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Incorrect mint")]
    IncorrectMint,
    #[msg("Incorrect collection")]
    IncorrectCollection,
    #[msg("Collection not verified")]
    CollectionNotVerified,
    #[msg("Unstaking unavailable for now")]
    UnstakeLimitUnreached,
}
