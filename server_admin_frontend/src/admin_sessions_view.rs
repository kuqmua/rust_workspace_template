use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(crate) fn AdminSessionsView(
    page: server_admin_contract::admin_sessions_page::AdminSessionsPage,
) -> impl leptos::prelude::IntoView {
    let total = page.total();
    let rows = page.items().iter().map(|item| {
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
                <crate::table_cell::TableCell data_label="current"><crate::admin_badge::AdminBadge variant=if is_current { crate::admin_badge_variant::AdminBadgeVariant::Success } else { crate::admin_badge_variant::AdminBadgeVariant::Neutral }>{current_text}</crate::admin_badge::AdminBadge></crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="actions"><div class="table-actions"><crate::admin_alert_dialog::AdminAlertDialog id=dialog_id title="Revoke session?" description="This administrator session will be signed out immediately." trigger="Revoke session" confirm="Revoke" on_confirm=leptos::prelude::Callback::new(move |()| {
                    if let Ok(path) = super::http::url::admin_route_path_url(server_admin_contract::admin_parameterized_route_path::admin_parameterized_route_path::<server_admin_contract::admin_revoke_session_route::AdminRevokeSessionRoute>(&revoke_session_id)) {
                        super::mutation::reload_after(super::mutation::AdminMutationMethod::Delete, path, server_admin_contract::admin_no_body::AdminNoBody);
                    }
                }) /></div></crate::table_cell::TableCell>
            </crate::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"session"</crate::table_head::TableHead><crate::table_head::TableHead>"created"</crate::table_head::TableHead><crate::table_head::TableHead>"expires"</crate::table_head::TableHead><crate::table_head::TableHead>"current"</crate::table_head::TableHead><crate::table_head::TableHead>"actions"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <p>{format!("{} total", total)}</p>
        </section>
    }
}
