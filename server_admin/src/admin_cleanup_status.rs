#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin cleanup status keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_generate_pg_table_derive_generate_pg_table::GeneratePgTable,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "db_table_name": "cleanup_status",
    "permission_prefix": "cleanup_status",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
pub struct AdminCleanupStatus {
    #[generate_pg_table_primary_key]
    #[generate_pg_table_db_default]
    id:
        pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    #[generate_pg_table_db_default]
    singleton: pg_types_numeric::generate_pg_types_mod::BoolAsNonNullBool,
    last_success_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    last_deleted_rows: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8,
}
