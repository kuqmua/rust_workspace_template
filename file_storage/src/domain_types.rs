#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules stay paired with their facade imports and reexports"
)]
const MAXIMUM_FILE_BYTES: usize = 104_857_600usize;
const MAXIMUM_OPERATION_ID_BYTES: usize = 128usize;
const MAXIMUM_PATH_BYTES: usize = 4_096usize;
#[path = "file_storage_io_error.rs"]
mod file_storage_io_error;
pub use file_storage_io_error::*;
#[path = "storage_path_ref.rs"]
mod storage_path_ref;
pub(crate) use storage_path_ref::*;
#[path = "storage_directory_name_ref.rs"]
mod storage_directory_name_ref;
pub(crate) use storage_directory_name_ref::*;
#[path = "file_storage_root_path_buf.rs"]
mod file_storage_root_path_buf;
pub use file_storage_root_path_buf::*;
#[path = "storage_relative_path_buf.rs"]
mod storage_relative_path_buf;
pub use storage_relative_path_buf::*;
#[path = "std_storage_operation_id.rs"]
mod std_storage_operation_id;
pub use std_storage_operation_id::*;
#[path = "std_file_bytes.rs"]
mod std_file_bytes;
pub use std_file_bytes::*;
#[path = "file_storage_path_error.rs"]
mod file_storage_path_error;
pub use file_storage_path_error::*;
#[path = "file_storage_error.rs"]
mod file_storage_error;
pub use file_storage_error::*;
#[path = "safe_file_storage.rs"]
mod safe_file_storage;
pub use safe_file_storage::*;
#[path = "file_storage_staging_area.rs"]
mod file_storage_staging_area;
pub use file_storage_staging_area::*;
#[path = "std_stale_staging_entry_limit.rs"]
mod std_stale_staging_entry_limit;
pub use std_stale_staging_entry_limit::*;
#[path = "stale_before_system_time.rs"]
mod stale_before_system_time;
pub use stale_before_system_time::*;
#[path = "stale_staging_cleanup_cfg.rs"]
mod stale_staging_cleanup_cfg;
pub use stale_staging_cleanup_cfg::*;
#[path = "stale_staging_cleanup_cfg_error.rs"]
mod stale_staging_cleanup_cfg_error;
pub use stale_staging_cleanup_cfg_error::*;
#[path = "std_stale_staging_entry_count.rs"]
mod std_stale_staging_entry_count;
pub use std_stale_staging_entry_count::*;
#[path = "stale_staging_cleanup_report.rs"]
mod stale_staging_cleanup_report;
pub use stale_staging_cleanup_report::*;
#[path = "atomic_replace_durability.rs"]
mod atomic_replace_durability;
pub use atomic_replace_durability::*;
#[path = "std_disk_cache_size.rs"]
mod std_disk_cache_size;
pub use std_disk_cache_size::*;
#[path = "disk_cache_modified_at_system_time.rs"]
mod disk_cache_modified_at_system_time;
pub use disk_cache_modified_at_system_time::*;
#[path = "disk_cache_entry.rs"]
mod disk_cache_entry;
pub use disk_cache_entry::*;
#[path = "disk_cache_eviction_plan.rs"]
mod disk_cache_eviction_plan;
pub use disk_cache_eviction_plan::*;
#[path = "disk_cache_budget_error.rs"]
mod disk_cache_budget_error;
pub use disk_cache_budget_error::*;
#[path = "plan_disk_cache_eviction.rs"]
mod plan_disk_cache_eviction;
pub use plan_disk_cache_eviction::*;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
