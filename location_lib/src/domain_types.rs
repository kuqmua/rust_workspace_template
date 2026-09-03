#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules and related behavior retain their intentional facade ordering"
)]

pub(crate) const LOC_DISPLAY_UTC_OFFSET_SECS: i32 = 10_800;
pub(crate) const LOC_FILE_MAX_LEN: usize = 1_048_576;
pub(crate) const LOC_COMMIT_MAX_LEN: usize = 1_048_576;
