use crate::AdminHtmlFormTextError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
pub(crate) struct AdminHtmlFormText(
    bounded_types::bounded_string::BoundedString<0, { constants_usize::VALUE_8_192 }>,
);
impl TryFrom<String> for AdminHtmlFormText {
    type Error = AdminHtmlFormTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(AdminHtmlFormTextError::from)
    }
}
