#![allow(clippy::wildcard_imports)] // split test fixtures share the private facade vocabulary

mod create_location_test_text;
mod display_struct;
pub mod domain_types;
mod error_one;
mod error_two;
mod error_unnamed_one;
mod loc_test_text_max_len;
mod location_test_count;
mod location_test_flag;
mod location_test_text;
mod run;
mod serde_struct;

fn main() {
    domain_types::run_location_macro_tests();
}
