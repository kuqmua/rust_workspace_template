// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(
    Debug,
    Clone,
    Copy,
    generate_pg_table::GeneratePgTable,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "db_table_name": "system_settings",
    "create_exclude_fields": ["updated_at"],
    "permission_prefix": "system_settings",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
pub struct AdminSystemSettings {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I16AsNonNullSmallSerialInitializationByPg,
    #[generate_pg_table_db_default]
    pub site_name: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub tab_title: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub main_logo: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub primary_color: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub default_admin_route: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub organization_name: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub organization_contacts: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub support_url: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub updated_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
