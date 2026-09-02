#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct AdminHtmlFormText(
    bounded_types::bounded_string::BoundedString<0, { constants_usize::VALUE_8_192 }>,
);
impl TryFrom<String> for AdminHtmlFormText {
    type Error = crate::admin_html_form_text_error::AdminHtmlFormTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(crate::admin_html_form_text_error::AdminHtmlFormTextError::from)
    }
}
