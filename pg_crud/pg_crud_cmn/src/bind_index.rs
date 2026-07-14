#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, optml::Optml, newtype::Newtype)]
#[newtype(display)]
pub struct QpIncr(u64);
impl From<u64> for QpIncr {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl QpIncr {
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}
pub trait QpIncrMut {
    fn checked_add_one(&mut self) -> Option<QpIncr>;
}
impl QpIncrMut for QpIncr {
    fn checked_add_one(&mut self) -> Option<QpIncr> {
        self.0.checked_add(1).map(|value| {
            *self = Self::from(value);
            Self::from(value)
        })
    }
}
impl QpIncrMut for u64 {
    fn checked_add_one(&mut self) -> Option<QpIncr> {
        self.checked_add(1).map(|value| {
            *self = value;
            QpIncr::from(value)
        })
    }
}
pub fn incr_checked_add_one_returning_incr<IncrTy>(incr: &mut IncrTy) -> Result<QpIncr, crate::QpEr>
where
    IncrTy: QpIncrMut + ?Sized,
{
    incr.checked_add_one().map_or_else(
        || {
            Err(crate::QpEr::CheckedAdd {
                loc: loc_macros::loc!(),
            })
        },
        Ok,
    )
}
#[cfg(test)]
mod tests {
    #[test]
    fn checked_add_one_returns_placeholder_and_updates_counter() {
        let mut counter = super::QpIncr::from(4);
        assert_eq!(
            super::QpIncrMut::checked_add_one(&mut counter),
            Some(super::QpIncr::from(5))
        );
        assert_eq!(counter.get(), 5);
    }
    #[test]
    fn checked_add_one_does_not_mutate_counter_on_overflow() {
        let mut counter = super::QpIncr::from(u64::MAX);
        assert_eq!(super::QpIncrMut::checked_add_one(&mut counter), None);
        assert_eq!(counter.get(), u64::MAX);
    }
    #[test]
    fn checked_add_one_has_same_behavior_for_legacy_counter() {
        let mut counter = 4u64;
        assert_eq!(
            super::QpIncrMut::checked_add_one(&mut counter),
            Some(super::QpIncr::from(5))
        );
        assert_eq!(counter, 5);
    }
    #[test]
    fn result_api_maps_overflow_without_mutating_counter() {
        let mut counter = super::QpIncr::from(u64::MAX);
        assert!(matches!(
            super::incr_checked_add_one_returning_incr(&mut counter),
            Err(crate::QpEr::CheckedAdd { .. })
        ));
        assert_eq!(counter.get(), u64::MAX);
    }
}
