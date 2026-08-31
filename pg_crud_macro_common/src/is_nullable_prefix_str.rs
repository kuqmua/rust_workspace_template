#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::BoundedStringWrapper,
    newtype::Display,
)]
#[bounded_string(max = crate::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN, description = "is nullable prefix string" )]
pub struct IsNullablePrefixStr(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN },
        false,
    >,
);
