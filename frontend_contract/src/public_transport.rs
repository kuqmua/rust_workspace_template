#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub struct PublicTransport;
impl crate::route_transport::RouteTransport for PublicTransport {}
