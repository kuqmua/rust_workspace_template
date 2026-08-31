#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedStringWrapper,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    chars,
    serde,
    utoipa,
    description = "administrator organization name"
)]
pub struct AdminOrganizationName(
    bounded_types::bounded_string::BoundedString<0usize, { constants_usize::VALUE_8_192 }, true>,
);
