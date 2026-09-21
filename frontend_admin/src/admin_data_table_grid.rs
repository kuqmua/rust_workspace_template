#![allow(
    clippy::unused_trait_names,
    reason = "the Leptos grid cells and column headings require attribute traits after macro expansion"
)]

use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

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
) -> impl leptos::prelude::IntoView + use<> {
    let columns = admin_data_table_view
        .columns()
        .iter()
        .map(|column| {
            let field = column.name().to_string();
            let label = column.name().to_string();
            let filter_count = column.filters().len().to_string();
            let table_path = admin_data_table_view.table().frontend_path();
            let filter = (bool::from(admin_data_table_view.table().supports_filters())
                && !column.filters().is_empty())
            .then(|| {
                crate::admin_column_filter::admin_column_filter(
                    &table_path,
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
                server_admin_contract::admin_data_table::AdminDataTable::RolePermissions => row_identifier
                    .and_then(|value| value.parse::<i64>().ok())
                    .and_then(|value| server_admin_contract::admin_role_permission_id::AdminRolePermissionId::try_from(value).ok())
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
                server_admin_contract::admin_data_table::AdminDataTable::Permissions
                | server_admin_contract::admin_data_table::AdminDataTable::Roles
                | server_admin_contract::admin_data_table::AdminDataTable::Users => None,
            };
            let read_link = read_path.map(|admin_route_path| {
                    leptos::view! { <singlestage::Link class=crate::admin_button_variant::AdminButtonVariant::Secondary.class() href=admin_route_path.to_string() attr:aria-label=constants_str::PG_CRUD_READ_PERMISSION_ACTION attr:title=constants_str::PG_CRUD_READ_PERMISSION_ACTION>
                        <svg viewBox="0 0 24 24" aria-hidden=constants_str::TRUE fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12Z"></path>
                            <circle cx="12" cy="12" r="3"></circle>
                        </svg>
                    </singlestage::Link> }
                });
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
            leptos::view! {
                <crate::table_row::TableRow>{cells}<crate::table_cell::TableCell data_label=constants_str::ADMIN_UI_ACTIONS bool=true>{read_link}</crate::table_cell::TableCell></crate::table_row::TableRow>
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
