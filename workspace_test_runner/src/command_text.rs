#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(super) struct CommandText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_16_777_216 },
        false,
    >,
);
