#![allow(clippy::needless_for_each, clippy::partial_pub_fields)] // generated contracts expose operation fields while source table fields stay private to protect password hashes
#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::partial_pub_fields
)]
// generated declarations follow PostgreSQL order while the source table fields stay private to protect password hashes
#[derive(
    Clone, Copy, generate_pg_table::GeneratePgTable, optimal_memory_layout::OptimalMemoryLayout,
)]
#[generate_pg_table::generate_pg_table_config{{
    "api_mode": "ReadOnly",
    "db_table_name": "users",
    "create_exclude_fields": ["password_hash", "must_change_password", "created_at", "updated_at"],
    "read_exclude_fields": ["password_hash", "must_change_password"],
    "permission_prefix": "users",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[allow(dead_code)] // private descriptor fields are consumed by proc-macro expansion and keep password hashes out of the public API
pub struct AdminUsers {
    #[generate_pg_table_primary_key]
    id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    login: pg_types_text_misc::StringAsNonNullText,
    display_name: pg_types_text_misc::StringAsNonNullText,
    password_hash: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    is_banned: pg_types_numeric::BoolAsNonNullBool,
    #[generate_pg_table_db_default]
    must_change_password: pg_types_numeric::BoolAsNonNullBool,
    #[generate_pg_table_db_default]
    created_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    #[generate_pg_table_db_default]
    updated_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
#[allow(clippy::missing_fields_in_debug)] // password_hash is intentionally represented by a redacted constant
impl std::fmt::Debug for AdminUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(constants_str::ADMINUSERS)
            .field(constants_str::SQL_NAMES_ID, &self.id)
            .field(constants_str::LOGIN, &self.login)
            .field(constants_str::DISPLAY_NAME, &self.display_name)
            .field(constants_str::PASSWORD_HASH, &constants_str::REDACTED_ALT_3)
            .field(constants_str::IS_BANNED, &self.is_banned)
            .field(constants_str::CREATED_AT, &self.created_at)
            .field(constants_str::UPDATED_AT, &self.updated_at)
            .finish()
    }
}
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
    "db_table_name": "user_roles",
    "create_exclude_fields": ["created_at"],
    "db_foreign_keys": [
        {"columns": ["user_id"], "referenced_columns": ["id"], "referenced_table": "users"},
        {"columns": ["role_id"], "referenced_columns": ["id"], "referenced_table": "roles"}
    ],
    "db_unique_keys": [["user_id", "role_id"]],
    "permission_prefix": "user_roles",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
pub struct AdminUserRoles {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    pub user_id: pg_types_numeric::I64AsNonNullInt8,
    pub role_id: pg_types_numeric::I64AsNonNullInt8,
    #[generate_pg_table_db_default]
    pub created_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
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
    "db_table_name": "role_permissions",
    "create_exclude_fields": ["created_at"],
    "db_foreign_keys": [
        {"columns": ["role_id"], "referenced_columns": ["id"], "referenced_table": "roles"},
        {"columns": ["permission_id"], "referenced_columns": ["id"], "referenced_table": "permissions"}
    ],
    "db_unique_keys": [["role_id", "permission_id"]],
    "permission_prefix": "role_permissions",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
pub struct AdminRolePermissions {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    pub role_id: pg_types_numeric::I64AsNonNullInt8,
    pub permission_id: pg_types_numeric::I64AsNonNullInt8,
    #[generate_pg_table_db_default]
    pub created_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
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
    "db_table_name": "roles",
    "create_exclude_fields": ["created_at", "updated_at"],
    "db_unique_keys": [["name"]],
    "permission_prefix": "roles",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
pub struct AdminRoles {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    pub name: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub is_system: pg_types_numeric::BoolAsNonNullBool,
    #[generate_pg_table_db_default]
    pub created_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    #[generate_pg_table_db_default]
    pub updated_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
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
    "db_table_name": "permissions",
    "create_exclude_fields": ["created_at"],
    "db_unique_keys": [["name"]],
    "permission_prefix": "permissions",
    "tests_write_into_file": "False",
    "common_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
pub struct AdminPermissions {
    #[generate_pg_table_primary_key]
    pub id: pg_types_numeric::I64AsNonNullBigSerialInitializationByPg,
    pub name: pg_types_text_misc::StringAsNonNullText,
    #[generate_pg_table_db_default]
    pub created_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    frontend_contract::domain_types::UnitEnumCatalog,
)]
pub(crate) enum AdminGeneratedTable {
    Roles,
    RolePermissions,
    Users,
    Permissions,
    SystemSettings,
    UserRoles,
}
impl AdminGeneratedTable {
    pub(crate) fn field_contracts(self) -> frontend_contract::domain_types::FieldContracts {
        match self {
            Self::Roles => AdminRoles::frontend_fields(),
            Self::RolePermissions => AdminRolePermissions::frontend_fields(),
            Self::Users => AdminUsers::frontend_fields(),
            Self::Permissions => AdminPermissions::frontend_fields(),
            Self::SystemSettings => AdminSystemSettings::frontend_fields(),
            Self::UserRoles => AdminUserRoles::frontend_fields(),
        }
    }

