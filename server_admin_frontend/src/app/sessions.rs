use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::app) fn AdminSessionsView(
    page: server_admin_contract::AdminSessionsPage,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div data-name="TableWrapper" class="table-scroll max-h-96 overflow-auto rounded-md border"><table data-name="Table" class="w-full max-w-7xl text-sm caption-bottom"><thead data-name="TableHeader" class="[&_tr]:border-b sticky top-0 z-10 bg-card"><tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50"><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"session"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"created"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"expires"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"current"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"actions"</th></tr></thead>
            <tbody data-name="TableBody" class="[&_tr:last-child]:border-0">{page.items().iter().map(|item| {
                let session_id = item.id().to_string();
                let created_at = item.created_at().to_string();
                let expires_at = item.expires_at().to_string();
                let is_current = bool::from(item.is_current());
                let current_text = item.is_current().to_string();
                let revoke_session_id = item.id().clone();
                let dialog_id = format!("revoke-session-{revoke_session_id}");
                leptos::view! {
                <tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50">
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="session">{session_id}</td>
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="created">{created_at}</td>
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="expires">{expires_at}</td>
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="current"><crate::ui::badge::AdminBadge variant=if is_current { crate::ui::badge::AdminBadgeVariant::Success } else { crate::ui::badge::AdminBadgeVariant::Neutral }>{current_text}</crate::ui::badge::AdminBadge></td>
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="actions"><div class="table-actions"><crate::ui::alert_dialog::AdminAlertDialog id=dialog_id title="Revoke session?" description="This administrator session will be signed out immediately." trigger="Revoke session" confirm="Revoke" on_confirm=leptos::prelude::Callback::new(move |()| {
                        if let Ok(path) = super::http::url::admin_route_path_url(server_admin_contract::admin_parameterized_route_path::<server_admin_contract::AdminRevokeSessionRoute>(&revoke_session_id)) {
                            super::mutation::reload_after(super::mutation::AdminMutationMethod::Delete, path, server_admin_contract::AdminNoBody);
                        }
                    }) /></div></td>
                </tr>
            }}).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
        </section>
    }
}
