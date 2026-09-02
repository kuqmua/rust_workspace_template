#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    chars,
    serde,
    utoipa,
    description = "administrator organization contacts"
)]
pub struct AdminOrganizationContacts(
    bounded_types::bounded_string::BoundedString<0usize, { constants_usize::VALUE_8_192 }, true>,
);
