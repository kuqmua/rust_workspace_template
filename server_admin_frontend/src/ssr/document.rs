#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "document and shell Leptos view branches require different attribute traits after macro expansion"
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

pub(super) fn render_document(
    title: &super::AdminSsrText,
    body: impl leptos::prelude::IntoAny,
) -> super::AdminSsrHtml {
    let rendered_body = body.render_admin_ssr();
    super::AdminSsrHtml::try_from(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css?v=20260729-35\"></head><body>{}</body></html>",
        String::from(rendered_body)
    ))
    .unwrap_or_else(super::AdminSsrHtml::from)
}

#[must_use]
pub(super) fn render_sign_in(
    error: Option<super::AdminSsrErrorMessage>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
) -> super::AdminSsrHtml {
    let tab_title = branding
        .and_then(server_admin_contract::AdminBrandingView::tab_title)
        .map_or_else(
            || String::from(str_constants::ADMINISTRATOR_SIGN_IN),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    render_document(
        &super::AdminSsrText::try_from(tab_title).unwrap_or_else(super::AdminSsrText::from),
        leptos::view! {
            <main class="auth-layout" style=primary_color>
                <section class="auth-card">
                    {error.map(|message| leptos::view! { <p class="field-error" role="alert">{message.to_string()}</p> })}
                    <p class="password-policy">{str_constants::ADMIN_PASSWORD_POLICY_DESCRIPTION}</p>
                    <form method="post" action=server_admin_contract::AdminHtmlAction::SignIn.get()>
                        <label><span>"Login"</span><input name="login" autocomplete="username" required /></label>
                        <label><span>"Password"</span><input name="password" type="password" autocomplete="current-password" required /></label>
                        <button type="submit">"Sign in"</button>
                    </form>
                </section>
            </main>
        },
    )
}

#[must_use]
pub(super) fn render_admin_page(
    page: server_admin_contract::AdminPage,
    content: super::AdminSsrHtml,
) -> super::AdminSsrHtml {
    render_admin_page_with_access(page, content, None, None)
}

pub(super) fn render_admin_page_with_access(
    page: server_admin_contract::AdminPage,
    content: super::AdminSsrHtml,
    admin: Option<&server_admin_contract::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
) -> super::AdminSsrHtml {
    render_admin_page_with_table_access(page, content, admin, branding, None)
}

pub(super) fn render_admin_page_with_table_access(
    page: server_admin_contract::AdminPage,
    content: super::AdminSsrHtml,
    admin: Option<&server_admin_contract::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
    active_table: Option<server_admin_contract::AdminDataTable>,
) -> super::AdminSsrHtml {
    let spec = page.spec();
    let title = spec.title();
    let document_title = branding
        .and_then(server_admin_contract::AdminBrandingView::tab_title)
        .map_or_else(
            || title.as_ref().to_owned(),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    render_document(
        &super::AdminSsrText::try_from(document_title).unwrap_or_else(super::AdminSsrText::from),
        leptos::view! {
            <div class="app-shell" style=primary_color>
                <header class="topbar">
                    <nav aria-label="Admin sections">
                        {server_admin_contract::AdminDataTable::PG_ORDER.into_iter().filter(|table| admin.is_none_or(|value| bool::from(value.has_permission(server_admin_contract::AdminPermission::TablesRead)) && bool::from(value.has_permission(table.permission())))).map(|table| {
                            let name = table.to_string();
                            let href = table.frontend_path().to_string();
                            leptos::view! {
                                <a class=(active_table == Some(table)).then_some("active") href=href>{name}</a>
                            }
                        }).collect::<Vec<_>>()}
                        {server_admin_contract::AdminPage::navigation().filter(|item_page| admin.is_none_or(|value| bool::from(value.can_access(*item_page)))).map(|item_page| {
                            let item = item_page.spec();
                            let href = String::from(item.path());
                            let label = item.route_name().as_ref().to_owned();
                            leptos::view! {
                                <a class=(item_page == page).then_some("active") href=href>{label}</a>
                            }
                        }).collect::<Vec<_>>()}
                        <form method="post" action=server_admin_contract::AdminHtmlAction::SignOut.get()><button type="submit">{server_admin_contract::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</button></form>
                    </nav>
                </header>
                <main class="main-content"><p id="saved" class="flash-success" role="status">"Changes saved."</p><div inner_html=String::from(content)></div></main>
            </div>
        },
    )
}
