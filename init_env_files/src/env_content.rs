use crate::domain_types::InitStringError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(error = InitStringError, validator = |value: &str| {
    if value.len() > usize::try_from(isize::MAX).unwrap_or(usize::MAX) { Err(InitStringError) } else { Ok(()) }
})]
pub(crate) struct EnvContent(String);
impl From<server_runtime_http::domain_types::BoundedText> for EnvContent {
    fn from(value: server_runtime_http::domain_types::BoundedText) -> Self {
        Self(value.into_inner())
    }
}
