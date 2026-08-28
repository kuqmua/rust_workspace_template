use super::StdTimeDurationNanosTryFromU32Error;

#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::TryFrom,
)]
#[try_from(
    error = StdTimeDurationNanosTryFromU32Error,
    validator = StdTimeDurationNanos::validate
)]
pub struct StdTimeDurationNanos(u32);
impl StdTimeDurationNanos {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u32) -> Result<(), StdTimeDurationNanosTryFromU32Error> {
        if *value < 1_000_000_000u32 {
            Ok(())
        } else {
            Err(StdTimeDurationNanosTryFromU32Error)
        }
    }
}
