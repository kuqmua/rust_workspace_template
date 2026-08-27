#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::DerefTarget,
    newtype::Display,
    newtype::IntoInner,
)]
#[bounded_string(
    max = super::ERROR_TEXT_MAX_LEN,
    serde,
    description = "error text"
)]
pub struct ErrorText(String);
