#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbSchemaTexts(Vec<crate::db_schema_text::DbSchemaText>);
