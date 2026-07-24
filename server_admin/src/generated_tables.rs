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
    "db_table_name": "users",
    "create_exclude_fields": ["password_hash", "created_at", "updated_at"],
    "read_exclude_fields": ["password_hash"],
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
    created_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
    #[generate_pg_table_db_default]
    updated_at: pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
#[allow(clippy::missing_fields_in_debug)] // password_hash is intentionally represented by a redacted constant
impl std::fmt::Debug for AdminUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(str_constants::ADMINUSERS)
            .field(str_constants::SQL_NAMES_ID, &self.id)
            .field(str_constants::LOGIN, &self.login)
            .field(str_constants::DISPLAY_NAME, &self.display_name)
            .field(str_constants::PASSWORD_HASH, &str_constants::REDACTED_ALT_3)
            .field(str_constants::IS_BANNED, &self.is_banned)
            .field(str_constants::CREATED_AT, &self.created_at)
            .field(str_constants::UPDATED_AT, &self.updated_at)
            .finish()
    }
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // generated declarations follow the PostgreSQL column order
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
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
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
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
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
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
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
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
#[derive(Debug, Clone, Copy, generate_pg_table::GeneratePgTable, optml::Optml)]
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
    pub tab_title: pg_types_text_misc::OptionalStringAsNullableText,
    pub main_logo: pg_types_text_misc::OptionalStringAsNullableText,
    pub primary_color: pg_types_text_misc::OptionalStringAsNullableText,
    #[generate_pg_table_db_default]
    pub default_admin_route: pg_types_text_misc::StringAsNonNullText,
    pub organization_name: pg_types_text_misc::OptionalStringAsNullableText,
    pub organization_contacts: pg_types_text_misc::OptionalStringAsNullableText,
    pub support_url: pg_types_text_misc::OptionalStringAsNullableText,
    #[generate_pg_table_db_default]
    pub updated_at:
        pg_types_chrono_net::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, frontend_contract::UnitEnumCatalog)]
pub(crate) enum AdminGeneratedTable {
    Roles,
    RolePermissions,
    Users,
    Permissions,
    SystemSettings,
    UserRoles,
}
impl AdminGeneratedTable {
    pub(crate) fn field_contracts(self) -> frontend_contract::FieldContracts {
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
        field: frontend_contract::FormFieldNameRef<'_>,
        value: frontend_contract::FormValueRef<'_>,
    ) -> Option<Result<frontend_contract::FilterWireJson, frontend_contract::FormValueError>> {
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
        payload: crate::StdAdminStrRef<'_>,
    ) -> Result<crate::repository::data_tables::DataFlt, crate::repository::AdminRepositoryError>
    {
        let parsed = match self {
            Self::Permissions => {
                serde_json::from_str::<StdOptionalOptionalAdminPermissionsWhereMany>(payload.get())
                    .map(crate::repository::data_tables::DataPermissionsFlt::from)
                    .map(crate::repository::data_tables::DataFlt::Permissions)
            }
            Self::RolePermissions => serde_json::from_str::<
                StdOptionalOptionalAdminRolePermissionsWhereMany,
            >(payload.get())
            .map(crate::repository::data_tables::DataRolePermissionsFlt::from)
            .map(crate::repository::data_tables::DataFlt::RolePermissions),
            Self::Roles => {
                serde_json::from_str::<StdOptionalOptionalAdminRolesWhereMany>(payload.get())
                    .map(crate::repository::data_tables::DataRolesFlt::from)
                    .map(crate::repository::data_tables::DataFlt::Roles)
            }
            Self::SystemSettings => serde_json::from_str::<
                StdOptionalOptionalAdminSystemSettingsWhereMany,
            >(payload.get())
            .map(crate::repository::data_tables::DataSystemSettingsFlt::from)
            .map(crate::repository::data_tables::DataFlt::SystemSettings),
            Self::UserRoles => {
                serde_json::from_str::<StdOptionalOptionalAdminUserRolesWhereMany>(payload.get())
                    .map(crate::repository::data_tables::DataUserRolesFlt::from)
                    .map(crate::repository::data_tables::DataFlt::UserRoles)
            }
            Self::Users => {
                serde_json::from_str::<StdOptionalOptionalAdminUsersWhereMany>(payload.get())
                    .map(crate::repository::data_tables::DataUsersFlt::from)
                    .map(crate::repository::data_tables::DataFlt::Users)
            }
        };
        parsed.map_err(|_error| crate::repository::AdminRepositoryError::InvalidStoredValue)
    }

