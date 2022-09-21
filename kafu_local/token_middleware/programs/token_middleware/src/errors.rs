use anchor_lang::error_code;

#[error_code]
pub enum MarketError {
    #[msg("InSufficientFunds")]
    InSufficientFunds,
    #[msg("NotSupportCoin")]
    NotSupportCoin,
    #[msg("NftNotMatched")]
    NftNotMatched,
    #[msg("SellerNotMatched")]
    SellerNotMatched,
    #[msg("UnknownError")]
    UnknownError,
}
