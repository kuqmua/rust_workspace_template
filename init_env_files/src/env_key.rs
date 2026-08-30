#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::AsRefStr,
    newtype::BorrowStr,
    newtype::TryFrom,
)]
#[try_from(error = crate::init_string_error::InitStringError, validator = |value: &str| {
    if value.is_empty() || value.len() > 1_024usize { Err(crate::init_string_error::InitStringError::Invalid) } else { Ok(()) }
})]
pub(crate) struct EnvKey(String);
