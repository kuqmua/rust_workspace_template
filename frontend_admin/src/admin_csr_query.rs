#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
)]
#[getters(bare)]
#[derive(proc_macro_new::New)]
pub(crate) struct AdminCsrQuery {
    access_session_id: Option<server_admin_contract::admin_access_session_id::AdminAccessSessionId>,
    #[getters(copy)]
    audit_log_id: Option<server_admin_contract::admin_audit_log_id::AdminAuditLogId>,
    direction: Option<server_admin_contract::admin_text::AdminText>,
    filter_end: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    filter_field: Option<server_admin_contract::admin_filter_field::AdminFilterField>,
    filter_operation:
        Option<server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey>,
    filter_value: Option<server_admin_contract::admin_filter_value::AdminFilterValue>,
    #[getters(copy)]
    limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    #[getters(copy)]
    login_attempt_id: Option<server_admin_contract::admin_login_attempt_id::AdminLoginAttemptId>,
    #[getters(copy)]
    offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    search: server_admin_contract::admin_table_search::AdminTableSearch,
    sort: server_admin_contract::admin_table_sort_key::AdminTableSortKey,
    #[getters(copy)]
    system_setting_id: Option<server_admin_contract::admin_system_setting_id::AdminSystemSettingId>,
    #[getters(copy)]
    table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
    #[getters(copy)]
    permission_id: Option<server_admin_contract::admin_permission_id::AdminPermissionId>,
    #[getters(copy)]
    role_id: Option<server_admin_contract::admin_role_id::AdminRoleId>,
    #[getters(copy)]
    user_id: Option<server_admin_contract::admin_user_id::AdminUserId>,
    #[getters(copy)]
    user_role_id: Option<server_admin_contract::admin_user_role_id::AdminUserRoleId>,
    #[getters(copy)]
    role_permission_id:
        Option<server_admin_contract::admin_role_permission_id::AdminRolePermissionId>,
    refresh_token_id: Option<server_admin_contract::admin_refresh_token_id::AdminRefreshTokenId>,
}
impl AdminCsrQuery {
    pub(crate) fn from_location() -> Result<Self, crate::admin_table_load_error::AdminTableLoadError>
    {
        let window =
            web_sys::window().ok_or(crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let search = window
            .location()
            .search()
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let params = web_sys::UrlSearchParams::new_with_str(&search)
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let pathname = window
            .location()
            .pathname()
            .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Fetch)?;
        let access_session_id = server_admin_contract::admin_access_session_id::AdminAccessSessionId::from_frontend_path(
            server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str()),
        );
        let audit_log_id =
            server_admin_contract::admin_audit_log_id::AdminAuditLogId::from_frontend_path(
                server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(
                    pathname.as_str(),
                ),
            );
        let login_attempt_id =
            server_admin_contract::admin_login_attempt_id::AdminLoginAttemptId::from_frontend_path(
                server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(
                    pathname.as_str(),
                ),
            );
        let user_role_id =
            server_admin_contract::admin_user_role_id::AdminUserRoleId::from_frontend_path(
                server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(
                    pathname.as_str(),
                ),
            );
        let role_permission_id = server_admin_contract::admin_role_permission_id::AdminRolePermissionId::from_frontend_path(
            server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str()),
        );
        let refresh_token_id =
            server_admin_contract::admin_refresh_token_id::AdminRefreshTokenId::from_frontend_path(
                server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(
                    pathname.as_str(),
                ),
            );
        let table = server_admin_contract::admin_data_table::AdminDataTable::from_frontend_path(
            server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str()),
        );
        let system_setting_id = server_admin_contract::admin_system_setting_id::AdminSystemSettingId::from_frontend_path(
            server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str()),
        );
        let user_id = server_admin_contract::admin_user_id::AdminUserId::from_frontend_path(
            server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str()),
        );
        let permission_id =
            server_admin_contract::admin_permission_id::AdminPermissionId::from_frontend_path(
                server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(
                    pathname.as_str(),
                ),
            );
        let role_id = server_admin_contract::admin_role_id::AdminRoleId::from_frontend_path(
            server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str()),
        );
        let resolved_table = table
            .or_else(|| {
                access_session_id.as_ref().map(|_access_session_id| {
                    server_admin_contract::admin_data_table::AdminDataTable::AccessSessions
                })
            })
            .or_else(|| {
                audit_log_id.map(|_audit_log_id| {
                    server_admin_contract::admin_data_table::AdminDataTable::AuditLog
                })
            })
            .or_else(|| {
                login_attempt_id.map(|_login_attempt_id| {
                    server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts
                })
            })
            .or_else(|| {
                user_role_id.map(|_user_role_id| {
                    server_admin_contract::admin_data_table::AdminDataTable::UserRoles
                })
            })
            .or_else(|| {
                role_permission_id.map(|_role_permission_id| {
                    server_admin_contract::admin_data_table::AdminDataTable::RolePermissions
                })
            })
            .or_else(|| {
                refresh_token_id.as_ref().map(|_refresh_token_id| {
                    server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens
                })
            })
            .or_else(|| {
                system_setting_id.map(|_system_setting_id| {
                    server_admin_contract::admin_data_table::AdminDataTable::SystemSettings
                })
            });
        Ok(Self::new(
            access_session_id,
            audit_log_id,
            params
                .get(constants_str::ADMIN_DIRECTION_QUERY_KEY)
                .map(server_admin_contract::admin_text::AdminText::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_FILTER_END_QUERY_KEY)
                .map(server_admin_contract::admin_filter_value::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_FILTER_FIELD_QUERY_KEY)
                .map(server_admin_contract::admin_filter_field::AdminFilterField::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_FILTER_OPERATION_QUERY_KEY)
                .map(server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_FILTER_VALUE_QUERY_KEY)
                .map(server_admin_contract::admin_filter_value::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?,
            params
                .get(constants_str::ADMIN_LIMIT_QUERY_KEY)
                .and_then(|value| value.parse::<u16>().ok())
                .and_then(|value| {
                    server_admin_contract::admin_page_limit::AdminPageLimit::try_from(value).ok()
                })
                .unwrap_or_default(),
            login_attempt_id,
            params
                .get(constants_str::ADMIN_OFFSET_QUERY_KEY)
                .and_then(|value| value.parse::<u32>().ok())
                .map_or_else(
                    server_admin_contract::admin_page_offset::AdminPageOffset::default,
                    server_admin_contract::admin_page_offset::AdminPageOffset::from,
                ),
            params
                .get(constants_str::ADMIN_SEARCH_QUERY_KEY)
                .map(server_admin_contract::admin_table_search::AdminTableSearch::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?
                .unwrap_or_default(),
            params
                .get(constants_str::ADMIN_SORT_QUERY_KEY)
                .map(server_admin_contract::admin_table_sort_key::AdminTableSortKey::try_from)
                .transpose()
                .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)?
                .unwrap_or_default(),
            system_setting_id,
            resolved_table,
            permission_id,
            role_id,
            user_id,
            user_role_id,
            role_permission_id,
            refresh_token_id,
        ))
    }
}
