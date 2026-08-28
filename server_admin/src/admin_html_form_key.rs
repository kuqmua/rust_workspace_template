use crate::AdminHtmlFormKeyError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
pub(crate) struct AdminHtmlFormKey(
    bounded_types::bounded_string::BoundedString<0, { constants_usize::VALUE_8_192 }>,
);
impl TryFrom<String> for AdminHtmlFormKey {
    type Error = AdminHtmlFormKeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(AdminHtmlFormKeyError::from)
    }
}
