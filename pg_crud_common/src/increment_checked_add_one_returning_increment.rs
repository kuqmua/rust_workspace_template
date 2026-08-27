pub fn increment_checked_add_one_returning_increment<IncrementTy>(
    increment: &mut IncrementTy,
) -> Result<super::QueryPartIncrement, crate::domain_types::QueryPartError>
where
    IncrementTy: super::QueryPartIncrementMut + ?Sized,
{
    increment.checked_add_one().map_or_else(
        || {
            Err(crate::domain_types::QueryPartError::CheckedAdd {
                location: location_macros::location!(),
            })
        },
        Ok,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn result_api_maps_overflow_without_mutating_counter() {
        let mut counter = super::super::QueryPartIncrement::from(u64::MAX);
        assert!(matches!(
            super::increment_checked_add_one_returning_increment(&mut counter),
            Err(crate::domain_types::QueryPartError::CheckedAdd { .. })
        ));
        assert_eq!(counter.get(), u64::MAX);
    }
}
