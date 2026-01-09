use crate::LUTS_ID;
use mollusk_helper::{MolluskContextHelper, ADDRESS_LOOKUP_TABLE_PROGRAM_ID};

pub fn create_context() -> MolluskContextHelper {
    let elf = std::fs::read("../../target/deploy/luts.so")
        .expect("Failed to read luts.so - run `anchor build` first");

    let alt_elf = std::fs::read("tests/fixtures/address_lookup_table.so")
        .expect("Failed to read address_lookup_table.so - download from mainnet");

    let mut ctx = MolluskContextHelper::new(&LUTS_ID, &elf);
    ctx.add_program(&ADDRESS_LOOKUP_TABLE_PROGRAM_ID, &alt_elf);

    ctx
}
