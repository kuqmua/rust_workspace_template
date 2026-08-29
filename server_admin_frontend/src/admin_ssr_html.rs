#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::IntoInnerFrom,
)]
pub struct AdminSsrHtml(String);
impl TryFrom<String> for AdminSsrHtml {
    type Error = crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= constants_usize::VALUE_16_777_216)
            .then_some(Self(value))
            .ok_or(crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError)
    }
}
impl From<crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError>
    for AdminSsrHtml
{
    fn from(
        value: crate::admin_ssr_html_try_from_string_error::AdminSsrHtmlTryFromStringError,
    ) -> Self {
        Self(value.to_string())
    }
}
