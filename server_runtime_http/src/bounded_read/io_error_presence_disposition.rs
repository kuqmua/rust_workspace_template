#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub enum IoErrorPresenceDisposition {
    Missing,
    Other(super::BoundedReadIoError),
}
