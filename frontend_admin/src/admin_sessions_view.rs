#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminSessionsView(
    admin_sessions_page: server_admin_contract::admin_sessions_page::AdminSessionsPage,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let total = admin_sessions_page.total();
    let rows = admin_sessions_page.items().iter().map(|item| {
        let session_id = item.id().to_string();
        let created_at = item.created_at().to_string();
        let expires_at = item.expires_at().to_string();
        let current_text = item.is_current().to_string();
        let revoke_session_id = item.id().clone();
        let dialog_id = format!("revoke-session-{revoke_session_id}");
        leptos::view! {
            <crate::table_row::TableRow>
                <crate::table_cell::TableCell data_label="session">{session_id}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="created">{created_at}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="expires">{expires_at}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="current">{current_text}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="actions" bool=true><div class="table-actions"><crate::admin_alert_dialog::AdminAlertDialog string=dialog_id title=constants_str::ADMIN_UI_REVOKE_SESSION description=constants_str::ADMIN_UI_THIS_ADMINISTRATOR_SESSION_WILL_BE_SIGNED_OUT_IMMEDIATELY trigger=constants_str::ADMIN_BUTTON_REVOKE_SESSION confirm=constants_str::ADMIN_BUTTON_REVOKE callback=leptos::prelude::Callback::new(move |()| {
                    if let Ok(path) = crate::admin_route_path_url::admin_route_path_url(&server_admin_contract::admin_parameterized_route_path::admin_parameterized_route_path::<server_admin_contract::admin_revoke_session_route::AdminRevokeSessionRoute>(&revoke_session_id)) {
                        crate::reload_after::reload_after(crate::admin_mutation_method::AdminMutationMethod::Delete, path, server_admin_contract::admin_no_body::AdminNoBody);
                    }
                }) /></div></crate::table_cell::TableCell>
            </crate::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    let identifier_filters = [
        frontend_contract::filter_operation::FilterOperation::Eq,
        frontend_contract::filter_operation::FilterOperation::In,
    ]
    .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let timestamp_filters = [
        frontend_contract::filter_operation::FilterOperation::Eq,
        frontend_contract::filter_operation::FilterOperation::Before,
        frontend_contract::filter_operation::FilterOperation::Between,
    ]
    .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let boolean_filters = [frontend_contract::filter_operation::FilterOperation::Eq]
        .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let sessions_path =
        server_admin_contract::admin_data_table_frontend_path::AdminDataTableFrontendPath::from(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions,
        );
    let column_filter =
        |admin_text: &server_admin_contract::admin_text::AdminText,
         input_kind: frontend_contract::input_kind::InputKind,
         filters: &[server_admin_contract::admin_data_filter::AdminDataFilter]| {
            crate::admin_column_filter::admin_column_filter(
                &sessions_path,
                admin_text,
                input_kind,
                filters,
                admin_csr_query.filter_field(),
                admin_csr_query.filter_operation(),
                admin_csr_query.filter_value(),
                admin_csr_query.filter_end(),
                admin_csr_query.limit(),
            )
        };
    let identifier = server_admin_contract::admin_text::AdminText::try_from(
        constants_str::SQL_NAMES_ID.to_owned(),
    )
    .ok();
    let created_at = server_admin_contract::admin_text::AdminText::try_from(
        constants_str::CREATED_AT.to_owned(),
    )
    .ok();
    let expires_at = server_admin_contract::admin_text::AdminText::try_from(
        constants_str::EXPIRES_AT.to_owned(),
    )
    .ok();
    let current =
        server_admin_contract::admin_text::AdminText::try_from(constants_str::CURRENT.to_owned())
            .ok();
    let identifier_filter = identifier.as_ref().map(|identifier| {
        column_filter(
            identifier,
            frontend_contract::input_kind::InputKind::Uuid,
            &identifier_filters,
        )
    });
    let created_at_filter = created_at.as_ref().map(|created_at| {
        column_filter(
            created_at,
            frontend_contract::input_kind::InputKind::DateTime,
            &timestamp_filters,
        )
    });
    let expires_at_filter = expires_at.as_ref().map(|expires_at| {
        column_filter(
            expires_at,
            frontend_contract::input_kind::InputKind::DateTime,
            &timestamp_filters,
        )
    });
    let current_filter = current.as_ref().map(|current| {
        column_filter(
            current,
            frontend_contract::input_kind::InputKind::Checkbox,
            &boolean_filters,
        )
    });
    leptos::view! {
        <section class="table-admin_sessions_page" data-renderer="csr">
            <div class="resource-actions">
                <crate::admin_alert_dialog::AdminAlertDialog
                    string=String::from(constants_str::ADMIN_REVOKE_ALL_SESSIONS_DIALOG)
                    title=constants_str::ADMIN_UI_REVOKE_ALL_SESSIONS
                    description=constants_str::ADMIN_UI_END_ALL_SESSIONS
                    trigger=constants_str::ADMIN_BUTTON_REVOKE_ALL_SESSIONS
                    confirm=constants_str::ADMIN_BUTTON_REVOKE_ALL_SESSIONS
                    callback=leptos::prelude::Callback::new(move |()| {
                        match crate::admin_api_url::admin_api_url(server_admin_contract::admin_route::AdminRoute::RevokeAllSessions) {
                            Ok(path) => crate::reload_after::reload_after(
                                crate::admin_mutation_method::AdminMutationMethod::Delete,
                                path,
                                server_admin_contract::admin_no_body::AdminNoBody,
                            ),
                            Err(error) => crate::show_mutation_error::show_mutation_error(&error),
                        }
                    })
                />
            </div>
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead data_field=constants_str::SQL_NAMES_ID.to_owned() data_filter_count=identifier_filters.len().to_string()><div class="table-column-heading"><span>"session"</span>{identifier_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead data_field=constants_str::CREATED_AT.to_owned() data_filter_count=timestamp_filters.len().to_string()><div class="table-column-heading"><span>"created"</span>{created_at_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead data_field=constants_str::EXPIRES_AT.to_owned() data_filter_count=timestamp_filters.len().to_string()><div class="table-column-heading"><span>"expires"</span>{expires_at_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead data_field=constants_str::CURRENT.to_owned() data_filter_count=boolean_filters.len().to_string()><div class="table-column-heading"><span>"current"</span>{current_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead>"actions"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination admin_frontend_path=server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions admin_csr_query=admin_csr_query admin_page_total=total />
        </section>
    }
}
