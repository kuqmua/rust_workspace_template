#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the administrator page shell requires its local set of document attribute traits"
)]

mod navigation;

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

pub(super) fn render(
    page: server_admin_contract::AdminPage,
    content: super::super::AdminSsrHtml,
) -> super::super::AdminSsrHtml {
    render_with_access(page, content, None, None)
}

pub(super) fn render_with_access(
    page: server_admin_contract::AdminPage,
    content: super::super::AdminSsrHtml,
    admin: Option<&server_admin_contract::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
) -> super::super::AdminSsrHtml {
    render_with_table_access(page, content, admin, branding, None)
}

pub(super) fn render_with_table_access(
    page: server_admin_contract::AdminPage,
    content: super::super::AdminSsrHtml,
    admin: Option<&server_admin_contract::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
    active_table: Option<server_admin_contract::AdminDataTable>,
) -> super::super::AdminSsrHtml {
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
    super::render_document(
        &super::super::AdminSsrText::try_from(document_title)
            .unwrap_or_else(super::super::AdminSsrText::from),
        leptos::view! {
            <div class="app-shell" style=primary_color>
                <header class="topbar">
                    {navigation::admin_nav(page, admin, active_table)}
                </header>
                <main class="main-content"><p id="saved" class="flash-success" role="status">"Changes saved."</p><div inner_html=String::from(content)></div></main>
            </div>
        },
    )
}
