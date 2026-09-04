#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct PgOperationalLimit(std::num::NonZeroU64);

impl TryFrom<u64> for PgOperationalLimit {
    type Error = crate::pg_operational_limit_error::PgOperationalLimitError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        std::num::NonZeroU64::new(value)
            .map(Self::from)
            .ok_or(crate::pg_operational_limit_error::PgOperationalLimitError::ZeroLimit)
    }
}
