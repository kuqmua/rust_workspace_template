#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AdminPageLimitVisitor;
impl serde::de::Visitor<'_> for AdminPageLimitVisitor {
    type Value = crate::admin_page_limit::AdminPageLimit;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an administrator page limit from {} through {}",
            crate::admin_page_limit::AdminPageLimit::MIN,
            crate::admin_page_limit::AdminPageLimit::MAX
        )
    }
    fn visit_str<Error>(self, str: &str) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        let parsed = str.parse::<u16>().map_err(serde::de::Error::custom)?;
        crate::admin_page_limit::AdminPageLimit::try_from(parsed).map_err(serde::de::Error::custom)
    }
    fn visit_u64<Error>(self, u64: u64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        let parsed = u16::try_from(u64).map_err(serde::de::Error::custom)?;
        crate::admin_page_limit::AdminPageLimit::try_from(parsed).map_err(serde::de::Error::custom)
    }
    fn visit_i64<Error>(self, i64: i64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        crate::visit_checked_unsigned_integer::visit_checked_unsigned_integer(self, i64)
    }
    fn visit_i128<Error>(self, i128: i128) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        crate::visit_checked_unsigned_integer::visit_checked_unsigned_integer(self, i128)
    }
    fn visit_u128<Error>(self, u128: u128) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        crate::visit_checked_unsigned_integer::visit_checked_unsigned_integer(self, u128)
    }
}
