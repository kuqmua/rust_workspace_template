#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    optml::Optml,
    newtype::Display,
    newtype::FromInner,
)]
pub struct QueryPartIncrement(u64);
impl QueryPartIncrement {
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}
pub trait QueryPartIncrementMut {
    fn checked_add_one(&mut self) -> Option<QueryPartIncrement>;
}
impl QueryPartIncrementMut for QueryPartIncrement {
    fn checked_add_one(&mut self) -> Option<QueryPartIncrement> {
        self.0.checked_add(1).map(|value| {
            *self = Self::from(value);
            Self::from(value)
        })
    }
}
impl QueryPartIncrementMut for u64 {
    fn checked_add_one(&mut self) -> Option<QueryPartIncrement> {
        self.checked_add(1).map(|value| {
            *self = value;
            QueryPartIncrement::from(value)
        })
    }
}
pub fn increment_checked_add_one_returning_increment<IncrementTy>(
    increment: &mut IncrementTy,
) -> Result<QueryPartIncrement, crate::QueryPartError>
where
    IncrementTy: QueryPartIncrementMut + ?Sized,
{
    increment.checked_add_one().map_or_else(
        || {
            Err(crate::QueryPartError::CheckedAdd {
                location: location_macros::location!(),
            })
        },
        Ok,
    )
}
#[cfg(test)]
mod tests {
    #[test]
    fn checked_add_one_returns_placeholder_and_updates_counter() {
        let mut counter = super::QueryPartIncrement::from(4);
        assert_eq!(
            super::QueryPartIncrementMut::checked_add_one(&mut counter),
            Some(super::QueryPartIncrement::from(5))
        );
        assert_eq!(counter.get(), 5);
    }
    #[test]
    fn checked_add_one_does_not_mutate_counter_on_overflow() {
        let mut counter = super::QueryPartIncrement::from(u64::MAX);
        assert_eq!(
            super::QueryPartIncrementMut::checked_add_one(&mut counter),
            None
        );
        assert_eq!(counter.get(), u64::MAX);
    }
    #[test]
    fn checked_add_one_has_same_behavior_for_legacy_counter() {
        let mut counter = 4u64;
        assert_eq!(
            super::QueryPartIncrementMut::checked_add_one(&mut counter),
            Some(super::QueryPartIncrement::from(5))
        );
        assert_eq!(counter, 5);
    }
    #[test]
    fn result_api_maps_overflow_without_mutating_counter() {
        let mut counter = super::QueryPartIncrement::from(u64::MAX);
        assert!(matches!(
            super::increment_checked_add_one_returning_increment(&mut counter),
            Err(crate::QueryPartError::CheckedAdd { .. })
        ));
        assert_eq!(counter.get(), u64::MAX);
    }
}
