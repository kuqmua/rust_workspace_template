use super::RouteTransport;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthenticatedTransport;
impl RouteTransport for AuthenticatedTransport {}
