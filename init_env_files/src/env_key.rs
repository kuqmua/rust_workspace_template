use crate::domain_types::InitStringError;

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
#[try_from(error = InitStringError, validator = |value: &str| {
    if value.is_empty() || value.len() > 1_024usize { Err(InitStringError) } else { Ok(()) }
})]
pub(crate) struct EnvKey(String);
