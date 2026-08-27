use super::QUOTED_LITERAL_MAX_LEN;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(max = QUOTED_LITERAL_MAX_LEN)]
pub struct QuotedLiteral(pub(super) String);
