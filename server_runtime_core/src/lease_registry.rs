const LEASE_TEXT_MAXIMUM_BYTES: usize = 1024usize;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Hash, PartialEq, newtype::AsRefStr,
)]
pub struct LeaseId(String);
impl TryFrom<String> for LeaseId {
    type Error = LeaseTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > LEASE_TEXT_MAXIMUM_BYTES {
            return Err(LeaseTextError::TooLong);
        }
        validate_lease_text(LeaseTextRef(&value)).map(|()| Self(value))
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Hash, PartialEq, newtype::AsRefStr,
)]
pub struct LeaseKey(String);
impl TryFrom<String> for LeaseKey {
    type Error = LeaseTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > LEASE_TEXT_MAXIMUM_BYTES {
            return Err(LeaseTextError::TooLong);
        }
        validate_lease_text(LeaseTextRef(&value)).map(|()| Self(value))
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum LeaseTextError {
    #[error("lease text contains a NUL character")]
    ContainsNul,
    #[error("lease text must not be empty")]
    Empty,
    #[error("lease text exceeds its maximum length")]
    TooLong,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeaseState {
    Ready,
    Reserved,
    Stale,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct StdLeaseRegistryMaximum(std::num::NonZeroUsize);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdLeaseStaleTimeout(std::time::Duration);
impl TryFrom<std::time::Duration> for StdLeaseStaleTimeout {
    type Error = StdLeaseStaleTimeoutError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdLeaseStaleTimeoutError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("lease stale timeout must be greater than zero")]
pub struct StdLeaseStaleTimeoutError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum LeaseReservation {
    Existing(LeaseId),
    LimitReached,
    Reserved,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeaseHeartbeat {
    Accepted,
    Missing,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct LeaseIds(bounded_types::BoundedVec<LeaseId, 0, { usize::MAX }>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct LeaseEntry {
    heartbeat: TokioLeaseInstant,
    key: LeaseKey,
    state: LeaseState,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
struct LeaseRegistryInner {
    by_id: bounded_types::StdBoundedHashMap<LeaseId, LeaseEntry, { usize::MAX }>,
    by_key: bounded_types::StdBoundedHashMap<LeaseKey, LeaseId, { usize::MAX }>,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub struct LeaseRegistry {
    inner: StdArcTokioLeaseRegistryRwLock,
}
impl LeaseRegistry {
    pub async fn heartbeat(&self, id: &LeaseId) -> LeaseHeartbeat {
        {
            let mut inner = self.inner.0.write().await;
            match inner.by_id.get_mut(id) {
                Some(entry) if entry.state != LeaseState::Stale => {
                    entry.heartbeat = TokioLeaseInstant::from(tokio::time::Instant::now());
                    entry.state = LeaseState::Ready;
                    LeaseHeartbeat::Accepted
                }
                Some(_) | None => LeaseHeartbeat::Missing,
            }
        }
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn release(&self, id: &LeaseId) -> LeaseHeartbeat {
        let mut inner = self.inner.0.write().await;
        let Some(entry) = inner.by_id.remove(id) else {
            return LeaseHeartbeat::Missing;
        };
        let _removed = inner.by_key.remove(&entry.key);
        LeaseHeartbeat::Accepted
    }

    pub async fn reserve(
        &self,
        id: LeaseId,
        key: LeaseKey,
        maximum: StdLeaseRegistryMaximum,
    ) -> LeaseReservation {
        {
            let mut inner = self.inner.0.write().await;
            if let Some(existing_id) = inner.by_key.get(&key)
                && inner
                    .by_id
                    .get(existing_id)
                    .is_some_and(|entry| entry.state != LeaseState::Stale)
            {
                return LeaseReservation::Existing(existing_id.clone());
            }
            remove_stale_entries(&mut inner);
            if inner.by_id.len().get() >= maximum.0.get() {
                return LeaseReservation::LimitReached;
            }
            remove_conflicting_entries(&mut inner, &id, &key);
            let id_insertion = inner.by_key.try_insert(key.clone(), id.clone());
            if id_insertion.is_err() {
                return LeaseReservation::LimitReached;
            }
            let entry_insertion = inner.by_id.try_insert(
                id,
                LeaseEntry {
                    heartbeat: TokioLeaseInstant::from(tokio::time::Instant::now()),
                    key: key.clone(),
                    state: LeaseState::Reserved,
                },
            );
            if entry_insertion.is_err() {
                let _removed_id = inner.by_key.remove(&key);
                drop(inner);
                return LeaseReservation::LimitReached;
            }
            drop(inner);
            LeaseReservation::Reserved
        }
    }

    pub async fn stale(&self, timeout: StdLeaseStaleTimeout) -> LeaseIds {
        LeaseIds::from({
            let mut inner = self.inner.0.write().await;
            let now = tokio::time::Instant::now();
            let mut stale_ids = bounded_types::BoundedVec::default();
            inner
                .by_id
                .iter_mut()
                .filter_map(|(id, entry)| {
                    (now.duration_since(entry.heartbeat.0) > timeout.0).then(|| {
                        entry.state = LeaseState::Stale;
                        id.clone()
                    })
                })
                .for_each(|id| stale_ids.push_max_capacity(id));
            stale_ids
        })
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct LeaseTextRef<'value_lt>(&'value_lt str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, newtype::FromInner)]
struct StdArcTokioLeaseRegistryRwLock(std::sync::Arc<tokio::sync::RwLock<LeaseRegistryInner>>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct TokioLeaseInstant(tokio::time::Instant);

#[allow(clippy::single_call_fn)] // keeps the two-index conflict update atomic and locally auditable
fn remove_conflicting_entries(inner: &mut LeaseRegistryInner, id: &LeaseId, key: &LeaseKey) {
    if let Some(previous) = inner.by_id.remove(id) {
        let _removed = inner.by_key.remove(&previous.key);
    }
    if let Some(previous_id) = inner.by_key.remove(key) {
        let _removed = inner.by_id.remove(&previous_id);
    }
}

#[allow(clippy::single_call_fn)] // keeps stale eviction synchronized across both indexes
fn remove_stale_entries(inner: &mut LeaseRegistryInner) {
    #[allow(clippy::needless_collect)] // ids must be owned before mutating both registry indexes
    let stale = inner
        .by_id
        .iter()
        .filter(|(_id, entry)| entry.state == LeaseState::Stale)
        .map(|(id, _entry)| id.clone())
        .collect::<Vec<_>>();
    stale.into_iter().fold((), |(), id| {
        if let Some(entry) = inner.by_id.remove(&id) {
            let _removed = inner.by_key.remove(&entry.key);
        }
    });
}

fn validate_lease_text(value: LeaseTextRef<'_>) -> Result<(), LeaseTextError> {
    if value.0.is_empty() {
        Err(LeaseTextError::Empty)
    } else if value.0.contains('\0') {
        Err(LeaseTextError::ContainsNul)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    fn id(value: &str) -> super::LeaseId {
        super::LeaseId::try_from(value.to_owned()).expect("f1f58adc id invariant must hold")
    }
    fn key(value: &str) -> super::LeaseKey {
        super::LeaseKey::try_from(value.to_owned()).expect("699f4283 key invariant must hold")
    }
    fn maximum() -> super::StdLeaseRegistryMaximum {
        super::StdLeaseRegistryMaximum::from(std::num::NonZeroUsize::MIN)
    }

    #[tokio::test]
    async fn reservation_is_unique_by_key_and_limit() {
        let registry = super::LeaseRegistry::new();
        let first_id = id(constants_str::TEST_LEASE_ID_ONE);
        let first_key = key(constants_str::TEST_LEASE_KEY_ONE);
        assert_eq!(
            registry
                .reserve(first_id.clone(), first_key.clone(), maximum())
                .await,
            super::LeaseReservation::Reserved
        );
        assert_eq!(
            registry
                .reserve(id(constants_str::TEST_LEASE_ID_TWO), first_key, maximum())
                .await,
            super::LeaseReservation::Existing(first_id)
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::TEST_LEASE_ID_TWO),
                    key(constants_str::TEST_LEASE_KEY_TWO),
                    maximum(),
                )
                .await,
            super::LeaseReservation::LimitReached
        );
    }

    #[tokio::test(start_paused = true)]
    async fn heartbeat_and_stale_transition_are_observable() {
        let registry = super::LeaseRegistry::new();
        let lease_id = id(constants_str::TEST_LEASE_ID_ONE);
        let _reservation = registry
            .reserve(
                lease_id.clone(),
                key(constants_str::TEST_LEASE_KEY_ONE),
                maximum(),
            )
            .await;
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            super::LeaseHeartbeat::Accepted
        );
        tokio::time::advance(std::time::Duration::from_secs(2u64)).await;
        let stale = registry
            .stale(
                super::StdLeaseStaleTimeout::try_from(std::time::Duration::from_secs(1u64)).expect(
                    "8cb64054 heartbeat_and_stale_transition_are_observable invariant must hold",
                ),
            )
            .await;
        assert_eq!(stale.as_ref(), std::slice::from_ref(&lease_id));
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            super::LeaseHeartbeat::Missing
        );
    }
}
