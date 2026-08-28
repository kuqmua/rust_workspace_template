use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(crate) fn AdminSessionsView(
    page: server_admin_contract::domain_types::AdminSessionsPage,
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
            <crate::domain_types::with_owner::tables::table_row::TableRow>
                <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="session">{session_id}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="created">{created_at}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="expires">{expires_at}</crate::domain_types::with_owner::tables::table_cell::TableCell>
                <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="current"><crate::domain_types::with_owner::badge::AdminBadge variant=if is_current { crate::domain_types::with_owner::badge::AdminBadgeVariant::Success } else { crate::domain_types::with_owner::badge::AdminBadgeVariant::Neutral }>{current_text}</crate::domain_types::with_owner::badge::AdminBadge></crate::domain_types::with_owner::tables::table_cell::TableCell>
                <crate::domain_types::with_owner::tables::table_cell::TableCell data_label="actions"><div class="table-actions"><crate::domain_types::with_owner::admin_alert_dialog::AdminAlertDialog id=dialog_id title="Revoke session?" description="This administrator session will be signed out immediately." trigger="Revoke session" confirm="Revoke" on_confirm=leptos::prelude::Callback::new(move |()| {
                    if let Ok(path) = super::http::url::admin_route_path_url(server_admin_contract::domain_types::admin_parameterized_route_path::<server_admin_contract::domain_types::AdminRevokeSessionRoute>(&revoke_session_id)) {
                        super::mutation::reload_after(super::mutation::AdminMutationMethod::Delete, path, server_admin_contract::domain_types::AdminNoBody);
                    }
                }) /></div></crate::domain_types::with_owner::tables::table_cell::TableCell>
            </crate::domain_types::with_owner::tables::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <crate::domain_types::with_owner::tables::table_wrapper::TableWrapper><crate::domain_types::with_owner::tables::table::Table><crate::domain_types::with_owner::tables::table_header::TableHeader><crate::domain_types::with_owner::tables::table_row::TableRow><crate::domain_types::with_owner::tables::table_head::TableHead>"session"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"created"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"expires"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"current"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"actions"</crate::domain_types::with_owner::tables::table_head::TableHead></crate::domain_types::with_owner::tables::table_row::TableRow></crate::domain_types::with_owner::tables::table_header::TableHeader>
            <crate::domain_types::with_owner::tables::table_body::TableBody>{rows}</crate::domain_types::with_owner::tables::table_body::TableBody></crate::domain_types::with_owner::tables::table::Table></crate::domain_types::with_owner::tables::table_wrapper::TableWrapper>
            <p>{format!("{} total", total)}</p>
        </section>
    }
}
