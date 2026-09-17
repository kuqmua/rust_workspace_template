#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct AdminSsrHtml(
    bounded_types::bounded_string::BoundedString<
        { constants_usize::ZERO },
        { constants_usize::VALUE_16_777_216 },
        false,
    >,
);
impl TryFrom<String> for AdminSsrHtml {
    type Error = crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_16_777_216 {
            return Err(
                crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError::TooLarge,
            );
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::TooLarge,
            })
    }
}
impl From<crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError>
    for AdminSsrHtml
{
    fn from(
        value: crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError,
    ) -> Self {
        bounded_types::try_from_bounded_error_text::try_from_bounded_error_text(value)
    }
}
