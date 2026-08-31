pub fn increment_checked_add_one_returning_increment<IncrementTy>(
    increment: &mut IncrementTy,
) -> Result<crate::query_part_increment::QueryPartIncrement, crate::query_part_error::QueryPartError>
where
    IncrementTy: crate::query_part_increment_mut::QueryPartIncrementMut + ?Sized,
{
    increment.checked_add_one().map_or_else(
        || {
            Err(crate::query_part_error::QueryPartError::CheckedAdd {
                location: location_macros::location!(),
            })
        },
        Ok,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_result_api_maps_overflow_without_mutating_counter() {
        let mut counter = crate::query_part_increment::QueryPartIncrement::from(u64::MAX);
        assert!(matches!(
            crate::increment_checked_add_one_returning_increment::increment_checked_add_one_returning_increment(&mut counter),
            Err(crate::query_part_error::QueryPartError::CheckedAdd { .. })
        ));
        assert_eq!(counter.get(), u64::MAX);
    }
}
