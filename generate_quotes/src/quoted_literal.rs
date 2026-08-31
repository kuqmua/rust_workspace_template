#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::BoundedStringWrapper,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(max = crate::quoted_literal_max_len::QUOTED_LITERAL_MAX_LEN)]
pub struct QuotedLiteral(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { crate::quoted_literal_max_len::QUOTED_LITERAL_MAX_LEN },
        false,
    >,
);
