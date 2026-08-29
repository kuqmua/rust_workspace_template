#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub enum IoErrorPresenceDisposition {
    Missing,
    Other(crate::bounded_read_io_error::BoundedReadIoError),
}
