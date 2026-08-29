pub fn plan_disk_cache_eviction(
    entries: &[crate::disk_cache_entry::DiskCacheEntry],
    maximum: crate::std_disk_cache_size::StdDiskCacheSize,
    incoming: crate::std_disk_cache_size::StdDiskCacheSize,
) -> Result<
    crate::disk_cache_eviction_plan::DiskCacheEvictionPlan,
    crate::disk_cache_budget_error::DiskCacheBudgetError,
> {
    if incoming.0 > maximum.0 {
        return Err(crate::disk_cache_budget_error::DiskCacheBudgetError::IncomingTooLarge);
    }
    let mut current = entries
        .iter()
        .try_fold(constants_u64::ZERO, |total, entry| {
            total
                .checked_add(entry.size.0)
                .ok_or(crate::disk_cache_budget_error::DiskCacheBudgetError::SizeOverflow)
        })?;
    let mut ordered = entries.iter().collect::<Vec<_>>();
    ordered.sort_unstable_by_key(|entry| entry.modified_at.0);
    let projected = current
        .checked_add(incoming.0)
        .ok_or(crate::disk_cache_budget_error::DiskCacheBudgetError::SizeOverflow)?;
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
        .ok_or(crate::disk_cache_budget_error::DiskCacheBudgetError::SizeOverflow)?
        > maximum.0
    {
        let Some(entry) = candidates.next() else {
            break;
        };
        current = current.saturating_sub(entry.size.0);
        remove.push(entry.path.clone());
    }
    Ok(
        crate::disk_cache_eviction_plan::DiskCacheEvictionPlan::from(
            bounded_types::bounded_vec::BoundedVec::from_max_iter(remove),
        ),
    )
}
