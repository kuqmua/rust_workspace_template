#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct DbStaticSchemaText(&'static str);