    pub(crate) fn filter_value(
        self,
        field: frontend_contract::domain_types::FormFieldNameRef<'_>,
        value: frontend_contract::domain_types::FormValueRef<'_>,
    ) -> Option<
        Result<
            frontend_contract::domain_types::FilterWireJson,
            frontend_contract::domain_types::FormValueError,
        >,
    > {
        match self {
            Self::Roles => AdminRoles::frontend_filter_value(field, value),
            Self::RolePermissions => AdminRolePermissions::frontend_filter_value(field, value),
            Self::Users => AdminUsers::frontend_filter_value(field, value),
            Self::Permissions => AdminPermissions::frontend_filter_value(field, value),
            Self::SystemSettings => AdminSystemSettings::frontend_filter_value(field, value),
            Self::UserRoles => AdminUserRoles::frontend_filter_value(field, value),
        }
    }

    pub(crate) fn parse_filter(
        self,
        payload: crate::domain_types::StdAdminStrRef<'_>,
    ) -> Result<
        crate::adapters::repository::data_tables::DataFlt,
        crate::adapters::repository::AdminRepositoryError,
    > {
        let parsed = match self {
            Self::Permissions => {
                serde_json::from_str::<StdOptionalOptionalAdminPermissionsWhereMany>(payload.get())
                    .map(crate::adapters::repository::data_tables::DataPermissionsFlt::from)
                    .map(crate::adapters::repository::data_tables::DataFlt::Permissions)
            }
            Self::RolePermissions => serde_json::from_str::<
                StdOptionalOptionalAdminRolePermissionsWhereMany,
            >(payload.get())
            .map(crate::adapters::repository::data_tables::DataRolePermissionsFlt::from)
            .map(crate::adapters::repository::data_tables::DataFlt::RolePermissions),
            Self::Roles => {
                serde_json::from_str::<StdOptionalOptionalAdminRolesWhereMany>(payload.get())
                    .map(crate::adapters::repository::data_tables::DataRolesFlt::from)
                    .map(crate::adapters::repository::data_tables::DataFlt::Roles)
            }
            Self::SystemSettings => serde_json::from_str::<
                StdOptionalOptionalAdminSystemSettingsWhereMany,
            >(payload.get())
            .map(crate::adapters::repository::data_tables::DataSystemSettingsFlt::from)
            .map(crate::adapters::repository::data_tables::DataFlt::SystemSettings),
            Self::UserRoles => {
                serde_json::from_str::<StdOptionalOptionalAdminUserRolesWhereMany>(payload.get())
                    .map(crate::adapters::repository::data_tables::DataUserRolesFlt::from)
                    .map(crate::adapters::repository::data_tables::DataFlt::UserRoles)
            }
            Self::Users => {
                serde_json::from_str::<StdOptionalOptionalAdminUsersWhereMany>(payload.get())
                    .map(crate::adapters::repository::data_tables::DataUsersFlt::from)
                    .map(crate::adapters::repository::data_tables::DataFlt::Users)
            }
        };
        parsed
            .map_err(|_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue)
    }

    pub(crate) const fn for_data_table(
        table: server_admin_contract::domain_types::AdminDataTable,
    ) -> Option<Self> {
        match table {
            server_admin_contract::domain_types::AdminDataTable::Permissions => {
                Some(Self::Permissions)
            }
            server_admin_contract::domain_types::AdminDataTable::RolePermissions => {
                Some(Self::RolePermissions)
            }
            server_admin_contract::domain_types::AdminDataTable::Roles => Some(Self::Roles),
            server_admin_contract::domain_types::AdminDataTable::SystemSettings => {
                Some(Self::SystemSettings)
            }
            server_admin_contract::domain_types::AdminDataTable::UserRoles => Some(Self::UserRoles),
            server_admin_contract::domain_types::AdminDataTable::Users => Some(Self::Users),
            server_admin_contract::domain_types::AdminDataTable::AccessSessions
            | server_admin_contract::domain_types::AdminDataTable::AuditLog
            | server_admin_contract::domain_types::AdminDataTable::CleanupStatus
            | server_admin_contract::domain_types::AdminDataTable::LoginAttempts
            | server_admin_contract::domain_types::AdminDataTable::RateLimits
            | server_admin_contract::domain_types::AdminDataTable::RefreshTokens => None,
        }
    }

