#![allow(clippy::needless_for_each, clippy::partial_pub_fields)] // generated contracts expose operation fields while source table fields stay private to protect password hashes
#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::partial_pub_fields
)]
// generated declarations follow PostgreSQL order while the source table fields stay private to protect password hashes
#[derive(Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
#[generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["password_hash", "created_at", "updated_at"],
    "read_exclude_fields": ["password_hash"],
    "permission_prefix": "users",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[generate_pg_table::cm_error_variants{enum CmErrorVariants{}}]
#[generate_pg_table::co_error_variants{enum CoErrorVariants{}}]
#[generate_pg_table::rm_error_variants{enum RmErrorVariants{}}]
#[generate_pg_table::ro_error_variants{enum RoErrorVariants{}}]
#[generate_pg_table::um_error_variants{enum UmErrorVariants{}}]
#[generate_pg_table::uo_error_variants{enum UoErrorVariants{}}]
#[generate_pg_table::dm_error_variants{enum DmErrorVariants{}}]
#[generate_pg_table::dlo_error_variants{enum DloErrorVariants{}}]
#[generate_pg_table::common_error_variants{enum CommonErrorVariants{}}]
#[generate_pg_table::cm_logic{}]
#[generate_pg_table::co_logic{}]
#[generate_pg_table::rm_logic{}]
#[generate_pg_table::ro_logic{}]
#[generate_pg_table::um_logic{}]
#[generate_pg_table::uo_logic{}]
#[generate_pg_table::dm_logic{}]
#[generate_pg_table::dlo_logic{}]
#[generate_pg_table::common_logic{}]
#[allow(dead_code)] // private descriptor fields are consumed by proc-macro expansion and keep password hashes out of the public API
pub struct AdminUsers {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    login: pg_types_text_misc::StringAsNonNullText,
    display_name: pg_types_text_misc::StringAsNonNullText,
    password_hash: pg_types_text_misc::StringAsNonNullText,
    is_banned: pg_types_numeric::BoolAsNonNullBool,
    created_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    updated_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
impl std::fmt::Debug for AdminUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdminUsers")
            .field("id", &self.id)
            .field("login", &self.login)
            .field("display_name", &self.display_name)
            .field("password_hash", &"[REDACTED]")
            .field("is_banned", &self.is_banned)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .finish()
    }
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
#[generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["created_at"],
    "permission_prefix": "user_roles",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[generate_pg_table::cm_error_variants{enum CmErrorVariants{}}]
