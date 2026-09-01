pub fn plan_disk_cache_eviction(
    entries: &[crate::disk_cache_entry::DiskCacheEntry],
    maximum: crate::std_disk_cache_size::StdDiskCacheSize,
    incoming: crate::std_disk_cache_size::StdDiskCacheSize,
) -> Result<
    crate::disk_cache_eviction_plan::DiskCacheEvictionPlan,
    crate::disk_cache_budget_error::DiskCacheBudgetError,
> {
    let incoming_size = u64::from(incoming);
    let maximum_size = u64::from(maximum);
    if incoming_size > maximum_size {
        return Err(crate::disk_cache_budget_error::DiskCacheBudgetError::IncomingTooLarge);
    }
    let mut current = entries
        .iter()
        .try_fold(constants_u64::ZERO, |total, entry| {
            total
                .checked_add(u64::from(entry.parts().2))
                .ok_or(crate::disk_cache_budget_error::DiskCacheBudgetError::SizeOverflow)
        })?;
    let mut ordered = entries.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|entry| std::time::SystemTime::from(entry.parts().0));
    let projected = current
        .checked_add(incoming_size)
        .ok_or(crate::disk_cache_budget_error::DiskCacheBudgetError::SizeOverflow)?;
    let required = projected.saturating_sub(maximum_size);
    let remove_capacity = ordered
        .iter()
        .scan(constants_u64::ZERO, |removed, entry| {
            if *removed >= required {
                None
            } else {
                *removed = removed.saturating_add(u64::from(entry.parts().2));
                Some(())
            }
        })
        .count();
    let mut remove = Vec::with_capacity(remove_capacity);
    let mut candidates = ordered.into_iter();
    while current
        .checked_add(incoming_size)
        .ok_or(crate::disk_cache_budget_error::DiskCacheBudgetError::SizeOverflow)?
        > maximum_size
    {
        let Some(entry) = candidates.next() else {
            break;
        };
        current = current.saturating_sub(u64::from(entry.parts().2));
        remove.push(entry.parts().1.clone());
    }
    Ok(
        crate::disk_cache_eviction_plan::DiskCacheEvictionPlan::from(
            bounded_types::bounded_vec::BoundedVec::from_max_iter(remove),
        ),
    )
}
