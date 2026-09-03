#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[bounded_string(max = crate::case_string_max_len::CASE_STRING_MAX_LEN)]
pub(super) struct CaseString(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::case_string_max_len::CASE_STRING_MAX_LEN },
        false,
    >,
);
