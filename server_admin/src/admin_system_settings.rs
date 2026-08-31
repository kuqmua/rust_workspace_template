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
#[derive(generate_accessor::Getters)]
pub struct AdminSystemSettings {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::generate_pg_types_mod::I16AsNonNullSmallSerialInitializationByPg,
    #[generate_pg_table_db_default]
    site_name: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    tab_title: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    main_logo: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    primary_color: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    default_admin_route: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    organization_name: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    organization_contacts: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    support_url: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText,
    #[generate_pg_table_db_default]
    updated_at:
        pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
