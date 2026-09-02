#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct AdminPageOffset(u32);
impl<'de> serde::Deserialize<'de> for AdminPageOffset {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = deserializer
            .deserialize_any(crate::admin_page_offset_visitor::AdminPageOffsetVisitor)?;
        Ok(Self::from(u32::from(value)))
    }
}
