use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough SOL to wrap")]
    InsufficientBalance,
    #[msg("Not authorized for this transaction")]
    UnauthorizedAccess,
}