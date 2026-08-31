#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::Display,
    newtype::FromInner,
)]
pub struct StdPanicLocation(&'static std::panic::Location<'static>);
