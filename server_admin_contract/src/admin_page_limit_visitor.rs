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
}
