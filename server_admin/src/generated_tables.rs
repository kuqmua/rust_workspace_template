#![allow(clippy::needless_for_each, clippy::partial_pub_fields)] // generated contracts expose operation fields while source table fields stay private to protect password hashes
#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::partial_pub_fields
)]
// generated declarations follow PostgreSQL order while the source table fields stay private to protect password hashes
#[derive(Clone, Copy, gen_pg_tbl::GenPgTbl, optml::Optml)]
#[gen_pg_tbl::gen_pg_tbl_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["password_hash", "created_at", "updated_at"],
    "read_exclude_fields": ["password_hash"],
    "permission_prefix": "users",
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
#[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
#[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
#[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
#[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
#[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
#[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
#[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
#[gen_pg_tbl::cmn_er_vrts{enum CmnErVrts{}}]
#[gen_pg_tbl::cm_logic{}]
#[gen_pg_tbl::co_logic{}]
#[gen_pg_tbl::rm_logic{}]
#[gen_pg_tbl::ro_logic{}]
#[gen_pg_tbl::um_logic{}]
#[gen_pg_tbl::uo_logic{}]
#[gen_pg_tbl::dm_logic{}]
#[gen_pg_tbl::dlo_logic{}]
#[gen_pg_tbl::cmn_logic{}]
#[allow(dead_code)] // private descriptor fields are consumed by proc-macro expansion and keep password hashes out of the public API
pub struct AdminUsers {
    #[gen_pg_tbl_pk]
    id: pg_types_numeric::I64AsNnBigSerialInitByPg,
    login: pg_types_text_misc::StringAsNnText,
    display_name: pg_types_text_misc::StringAsNnText,
    password_hash: pg_types_text_misc::StringAsNnText,
    is_banned: pg_types_numeric::BoolAsNnBool,
    created_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTz,
    updated_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTz,
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
#[derive(Debug, Clone, Copy, gen_pg_tbl::GenPgTbl, optml::Optml)]
#[gen_pg_tbl::gen_pg_tbl_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["created_at"],
    "permission_prefix": "user_roles",
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
#[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
#[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
#[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
#[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
#[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
#[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
#[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
#[gen_pg_tbl::cmn_er_vrts{enum CmnErVrts{}}]
#[gen_pg_tbl::cm_logic{}]
#[gen_pg_tbl::co_logic{}]
#[gen_pg_tbl::rm_logic{}]
#[gen_pg_tbl::ro_logic{}]
#[gen_pg_tbl::um_logic{}]
#[gen_pg_tbl::uo_logic{}]
#[gen_pg_tbl::dm_logic{}]
#[gen_pg_tbl::dlo_logic{}]
#[gen_pg_tbl::cmn_logic{}]
pub struct AdminUserRoles {
    #[gen_pg_tbl_pk]
    pub id: pg_types_numeric::I64AsNnBigSerialInitByPg,
    pub user_id: pg_types_numeric::I64AsNnInt8,
    pub role_id: pg_types_numeric::I64AsNnInt8,
    pub created_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTz,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, gen_pg_tbl::GenPgTbl, optml::Optml)]
#[gen_pg_tbl::gen_pg_tbl_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["created_at"],
    "permission_prefix": "role_permissions",
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
#[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
#[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
#[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
#[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
#[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
#[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
#[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
#[gen_pg_tbl::cmn_er_vrts{enum CmnErVrts{}}]
#[gen_pg_tbl::cm_logic{}]
#[gen_pg_tbl::co_logic{}]
#[gen_pg_tbl::rm_logic{}]
#[gen_pg_tbl::ro_logic{}]
#[gen_pg_tbl::um_logic{}]
#[gen_pg_tbl::uo_logic{}]
#[gen_pg_tbl::dm_logic{}]
#[gen_pg_tbl::dlo_logic{}]
#[gen_pg_tbl::cmn_logic{}]
pub struct AdminRolePermissions {
    #[gen_pg_tbl_pk]
    pub id: pg_types_numeric::I64AsNnBigSerialInitByPg,
    pub role_id: pg_types_numeric::I64AsNnInt8,
    pub permission_id: pg_types_numeric::I64AsNnInt8,
    pub created_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTz,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, gen_pg_tbl::GenPgTbl, optml::Optml)]
