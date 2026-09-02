#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::Display,
    proc_macro_newtype::IntoInner,
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
