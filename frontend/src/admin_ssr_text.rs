#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct AdminSsrText(String);
impl TryFrom<String> for AdminSsrText {
    type Error = crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        match string.len() {
            0..=constants_usize::VALUE_16_777_216 => Ok(Self(string)),
            _ => Err(
                crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError::TooLarge,
            ),
        }
    }
}
impl From<crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError>
    for AdminSsrText
{
    fn from(
        admin_ssr_text_try_from_string_error: crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError,
    ) -> Self {
        Self(admin_ssr_text_try_from_string_error.to_string())
    }
}
