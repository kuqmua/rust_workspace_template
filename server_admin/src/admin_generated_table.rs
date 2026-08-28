use crate::{
    AdminGeneratedRouteContract, AdminPermissions, AdminPermissionsOpenApi,
    AdminPermissionsRouteContract, AdminRolePermissions, AdminRolePermissionsOpenApi,
    AdminRolePermissionsRouteContract, AdminRoles, AdminRolesOpenApi, AdminRolesRouteContract,
    AdminSystemSettings, AdminSystemSettingsOpenApi, AdminSystemSettingsRouteContract,
    AdminUserRoles, AdminUserRolesOpenApi, AdminUserRolesRouteContract, AdminUsers,
    AdminUsersOpenApi, AdminUsersRouteContract, SharedAdminGeneratedTableStateArc,
    StdOptionalOptionalAdminPermissionsWhereMany, StdOptionalOptionalAdminRolePermissionsWhereMany,
    StdOptionalOptionalAdminRolesWhereMany, StdOptionalOptionalAdminSystemSettingsWhereMany,
    StdOptionalOptionalAdminUserRolesWhereMany, StdOptionalOptionalAdminUsersWhereMany,
    UtoipaAdminOpenApi,
};

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

    pub(crate) fn open_api(self) -> UtoipaAdminOpenApi {
        UtoipaAdminOpenApi::from(match self {
            Self::Roles => AdminRolesOpenApi::open_api(),
            Self::RolePermissions => AdminRolePermissionsOpenApi::open_api(),
            Self::Users => AdminUsersOpenApi::open_api(),
            Self::Permissions => AdminPermissionsOpenApi::open_api(),
            Self::SystemSettings => AdminSystemSettingsOpenApi::open_api(),
            Self::UserRoles => AdminUserRolesOpenApi::open_api(),
        })
    }

    pub(crate) fn routes(
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
