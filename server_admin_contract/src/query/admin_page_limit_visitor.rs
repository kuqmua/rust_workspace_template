use super::AdminPageLimit;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AdminPageLimitVisitor;
impl serde::de::Visitor<'_> for AdminPageLimitVisitor {
    type Value = AdminPageLimit;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an administrator page limit from {} through {}",
            AdminPageLimit::MIN,
            AdminPageLimit::MAX
        )
    }
    fn visit_str<Error>(self, v: &str) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        let parsed = v.parse::<u16>().map_err(serde::de::Error::custom)?;
        AdminPageLimit::try_from(parsed).map_err(serde::de::Error::custom)
    }
    fn visit_u64<Error>(self, v: u64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        let parsed = u16::try_from(v).map_err(serde::de::Error::custom)?;
        AdminPageLimit::try_from(parsed).map_err(serde::de::Error::custom)
    }
}
