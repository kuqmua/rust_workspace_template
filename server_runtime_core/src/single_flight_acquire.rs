#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub enum SingleFlightAcquire {
    Full,
    Owner(crate::single_flight_owner::SingleFlightOwner),
    Waiter(crate::single_flight_waiter::SingleFlightWaiter),
}
