#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbStaticSchemaTexts(Vec<crate::db_static_schema_text::DbStaticSchemaText>);
