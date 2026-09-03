#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[bounded_string(
    max = crate::error_text_max_len::ERROR_TEXT_MAX_LEN,
    serde,
    description = "error text"
)]
pub struct ErrorText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::error_text_max_len::ERROR_TEXT_MAX_LEN },
        false,
    >,
);
