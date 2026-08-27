use super::AdminPageOffsetVisitor;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct AdminPageOffset(u32);
impl<'de> serde::Deserialize<'de> for AdminPageOffset {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = deserializer.deserialize_any(AdminPageOffsetVisitor)?;
        Ok(Self::from(u32::from(value)))
    }
}
