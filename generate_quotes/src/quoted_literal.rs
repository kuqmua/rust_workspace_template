#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
)]
#[bounded_string(max = crate::quoted_literal_max_len::QUOTED_LITERAL_MAX_LEN)]
pub struct QuotedLiteral(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::quoted_literal_max_len::QUOTED_LITERAL_MAX_LEN },
        false,
    >,
);
