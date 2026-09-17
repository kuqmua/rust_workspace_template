#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin audit log keeps declaration order aligned with generated layout or processing flow"
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
    "db_table_name": "audit_log",
    "create_exclude_fields": ["created_at"],
    "read_exclude_fields": ["details"],
    "permission_prefix": "audit_log",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
pub struct AdminAuditLog {
    #[generate_pg_table_primary_key]
    #[generate_pg_table_db_default]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    user_id: pg_types_numeric::generate_pg_types_mod::OptionalI64AsNullableInt8,
    user_login: pg_types_text_misc::generate_pg_types_mod::OptionalStringAsNullableText,
    action: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    resource: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    resource_id: pg_types_text_misc::generate_pg_types_mod::OptionalStringAsNullableText,
    request_id:
        pg_types_text_misc::generate_pg_types_mod::OptionalSqlxTypesUuidUuidAsNullableUuidInitializationByClient,
    succeeded: pg_types_numeric::generate_pg_types_mod::BoolAsNonNullBool,
    details: pg_types_text_misc::generate_pg_types_mod::OptionalStringAsNullableText,
    #[generate_pg_table_db_default]
    created_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
