#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> super::AdminSsrHtml;
}
impl<View> AdminSsrViewExt for View
where
    View: leptos::prelude::IntoAny,
{
    fn render_admin_ssr(self) -> super::AdminSsrHtml {
        super::AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(
            leptos::prelude::IntoAny::into_any(self),
        ))
        .unwrap_or_else(super::AdminSsrHtml::from)
    }
}

#[must_use]
pub(super) fn render_sessions(
    page: &server_admin_contract::AdminSessionsPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let content = leptos::view! {
        <section class="table-page">
        <div class="table-scroll"><table><thead><tr><th>"session"</th><th>"created"</th><th>"expires"</th><th>"current"</th><th>"actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td data-label="session">{item.id().to_string()}</td><td data-label="created">{item.created_at().to_string()}</td><td data-label="expires">{item.expires_at().to_string()}</td><td data-label="current">{item.is_current().to_string()}</td><td data-label="actions"><details><summary>"Revoke"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::SessionRevoke.get()><input type="hidden" name="session_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm session revocation"</label><button class="danger-button" type="submit">"Revoke session"</button></form></details></td></tr>
        }).collect::<Vec<_>>()}</tbody></table></div>
        {super::table_pagination(server_admin_contract::AdminPage::Sessions, query, page.total(), None, None)}
        </section>
    }.render_admin_ssr();
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Sessions,
        content,
        Some(admin),
        Some(branding),
    )
}
