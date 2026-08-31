#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    newtype::FromInner,
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
