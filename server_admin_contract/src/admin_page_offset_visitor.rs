#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AdminPageOffsetVisitor;
impl serde::de::Visitor<'_> for AdminPageOffsetVisitor {
    type Value = crate::admin_page_offset::AdminPageOffset;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::catalog::ADMIN_PAGE_OFFSET_EXPECTING)
    }
    fn visit_str<Error>(self, v: &str) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        v.parse::<u32>()
            .map(crate::admin_page_offset::AdminPageOffset::from)
            .map_err(serde::de::Error::custom)
    }
    fn visit_u64<Error>(self, v: u64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        u32::try_from(v)
            .map(crate::admin_page_offset::AdminPageOffset::from)
            .map_err(serde::de::Error::custom)
    }
}
