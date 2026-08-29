#![allow(
    unused_imports,
    reason = "document and shell Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

pub(crate) fn render_document(
    title: &crate::admin_ssr_text::AdminSsrText,
    body: impl leptos::prelude::IntoAny,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let rendered_body = crate::render_view::render_view(body);
    crate::admin_ssr_html::AdminSsrHtml::try_from(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css?v=20260801-37\"><link rel=\"stylesheet\" href=\"/admin/assets/rust-ui.css?v=20260801-38\"></head><body>{}</body></html>",
        String::from(rendered_body)
    ))
    .unwrap_or_else(crate::admin_ssr_html::AdminSsrHtml::from)
}
