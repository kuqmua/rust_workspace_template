#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "document and shell Leptos view branches require different attribute traits after macro expansion"
)]

#[path = "page.rs"]
mod page;
#[path = "render.rs"]
mod render;

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

pub(super) fn render_document(
    title: &super::AdminSsrText,
    body: impl leptos::prelude::IntoAny,
) -> super::AdminSsrHtml {
    let rendered_body = super::render_view(body);
    super::AdminSsrHtml::try_from(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css?v=20260801-37\"><link rel=\"stylesheet\" href=\"/admin/assets/rust-ui.css?v=20260801-38\"></head><body>{}</body></html>",
        String::from(rendered_body)
    ))
    .unwrap_or_else(super::AdminSsrHtml::from)
}

#[must_use]
pub(super) fn render_sign_in(
    error: Option<super::AdminSsrErrorMessage>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
) -> super::AdminSsrHtml {
    render::render(error, branding)
}

#[must_use]
pub(super) fn render_admin_page(
    page: server_admin_contract::domain_types::AdminPage,
    content: super::AdminSsrHtml,
) -> super::AdminSsrHtml {
    page::render(page, content)
}

pub(super) fn render_admin_page_with_access(
    page: server_admin_contract::domain_types::AdminPage,
    content: super::AdminSsrHtml,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
) -> super::AdminSsrHtml {
    page::render_with_access(page, content, admin, branding)
}

pub(super) fn render_admin_page_with_table_access(
    page: server_admin_contract::domain_types::AdminPage,
    content: super::AdminSsrHtml,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
) -> super::AdminSsrHtml {
    page::render_with_table_access(page, content, admin, branding, active_table)
}
