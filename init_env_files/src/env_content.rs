#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub(crate) struct EnvContent(
    bounded_types::bounded_string::BoundedString<0usize, { usize::MAX >> 1usize }, false>,
);
impl TryFrom<String> for EnvContent {
    type Error = crate::init_string_error::InitStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if usize::try_from(isize::MAX).is_ok_and(|max| value.len() > max) {
            Err(Self::Error::Invalid)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::Invalid,
                })
        }
    }
}
impl TryFrom<server_runtime_http::bounded_text::BoundedText> for EnvContent {
    type Error = server_runtime_http::bounded_read_error::BoundedReadError;

    fn try_from(
        value: server_runtime_http::bounded_text::BoundedText,
    ) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner()).map_err(|source| match source {
            crate::init_string_error::InitStringError::Invalid => Self::Error::ExceedsMaximum {
                maximum_bytes:
                    server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                        usize::MAX >> 1usize,
                    ),
            },
        })
    }
}
