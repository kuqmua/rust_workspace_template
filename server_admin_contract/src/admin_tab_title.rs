#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| !value.trim().is_empty(),
    description = "administrator tab title"
)]
pub struct AdminTabTitle(
    bounded_types::bounded_string::BoundedString<
        { constants_usize::ONE },
        { constants_usize::VALUE_8_192 },
        true,
    >,
);
