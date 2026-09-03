#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::Display,
    proc_macro_newtype::IntoInner,
)]
#[bounded_string(max = crate::case_string_max_len::CASE_STRING_MAX_LEN)]
pub(super) struct CaseString(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::case_string_max_len::CASE_STRING_MAX_LEN },
        false,
    >,
);
