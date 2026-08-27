use super::{ArcSingleFlightRwLock, SingleFlightRwLockWriteGuard};

pub(super) fn write_inner(inner: &ArcSingleFlightRwLock) -> SingleFlightRwLockWriteGuard<'_> {
    match inner.0.write() {
        Ok(guard) => SingleFlightRwLockWriteGuard::from(guard),
        Err(poisoned) => SingleFlightRwLockWriteGuard::from(poisoned.into_inner()),
    }
}
