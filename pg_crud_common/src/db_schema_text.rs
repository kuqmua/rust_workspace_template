#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub struct DbSchemaText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_1_048_576 },
        false,
    >,
);
