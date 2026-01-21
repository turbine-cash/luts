use crate::codama_rust_luts::instructions::CreateAddressLookupTableBuilder;
use crate::common::helpers::create_context;
use crate::common::pda::{derive_address_lookup_table, get_user_address_lookup_table_pda};
use solana_pubkey::Pubkey;

#[test]
fn test_create_address_lookup_table() {
    let mut ctx = create_context();

    let signer = Pubkey::new_unique();
    let id: u64 = 0;

    ctx.fund_account(&signer, 10_000_000_000);

    ctx.warp_to_slot(100);
    ctx.warp_to_slot(101);
    let recent_slot: u64 = 100;

    let (user_address_lookup_table, _bump) = get_user_address_lookup_table_pda(&signer, id);
    let (address_lookup_table, _) =
        derive_address_lookup_table(&user_address_lookup_table, recent_slot);

    let instruction = CreateAddressLookupTableBuilder::new()
        .signer(signer)
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

    let wrapper_account = ctx.get_account(&user_address_lookup_table);
    assert!(
        wrapper_account.is_some(),
        "UserAddressLookupTable account should exist"
    );
}

#[test]
fn test_create_multiple_luts_same_signer() {
    let mut ctx = create_context();

    let signer = Pubkey::new_unique();

    ctx.fund_account(&signer, 10_000_000_000);

    for id in 0..3u64 {
        let recent_slot = 100 + id;
        ctx.warp_to_slot(recent_slot);
        ctx.warp_to_slot(recent_slot + 1);

        let (user_address_lookup_table, _) = get_user_address_lookup_table_pda(&signer, id);
        let (address_lookup_table, _) =
            derive_address_lookup_table(&user_address_lookup_table, recent_slot);

        let instruction = CreateAddressLookupTableBuilder::new()
            .signer(signer)
            .address_lookup_table(address_lookup_table)
            .user_address_lookup_table(user_address_lookup_table)
            .recent_slot(recent_slot)
            .id(id)
            .instruction();

        let result = ctx.process_instruction(&instruction);
        assert!(
            result.is_ok(),
            "CreateAddressLookupTable for id {} should succeed: {:?}",
            id,
            result
        );
    }
}

#[test]
fn test_create_lut_wrong_address_fails() {
    let mut ctx = create_context();

    let signer = Pubkey::new_unique();
    let id: u64 = 0;

    ctx.fund_account(&signer, 10_000_000_000);
    ctx.warp_to_slot(100);
    ctx.warp_to_slot(101);
    let recent_slot: u64 = 100;

    let (user_address_lookup_table, _) = get_user_address_lookup_table_pda(&signer, id);
    let wrong_address_lookup_table = Pubkey::new_unique();

    let instruction = CreateAddressLookupTableBuilder::new()
        .signer(signer)
        .address_lookup_table(wrong_address_lookup_table)
        .user_address_lookup_table(user_address_lookup_table)
        .recent_slot(recent_slot)
        .id(id)
        .instruction();

    let result = ctx.process_instruction(&instruction);
    assert!(
        result.is_err(),
        "CreateAddressLookupTable with wrong address should fail"
    );
}
