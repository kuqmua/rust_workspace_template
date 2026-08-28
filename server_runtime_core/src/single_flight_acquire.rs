use super::{SingleFlightOwner, SingleFlightWaiter};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub enum SingleFlightAcquire {
    Full,
    Owner(SingleFlightOwner),
    Waiter(SingleFlightWaiter),
}
