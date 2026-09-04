#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub(crate) struct EnvContent(String);
impl TryFrom<String> for EnvContent {
    type Error = crate::init_string_error::InitStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if usize::try_from(isize::MAX).is_ok_and(|max| value.len() > max) {
            Err(Self::Error::Invalid)
        } else {
            Ok(Self(value))
        }
    }
}
impl From<server_runtime_http::bounded_text::BoundedText> for EnvContent {
    fn from(value: server_runtime_http::bounded_text::BoundedText) -> Self {
        Self(value.into_inner())
    }
}
