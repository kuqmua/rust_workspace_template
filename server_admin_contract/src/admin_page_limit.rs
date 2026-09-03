#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminPageLimit(u16);
impl From<crate::admin_default_page_limit::AdminDefaultPageLimit> for AdminPageLimit {
    fn from(
        admin_default_page_limit: crate::admin_default_page_limit::AdminDefaultPageLimit,
    ) -> Self {
        let _: crate::admin_default_page_limit::AdminDefaultPageLimit = admin_default_page_limit;
        Self(Self::DEFAULT)
    }
}
impl<'de> serde::Deserialize<'de> for AdminPageLimit {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value =
            deserializer.deserialize_any(crate::admin_page_limit_visitor::AdminPageLimitVisitor)?;
        Self::try_from(u16::from(value)).map_err(serde::de::Error::custom)
    }
}
impl Default for AdminPageLimit {
    fn default() -> Self {
        Self::from(crate::admin_default_page_limit::AdminDefaultPageLimit)
    }
}
impl TryFrom<u16> for AdminPageLimit {
    type Error = crate::admin_page_limit_error::AdminPageLimitError;
    fn try_from(u16: u16) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&u16) {
            Ok(Self(u16))
        } else {
            Err(crate::admin_page_limit_error::AdminPageLimitError::OutOfRange)
        }
    }
}
impl AdminPageLimit {
    pub const DEFAULT: u16 = 20u16;
    pub const MAX: u16 = 100u16;
    pub const MIN: u16 = 1u16;
}
