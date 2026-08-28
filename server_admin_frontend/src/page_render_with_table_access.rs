#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the administrator page shell requires its local set of document attribute traits"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

pub(super) fn page_render_with_table_access(
    page: server_admin_contract::domain_types::AdminPage,
    content: crate::AdminSsrHtml,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
) -> crate::AdminSsrHtml {
    let spec = page.spec();
    let title = spec.title();
    let document_title = branding
        .and_then(server_admin_contract::domain_types::AdminBrandingView::tab_title)
        .map_or_else(
            || title.as_ref().to_owned(),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::domain_types::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    super::render_document::render_document(
        &crate::AdminSsrText::try_from(document_title).unwrap_or_else(crate::AdminSsrText::from),
        leptos::view! {
            <div class="app-shell" style=primary_color>
                <header class="topbar">
                    {ssr_admin_nav::ssr_admin_nav(page, admin, active_table)}
                </header>
                <main class="main-content"><div class="page-frame"><crate::domain_types::with_owner::alert::AdminAlert variant=crate::domain_types::with_owner::alert::AdminAlertVariant::Success id="saved">"Changes saved."</crate::domain_types::with_owner::alert::AdminAlert><div inner_html=String::from(content)></div></div></main>
            </div>
        },
    )
}

// Root-owned module compatibility wrappers.
pub(crate) mod ssr_admin_nav {
    pub use crate::ssr_admin_nav::*;
}
