use anchor_lang::error_code;

#[error_code]
pub enum MiddlewareError {
    #[msg("InSufficientFunds")]
    InSufficientFunds,
    #[msg("PermissionDenied")]
    PermissionDenied,
    #[msg("UnknownError")]
    UnknownError,
}