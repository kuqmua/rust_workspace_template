#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "document and shell Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

pub(crate) fn render_document(
    title: &crate::AdminSsrText,
    body: impl leptos::prelude::IntoAny,
) -> crate::AdminSsrHtml {
    let rendered_body = crate::render_view(body);
    crate::AdminSsrHtml::try_from(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css?v=20260801-37\"><link rel=\"stylesheet\" href=\"/admin/assets/rust-ui.css?v=20260801-38\"></head><body>{}</body></html>",
        String::from(rendered_body)
    ))
    .unwrap_or_else(crate::AdminSsrHtml::from)
}
