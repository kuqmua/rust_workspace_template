#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct AdminSsrHtml(String);
impl TryFrom<String> for AdminSsrHtml {
    type Error = crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > constants_usize::VALUE_16_777_216 {
            return Err(
                crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError::TooLarge,
            );
        }
        Ok(Self(string))
    }
}
impl From<crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError>
    for AdminSsrHtml
{
    fn from(
        admin_ssr_html_try_from_string_error: crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError,
    ) -> Self {
        Self(admin_ssr_html_try_from_string_error.to_string())
    }
}
