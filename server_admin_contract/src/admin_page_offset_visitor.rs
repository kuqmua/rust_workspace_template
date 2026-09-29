#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AdminPageOffsetVisitor;
impl serde::de::Visitor<'_> for AdminPageOffsetVisitor {
    type Value = crate::admin_page_offset::AdminPageOffset;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::ADMIN_PAGE_OFFSET_EXPECTING)
    }
    fn visit_str<Error>(self, str: &str) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        str.parse::<u32>()
            .map(crate::admin_page_offset::AdminPageOffset::from)
            .map_err(serde::de::Error::custom)
    }
    fn visit_u64<Error>(self, u64: u64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        u32::try_from(u64)
            .map(crate::admin_page_offset::AdminPageOffset::from)
            .map_err(serde::de::Error::custom)
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
