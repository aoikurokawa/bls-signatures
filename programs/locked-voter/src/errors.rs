use crate::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Mint Metadata")]
    InvalidMintMetadata,
}