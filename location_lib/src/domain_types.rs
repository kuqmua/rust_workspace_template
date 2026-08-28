#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules and related behavior retain their intentional facade ordering"
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::module_name_repetitions)]
const LOC_DISPLAY_UTC_OFFSET_SECS: i32 = 10_800;
const LOC_FILE_MAX_LEN: usize = 1_048_576;
const LOC_COMMIT_MAX_LEN: usize = 1_048_576;

#[path = "location_file.rs"]
mod location_file;
pub use location_file::*;
#[path = "location_line.rs"]
mod location_line;
pub use location_line::*;
#[path = "location_column.rs"]
mod location_column;
pub use location_column::*;
#[path = "location_coordinate_try_from_u32_error.rs"]
mod location_coordinate_try_from_u32_error;
pub use location_coordinate_try_from_u32_error::*;
#[path = "location_commit.rs"]
mod location_commit;
pub use location_commit::*;
#[path = "location_duration.rs"]
mod location_duration;
pub use location_duration::*;
#[path = "occr.rs"]
mod occr;
pub use occr::*;
#[path = "location.rs"]
mod location;
pub use location::*;
#[path = "std_time_duration.rs"]
mod std_time_duration;
pub use std_time_duration::*;
#[path = "std_time_duration_secs.rs"]
mod std_time_duration_secs;
pub use std_time_duration_secs::*;
#[path = "std_time_duration_nanos.rs"]
mod std_time_duration_nanos;
pub use std_time_duration_nanos::*;
#[path = "std_time_duration_nanos_try_from_u32_error.rs"]
mod std_time_duration_nanos_try_from_u32_error;
pub use std_time_duration_nanos_try_from_u32_error::*;
#[path = "location_line_non_zero_u32.rs"]
mod location_line_non_zero_u32;
use location_line_non_zero_u32::LocationLineNonZeroU32;
#[path = "location_column_non_zero_u32.rs"]
mod location_column_non_zero_u32;
use location_column_non_zero_u32::LocationColumnNonZeroU32;
#[path = "location_file_ref.rs"]
mod location_file_ref;
use location_file_ref::LocationFileRef;
#[path = "formatter_ref_mut.rs"]
mod formatter_ref_mut;
use formatter_ref_mut::FormatterRefMut;
#[path = "chrono_location_display_timezone.rs"]
mod chrono_location_display_timezone;
use chrono_location_display_timezone::ChronoLocationDisplayTimezone;
#[path = "chrono_location_date_time.rs"]
mod chrono_location_date_time;
use chrono_location_date_time::ChronoLocationDateTime;

#[cfg(test)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[path = "tests.rs"]
mod tests;
