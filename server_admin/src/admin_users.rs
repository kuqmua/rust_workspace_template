// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::partial_pub_fields
)]
// generated declarations follow PostgreSQL order while the source table fields stay private to protect password hashes
#[derive(
    Clone,
    Copy,
    proc_macro_generate_pg_table::GeneratePgTable,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[proc_macro_generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "db_table_name": "users",
    "create_exclude_fields": ["password_hash", "must_change_password", "created_at", "updated_at"],
    "read_exclude_fields": ["password_hash", "must_change_password"],
    "permission_prefix": "users",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[allow(dead_code)] // private descriptor fields are consumed by the macro and keep password hashes out of the API
#[derive(proc_macro_getters::Getters)]
pub struct AdminUsers {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg,
    login: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    display_name: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    password_hash: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    is_banned: pg_types_numeric::generate_pg_types_mod::BoolAsNonNullBool,
    #[generate_pg_table_db_default]
    must_change_password: pg_types_numeric::generate_pg_types_mod::BoolAsNonNullBool,
    #[generate_pg_table_db_default]
    created_at: pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    #[generate_pg_table_db_default]
    updated_at: pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
#[allow(clippy::missing_fields_in_debug)] // password_hash is intentionally represented by a redacted constant
impl std::fmt::Debug for AdminUsers {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(constants_str::ADMINUSERS)
            .field(constants_str::SQL_NAMES_ID, self.get_id())
            .field(constants_str::LOGIN, self.get_login())
            .field(constants_str::DISPLAY_NAME, self.get_display_name())
            .field(constants_str::PASSWORD_HASH, &constants_str::REDACTED_ALT_3)
            .field(constants_str::IS_BANNED, self.get_is_banned())
            .field(constants_str::CREATED_AT, &self.created_at)
            .field(constants_str::UPDATED_AT, &self.updated_at)
            .finish()
    }
}
