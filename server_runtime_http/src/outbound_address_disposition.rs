#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OutboundAddressDisposition {
    Allowed,
    Forbidden,
}
