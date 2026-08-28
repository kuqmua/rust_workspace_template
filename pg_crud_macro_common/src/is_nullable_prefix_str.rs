use super::IS_NL_PREFIX_STR_MAX_LEN;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = IS_NL_PREFIX_STR_MAX_LEN, description = "is nullable prefix string" )]
pub struct IsNullablePrefixStr(String);
