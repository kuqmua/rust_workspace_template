#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_frontend_contract_derive_unit_enum_catalog::UnitEnumCatalog,
)]
pub(crate) enum AdminGeneratedTable {
    AccessSessions,
    AuditLog,
    PermissionActions,
    PermissionResourceActions,
    PermissionResources,
    Roles,
    RoleRules,
    UsersDatabaseRead,
    Rules,
    SystemSettings,
    UserRoles,
}
impl AdminGeneratedTable {
    pub(crate) fn field_contracts(self) -> frontend_contract::field_contracts::FieldContracts {
        match self {
            Self::AccessSessions => {
                crate::admin_access_sessions::AdminAccessSessions::frontend_fields()
            }
            Self::AuditLog => crate::admin_audit_log::AdminAuditLog::frontend_fields(),
            Self::PermissionActions => {
                crate::admin_permission_actions::AdminPermissionActions::frontend_fields()
            }
            Self::PermissionResourceActions => crate::admin_permission_resource_actions::AdminPermissionResourceActions::frontend_fields(),
            Self::PermissionResources => {
                crate::admin_permission_resources::AdminPermissionResources::frontend_fields()
            }
            Self::Roles => crate::admin_roles::AdminRoles::frontend_fields(),
            Self::RoleRules => crate::admin_role_rules::AdminRoleRules::frontend_fields(),
            Self::UsersDatabaseRead => {
                crate::admin_users_database_read::AdminUsersDatabaseRead::frontend_fields()
            }
            Self::Rules => crate::admin_rules::AdminRules::frontend_fields(),
            Self::SystemSettings => {
                crate::admin_system_settings::AdminSystemSettings::frontend_fields()
            }
            Self::UserRoles => crate::admin_user_roles::AdminUserRoles::frontend_fields(),
        }
    }

    pub(crate) fn filter_value(
        self,
        form_field_name_ref: frontend_contract::form_field_name_ref::FormFieldNameRef<'_>,
        form_value_ref: frontend_contract::form_value_ref::FormValueRef<'_>,
    ) -> Option<
        Result<
            frontend_contract::filter_wire_json::FilterWireJson,
            frontend_contract::form_value_error::FormValueError,
        >,
    > {
        match self {
            Self::AccessSessions => {
                crate::admin_access_sessions::AdminAccessSessions::frontend_filter_value(
                    form_field_name_ref,
                    form_value_ref,
                )
            }
            Self::AuditLog => crate::admin_audit_log::AdminAuditLog::frontend_filter_value(
                form_field_name_ref,
                form_value_ref,
            ),
            Self::PermissionActions => {
                crate::admin_permission_actions::AdminPermissionActions::frontend_filter_value(
                    form_field_name_ref,
                    form_value_ref,
                )
            }
            Self::PermissionResourceActions => crate::admin_permission_resource_actions::AdminPermissionResourceActions::frontend_filter_value(
                form_field_name_ref,
                form_value_ref,
            ),
            Self::PermissionResources => {
                crate::admin_permission_resources::AdminPermissionResources::frontend_filter_value(
                    form_field_name_ref,
                    form_value_ref,
                )
            }
            Self::Roles => crate::admin_roles::AdminRoles::frontend_filter_value(
                form_field_name_ref,
                form_value_ref,
            ),
            Self::RoleRules => crate::admin_role_rules::AdminRoleRules::frontend_filter_value(
                form_field_name_ref,
                form_value_ref,
            ),
            Self::UsersDatabaseRead => {
                crate::admin_users_database_read::AdminUsersDatabaseRead::frontend_filter_value(
                    form_field_name_ref,
                    form_value_ref,
                )
            }
            Self::Rules => crate::admin_rules::AdminRules::frontend_filter_value(
                form_field_name_ref,
                form_value_ref,
            ),
            Self::SystemSettings => {
                crate::admin_system_settings::AdminSystemSettings::frontend_filter_value(
                    form_field_name_ref,
                    form_value_ref,
                )
            }
            Self::UserRoles => crate::admin_user_roles::AdminUserRoles::frontend_filter_value(
                form_field_name_ref,
                form_value_ref,
            ),
        }
    }