    fn open_api(self) -> UtoipaAdminOpenApi {
        UtoipaAdminOpenApi::from(match self {
            Self::Roles => AdminRolesOpenApi::open_api(),
            Self::RolePermissions => AdminRolePermissionsOpenApi::open_api(),
            Self::Users => AdminUsersOpenApi::open_api(),
            Self::Permissions => AdminPermissionsOpenApi::open_api(),
            Self::SystemSettings => AdminSystemSettingsOpenApi::open_api(),
            Self::UserRoles => AdminUserRolesOpenApi::open_api(),
        })
    }

    fn routes(
        self,
        app_state: &SharedAdminGeneratedTableStateArc,
    ) -> server_runtime_http::domain_types::AxumRouter {
        server_runtime_http::domain_types::AxumRouter::from(match self {
            Self::Roles => AdminRoles::routes(std::sync::Arc::clone(&app_state.0)),
            Self::RolePermissions => {
                AdminRolePermissions::routes(std::sync::Arc::clone(&app_state.0))
            }
            Self::Users => AdminUsers::routes(std::sync::Arc::clone(&app_state.0)),
            Self::Permissions => AdminPermissions::routes(std::sync::Arc::clone(&app_state.0)),
            Self::SystemSettings => {
                AdminSystemSettings::routes(std::sync::Arc::clone(&app_state.0))
            }
            Self::UserRoles => AdminUserRoles::routes(std::sync::Arc::clone(&app_state.0)),
        })
    }

