pub(super) fn write_inner(
    arc_single_flight_rw_lock: &crate::arc_single_flight_rw_lock::ArcSingleFlightRwLock,
) -> crate::single_flight_rw_lock_write_guard::SingleFlightRwLockWriteGuard<'_> {
    match arc_single_flight_rw_lock.write() {
        Ok(guard) => {
            crate::single_flight_rw_lock_write_guard::SingleFlightRwLockWriteGuard::from(guard)
        }
        Err(poisoned) => {
            crate::single_flight_rw_lock_write_guard::SingleFlightRwLockWriteGuard::from(
                poisoned.into_inner(),
            )
        }
    }
}
