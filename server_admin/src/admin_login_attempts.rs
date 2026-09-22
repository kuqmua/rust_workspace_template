#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin login attempts keeps declaration order aligned with generated layout or processing flow"
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
    "db_table_name": "login_attempts",
    "create_exclude_fields": ["attempted_at"],
    "rule_prefix": "login_attempts",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
pub struct AdminLoginAttempts {
    #[generate_pg_table_primary_key]
    #[generate_pg_table_db_default]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    login: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    ip_address:
        pg_types_chrono_net::generate_pg_types_mod::OptionalSqlxTypesIpnetworkIpNetworkAsNullableInet,
    succeeded: pg_types_numeric::generate_pg_types_mod::BoolAsNonNullBool,
    #[generate_pg_table_db_default]
    attempted_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
