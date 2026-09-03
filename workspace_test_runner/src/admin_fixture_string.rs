#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub(crate) struct AdminFixtureString(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_1_048_576 },
        false,
    >,
);
