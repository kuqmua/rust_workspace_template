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
) -> impl leptos::prelude::IntoView {
    let total = admin_sessions_page.total();
    let rows = admin_sessions_page.items().iter().map(|item| {
        let session_id = item.id().to_string();
        let created_at = item.created_at().to_string();
        let expires_at = item.expires_at().to_string();
        let is_current = bool::from(item.is_current());
        let current_text = item.is_current().to_string();
        let revoke_session_id = item.id().clone();
        let dialog_id = format!("revoke-session-{revoke_session_id}");
        leptos::view! {
            <crate::table_row::TableRow>
                <crate::table_cell::TableCell data_label="session">{session_id}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="created">{created_at}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="expires">{expires_at}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="current"><crate::admin_badge::AdminBadge admin_badge_variant=if is_current { crate::admin_badge_variant::AdminBadgeVariant::Success } else { crate::admin_badge_variant::AdminBadgeVariant::Neutral }>{current_text}</crate::admin_badge::AdminBadge></crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="actions"><div class="table-actions"><crate::admin_alert_dialog::AdminAlertDialog string=dialog_id title="Revoke session?" description="This administrator session will be signed out immediately." trigger="Revoke session" confirm="Revoke" callback=leptos::prelude::Callback::new(move |()| {
                    if let Ok(path) = crate::admin_route_path_url::admin_route_path_url(&server_admin_contract::admin_parameterized_route_path::admin_parameterized_route_path::<server_admin_contract::admin_revoke_session_route::AdminRevokeSessionRoute>(&revoke_session_id)) {
                        crate::reload_after::reload_after(crate::admin_mutation_method::AdminMutationMethod::Delete, path, server_admin_contract::admin_no_body::AdminNoBody);
                    }
                }) /></div></crate::table_cell::TableCell>
            </crate::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    leptos::view! {
        <section class="table-admin_sessions_page" data-renderer="csr">
            <div class="resource-actions">
                <crate::admin_alert_dialog::AdminAlertDialog
                    string=String::from(constants_str::ADMIN_REVOKE_ALL_SESSIONS_DIALOG)
                    title=constants_str::ADMIN_REVOKE_ALL_SESSIONS_LABEL
                    description=constants_str::ADMIN_REVOKE_ALL_SESSIONS_DESCRIPTION
                    trigger=constants_str::ADMIN_REVOKE_ALL_SESSIONS_LABEL
                    confirm=constants_str::ADMIN_REVOKE_ALL_SESSIONS_LABEL
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
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"session"</crate::table_head::TableHead><crate::table_head::TableHead>"created"</crate::table_head::TableHead><crate::table_head::TableHead>"expires"</crate::table_head::TableHead><crate::table_head::TableHead>"current"</crate::table_head::TableHead><crate::table_head::TableHead>"actions"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <p>{format!("{total} total")}</p>
        </section>
    }
}
