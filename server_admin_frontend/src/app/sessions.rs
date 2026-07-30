use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

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
            <div class="table-scroll"><table><thead><tr><th>"session"</th><th>"created"</th><th>"expires"</th><th>"current"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                let session_id = item.id().to_string();
                let revoke_session_id = item.id().clone();
                leptos::view! {
                <tr>
                    <td data-label="session">{session_id}</td>
                    <td data-label="created">{item.created_at().to_string()}</td>
                    <td data-label="expires">{item.expires_at().to_string()}</td>
                    <td data-label="current">{item.is_current().to_string()}</td>
                    <td data-label="actions"><div class="table-actions"><button type="button" on:click=move |_event| {
                        if bool::from(super::mutation::mutation_confirmed(super::mutation::MutationConfirmationMessageRef::from("Revoke this session?"))) && let Ok(path) = super::http::url::admin_route_path_url(server_admin_contract::admin_parameterized_route_path::<server_admin_contract::AdminRevokeSessionRoute>(&revoke_session_id)) {
                            super::mutation::reload_after(super::mutation::AdminMutationMethod::Delete, path, server_admin_contract::AdminNoBody);
                        }
                    }>"Revoke session"</button></div></td>
                </tr>
            }}).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
        </section>
    }
}
