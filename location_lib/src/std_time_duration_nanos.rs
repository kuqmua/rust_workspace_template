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
    error = crate::std_time_duration_nanos_try_from_u32_error::StdTimeDurationNanosTryFromU32Error,
    validator = |value: &u32| {
        if *value < 1_000_000_000u32 { Ok(()) } else { Err(crate::std_time_duration_nanos_try_from_u32_error::StdTimeDurationNanosTryFromU32Error) }
    }
)]
pub struct StdTimeDurationNanos(u32);
