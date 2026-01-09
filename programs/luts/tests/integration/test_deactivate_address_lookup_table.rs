use crate::codama_rust_luts::instructions::{
    CreateAddressLookupTableBuilder, DeactivateAddressLookupTableBuilder,
};
use crate::common::helpers::create_context;
use crate::common::pda::{derive_address_lookup_table, get_user_address_lookup_table_pda};
use mollusk_helper::MolluskContextHelper;
use solana_pubkey::Pubkey;

fn create_lut(
    ctx: &mut MolluskContextHelper,
    signer: &Pubkey,
    id: u64,
    recent_slot: u64,
) -> (Pubkey, Pubkey) {
    ctx.warp_to_slot(recent_slot);
    ctx.warp_to_slot(recent_slot + 1);

    let (user_address_lookup_table, _) = get_user_address_lookup_table_pda(signer, id);
    let (address_lookup_table, _) =
        derive_address_lookup_table(&user_address_lookup_table, recent_slot);

    let instruction = CreateAddressLookupTableBuilder::new()
        .signer(*signer)
        .address_lookup_table(address_lookup_table)
        .user_address_lookup_table(user_address_lookup_table)
        .recent_slot(recent_slot)
        .id(id)
        .instruction();

    let result = ctx.process_instruction(&instruction);
    assert!(
        result.is_ok(),
        "CreateAddressLookupTable should succeed: {:?}",
        result
    );

    (user_address_lookup_table, address_lookup_table)
}

#[test]
fn test_deactivate_address_lookup_table() {
    let mut ctx = create_context();

    let signer = Pubkey::new_unique();
    let id: u64 = 0;
    let recent_slot: u64 = 100;

    ctx.fund_account(&signer, 10_000_000_000);

    let (user_address_lookup_table, address_lookup_table) =
        create_lut(&mut ctx, &signer, id, recent_slot);

    let instruction = DeactivateAddressLookupTableBuilder::new()
        .signer(signer)
        .address_lookup_table(address_lookup_table)
        .user_address_lookup_table(user_address_lookup_table)
        .instruction();

    let result = ctx.process_instruction(&instruction);
    assert!(
        result.is_ok(),
        "DeactivateAddressLookupTable should succeed: {:?}",
        result
    );
}

#[test]
fn test_deactivate_wrong_signer_fails() {
    let mut ctx = create_context();

    let signer = Pubkey::new_unique();
    let wrong_signer = Pubkey::new_unique();
    let id: u64 = 0;
    let recent_slot: u64 = 100;

    ctx.fund_account(&signer, 10_000_000_000);
    ctx.fund_account(&wrong_signer, 10_000_000_000);

    let (user_address_lookup_table, address_lookup_table) =
        create_lut(&mut ctx, &signer, id, recent_slot);

    let instruction = DeactivateAddressLookupTableBuilder::new()
        .signer(wrong_signer)
        .address_lookup_table(address_lookup_table)
        .user_address_lookup_table(user_address_lookup_table)
        .instruction();

    let result = ctx.process_instruction(&instruction);
    assert!(
        result.is_err(),
        "DeactivateAddressLookupTable with wrong signer should fail"
    );
}
