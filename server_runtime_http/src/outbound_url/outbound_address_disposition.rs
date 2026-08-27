#[path = "outbound_address_disposition/outbound_address_disposition.rs"]
mod outbound_address_disposition;

pub(super) use outbound_address_disposition::outbound_address_disposition;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OutboundAddressDisposition {
    Allowed,
    Forbidden,
}
