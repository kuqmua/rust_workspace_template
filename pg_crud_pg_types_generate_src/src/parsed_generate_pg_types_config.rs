#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct ParsedGeneratePgTypesConfig(crate::generate_pg_types_config::GeneratePgTypesConfig);
