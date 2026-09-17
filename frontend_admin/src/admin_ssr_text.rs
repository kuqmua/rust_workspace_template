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
pub struct AdminSsrText(
    bounded_types::bounded_string::BoundedString<0usize, 16_777_216usize, false>,
);
impl TryFrom<String> for AdminSsrText {
    type Error = crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len() {
            0..=constants_usize::VALUE_16_777_216 => {
                bounded_types::bounded_string::BoundedString::try_from(value)
                    .map(Self)
                    .map_err(|source| match source {
                        bounded_types::bounded_string_error::BoundedStringError::AboveMaximum { .. }
                        | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum { .. } => Self::Error::TooLarge,
                    })
            }
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
        value: crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError,
    ) -> Self {
        bounded_types::try_from_bounded_error_text::try_from_bounded_error_text(value)
    }
}