#[gen_pg_tbl::gen_pg_tbl_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["created_at", "updated_at"],
    "permission_prefix": "roles",
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
#[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
#[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
#[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
#[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
#[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
#[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
#[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
#[gen_pg_tbl::cmn_er_vrts{enum CmnErVrts{}}]
#[gen_pg_tbl::cm_logic{}]
#[gen_pg_tbl::co_logic{}]
#[gen_pg_tbl::rm_logic{}]
#[gen_pg_tbl::ro_logic{}]
#[gen_pg_tbl::um_logic{}]
#[gen_pg_tbl::uo_logic{}]
#[gen_pg_tbl::dm_logic{}]
#[gen_pg_tbl::dlo_logic{}]
#[gen_pg_tbl::cmn_logic{}]
pub struct AdminRoles {
    #[gen_pg_tbl_pk]
    pub id: pg_types_numeric::I64AsNnBigSerialInitByPg,
    pub name: pg_types_text_misc::StringAsNnText,
    pub is_system: pg_types_numeric::BoolAsNnBool,
    pub created_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTz,
    pub updated_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTz,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, gen_pg_tbl::GenPgTbl, optml::Optml)]
#[gen_pg_tbl::gen_pg_tbl_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["created_at"],
    "permission_prefix": "permissions",
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
#[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
#[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
#[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
#[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
#[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
#[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
#[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
#[gen_pg_tbl::cmn_er_vrts{enum CmnErVrts{}}]
#[gen_pg_tbl::cm_logic{}]
#[gen_pg_tbl::co_logic{}]
#[gen_pg_tbl::rm_logic{}]
#[gen_pg_tbl::ro_logic{}]
#[gen_pg_tbl::um_logic{}]
#[gen_pg_tbl::uo_logic{}]
#[gen_pg_tbl::dm_logic{}]
#[gen_pg_tbl::dlo_logic{}]
#[gen_pg_tbl::cmn_logic{}]
pub struct AdminPermissions {
    #[gen_pg_tbl_pk]
    pub id: pg_types_numeric::I64AsNnBigSerialInitByPg,
    pub name: pg_types_text_misc::StringAsNnText,
    pub created_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTz,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, gen_pg_tbl::GenPgTbl, optml::Optml)]
#[gen_pg_tbl::gen_pg_tbl_config{{
    "api_mode": "ReadOnly",
    "create_exclude_fields": ["updated_at"],
    "permission_prefix": "system_settings",
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
#[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
#[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
#[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
#[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
#[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
#[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
#[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
#[gen_pg_tbl::cmn_er_vrts{enum CmnErVrts{}}]
#[gen_pg_tbl::cm_logic{}]
#[gen_pg_tbl::co_logic{}]
#[gen_pg_tbl::rm_logic{}]
#[gen_pg_tbl::ro_logic{}]
#[gen_pg_tbl::um_logic{}]
#[gen_pg_tbl::uo_logic{}]
#[gen_pg_tbl::dm_logic{}]
#[gen_pg_tbl::dlo_logic{}]
#[gen_pg_tbl::cmn_logic{}]
pub struct AdminSystemSettings {
    #[gen_pg_tbl_pk]
    pub id: pg_types_numeric::I16AsNnSmallSerialInitByPg,
    pub site_name: pg_types_text_misc::StringAsNnText,
    pub tab_title: pg_types_text_misc::OptStringAsNlText,
    pub main_logo: pg_types_text_misc::OptStringAsNlText,
    pub primary_color: pg_types_text_misc::OptStringAsNlText,
    pub default_admin_route: pg_types_text_misc::StringAsNnText,
    pub organization_name: pg_types_text_misc::OptStringAsNlText,
    pub organization_contacts: pg_types_text_misc::OptStringAsNlText,
    pub support_url: pg_types_text_misc::OptStringAsNlText,
    pub updated_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTz,
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
}
