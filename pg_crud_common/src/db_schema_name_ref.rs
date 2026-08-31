#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
)]
pub struct DbSchemaNameRef<'value_lt>(&'value_lt str);
