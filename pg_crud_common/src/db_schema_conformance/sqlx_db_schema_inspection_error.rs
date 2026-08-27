#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub struct SqlxDbSchemaInspectionError(sqlx::Error);
