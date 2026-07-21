const SINGLE_FLIGHT_KEY_MAXIMUM_BYTES: usize = 1024usize;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SingleFlightKey(String);
impl TryFrom<String> for SingleFlightKey {
    type Error = SingleFlightKeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SINGLE_FLIGHT_KEY_MAXIMUM_BYTES {
            return Err(SingleFlightKeyError::TooLong);
        }
        if value.is_empty() {
            Err(SingleFlightKeyError::Empty)
        } else if value.contains('\0') {
            Err(SingleFlightKeyError::ContainsNul)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SingleFlightKeyError {
    #[error("single-flight key contains a NUL character")]
    ContainsNul,
    #[error("single-flight key must not be empty")]
    Empty,
    #[error("single-flight key exceeds its maximum length")]
    TooLong,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdSingleFlightMaximum(std::num::NonZeroUsize);
impl From<std::num::NonZeroUsize> for StdSingleFlightMaximum {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
pub struct SingleFlight {
    inner: StdArcStdSingleFlightRwLock,
    maximum: StdSingleFlightMaximum,
}
impl SingleFlight {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // the lock-backed return value cannot be constructed in const context
    pub fn acquire(&self, key: SingleFlightKey) -> SingleFlightAcquire {
        let mut inner = write_inner(&self.inner);
        if let Some(sender) = inner.flights.get(&key) {
            return SingleFlightAcquire::Waiter(SingleFlightWaiter::from(TokioSingleFlightReceiver::from(
                sender.0.subscribe(),
            )));
        }
        if inner.flights.len() >= self.maximum.0.get() {
            return SingleFlightAcquire::Full;
        }
        let (sender, receiver) = tokio::sync::watch::channel(SingleFlightSignal::Running);
        drop(receiver);
        let _previous = inner
            .flights
            .insert(key.clone(), TokioSingleFlightSender::from(sender));
        drop(inner);
        SingleFlightAcquire::Owner(SingleFlightOwner {
            inner: self.inner.clone(),
            key: Some(key),
        })
    }

    #[must_use]
    pub fn new(maximum: StdSingleFlightMaximum) -> Self {
        Self {
            inner: StdArcStdSingleFlightRwLock::default(),
            maximum,
        }
    }
}

#[derive(Debug)]
pub enum SingleFlightAcquire {
    Full,
    Owner(SingleFlightOwner),
    Waiter(SingleFlightWaiter),
}

#[derive(Debug)]
#[must_use]
pub struct SingleFlightOwner {
    inner: StdArcStdSingleFlightRwLock,
    key: Option<SingleFlightKey>,
}
impl Drop for SingleFlightOwner {
    fn drop(&mut self) {
        let Some(key) = self.key.take() else {
            return;
        };
        let optional_sender = write_inner(&self.inner).flights.remove(&key);
        if let Some(sender) = optional_sender {
            let _send_result = sender.0.send(SingleFlightSignal::Retry);
        }
    }
}

#[derive(Debug)]
pub struct SingleFlightWaiter(TokioSingleFlightReceiver);
impl From<TokioSingleFlightReceiver> for SingleFlightWaiter {
    fn from(value: TokioSingleFlightReceiver) -> Self {
        Self(value)
    }
}
impl SingleFlightWaiter {
    pub async fn wait(mut self) -> SingleFlightWaitOutcome {
        match self.0.0.changed().await {
            Ok(()) | Err(_) => SingleFlightWaitOutcome::Retry,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SingleFlightWaitOutcome {
    Retry,
}

#[derive(Debug, Default)]
struct SingleFlightInner {
    flights: std::collections::HashMap<SingleFlightKey, TokioSingleFlightSender>,
}

#[derive(Clone, Debug, Default)]
struct StdArcStdSingleFlightRwLock(std::sync::Arc<StdSingleFlightRwLock>);
impl From<std::sync::Arc<StdSingleFlightRwLock>> for StdArcStdSingleFlightRwLock {
    fn from(value: std::sync::Arc<StdSingleFlightRwLock>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default)]
struct StdSingleFlightRwLock(std::sync::RwLock<SingleFlightInner>);
impl From<std::sync::RwLock<SingleFlightInner>> for StdSingleFlightRwLock {
    fn from(value: std::sync::RwLock<SingleFlightInner>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
struct StdSingleFlightWriteGuard<'value_lt>(
    std::sync::RwLockWriteGuard<'value_lt, SingleFlightInner>,
);
impl<'value_lt> From<std::sync::RwLockWriteGuard<'value_lt, SingleFlightInner>>
    for StdSingleFlightWriteGuard<'value_lt>
{
    fn from(value: std::sync::RwLockWriteGuard<'value_lt, SingleFlightInner>) -> Self {
        Self(value)
    }
}
impl std::ops::Deref for StdSingleFlightWriteGuard<'_> {
    type Target = SingleFlightInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for StdSingleFlightWriteGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SingleFlightSignal {
    Retry,
    Running,
}

#[derive(Clone, Debug)]
struct TokioSingleFlightReceiver(tokio::sync::watch::Receiver<SingleFlightSignal>);
impl From<tokio::sync::watch::Receiver<SingleFlightSignal>> for TokioSingleFlightReceiver {
    fn from(value: tokio::sync::watch::Receiver<SingleFlightSignal>) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
struct TokioSingleFlightSender(tokio::sync::watch::Sender<SingleFlightSignal>);
impl From<tokio::sync::watch::Sender<SingleFlightSignal>> for TokioSingleFlightSender {
    fn from(value: tokio::sync::watch::Sender<SingleFlightSignal>) -> Self {
        Self(value)
    }
}

fn write_inner(inner: &StdArcStdSingleFlightRwLock) -> StdSingleFlightWriteGuard<'_> {
    match inner.0.0.write() {
        Ok(guard) => StdSingleFlightWriteGuard::from(guard),
        Err(poisoned) => StdSingleFlightWriteGuard::from(poisoned.into_inner()),
    }
}

#[cfg(test)]
mod tests {
    fn key() -> super::SingleFlightKey {
        super::SingleFlightKey::try_from(String::from(str_constants::TEST_SINGLE_FLIGHT_KEY))
            .expect("68276323")
    }

    #[tokio::test]
    async fn one_owner_notifies_waiters_and_releases_key() {
        let flights = super::SingleFlight::new(super::StdSingleFlightMaximum::from(
            std::num::NonZeroUsize::MIN,
        ));
        let super::SingleFlightAcquire::Owner(owner) = flights.acquire(key()) else {
            panic!("7961dd01");
        };
        let super::SingleFlightAcquire::Waiter(waiter) = flights.acquire(key()) else {
            panic!("5b54e5a1");
        };
        drop(owner);
        assert_eq!(waiter.wait().await, super::SingleFlightWaitOutcome::Retry);
        assert!(matches!(
            flights.acquire(key()),
            super::SingleFlightAcquire::Owner(_)
        ));
    }
}
