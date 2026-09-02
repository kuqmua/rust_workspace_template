#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::TryFrom,
)]
#[try_from(error = crate::init_string_error::InitStringError, validator = |value: &str| {
    if value.len() > usize::try_from(isize::MAX).unwrap_or(usize::MAX) { Err(crate::init_string_error::InitStringError::Invalid) } else { Ok(()) }
})]
pub(crate) struct EnvContent(String);
impl From<server_runtime_http::bounded_text::BoundedText> for EnvContent {
    fn from(value: server_runtime_http::bounded_text::BoundedText) -> Self {
        Self(value.into_inner())
    }
}