    pub(crate) fn route_contract(
        self,
        path: crate::domain_types::StdAdminStrRef<'_>,
    ) -> Option<AdminGeneratedRouteContract> {
        match self {
            Self::Roles => AdminRolesRouteContract::for_path(path.get()).map(|contract| {
                AdminGeneratedRouteContract::new(
                    contract
                        .permission()
                        .map(crate::domain_types::StdAdminStrRef::from),
                    crate::domain_types::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::RolePermissions => {
                AdminRolePermissionsRouteContract::for_path(path.get()).map(|contract| {
                    AdminGeneratedRouteContract::new(
                        contract
                            .permission()
                            .map(crate::domain_types::StdAdminStrRef::from),
                        crate::domain_types::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::Users => AdminUsersRouteContract::for_path(path.get()).map(|contract| {
                AdminGeneratedRouteContract::new(
                    contract
                        .permission()
                        .map(crate::domain_types::StdAdminStrRef::from),
                    crate::domain_types::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::Permissions => {
                AdminPermissionsRouteContract::for_path(path.get()).map(|contract| {
                    AdminGeneratedRouteContract::new(
                        contract
                            .permission()
                            .map(crate::domain_types::StdAdminStrRef::from),
                        crate::domain_types::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::SystemSettings => {
                AdminSystemSettingsRouteContract::for_path(path.get()).map(|contract| {
                    AdminGeneratedRouteContract::new(
                        contract
                            .permission()
                            .map(crate::domain_types::StdAdminStrRef::from),
                        crate::domain_types::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::UserRoles => AdminUserRolesRouteContract::for_path(path.get()).map(|contract| {
                AdminGeneratedRouteContract::new(
                    contract
                        .permission()
                        .map(crate::domain_types::StdAdminStrRef::from),
                    crate::domain_types::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) struct AdminGeneratedRouteContract {
    permission: Option<crate::domain_types::StdAdminStrRef<'static>>,
    mutates: crate::domain_types::StdAdminBool,
    method: frontend_contract::domain_types::HttpMethod,
}
impl AdminGeneratedRouteContract {
    const fn new(
        permission: Option<crate::domain_types::StdAdminStrRef<'static>>,
        mutates: crate::domain_types::StdAdminBool,
        method: frontend_contract::domain_types::HttpMethod,
    ) -> Self {
        Self {
            permission,
            mutates,
            method,
        }
    }

    pub(crate) const fn method(self) -> frontend_contract::domain_types::HttpMethod {
        self.method
    }

    pub(crate) const fn mutates(self) -> crate::domain_types::StdAdminBool {
        self.mutates
    }

    pub(crate) const fn permission(self) -> Option<crate::domain_types::StdAdminStrRef<'static>> {
        self.permission
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::IntoInnerFrom, newtype::FromInner,
)]
pub struct UtoipaAdminOpenApi(utoipa::openapi::OpenApi);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct SharedAdminGeneratedTableStateArc(
    std::sync::Arc<dyn pg_table::domain_types::CombinationOfAppStateLogicTraits>,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct AdminGeneratedTablesValidationError(
    pg_crud_common::domain_types::DbSchemaConformanceError,
);
impl std::fmt::Debug for UtoipaAdminOpenApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::UTOIPAADMINOPENAPI)
    }
}
#[must_use]
pub fn generated_routes(
    app_state: &SharedAdminGeneratedTableStateArc,
) -> server_runtime_http::domain_types::AxumRouter {
    server_runtime_http::domain_types::AxumRouter::from(
        AdminGeneratedTable::ALL
            .into_iter()
            .fold(axum::Router::new(), |routes, table| {
                routes.merge(axum::Router::from(table.routes(app_state)))
            }),
    )
}
pub async fn validate_catalog_schema(
    pool: pg_crud_common::domain_types::SqlxPgPoolRef<'_>,
    schema: pg_crud_common::domain_types::DbSchemaNameRef<'_>,
) -> Result<(), AdminGeneratedTablesValidationError> {
    futures::future::try_join_all(AdminGeneratedTable::ALL.into_iter().map(|table| {
        let table_pool = pool;
        let table_schema = schema;
        async move {
            async fn validate<Table>(
                pool: pg_crud_common::domain_types::SqlxPgPoolRef<'_>,
                schema: pg_crud_common::domain_types::DbSchemaNameRef<'_>,
            ) -> Result<(), AdminGeneratedTablesValidationError>
            where
                Table: pg_crud_common::domain_types::DbTableSchema,
            {
                pg_crud_common::domain_types::validate_generated_postgres_table::<Table>(
                    pool, schema,
                )
                .await
                .map_err(AdminGeneratedTablesValidationError::from)
            }
            match table {
                AdminGeneratedTable::Roles => {
                    validate::<AdminRoles>(table_pool, table_schema).await
                }
                AdminGeneratedTable::RolePermissions => {
                    validate::<AdminRolePermissions>(table_pool, table_schema).await
                }
                AdminGeneratedTable::Users => {
                    validate::<AdminUsers>(table_pool, table_schema).await
                }
                AdminGeneratedTable::Permissions => {
                    validate::<AdminPermissions>(table_pool, table_schema).await
                }
                AdminGeneratedTable::SystemSettings => {
                    validate::<AdminSystemSettings>(table_pool, table_schema).await
                }
                AdminGeneratedTable::UserRoles => {
                    validate::<AdminUserRoles>(table_pool, table_schema).await
                }
            }
        }
    }))
    .await
    .map(|_validated| ())
}
#[must_use]
pub fn generated_open_api() -> UtoipaAdminOpenApi {
    fn collect_schema_refs(
        value: &serde_json::Value,
        refs: &mut std::collections::BTreeSet<String>,
    ) {
        match value {
            serde_json::Value::Array(values) => values
                .iter()
                .for_each(|child| collect_schema_refs(child, refs)),
            serde_json::Value::Object(values) => values.iter().for_each(|(key, child)| {
                if key == constants_str::DOLLAR_REF
                    && let Some(name) = child.as_str().and_then(|reference| {
                        reference.strip_prefix(constants_str::COMPONENTS_SCHEMAS)
                    })
                {
                    let _inserted = refs.insert(name.to_owned());
                }
                collect_schema_refs(child, refs);
            }),
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    let mut document = utoipa::openapi::OpenApi::from(AdminGeneratedTable::ALL[0].open_api());
    document.merge(utoipa::openapi::OpenApi::from(
        crate::domain_types::auth::open_api(),
    ));
    AdminGeneratedTable::ALL[1..]
        .iter()
        .copied()
        .for_each(|table| {
            document.merge(utoipa::openapi::OpenApi::from(table.open_api()));
        });
    let mut refs = std::collections::BTreeSet::new();
    if let Ok(value) = serde_json::to_value(&document) {
        collect_schema_refs(&value, &mut refs);
    }
    if let Some(components) = document.components.as_mut() {
        refs.into_iter().for_each(|name| {
            if !components.schemas.contains_key(name.as_str())
                && let Some(suffix) = name.rsplit('.').next()
                && let Some(schema) = components.schemas.get(suffix).cloned()
            {
                let _previous = components.schemas.insert(name, schema);
            }
        });
    }
    UtoipaAdminOpenApi::from(document)
}
#[cfg(test)]
#[path = "domain_types__generated_tables__tests.rs"]
mod tests;
