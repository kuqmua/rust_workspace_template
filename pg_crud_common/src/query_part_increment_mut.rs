pub trait QueryPartIncrementMut {
    fn checked_add_one(&mut self) -> Option<crate::query_part_increment::QueryPartIncrement>;
}

impl QueryPartIncrementMut for u64 {
    fn checked_add_one(&mut self) -> Option<crate::query_part_increment::QueryPartIncrement> {
        self.checked_add(1).map(|value| {
            *self = value;
            crate::query_part_increment::QueryPartIncrement::from(value)
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_checked_add_one_returns_placeholder_and_updates_counter() {
        let mut counter = crate::query_part_increment::QueryPartIncrement::from(4);
        assert_eq!(
            crate::query_part_increment_mut::QueryPartIncrementMut::checked_add_one(&mut counter),
            Some(crate::query_part_increment::QueryPartIncrement::from(5))
        );
        assert_eq!(counter.get(), 5);
    }

    #[test]
    fn test_checked_add_one_does_not_mutate_counter_on_overflow() {
        let mut counter = crate::query_part_increment::QueryPartIncrement::from(u64::MAX);
        assert_eq!(
            crate::query_part_increment_mut::QueryPartIncrementMut::checked_add_one(&mut counter),
            None
        );
        assert_eq!(counter.get(), u64::MAX);
    }

    #[test]
    fn test_checked_add_one_has_same_behavior_for_legacy_counter() {
        [
            (0u64, Some(1u64), 1u64),
            (4u64, Some(5u64), 5u64),
            (u64::MAX - 1u64, Some(u64::MAX), u64::MAX),
            (u64::MAX, None, u64::MAX),
        ]
        .into_iter()
        .fold((), |(), (value, expected, final_value)| {
            let mut counter = value;
            assert_eq!(
                crate::query_part_increment_mut::QueryPartIncrementMut::checked_add_one(
                    &mut counter,
                ),
                expected.map(crate::query_part_increment::QueryPartIncrement::from),
            );
            assert_eq!(counter, final_value);
        });
    }
}
