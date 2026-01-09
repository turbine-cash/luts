use anchor_lang::prelude::*;

#[error_code]
pub enum LutError {
    #[msg("Invalid Lookup Table address")]
    InvalidLookupTable,
    #[msg("LUT not yet ready - cooldown period not passed")]
    LutNotReady,
    #[msg("Maximum addresses exceeded (256 limit)")]
    MaxAddressesExceeded,
    #[msg("No new addresses to add")]
    NoNewAddresses,
}
