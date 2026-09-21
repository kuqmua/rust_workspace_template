#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin rate limits keeps declaration order aligned with generated layout or processing flow"
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
    "db_table_name": "rate_limits",
    "create_exclude_fields": ["window_started_at"],
    "permission_prefix": "rate_limits",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
pub struct AdminRateLimits {
    #[generate_pg_table_db_default]
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    scope: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    subject: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    window_started_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    #[generate_pg_table_db_default]
    request_count: pg_types_numeric::generate_pg_types_mod::I64AsNonNullInt8,
}
