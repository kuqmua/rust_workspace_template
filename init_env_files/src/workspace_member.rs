use crate::domain_types::InitStringError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::AsRefStr,
    newtype::Display,
    newtype::TryFrom,
)]
#[try_from(error = InitStringError, validator = |value: &str| {
    if value.is_empty() || value.len() > 4_096usize { Err(InitStringError) } else { Ok(()) }
})]
pub(crate) struct WorkspaceMember(String);
