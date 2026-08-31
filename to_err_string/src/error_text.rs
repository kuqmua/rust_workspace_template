#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    newtype::BoundedStringWrapper,
    newtype::AsRefStr,
    newtype::DerefTarget,
    newtype::Display,
    newtype::IntoInner,
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
