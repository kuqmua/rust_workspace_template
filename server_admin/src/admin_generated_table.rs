#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    frontend_contract_macros::UnitEnumCatalog,
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
    pub(crate) fn field_contracts(self) -> frontend_contract::field_contracts::FieldContracts {
        match self {
            Self::Roles => crate::admin_roles::AdminRoles::frontend_fields(),
            Self::RolePermissions => {
                crate::admin_role_permissions::AdminRolePermissions::frontend_fields()
            }
            Self::Users => crate::admin_users::AdminUsers::frontend_fields(),
            Self::Permissions => crate::admin_permissions::AdminPermissions::frontend_fields(),
            Self::SystemSettings => {
                crate::admin_system_settings::AdminSystemSettings::frontend_fields()
            }
            Self::UserRoles => crate::admin_user_roles::AdminUserRoles::frontend_fields(),
        }
    }

    pub(crate) fn filter_value(
        self,
        field: frontend_contract::form_field_name_ref::FormFieldNameRef<'_>,
        value: frontend_contract::form_value_ref::FormValueRef<'_>,
    ) -> Option<
        Result<
            frontend_contract::filter_wire_json::FilterWireJson,
            frontend_contract::form_value_error::FormValueError,
        >,
    > {
        match self {
            Self::Roles => crate::admin_roles::AdminRoles::frontend_filter_value(field, value),
            Self::RolePermissions => {
                crate::admin_role_permissions::AdminRolePermissions::frontend_filter_value(
                    field, value,
                )
            }
            Self::Users => crate::admin_users::AdminUsers::frontend_filter_value(field, value),
            Self::Permissions => {
                crate::admin_permissions::AdminPermissions::frontend_filter_value(field, value)
            }
            Self::SystemSettings => {
                crate::admin_system_settings::AdminSystemSettings::frontend_filter_value(
                    field, value,
                )
            }
            Self::UserRoles => {
                crate::admin_user_roles::AdminUserRoles::frontend_filter_value(field, value)
            }
        }
    }

    pub(crate) fn parse_filter(
        self,
        payload: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
    ) -> Result<crate::data_flt::DataFlt, crate::admin_repository_error::AdminRepositoryError> {
        let parsed = match self {
            Self::Permissions => serde_json::from_str::<
                crate::admin_permissions::StdOptionalOptionalAdminPermissionsWhereMany,
            >(payload.get())
            .map(crate::data_permissions_flt::DataPermissionsFlt::from)
            .map(crate::data_flt::DataFlt::Permissions),
            Self::RolePermissions => serde_json::from_str::<
                crate::admin_role_permissions::StdOptionalOptionalAdminRolePermissionsWhereMany,
            >(payload.get())
            .map(crate::data_role_permissions_flt::DataRolePermissionsFlt::from)
            .map(crate::data_flt::DataFlt::RolePermissions),
            Self::Roles => serde_json::from_str::<
                crate::admin_roles::StdOptionalOptionalAdminRolesWhereMany,
            >(payload.get())
            .map(crate::data_roles_flt::DataRolesFlt::from)
            .map(crate::data_flt::DataFlt::Roles),
            Self::SystemSettings => serde_json::from_str::<
                crate::admin_system_settings::StdOptionalOptionalAdminSystemSettingsWhereMany,
            >(payload.get())
            .map(crate::data_system_settings_flt::DataSystemSettingsFlt::from)
            .map(crate::data_flt::DataFlt::SystemSettings),
            Self::UserRoles => serde_json::from_str::<
                crate::admin_user_roles::StdOptionalOptionalAdminUserRolesWhereMany,
            >(payload.get())
            .map(crate::data_user_roles_flt::DataUserRolesFlt::from)
            .map(crate::data_flt::DataFlt::UserRoles),
            Self::Users => serde_json::from_str::<
                crate::admin_users::StdOptionalOptionalAdminUsersWhereMany,
            >(payload.get())
            .map(crate::data_users_flt::DataUsersFlt::from)
            .map(crate::data_flt::DataFlt::Users),
        };
        parsed.map_err(|_error| {
            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
        })
    }

    pub(crate) const fn for_data_table(
        table: server_admin_contract::admin_data_table::AdminDataTable,
    ) -> Option<Self> {
        match table {
            server_admin_contract::admin_data_table::AdminDataTable::Permissions => {
                Some(Self::Permissions)
            }
            server_admin_contract::admin_data_table::AdminDataTable::RolePermissions => {
                Some(Self::RolePermissions)
            }
            server_admin_contract::admin_data_table::AdminDataTable::Roles => Some(Self::Roles),
            server_admin_contract::admin_data_table::AdminDataTable::SystemSettings => {
                Some(Self::SystemSettings)
            }
            server_admin_contract::admin_data_table::AdminDataTable::UserRoles => {
                Some(Self::UserRoles)
            }
            server_admin_contract::admin_data_table::AdminDataTable::Users => Some(Self::Users),
            server_admin_contract::admin_data_table::AdminDataTable::AccessSessions
            | server_admin_contract::admin_data_table::AdminDataTable::AuditLog
            | server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus
            | server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts
            | server_admin_contract::admin_data_table::AdminDataTable::RateLimits
            | server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens => None,
        }
    }

    pub(crate) fn open_api(self) -> crate::utoipa_admin_open_api::UtoipaAdminOpenApi {
        crate::utoipa_admin_open_api::UtoipaAdminOpenApi::from(match self {
            Self::Roles => crate::admin_roles::AdminRolesOpenApi::open_api(),
            Self::RolePermissions => {
                crate::admin_role_permissions::AdminRolePermissionsOpenApi::open_api()
            }
            Self::Users => crate::admin_users::AdminUsersOpenApi::open_api(),
            Self::Permissions => crate::admin_permissions::AdminPermissionsOpenApi::open_api(),
            Self::SystemSettings => {
                crate::admin_system_settings::AdminSystemSettingsOpenApi::open_api()
            }
            Self::UserRoles => crate::admin_user_roles::AdminUserRolesOpenApi::open_api(),
        })
    }

    pub(crate) fn routes(
        self,
        app_state: &crate::shared_admin_generated_table_state_arc::SharedAdminGeneratedTableStateArc,
    ) -> server_runtime_http::axum_router::AxumRouter {
        server_runtime_http::axum_router::AxumRouter::from(match self {
            Self::Roles => {
                crate::admin_roles::AdminRoles::routes(std::sync::Arc::clone(&app_state.0))
            }
            Self::RolePermissions => crate::admin_role_permissions::AdminRolePermissions::routes(
                std::sync::Arc::clone(&app_state.0),
            ),
            Self::Users => {
                crate::admin_users::AdminUsers::routes(std::sync::Arc::clone(&app_state.0))
            }
            Self::Permissions => crate::admin_permissions::AdminPermissions::routes(
                std::sync::Arc::clone(&app_state.0),
            ),
            Self::SystemSettings => crate::admin_system_settings::AdminSystemSettings::routes(
                std::sync::Arc::clone(&app_state.0),
            ),
            Self::UserRoles => {
                crate::admin_user_roles::AdminUserRoles::routes(std::sync::Arc::clone(&app_state.0))
            }
        })
    }

    pub(crate) fn route_contract(
        self,
        path: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
    ) -> Option<crate::admin_generated_route_contract::AdminGeneratedRouteContract> {
        match self {
            Self::Roles => {
                crate::admin_roles::AdminRolesRouteContract::for_path(path.get()).map(|contract| {
                    crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                        contract
                            .permission()
                            .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                        server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::RolePermissions => {
                crate::admin_role_permissions::AdminRolePermissionsRouteContract::for_path(
                    path.get(),
                )
                .map(|contract| {
                    crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                        contract
                            .permission()
                            .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                        server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::Users => {
                crate::admin_users::AdminUsersRouteContract::for_path(path.get()).map(|contract| {
                    crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                        contract
                            .permission()
                            .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                        server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::Permissions => crate::admin_permissions::AdminPermissionsRouteContract::for_path(
                path.get(),
            )
            .map(|contract| {
                crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                    contract
                        .permission()
                        .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                    server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::SystemSettings => {
                crate::admin_system_settings::AdminSystemSettingsRouteContract::for_path(path.get())
                    .map(|contract| {
                        crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                            contract
                                .permission()
                                .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                            server_admin_core::std_admin_bool::StdAdminBool::from(
                                contract.mutates(),
                            ),
                            contract.frontend_contract().method(),
                        )
                    })
            }
            Self::UserRoles => crate::admin_user_roles::AdminUserRolesRouteContract::for_path(
                path.get(),
            )
            .map(|contract| {
                crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                    contract
                        .permission()
                        .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                    server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
        }
    }
}
