#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules and related behavior retain their intentional facade ordering"
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::module_name_repetitions)]
pub(crate) const LOC_DISPLAY_UTC_OFFSET_SECS: i32 = 10_800;
pub(crate) const LOC_FILE_MAX_LEN: usize = 1_048_576;
pub(crate) const LOC_COMMIT_MAX_LEN: usize = 1_048_576;

pub(crate) use crate::chrono_location_date_time::ChronoLocationDateTime;
pub(crate) use crate::chrono_location_display_timezone::ChronoLocationDisplayTimezone;
pub(crate) use crate::formatter_ref_mut::FormatterRefMut;
pub use crate::location::*;
pub use crate::location_column::*;
pub(crate) use crate::location_column_non_zero_u32::LocationColumnNonZeroU32;
pub use crate::location_commit::*;
pub use crate::location_coordinate_try_from_u32_error::*;
pub use crate::location_duration::*;
pub use crate::location_file::*;
pub(crate) use crate::location_file_ref::LocationFileRef;
pub use crate::location_line::*;
pub(crate) use crate::location_line_non_zero_u32::LocationLineNonZeroU32;
pub use crate::occr::*;
pub use crate::std_time_duration::*;
pub use crate::std_time_duration_nanos::*;
pub use crate::std_time_duration_nanos_try_from_u32_error::*;
pub use crate::std_time_duration_secs::*;
