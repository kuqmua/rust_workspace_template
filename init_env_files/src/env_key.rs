#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BorrowStr,
    proc_macro_newtype::TryFrom,
)]
#[try_from(error = crate::init_string_error::InitStringError, validator = |value: &str| {
    if value.is_empty() || value.len() > 1_024usize { Err(crate::init_string_error::InitStringError::Invalid) } else { Ok(()) }
})]
pub(crate) struct EnvKey(String);
