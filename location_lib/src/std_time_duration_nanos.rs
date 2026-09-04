#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_deref_inner::DerefInner,
)]
pub struct StdTimeDurationNanos(u32);
impl TryFrom<u32> for StdTimeDurationNanos {
    type Error =
        crate::std_time_duration_nanos_try_from_u32_error::StdTimeDurationNanosTryFromU32Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 1_000_000_000u32 {
            Ok(Self(value))
        } else {
            Err(Self::Error::OutOfRange)
        }
    }
}