#[generate_pg_table::co_error_variants{enum CoErrorVariants{}}]
#[generate_pg_table::rm_error_variants{enum RmErrorVariants{}}]
#[generate_pg_table::ro_error_variants{enum RoErrorVariants{}}]
#[generate_pg_table::um_error_variants{enum UmErrorVariants{}}]
#[generate_pg_table::uo_error_variants{enum UoErrorVariants{}}]
#[generate_pg_table::dm_error_variants{enum DmErrorVariants{}}]
#[generate_pg_table::dlo_error_variants{enum DloErrorVariants{}}]
#[generate_pg_table::common_error_variants{enum CommonErrorVariants{}}]
#[generate_pg_table::cm_logic{}]
#[generate_pg_table::co_logic{}]
#[generate_pg_table::rm_logic{}]
#[generate_pg_table::ro_logic{}]
#[generate_pg_table::um_logic{}]
#[generate_pg_table::uo_logic{}]
#[generate_pg_table::dm_logic{}]
#[generate_pg_table::dlo_logic{}]
#[generate_pg_table::common_logic{}]
pub struct AdminUserRoles {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    pub user_id: pg_types_numeric::I64AsNonNullInt8,
    pub role_id: pg_types_numeric::I64AsNonNullInt8,
    pub created_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
#[generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["created_at"],
    "permission_prefix": "role_permissions",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[generate_pg_table::cm_error_variants{enum CmErrorVariants{}}]
#[generate_pg_table::co_error_variants{enum CoErrorVariants{}}]
#[generate_pg_table::rm_error_variants{enum RmErrorVariants{}}]
#[generate_pg_table::ro_error_variants{enum RoErrorVariants{}}]
#[generate_pg_table::um_error_variants{enum UmErrorVariants{}}]
#[generate_pg_table::uo_error_variants{enum UoErrorVariants{}}]
#[generate_pg_table::dm_error_variants{enum DmErrorVariants{}}]
#[generate_pg_table::dlo_error_variants{enum DloErrorVariants{}}]
#[generate_pg_table::common_error_variants{enum CommonErrorVariants{}}]
#[generate_pg_table::cm_logic{}]
#[generate_pg_table::co_logic{}]
#[generate_pg_table::rm_logic{}]
#[generate_pg_table::ro_logic{}]
#[generate_pg_table::um_logic{}]
#[generate_pg_table::uo_logic{}]
#[generate_pg_table::dm_logic{}]
#[generate_pg_table::dlo_logic{}]
#[generate_pg_table::common_logic{}]
pub struct AdminRolePermissions {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    pub role_id: pg_types_numeric::I64AsNonNullInt8,
    pub permission_id: pg_types_numeric::I64AsNonNullInt8,
    pub created_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
#[generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["created_at", "updated_at"],
    "permission_prefix": "roles",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[generate_pg_table::cm_error_variants{enum CmErrorVariants{}}]
#[generate_pg_table::co_error_variants{enum CoErrorVariants{}}]
#[generate_pg_table::rm_error_variants{enum RmErrorVariants{}}]
#[generate_pg_table::ro_error_variants{enum RoErrorVariants{}}]
#[generate_pg_table::um_error_variants{enum UmErrorVariants{}}]
#[generate_pg_table::uo_error_variants{enum UoErrorVariants{}}]
#[generate_pg_table::dm_error_variants{enum DmErrorVariants{}}]
#[generate_pg_table::dlo_error_variants{enum DloErrorVariants{}}]
#[generate_pg_table::common_error_variants{enum CommonErrorVariants{}}]
#[generate_pg_table::cm_logic{}]
#[generate_pg_table::co_logic{}]
#[generate_pg_table::rm_logic{}]
#[generate_pg_table::ro_logic{}]
#[generate_pg_table::um_logic{}]
#[generate_pg_table::uo_logic{}]
#[generate_pg_table::dm_logic{}]
#[generate_pg_table::dlo_logic{}]
#[generate_pg_table::common_logic{}]
pub struct AdminRoles {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    pub name: pg_types_text_misc::StringAsNonNullText,
    pub is_system: pg_types_numeric::BoolAsNonNullBool,
    pub created_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    pub updated_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
#[generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["created_at"],
    "permission_prefix": "permissions",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[generate_pg_table::cm_error_variants{enum CmErrorVariants{}}]
#[generate_pg_table::co_error_variants{enum CoErrorVariants{}}]
#[generate_pg_table::rm_error_variants{enum RmErrorVariants{}}]
#[generate_pg_table::ro_error_variants{enum RoErrorVariants{}}]
#[generate_pg_table::um_error_variants{enum UmErrorVariants{}}]
#[generate_pg_table::uo_error_variants{enum UoErrorVariants{}}]
#[generate_pg_table::dm_error_variants{enum DmErrorVariants{}}]
#[generate_pg_table::dlo_error_variants{enum DloErrorVariants{}}]
#[generate_pg_table::common_error_variants{enum CommonErrorVariants{}}]
#[generate_pg_table::cm_logic{}]
#[generate_pg_table::co_logic{}]
#[generate_pg_table::rm_logic{}]
#[generate_pg_table::ro_logic{}]
#[generate_pg_table::um_logic{}]
#[generate_pg_table::uo_logic{}]
#[generate_pg_table::dm_logic{}]
#[generate_pg_table::dlo_logic{}]
#[generate_pg_table::common_logic{}]
pub struct AdminPermissions {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    pub name: pg_types_text_misc::StringAsNonNullText,
    pub created_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
#[generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["updated_at"],
    "permission_prefix": "system_settings",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[generate_pg_table::cm_error_variants{enum CmErrorVariants{}}]
#[generate_pg_table::co_error_variants{enum CoErrorVariants{}}]
#[generate_pg_table::rm_error_variants{enum RmErrorVariants{}}]
#[generate_pg_table::ro_error_variants{enum RoErrorVariants{}}]
#[generate_pg_table::um_error_variants{enum UmErrorVariants{}}]
#[generate_pg_table::uo_error_variants{enum UoErrorVariants{}}]
#[generate_pg_table::dm_error_variants{enum DmErrorVariants{}}]
#[generate_pg_table::dlo_error_variants{enum DloErrorVariants{}}]
#[generate_pg_table::common_error_variants{enum CommonErrorVariants{}}]
#[generate_pg_table::cm_logic{}]
#[generate_pg_table::co_logic{}]
#[generate_pg_table::rm_logic{}]
#[generate_pg_table::ro_logic{}]
#[generate_pg_table::um_logic{}]
#[generate_pg_table::uo_logic{}]
#[generate_pg_table::dm_logic{}]
#[generate_pg_table::dlo_logic{}]
#[generate_pg_table::common_logic{}]
pub struct AdminSystemSettings {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I16AsNonNullSmallSerialInitializationByPg,
    pub site_name: pg_types_text_misc::StringAsNonNullText,
    pub tab_title: pg_types_text_misc::OptionalStringAsNullableText,
    pub main_logo: pg_types_text_misc::OptionalStringAsNullableText,
    pub primary_color: pg_types_text_misc::OptionalStringAsNullableText,
    pub default_admin_route: pg_types_text_misc::StringAsNonNullText,
    pub organization_name: pg_types_text_misc::OptionalStringAsNullableText,
    pub organization_contacts: pg_types_text_misc::OptionalStringAsNullableText,
    pub support_url: pg_types_text_misc::OptionalStringAsNullableText,
    pub updated_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
#[derive(Clone, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct UtoipaAdminOpenApi(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaAdminOpenApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UtoipaAdminOpenApi")
    }
}
#[must_use]
pub fn generated_open_api() -> UtoipaAdminOpenApi {
    let mut document = AdminRolesOpenApi::open_api();
    document.merge(utoipa::openapi::OpenApi::from(crate::auth::open_api()));
    document.merge(AdminRolePermissionsOpenApi::open_api());
    document.merge(AdminUsersOpenApi::open_api());
    document.merge(AdminPermissionsOpenApi::open_api());
    document.merge(AdminSystemSettingsOpenApi::open_api());
    document.merge(AdminUserRolesOpenApi::open_api());
    UtoipaAdminOpenApi(document)
}
#[cfg(test)]
mod tests {
    #[test]
    fn generated_admin_open_api_combines_enabled_routes_only() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("87b2e8fb");
        let paths = document
            .get("paths")
            .and_then(serde_json::Value::as_object)
            .expect("274479a7");
        assert_eq!(paths.len(), 29usize);
        assert!(paths.contains_key("/auth/sign-in"));
        assert!(paths.contains_key("/auth/sessions/{session_id}"));
        assert!(paths.contains_key("/users/{user_id}/password"));
        assert!(paths.contains_key("/admin_users/rm"));
        assert!(paths.contains_key("/admin_users/ro"));
        assert!(!paths.contains_key("/admin_users/cm"));
        assert!(paths.contains_key("/admin_permissions/rm"));
        assert!(paths.contains_key("/admin_permissions/ro"));
        assert!(!paths.contains_key("/admin_permissions/cm"));
        assert!(!paths.contains_key("/admin_permissions/dm"));
        assert!(paths.contains_key("/admin_system_settings/rm"));
        assert!(!paths.contains_key("/admin_system_settings/um"));
        assert!(paths.contains_key("/system-settings"));
        assert!(!paths.contains_key("/admin_system_settings/cm"));
        assert!(!paths.contains_key("/admin_system_settings/dm"));
    }
    #[test]
    fn generated_read_routes_expose_filter_sort_and_pagination_contract() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("8457a8ca");
        let paths = document
            .get("paths")
            .and_then(serde_json::Value::as_object)
            .expect("44d17ab0");
        [
            "/admin_users/rm",
            "/admin_roles/rm",
            "/admin_permissions/rm",
            "/admin_role_permissions/rm",
            "/admin_user_roles/rm",
            "/admin_system_settings/rm",
        ]
        .into_iter()
        .for_each(|path| {
            assert!(
                paths
                    .get(path)
                    .and_then(|item| item.get("post"))
                    .and_then(|operation| operation.get("requestBody"))
                    .is_some(),
                "generated read route must accept a typed query body: {path}"
            );
        });
        let schemas = document
            .pointer("/components/schemas")
            .and_then(serde_json::Value::as_object)
            .expect("8dcf412e");
        [
            "AdminUsersRmPayload",
            "AdminRolesRmPayload",
            "AdminPermissionsRmPayload",
            "AdminRolePermissionsRmPayload",
            "AdminUserRolesRmPayload",
            "AdminSystemSettingsRmPayload",
        ]
        .into_iter()
        .for_each(|schema_name| {
            let properties = schemas
                .get(schema_name)
                .and_then(|schema| schema.get("properties"))
                .and_then(serde_json::Value::as_object)
                .expect("5b8bbdd1");
            ["where_many", "select", "order_by", "pagination"]
                .into_iter()
                .for_each(|property| {
                    assert!(
                        properties.contains_key(property),
                        "{schema_name} must expose {property}"
                    );
                });
        });
    }
}