    pub(crate) const fn for_data_table(
        table: server_admin_contract::AdminDataTable,
    ) -> Option<Self> {
        match table {
            server_admin_contract::AdminDataTable::Permissions => Some(Self::Permissions),
            server_admin_contract::AdminDataTable::RolePermissions => Some(Self::RolePermissions),
            server_admin_contract::AdminDataTable::Roles => Some(Self::Roles),
            server_admin_contract::AdminDataTable::SystemSettings => Some(Self::SystemSettings),
            server_admin_contract::AdminDataTable::UserRoles => Some(Self::UserRoles),
            server_admin_contract::AdminDataTable::Users => Some(Self::Users),
            server_admin_contract::AdminDataTable::AccessSessions
            | server_admin_contract::AdminDataTable::AuditLog
            | server_admin_contract::AdminDataTable::CleanupStatus
            | server_admin_contract::AdminDataTable::LoginAttempts
            | server_admin_contract::AdminDataTable::RateLimits
            | server_admin_contract::AdminDataTable::RefreshTokens => None,
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

    fn routes(self, app_state: &StdSharedAdminGeneratedTableState) -> server_runtime::AxumRouter {
        server_runtime::AxumRouter::from(match self {
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

    async fn validate_schema(
        self,
        pool: pg_crud_common::SqlxPgPoolRef<'_>,
        schema: pg_crud_common::DbSchemaNameRef<'_>,
    ) -> Result<(), AdminGeneratedTablesValidationError> {
        async fn validate<Table>(
            pool: pg_crud_common::SqlxPgPoolRef<'_>,
            schema: pg_crud_common::DbSchemaNameRef<'_>,
        ) -> Result<(), AdminGeneratedTablesValidationError>
        where
            Table: pg_crud_common::DbExtendedTableSchema,
        {
            pg_crud_common::validate_generated_postgres_table::<Table>(pool, schema)
                .await
                .map_err(AdminGeneratedTablesValidationError::from)?;
            pg_crud_common::validate_postgres_table_extensions::<Table>(pool, schema)
                .await
                .map_err(AdminGeneratedTablesValidationError::from)
        }
        match self {
            Self::Roles => validate::<AdminRoles>(pool, schema).await,
            Self::RolePermissions => validate::<AdminRolePermissions>(pool, schema).await,
            Self::Users => validate::<AdminUsers>(pool, schema).await,
            Self::Permissions => validate::<AdminPermissions>(pool, schema).await,
            Self::SystemSettings => validate::<AdminSystemSettings>(pool, schema).await,
            Self::UserRoles => validate::<AdminUserRoles>(pool, schema).await,
        }
    }

    pub(crate) fn route_contract(
        self,
        path: crate::StdAdminStrRef<'_>,
    ) -> Option<AdminGeneratedRouteContract> {
        match self {
            Self::Roles => AdminRolesRouteContract::for_path(path.get()).map(|contract| {
                AdminGeneratedRouteContract::new(
                    contract.permission().map(crate::StdAdminStrRef::from),
                    crate::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::RolePermissions => {
                AdminRolePermissionsRouteContract::for_path(path.get()).map(|contract| {
                    AdminGeneratedRouteContract::new(
                        contract.permission().map(crate::StdAdminStrRef::from),
                        crate::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::Users => AdminUsersRouteContract::for_path(path.get()).map(|contract| {
                AdminGeneratedRouteContract::new(
                    contract.permission().map(crate::StdAdminStrRef::from),
                    crate::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::Permissions => {
                AdminPermissionsRouteContract::for_path(path.get()).map(|contract| {
                    AdminGeneratedRouteContract::new(
                        contract.permission().map(crate::StdAdminStrRef::from),
                        crate::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::SystemSettings => {
                AdminSystemSettingsRouteContract::for_path(path.get()).map(|contract| {
                    AdminGeneratedRouteContract::new(
                        contract.permission().map(crate::StdAdminStrRef::from),
                        crate::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::UserRoles => AdminUserRolesRouteContract::for_path(path.get()).map(|contract| {
                AdminGeneratedRouteContract::new(
                    contract.permission().map(crate::StdAdminStrRef::from),
                    crate::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct AdminGeneratedRouteContract {
    permission: Option<crate::StdAdminStrRef<'static>>,
    mutates: crate::StdAdminBool,
    method: frontend_contract::HttpMethod,
}
impl AdminGeneratedRouteContract {
    const fn new(
        permission: Option<crate::StdAdminStrRef<'static>>,
        mutates: crate::StdAdminBool,
        method: frontend_contract::HttpMethod,
    ) -> Self {
        Self {
            permission,
            mutates,
            method,
        }
    }

    pub(crate) const fn method(self) -> frontend_contract::HttpMethod {
        self.method
    }

    pub(crate) const fn mutates(self) -> crate::StdAdminBool {
        self.mutates
    }

    pub(crate) const fn permission(self) -> Option<crate::StdAdminStrRef<'static>> {
        self.permission
    }
}
#[derive(Clone, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct UtoipaAdminOpenApi(utoipa::openapi::OpenApi);
#[derive(Clone, newtype::DebugRedacted, newtype::FromInner)]
pub struct StdSharedAdminGeneratedTableState(
    std::sync::Arc<dyn pg_table::CombinationOfAppStateLogicTraits>,
);
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct AdminGeneratedTablesValidationError(pg_crud_common::DbSchemaConformanceError);
impl std::fmt::Debug for UtoipaAdminOpenApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::UTOIPAADMINOPENAPI)
    }
}
#[must_use]
pub fn generated_routes(
    app_state: &StdSharedAdminGeneratedTableState,
) -> server_runtime::AxumRouter {
    server_runtime::AxumRouter::from(
        AdminGeneratedTable::ALL
            .into_iter()
            .fold(axum::Router::new(), |routes, table| {
                routes.merge(axum::Router::from(table.routes(app_state)))
            }),
    )
}
pub async fn validate_catalog_schema(
    pool: pg_crud_common::SqlxPgPoolRef<'_>,
    schema: pg_crud_common::DbSchemaNameRef<'_>,
) -> Result<(), AdminGeneratedTablesValidationError> {
    futures::future::try_join_all(
        AdminGeneratedTable::ALL
            .into_iter()
            .map(|table| table.validate_schema(pool, schema)),
    )
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
                if key == str_constants::DOLLAR_REF
                    && let Some(name) = child.as_str().and_then(|reference| {
                        reference.strip_prefix(str_constants::COMPONENTS_SCHEMAS)
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
    document.merge(utoipa::openapi::OpenApi::from(crate::auth::open_api()));
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
mod tests {
    fn typed_operation(
        document: &serde_json::Value,
        metadata: frontend_contract::RouteMetadata,
    ) -> &serde_json::Value {
        document
            .get(str_constants::PATHS)
            .and_then(|paths| paths.get(metadata.path().as_ref()))
            .and_then(|path| path.get(metadata.method().as_ref().to_ascii_lowercase()))
            .expect("61b8f042")
    }

    fn parameter_names(operation: &serde_json::Value, location: &str) -> Vec<String> {
        operation
            .get("parameters")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter(|parameter| {
                parameter.get("in").and_then(serde_json::Value::as_str) == Some(location)
            })
            .filter_map(|parameter| parameter.get("name").and_then(serde_json::Value::as_str))
            .map(str::to_owned)
            .collect()
    }

    fn assert_local_references_resolve(document: &serde_json::Value, value: &serde_json::Value) {
        match value {
            serde_json::Value::Array(values) => values
                .iter()
                .for_each(|child| assert_local_references_resolve(document, child)),
            serde_json::Value::Object(values) => {
                if let Some(reference) = values
                    .get(str_constants::DOLLAR_REF)
                    .and_then(serde_json::Value::as_str)
                    .and_then(|reference| reference.strip_prefix('#'))
                {
                    assert!(
                        document.pointer(reference).is_some(),
                        "unresolved local OpenAPI reference: {reference}"
                    );
                }
                values
                    .values()
                    .for_each(|child| assert_local_references_resolve(document, child));
            }
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }

    #[test]
    fn generated_table_catalog_maps_every_supported_data_table_once() {
        let expected = [
            (
                super::AdminGeneratedTable::Roles,
                server_admin_contract::AdminDataTable::Roles,
            ),
            (
                super::AdminGeneratedTable::RolePermissions,
                server_admin_contract::AdminDataTable::RolePermissions,
            ),
            (
                super::AdminGeneratedTable::Users,
                server_admin_contract::AdminDataTable::Users,
            ),
            (
                super::AdminGeneratedTable::Permissions,
                server_admin_contract::AdminDataTable::Permissions,
            ),
            (
                super::AdminGeneratedTable::SystemSettings,
                server_admin_contract::AdminDataTable::SystemSettings,
            ),
            (
                super::AdminGeneratedTable::UserRoles,
                server_admin_contract::AdminDataTable::UserRoles,
            ),
        ];
        assert_eq!(super::AdminGeneratedTable::ALL.len(), expected.len());
        expected.into_iter().for_each(|(generated, data_table)| {
            assert!(super::AdminGeneratedTable::ALL.contains(&generated));
            assert_eq!(
                super::AdminGeneratedTable::for_data_table(data_table),
                Some(generated)
            );
        });
        [
            server_admin_contract::AdminDataTable::AccessSessions,
            server_admin_contract::AdminDataTable::AuditLog,
            server_admin_contract::AdminDataTable::CleanupStatus,
            server_admin_contract::AdminDataTable::LoginAttempts,
            server_admin_contract::AdminDataTable::RateLimits,
            server_admin_contract::AdminDataTable::RefreshTokens,
        ]
        .into_iter()
        .for_each(|data_table| {
            assert_eq!(super::AdminGeneratedTable::for_data_table(data_table), None);
        });
    }

    #[test]
    fn generated_admin_open_api_has_no_unresolved_local_references() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("f514a558");
        assert_local_references_resolve(&document, &document);
    }

    #[test]
    fn every_typed_route_path_and_each_path_parameter_match_open_api() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("ab2e610c");
        <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::route_metadata()
            .as_ref()
            .iter()
            .copied()
            .for_each(|metadata| {
                let operation = typed_operation(&document, metadata);
                assert_eq!(
                    operation.get("operationId").and_then(serde_json::Value::as_str),
                    Some(metadata.openapi_operation_id().as_ref()),
                    "operation id differs for {} {}",
                    metadata.method().as_ref(),
                    metadata.path().as_ref(),
                );
                let success_status = u16::from(metadata.success_status().transport_status()).to_string();
                let success_response = operation
                    .get("responses")
                    .and_then(|responses| responses.get(success_status.as_str()))
                    .expect("021e4af7");
                if success_status == "204" {
                    assert!(success_response.get("content").is_none());
                } else {
                    assert!(success_response.pointer("/content/application~1json/schema").is_some());
                }
                let expected = metadata
                    .path()
                    .as_ref()
                    .split('{')
                    .skip(1)
                    .filter_map(|suffix| suffix.split_once('}').map(|(name, _suffix)| name.to_owned()))
                    .collect::<Vec<_>>();
                let actual = parameter_names(operation, "path");
                assert_eq!(actual, expected, "path parameters differ for {}", metadata.path().as_ref());
                actual.iter().for_each(|name| {
                    let parameter = operation
                        .get("parameters")
                        .and_then(serde_json::Value::as_array)
                        .and_then(|parameters| parameters.iter().find(|parameter| {
                            parameter.get("name").and_then(serde_json::Value::as_str) == Some(name)
                                && parameter.get("in").and_then(serde_json::Value::as_str) == Some("path")
                        }))
                        .expect("7e45cd91");
                    assert_eq!(parameter.get("required").and_then(serde_json::Value::as_bool), Some(true));
                    assert!(parameter.get("schema").is_some(), "missing schema for path parameter {name}");
                });
            });
    }

    #[test]
    fn every_typed_route_query_parameter_matches_open_api_individually() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("d083c1a9");
        <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::route_metadata()
            .as_ref()
            .iter()
            .copied()
            .for_each(|metadata| {
                let expected: &[&str] = match metadata.openapi_operation_id().as_ref() {
                    "audit_log" | "export_audit_log" => &["action", "created_after", "created_before", "cursor_created_at", "cursor_id", "limit", "offset", "resource", "resource_id", "succeeded", "user_id", "user_login"],
                    "list_permissions" | "list_roles" | "list_users" | "sessions" => &["limit", "offset", "search", "sort", "direction"],
                    "read_data_table" => &["filter_field", "filter_operation", "filter_value", "filter_end", "limit", "offset", "search", "sort", "direction"],
                    _ => &[],
                };
                let operation = typed_operation(&document, metadata);
                let actual = parameter_names(operation, "query");
                assert_eq!(actual, expected, "query parameters differ for {}", metadata.openapi_operation_id().as_ref());
                actual.iter().for_each(|name| {
                    let parameter = operation
                        .get("parameters")
                        .and_then(serde_json::Value::as_array)
                        .and_then(|parameters| parameters.iter().find(|parameter| parameter.get("name").and_then(serde_json::Value::as_str) == Some(name)))
                        .expect("ba482f35");
                    assert!(parameter.get("schema").is_some(), "missing schema for query parameter {name}");
                    let schema = parameter.get("schema").expect("cf18a7d5");
                    match name.as_str() {
                        "direction" => assert_eq!(
                            schema.get("enum"),
                            Some(&serde_json::json!(["asc", "desc"])),
                        ),
                        "limit" => {
                            assert_eq!(
                                schema.get("minimum").and_then(serde_json::Value::as_u64),
                                Some(u64::from(server_admin_contract::AdminPageLimit::MIN))
                            );
                            assert_eq!(
                                schema.get("maximum").and_then(serde_json::Value::as_u64),
                                Some(u64::from(server_admin_contract::AdminPageLimit::MAX))
                            );
                        }
                        "offset" => assert_eq!(schema.get("minimum").and_then(serde_json::Value::as_u64), Some(0)),
                        "search" => assert_eq!(schema.get("maxLength").and_then(serde_json::Value::as_u64), Some(128)),
                        "sort" => assert_eq!(schema.get("maxLength").and_then(serde_json::Value::as_u64), Some(32)),
                        _ => {}
                    }
                });
            });
    }

    #[test]
    fn proc_macro_generated_request_contracts_match_open_api_and_each_field() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("40a639b7");
        let no_body_schema = serde_json::to_value(
            <server_admin_contract::AdminNoBody as utoipa::ToSchema>::schema().1,
        )
        .expect("e185e575");
        <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::schema_contracts()
            .as_ref()
            .iter()
            .for_each(|contract| {
                let metadata = contract.metadata();
                let operation = typed_operation(&document, metadata);
                let request_body = operation.get("requestBody");
                let expected_schema = contract
                    .request_schema()
                    .cloned()
                    .map(|schema| {
                        let openapi_schema: utoipa::openapi::RefOr<utoipa::openapi::Schema> = schema.into();
                        serde_json::to_value(openapi_schema)
                    })
                    .transpose()
                    .expect("506e754a")
                    .expect("eb67c5a0");
                if expected_schema == no_body_schema {
                    assert!(request_body.is_none(), "unexpected request body for {}", metadata.openapi_operation_id().as_ref());
                    return;
                }
                let reference = request_body
                    .and_then(|body| body.pointer("/content/application~1json/schema/$ref"))
                    .and_then(serde_json::Value::as_str)
                    .expect("26d0f83b");
                let actual_schema = document.pointer(reference.trim_start_matches('#')).expect("3754bca2");
                assert_eq!(actual_schema, &expected_schema, "request schema differs for {}", metadata.openapi_operation_id().as_ref());
                expected_schema
                    .get(str_constants::PROPERTIES)
                    .and_then(serde_json::Value::as_object)
                    .into_iter()
                    .flatten()
                    .for_each(|(property, expected)| {
                        assert_eq!(actual_schema.get(str_constants::PROPERTIES).and_then(|properties| properties.get(property)), Some(expected), "request field differs for {}.{property}", metadata.openapi_operation_id().as_ref());
                    });
            });
    }

    #[test]
    fn proc_macro_generated_response_contracts_match_open_api() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("c4ddf19e");
        <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::schema_contracts()
            .as_ref()
            .iter()
            .for_each(|contract| {
                let metadata = contract.metadata();
                let status = u16::from(metadata.success_status().transport_status()).to_string();
                let actual_schema = typed_operation(&document, metadata)
                    .pointer(format!("/responses/{status}/content/application~1json/schema").as_str());
                if metadata.success_status() == frontend_contract::SuccessStatus::Code204 {
                    assert!(actual_schema.is_none(), "unexpected response body for {}", metadata.openapi_operation_id().as_ref());
                    return;
                }
                let expected_schema = contract
                    .response_schema()
                    .cloned()
                    .map(|schema| {
                        let openapi_schema: utoipa::openapi::RefOr<utoipa::openapi::Schema> = schema.into();
                        serde_json::to_value(openapi_schema)
                    })
                    .transpose()
                    .expect("2edb7155")
                    .expect("54d97b5d");
                assert_eq!(actual_schema, Some(&expected_schema), "response schema differs for {}", metadata.openapi_operation_id().as_ref());
            });
    }

    #[test]
    fn generated_admin_open_api_combines_enabled_routes_only() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("87b2e8fb");
        let paths = document
            .get(str_constants::PATHS)
            .and_then(serde_json::Value::as_object)
            .expect("274479a7");
        assert_eq!(paths.len(), 34usize);
        assert!(paths.contains_key("/auth/sign_in"));
        assert!(!paths.contains_key("/auth/mfa"));
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
        assert!(paths.contains_key("/system_settings"));
        assert!(!paths.contains_key("/admin_system_settings/cm"));
        assert!(!paths.contains_key("/admin_system_settings/dm"));
    }
    #[test]
    fn every_admin_open_api_operation_has_a_unique_identifier() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("c731d604");
        let operation_ids = document
            .get(str_constants::PATHS)
            .and_then(serde_json::Value::as_object)
            .expect("f9b402ac")
            .values()
            .filter_map(serde_json::Value::as_object)
            .flat_map(|operations| operations.values())
            .map(|operation| {
                operation
                    .get("operationId")
                    .and_then(serde_json::Value::as_str)
                    .expect("18f4ae63")
            })
            .collect::<Vec<_>>();
        let unique = operation_ids
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(unique.len(), operation_ids.len());
    }
    #[test]
    fn generated_read_routes_expose_filter_sort_and_pagination_contract() {
        let document =
            serde_json::to_value(utoipa::openapi::OpenApi::from(super::generated_open_api()))
                .expect("8457a8ca");
        let paths = document
            .get(str_constants::PATHS)
            .and_then(serde_json::Value::as_object)
            .expect("44d17ab0");
        [
            str_constants::ADMIN_USERS_RM,
            str_constants::ADMIN_ROLES_RM,
            str_constants::ADMIN_PERMISSIONS_RM,
            str_constants::ADMIN_ROLE_PERMISSIONS_RM,
            str_constants::ADMIN_USER_ROLES_RM,
            str_constants::ADMIN_SYSTEM_SETTINGS_RM,
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
            .pointer(str_constants::COMPONENTS_SCHEMAS_ALT)
            .and_then(serde_json::Value::as_object)
            .expect("8dcf412e");
        [
            str_constants::ADMINUSERSRMPAYLOAD,
            str_constants::ADMINROLESRMPAYLOAD,
            str_constants::ADMINPERMISSIONSRMPAYLOAD,
            str_constants::ADMINROLEPERMISSIONSRMPAYLOAD,
            str_constants::ADMINUSERROLESRMPAYLOAD,
            str_constants::ADMINSYSTEMSETTINGSRMPAYLOAD,
        ]
        .into_iter()
        .for_each(|schema_name| {
            let properties = schemas
                .get(schema_name)
                .and_then(|schema| schema.get(str_constants::PROPERTIES))
                .and_then(serde_json::Value::as_object)
                .expect("5b8bbdd1");
            [
                str_constants::WHERE_MANY,
                str_constants::SELECT_ALT_3,
                str_constants::ORDER_BY,
                str_constants::PAGINATION,
            ]
            .into_iter()
            .for_each(|property| {
                assert!(
                    properties.contains_key(property),
                    "{schema_name} must expose {property}"
                );
            });
        });
    }
    #[test]
    fn generated_frontend_filter_metadata_matches_api_filter_schema() {
        let fields = super::AdminUsers::frontend_fields();
        let login = fields
            .as_ref()
            .iter()
            .find(|field| field.name().as_ref() == str_constants::LOGIN)
            .expect("c2a69d51");
        assert_eq!(
            login.filters().to_vec(),
            vec![
                frontend_contract::FilterOperation::Eq,
                frontend_contract::FilterOperation::Regex,
            ]
        );
        let (_, schema) =
            <pg_types_text_misc::StringAsNonNullTextWhere as utoipa::ToSchema>::schema();
        let variants = serde_json::to_value(schema)
            .expect("84d658fc")
            .get("oneOf")
            .and_then(serde_json::Value::as_array)
            .map(Vec::len);
        assert_eq!(variants, Some(login.filters().len()));
    }
}
