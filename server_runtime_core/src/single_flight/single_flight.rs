use super::{
    ArcSingleFlightRwLock, SingleFlightAcquire, SingleFlightKey, SingleFlightMaximumNonZeroUsize,
    SingleFlightOwner, SingleFlightSignal, SingleFlightWaiter, TokioSingleFlightReceiver,
    TokioSingleFlightSender, write_inner,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct SingleFlight {
    inner: ArcSingleFlightRwLock,
    maximum: SingleFlightMaximumNonZeroUsize,
}
impl SingleFlight {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // the lock-backed return value cannot be constructed in const context
    pub fn acquire(&self, key: SingleFlightKey) -> SingleFlightAcquire {
        let mut inner = write_inner(&self.inner);
        if let Some(sender) = inner.flights.get(&key) {
            return SingleFlightAcquire::Waiter(SingleFlightWaiter::from(
                TokioSingleFlightReceiver::from(sender.0.subscribe()),
            ));
        }
        if inner.flights.len().get() >= self.maximum.0.get() {
            return SingleFlightAcquire::Full;
        }
        let (sender, _) = tokio::sync::watch::channel(SingleFlightSignal::Running);
        let insertion = inner
            .flights
            .try_insert(key.clone(), TokioSingleFlightSender::from(sender));
        if insertion.is_err() {
            return SingleFlightAcquire::Full;
        }
        drop(inner);
        SingleFlightAcquire::Owner(SingleFlightOwner {
            inner: self.inner.clone(),
            key: Some(key),
        })
    }

    #[must_use]
    pub fn new(maximum: SingleFlightMaximumNonZeroUsize) -> Self {
        Self {
            inner: ArcSingleFlightRwLock::default(),
            maximum,
        }
    }
}
