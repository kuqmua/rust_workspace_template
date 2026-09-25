#![allow(
    clippy::unused_trait_names,
    reason = "the Leptos grid cells and column headings require attribute traits after macro expansion"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[allow(
    clippy::single_call_fn,
    reason = "admin data table grid remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn admin_data_table_grid(
    admin_data_table_view: &server_admin_contract::admin_data_table_view::AdminDataTableView,
    active_field: Option<&server_admin_contract::admin_filter_field::AdminFilterField>,
    active_operation: Option<
        &server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey,
    >,
    active_value: Option<&server_admin_contract::admin_filter_value::AdminFilterValue>,
    active_end: Option<&server_admin_contract::admin_filter_value::AdminFilterValue>,
    admin_page_limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    admin_data_table_frontend_path: &server_admin_contract::admin_data_table_frontend_path::AdminDataTableFrontendPath,
    is_sessions: bool,
    can_update: bool,
) -> impl leptos::prelude::IntoView + use<> {
    #[cfg(not(target_arch = "wasm32"))]
    let _: bool = is_sessions;
    let columns = admin_data_table_view
        .columns()
        .iter()
        .map(|column| {
            let field = column.name().to_string();
            let label = column.name().to_string();
            let filter_count = column.filters().len().to_string();
            let filter = (bool::from(admin_data_table_view.table().supports_filters())
                && !column.filters().is_empty())
            .then(|| {
                crate::admin_column_filter::admin_column_filter(
                    admin_data_table_frontend_path,
                    column.name(),
                    column.input_kind(),
                    column.filters(),
                    active_field,
                    active_operation,
                    active_value,
                    active_end,
                    admin_page_limit,
                )
            });
            leptos::view! {
                <crate::table_head::TableHead data_field=field data_filter_count=filter_count>
                    <div class="table-column-heading">
                        <span>{label}</span>
                        {filter}
                    </div>
                </crate::table_head::TableHead>
            }
        })
        .collect::<Vec<_>>();
    let rows = admin_data_table_view
        .items()
        .iter()
        .map(|item| {
            let value_for_column = |column_name| {
                let index = admin_data_table_view
                    .columns()
                    .iter()
                    .position(|admin_data_column| admin_data_column.name().as_ref() == column_name)?;
                item.values().get(index)
            };
            let row_identifier = value_for_column(constants_str::SQL_NAMES_ID)
                .map(|admin_text| admin_text.as_ref().as_str());
            let read_path = match admin_data_table_view.table() {
                server_admin_contract::admin_data_table::AdminDataTable::UserRoles => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_user_role_id::AdminUserRoleId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::RoleRules => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_role_rule_id::AdminRoleRuleId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens => row_identifier
                    .map(str::to_owned)
                    .map(server_admin_contract::admin_refresh_token_id::AdminRefreshTokenId::try_from)
                    .and_then(Result::ok)
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::AccessSessions => row_identifier
                    .map(str::to_owned)
                    .map(server_admin_contract::admin_access_session_id::AdminAccessSessionId::try_from)
                    .and_then(Result::ok)
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_login_attempt_id::AdminLoginAttemptId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::AuditLog => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_audit_log_id::AdminAuditLogId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_cleanup_status_id::AdminCleanupStatusId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::RateLimits => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_rate_limit_id::AdminRateLimitId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::SystemSettings => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_system_setting_id::AdminSystemSettingId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::Rules => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_rule_id::AdminRuleId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::Roles => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_role_id::AdminRoleId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::PermissionActions => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_permission_action_id::AdminPermissionActionId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::PermissionResourceActions => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_permission_resource_action_id::AdminPermissionResourceActionId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::PermissionResources => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_permission_resource_id::AdminPermissionResourceId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
                server_admin_contract::admin_data_table::AdminDataTable::Users => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_user_id::AdminUserId::try_from(value).ok())
                    .map(server_admin_contract::admin_route_path::AdminRoutePath::from),
            };
            let read_action = read_path.map(|admin_route_path| {
                let fields = admin_data_table_view.columns().iter().zip(item.values()).map(|(column, value)| {
                    let label = column.name().to_string();
                    let value = value.to_string();
                    leptos::view! {
                        <div class="health-label">{label}</div>
                        <div class="health-result">{value}</div>
                    }
                }).collect::<Vec<_>>();
                leptos::prelude::IntoAny::into_any(leptos::view! {
                    <crate::admin_read_action::AdminReadAction read_path=admin_route_path>{fields}</crate::admin_read_action::AdminReadAction>
                })
                });
            let update_action = can_update.then(|| {
                if admin_data_table_view.table()
                    == server_admin_contract::admin_data_table::AdminDataTable::Users
                {
                    Some(leptos::prelude::IntoAny::into_any(leptos::view! {
                        <crate::admin_user_update_action::AdminUserUpdateAction />
                    }))
                } else if admin_data_table_view.table()
                    == server_admin_contract::admin_data_table::AdminDataTable::Roles
                {
                    Some(leptos::prelude::IntoAny::into_any(leptos::view! {
                        <crate::admin_role_update_action::AdminRoleUpdateAction />
                    }))
                } else {
                    None
                }
            }).flatten();
            let cells = item
                .values()
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let column = admin_data_table_view.columns().get(index);
                    let label =
                        column.map_or_else(String::new, |column| column.name().to_string());
                    let field =
                        column.map_or_else(String::new, |column| column.name().to_string());
                    let numeric = column.is_some_and(|column| {
                        matches!(column.input_kind(), frontend_contract::input_kind::InputKind::Number)
                    });
                    let value_text = value.to_string();
                    leptos::view! { <crate::table_cell::TableCell data_label=label data_field=field class=if numeric { "numeric-cell" } else { "" }>{value_text}</crate::table_cell::TableCell> }
                })
                .collect::<Vec<_>>();
            let action_cell = {
                #[cfg(target_arch = "wasm32")]
                let actions = if is_sessions {
                    let session_identifier = row_identifier
                        .and_then(|value| server_admin_contract::admin_session_identifier::AdminSessionIdentifier::try_from(value.to_owned()).ok());
                    if let Some(revoke_session_id) = session_identifier {
                        let dialog_id = format!("revoke-session-{revoke_session_id}");
                        leptos::prelude::IntoAny::into_any(leptos::view! {
                            <crate::admin_table_actions::AdminTableActions read_action=read_action command_for=dialog_id.clone()>
                                <crate::admin_alert_dialog::AdminAlertDialog string=dialog_id title=constants_str::ADMIN_UI_REVOKE_SESSION description=constants_str::ADMIN_UI_THIS_ADMINISTRATOR_SESSION_WILL_BE_SIGNED_OUT_IMMEDIATELY trigger=constants_str::ADMIN_BUTTON_REVOKE_SESSION confirm=constants_str::ADMIN_BUTTON_REVOKE dialog_only=true callback=leptos::prelude::Callback::new(move |()| {
                                    if let Ok(path) = crate::admin_route_path_url::admin_route_path_url(&server_admin_contract::admin_parameterized_route_path::admin_parameterized_route_path::<server_admin_contract::admin_revoke_session_route::AdminRevokeSessionRoute>(&revoke_session_id)) {
                                        crate::reload_after::reload_after(crate::admin_mutation_method::AdminMutationMethod::Delete, path, server_admin_contract::admin_no_body::AdminNoBody);
                                    }
                                }) />
                            </crate::admin_table_actions::AdminTableActions>
                        })
                    } else {
                        leptos::prelude::IntoAny::into_any(read_action)
                    }
                } else {
                    leptos::prelude::IntoAny::into_any(leptos::view! { <div class="table-actions">{read_action}{update_action}</div> })
                };
                #[cfg(not(target_arch = "wasm32"))]
                let actions = leptos::view! { <div class="table-actions">{read_action}{update_action}</div> };
                leptos::view! { <crate::table_cell::TableCell data_label=constants_str::ADMIN_UI_ACTIONS bool=true>{actions}</crate::table_cell::TableCell> }
            };
            leptos::view! {
                <crate::table_row::TableRow>{cells}{action_cell}</crate::table_row::TableRow>
            }
        })
        .collect::<Vec<_>>();
    leptos::view! {
        <crate::table_wrapper::TableWrapper><crate::table::Table>
            <crate::table_header::TableHeader><crate::table_row::TableRow>{columns}<crate::table_head::TableHead>{constants_str::ADMIN_UI_ACTIONS}</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody>
        </crate::table::Table></crate::table_wrapper::TableWrapper>
    }
}
