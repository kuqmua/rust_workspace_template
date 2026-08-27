#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::Display,
)]
pub struct OrderUpperCamelCaseStr(crate::domain_types::OrderTextString);

impl From<crate::domain_types::PgCrudStringWrapperTryFromStringError> for OrderUpperCamelCaseStr {
    fn from(value: crate::domain_types::PgCrudStringWrapperTryFromStringError) -> Self {
        Self(crate::domain_types::OrderTextString::from(value))
    }
}

impl TryFrom<String> for OrderUpperCamelCaseStr {
    type Error = crate::domain_types::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::domain_types::OrderTextString::try_from(value).map(Self)
    }
}
