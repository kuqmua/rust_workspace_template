use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::app) fn AdminSessionsView(
    page: server_admin_contract::AdminSessionsPage,
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
            <crate::ui::table::TableRow>
                <crate::ui::table::TableCell data_label="session">{session_id}</crate::ui::table::TableCell>
                <crate::ui::table::TableCell data_label="created">{created_at}</crate::ui::table::TableCell>
                <crate::ui::table::TableCell data_label="expires">{expires_at}</crate::ui::table::TableCell>
                <crate::ui::table::TableCell data_label="current"><crate::ui::badge::AdminBadge variant=if is_current { crate::ui::badge::AdminBadgeVariant::Success } else { crate::ui::badge::AdminBadgeVariant::Neutral }>{current_text}</crate::ui::badge::AdminBadge></crate::ui::table::TableCell>
                <crate::ui::table::TableCell data_label="actions"><div class="table-actions"><crate::ui::alert_dialog::AdminAlertDialog id=dialog_id title="Revoke session?" description="This administrator session will be signed out immediately." trigger="Revoke session" confirm="Revoke" on_confirm=leptos::prelude::Callback::new(move |()| {
                    if let Ok(path) = super::http::url::admin_route_path_url(server_admin_contract::admin_parameterized_route_path::<server_admin_contract::AdminRevokeSessionRoute>(&revoke_session_id)) {
                        super::mutation::reload_after(super::mutation::AdminMutationMethod::Delete, path, server_admin_contract::AdminNoBody);
                    }
                }) /></div></crate::ui::table::TableCell>
            </crate::ui::table::TableRow>
        }
    }).collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <crate::ui::table::TableWrapper><crate::ui::table::Table><crate::ui::table::TableHeader><crate::ui::table::TableRow><crate::ui::table::TableHead>"session"</crate::ui::table::TableHead><crate::ui::table::TableHead>"created"</crate::ui::table::TableHead><crate::ui::table::TableHead>"expires"</crate::ui::table::TableHead><crate::ui::table::TableHead>"current"</crate::ui::table::TableHead><crate::ui::table::TableHead>"actions"</crate::ui::table::TableHead></crate::ui::table::TableRow></crate::ui::table::TableHeader>
            <crate::ui::table::TableBody>{rows}</crate::ui::table::TableBody></crate::ui::table::Table></crate::ui::table::TableWrapper>
            <p>{format!("{} total", total)}</p>
        </section>
    }
}
