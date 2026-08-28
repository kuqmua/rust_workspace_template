use crate::domain_types::{
    DiskCacheBudgetError, DiskCacheEntry, DiskCacheEvictionPlan, StdDiskCacheSize,
};

pub fn plan_disk_cache_eviction(
    entries: &[DiskCacheEntry],
    maximum: StdDiskCacheSize,
    incoming: StdDiskCacheSize,
) -> Result<DiskCacheEvictionPlan, DiskCacheBudgetError> {
    if incoming.0 > maximum.0 {
        return Err(DiskCacheBudgetError::IncomingTooLarge);
    }
    let mut current = entries
        .iter()
        .try_fold(constants_u64::ZERO, |total, entry| {
            total
                .checked_add(entry.size.0)
                .ok_or(DiskCacheBudgetError::SizeOverflow)
        })?;
    let mut ordered = entries.iter().collect::<Vec<_>>();
    ordered.sort_unstable_by_key(|entry| entry.modified_at.0);
    let projected = current
        .checked_add(incoming.0)
        .ok_or(DiskCacheBudgetError::SizeOverflow)?;
    let required = projected.saturating_sub(maximum.0);
    let remove_capacity = ordered
        .iter()
        .scan(constants_u64::ZERO, |removed, entry| {
            if *removed >= required {
                None
            } else {
                *removed = removed.saturating_add(entry.size.0);
                Some(())
            }
        })
        .count();
    let mut remove = Vec::with_capacity(remove_capacity);
    let mut candidates = ordered.into_iter();
    while current
        .checked_add(incoming.0)
        .ok_or(DiskCacheBudgetError::SizeOverflow)?
        > maximum.0
    {
        let Some(entry) = candidates.next() else {
            break;
        };
        current = current.saturating_sub(entry.size.0);
        remove.push(entry.path.clone());
    }
    Ok(DiskCacheEvictionPlan::from(
        bounded_types::BoundedVec::from_max_iter(remove),
    ))
}