    pub(crate) fn parse_filter(
        self,
        std_admin_str_ref: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
    ) -> Result<crate::data_flt::DataFlt, crate::admin_repository_error::AdminRepositoryError> {
        let parsed = match self {
            Self::AccessSessions => serde_json::from_str::<
                crate::admin_access_sessions::StdOptionalOptionalAdminAccessSessionsWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_access_sessions_flt::DataAccessSessionsFlt::from)
            .map(crate::data_flt::DataFlt::AccessSessions),
            Self::AuditLog => serde_json::from_str::<
                crate::admin_audit_log::StdOptionalOptionalAdminAuditLogWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_audit_log_flt::DataAuditLogFlt::from)
            .map(crate::data_flt::DataFlt::AuditLog),
            Self::PermissionActions => serde_json::from_str::<
                crate::admin_permission_actions::StdOptionalOptionalAdminPermissionActionsWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_permission_actions_flt::DataPermissionActionsFlt::from)
            .map(crate::data_flt::DataFlt::PermissionActions),
            Self::PermissionResourceActions => serde_json::from_str::<
                crate::admin_permission_resource_actions::StdOptionalOptionalAdminPermissionResourceActionsWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_permission_resource_actions_flt::DataPermissionResourceActionsFlt::from)
            .map(crate::data_flt::DataFlt::PermissionResourceActions),
            Self::PermissionResources => serde_json::from_str::<
                crate::admin_permission_resources::StdOptionalOptionalAdminPermissionResourcesWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_permission_resources_flt::DataPermissionResourcesFlt::from)
            .map(crate::data_flt::DataFlt::PermissionResources),
            Self::Rules => serde_json::from_str::<
                crate::admin_rules::StdOptionalOptionalAdminRulesWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_rules_flt::DataRulesFlt::from)
            .map(crate::data_flt::DataFlt::Rules),
            Self::RoleRules => serde_json::from_str::<
                crate::admin_role_rules::StdOptionalOptionalAdminRoleRulesWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_role_rules_flt::DataRoleRulesFlt::from)
            .map(crate::data_flt::DataFlt::RoleRules),
            Self::Roles => serde_json::from_str::<
                crate::admin_roles::StdOptionalOptionalAdminRolesWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_roles_flt::DataRolesFlt::from)
            .map(crate::data_flt::DataFlt::Roles),
            Self::SystemSettings => serde_json::from_str::<
                crate::admin_system_settings::StdOptionalOptionalAdminSystemSettingsWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_system_settings_flt::DataSystemSettingsFlt::from)
            .map(crate::data_flt::DataFlt::SystemSettings),
            Self::UserRoles => serde_json::from_str::<
                crate::admin_user_roles::StdOptionalOptionalAdminUserRolesWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_user_roles_flt::DataUserRolesFlt::from)
            .map(crate::data_flt::DataFlt::UserRoles),
            Self::UsersDatabaseRead => serde_json::from_str::<
                crate::admin_users_database_read::StdOptionalOptionalAdminUsersDatabaseReadWhereMany,
            >(std_admin_str_ref.get())
            .map(crate::data_users_flt::DataUsersFlt::from)
            .map(crate::data_flt::DataFlt::Users),
        };
        parsed.map_err(|_error| {
            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
        })
    }

    pub(crate) const fn for_data_table(
        admin_data_table: server_admin_contract::admin_data_table::AdminDataTable,
    ) -> Option<Self> {
        match admin_data_table {
            server_admin_contract::admin_data_table::AdminDataTable::AccessSessions => {
                Some(Self::AccessSessions)
            }
            server_admin_contract::admin_data_table::AdminDataTable::AuditLog => {
                Some(Self::AuditLog)
            }
            server_admin_contract::admin_data_table::AdminDataTable::PermissionActions => {
                Some(Self::PermissionActions)
            }
            server_admin_contract::admin_data_table::AdminDataTable::PermissionResourceActions => {
                Some(Self::PermissionResourceActions)
            }
            server_admin_contract::admin_data_table::AdminDataTable::PermissionResources => {
                Some(Self::PermissionResources)
            }
            server_admin_contract::admin_data_table::AdminDataTable::Rules => Some(Self::Rules),
            server_admin_contract::admin_data_table::AdminDataTable::RoleRules => {
                Some(Self::RoleRules)
            }
            server_admin_contract::admin_data_table::AdminDataTable::Roles => Some(Self::Roles),
            server_admin_contract::admin_data_table::AdminDataTable::SystemSettings => {
                Some(Self::SystemSettings)
            }
            server_admin_contract::admin_data_table::AdminDataTable::UserRoles => {
                Some(Self::UserRoles)
            }
            server_admin_contract::admin_data_table::AdminDataTable::Users => {
                Some(Self::UsersDatabaseRead)
            }
            server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus
            | server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts
            | server_admin_contract::admin_data_table::AdminDataTable::RateLimits
            | server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens => None,
        }
    }

    pub(crate) fn open_api(self) -> crate::utoipa_admin_open_api::UtoipaAdminOpenApi {
        crate::utoipa_admin_open_api::UtoipaAdminOpenApi::from(match self {
            Self::AccessSessions => {
                crate::admin_access_sessions::AdminAccessSessionsOpenApi::open_api()
            }
            Self::AuditLog => crate::admin_audit_log::AdminAuditLogOpenApi::open_api(),
            Self::PermissionActions => {
                crate::admin_permission_actions::AdminPermissionActionsOpenApi::open_api()
            }
            Self::PermissionResourceActions => crate::admin_permission_resource_actions::AdminPermissionResourceActionsOpenApi::open_api(),
            Self::PermissionResources => {
                crate::admin_permission_resources::AdminPermissionResourcesOpenApi::open_api()
            }
            Self::Roles => crate::admin_roles::AdminRolesOpenApi::open_api(),
            Self::RoleRules => crate::admin_role_rules::AdminRoleRulesOpenApi::open_api(),
            Self::UsersDatabaseRead => {
                crate::admin_users_database_read::AdminUsersDatabaseReadOpenApi::open_api()
            }
            Self::Rules => crate::admin_rules::AdminRulesOpenApi::open_api(),
            Self::SystemSettings => {
                crate::admin_system_settings::AdminSystemSettingsOpenApi::open_api()
            }
            Self::UserRoles => crate::admin_user_roles::AdminUserRolesOpenApi::open_api(),
        })
    }

    pub(crate) fn routes(
        self,
        shared_admin_generated_table_state_arc: &crate::shared_admin_generated_table_state_arc::SharedAdminGeneratedTableStateArc,
    ) -> server_runtime_http::axum_router::AxumRouter {
        server_runtime_http::axum_router::AxumRouter::from(match self {
            Self::AccessSessions => crate::admin_access_sessions::AdminAccessSessions::routes(
                std::sync::Arc::clone(shared_admin_generated_table_state_arc.get_inner()),
            ),
            Self::AuditLog => crate::admin_audit_log::AdminAuditLog::routes(std::sync::Arc::clone(
                shared_admin_generated_table_state_arc.get_inner(),
            )),
            Self::PermissionActions => {
                crate::admin_permission_actions::AdminPermissionActions::routes(
                    std::sync::Arc::clone(shared_admin_generated_table_state_arc.get_inner()),
                )
            }
            Self::PermissionResourceActions => {
                crate::admin_permission_resource_actions::AdminPermissionResourceActions::routes(
                    std::sync::Arc::clone(shared_admin_generated_table_state_arc.get_inner()),
                )
            }
            Self::PermissionResources => {
                crate::admin_permission_resources::AdminPermissionResources::routes(
                    std::sync::Arc::clone(shared_admin_generated_table_state_arc.get_inner()),
                )
            }
            Self::Roles => crate::admin_roles::AdminRoles::routes(std::sync::Arc::clone(
                shared_admin_generated_table_state_arc.get_inner(),
            )),
            Self::RoleRules => crate::admin_role_rules::AdminRoleRules::routes(
                std::sync::Arc::clone(shared_admin_generated_table_state_arc.get_inner()),
            ),
            Self::UsersDatabaseRead => {
                crate::admin_users_database_read::AdminUsersDatabaseRead::routes(
                    std::sync::Arc::clone(shared_admin_generated_table_state_arc.get_inner()),
                )
            }
            Self::Rules => crate::admin_rules::AdminRules::routes(std::sync::Arc::clone(
                shared_admin_generated_table_state_arc.get_inner(),
            )),
            Self::SystemSettings => crate::admin_system_settings::AdminSystemSettings::routes(
                std::sync::Arc::clone(shared_admin_generated_table_state_arc.get_inner()),
            ),
            Self::UserRoles => crate::admin_user_roles::AdminUserRoles::routes(
                std::sync::Arc::clone(shared_admin_generated_table_state_arc.get_inner()),
            ),
        })
    }

    pub(crate) fn route_contract(
        self,
        std_admin_str_ref: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
    ) -> Option<crate::admin_generated_route_contract::AdminGeneratedRouteContract> {
        match self {
            Self::AccessSessions => {
                crate::admin_access_sessions::AdminAccessSessionsRouteContract::for_path(
                    std_admin_str_ref.get(),
                )
                .map(|contract| {
                    crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                        contract
                            .rule()
                            .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                        server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::AuditLog => crate::admin_audit_log::AdminAuditLogRouteContract::for_path(
                std_admin_str_ref.get(),
            )
            .map(|contract| {
                crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                    contract
                        .rule()
                        .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                    server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::PermissionActions => {
                crate::admin_permission_actions::AdminPermissionActionsRouteContract::for_path(
                    std_admin_str_ref.get(),
                )
                .map(|contract| {
                    crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                        contract
                            .rule()
                            .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                        server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::PermissionResourceActions => crate::admin_permission_resource_actions::AdminPermissionResourceActionsRouteContract::for_path(
                std_admin_str_ref.get(),
            )
            .map(|contract| {
                crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                    contract
                        .rule()
                        .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                    server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::PermissionResources => {
                crate::admin_permission_resources::AdminPermissionResourcesRouteContract::for_path(
                    std_admin_str_ref.get(),
                )
                .map(|contract| {
                    crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                        contract
                            .rule()
                            .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                        server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::Roles => crate::admin_roles::AdminRolesRouteContract::for_path(
                std_admin_str_ref.get(),
            )
            .map(|contract| {
                crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                    contract
                        .rule()
                        .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                    server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::RoleRules => crate::admin_role_rules::AdminRoleRulesRouteContract::for_path(
                std_admin_str_ref.get(),
            )
            .map(|contract| {
                crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                    contract
                        .rule()
                        .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                    server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::UsersDatabaseRead => {
                crate::admin_users_database_read::AdminUsersDatabaseReadRouteContract::for_path(
                    std_admin_str_ref.get(),
                )
                .map(|contract| {
                    crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                        contract
                            .rule()
                            .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                        server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::Rules => crate::admin_rules::AdminRulesRouteContract::for_path(
                std_admin_str_ref.get(),
            )
            .map(|contract| {
                crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                    contract
                        .rule()
                        .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                    server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
            Self::SystemSettings => {
                crate::admin_system_settings::AdminSystemSettingsRouteContract::for_path(
                    std_admin_str_ref.get(),
                )
                .map(|contract| {
                    crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                        contract
                            .rule()
                            .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                        server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                        contract.frontend_contract().method(),
                    )
                })
            }
            Self::UserRoles => crate::admin_user_roles::AdminUserRolesRouteContract::for_path(
                std_admin_str_ref.get(),
            )
            .map(|contract| {
                crate::admin_generated_route_contract::AdminGeneratedRouteContract::new(
                    contract
                        .rule()
                        .map(server_admin_core::std_admin_str_ref::StdAdminStrRef::from),
                    server_admin_core::std_admin_bool::StdAdminBool::from(contract.mutates()),
                    contract.frontend_contract().method(),
                )
            }),
        }
    }
}
