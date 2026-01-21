#![cfg(feature = "test-sbf")]

#[path = "../../../codama-rust-luts/mod.rs"]
pub mod codama_rust_luts;

pub use codama_rust_luts::LUTS_ID;

pub mod common;

mod integration {
    pub mod test_close_address_lookup_table;
    pub mod test_create_address_lookup_table;
    pub mod test_deactivate_address_lookup_table;
    pub mod test_extend_address_lookup_table;
}
