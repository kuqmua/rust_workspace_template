#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct ProcessOutput(std::process::Output);
