pub trait QueryPartIncrementMut {
    fn checked_add_one(&mut self) -> Option<crate::query_part_increment::QueryPartIncrement>;
}

impl QueryPartIncrementMut for crate::query_part_increment::QueryPartIncrement {
    fn checked_add_one(&mut self) -> Option<crate::query_part_increment::QueryPartIncrement> {
        self.get().checked_add(1).map(|value| {
            *self = Self::from(value);
            Self::from(value)
        })
    }
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
        let mut counter = 4u64;
        assert_eq!(
            crate::query_part_increment_mut::QueryPartIncrementMut::checked_add_one(&mut counter),
            Some(crate::query_part_increment::QueryPartIncrement::from(5))
        );
        assert_eq!(counter, 5);
    }
}
