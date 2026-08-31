#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct BatchInvalidItems<InvalidItem>(Vec<InvalidItem>);
