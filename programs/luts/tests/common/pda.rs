use crate::LUTS_ID;
use mollusk_helper::ADDRESS_LOOKUP_TABLE_PROGRAM_ID;
use solana_pubkey::Pubkey;

pub const USER_ADDRESS_LOOKUP_TABLE_SEED: &str = "UserAddressLookupTable";

pub fn get_user_address_lookup_table_pda(signer: &Pubkey, id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            USER_ADDRESS_LOOKUP_TABLE_SEED.as_bytes(),
            signer.as_ref(),
            &id.to_le_bytes(),
        ],
        &LUTS_ID,
    )
}

pub fn derive_address_lookup_table(authority: &Pubkey, recent_slot: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[authority.as_ref(), &recent_slot.to_le_bytes()],
        &ADDRESS_LOOKUP_TABLE_PROGRAM_ID,
    )
}
