#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "admin system settings keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_generate_pg_table::GeneratePgTable,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[proc_macro_generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "db_table_name": "system_settings",
    "create_exclude_fields": ["updated_at"],
    "permission_prefix": "system_settings",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[derive(proc_macro_getters::Getters)]
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
