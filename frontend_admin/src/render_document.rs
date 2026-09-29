pub(crate) fn render_document(
    admin_ssr_text: &crate::admin_ssr_text::AdminSsrText,
    body: impl leptos::prelude::IntoAny,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let rendered_body = crate::render_view::render_view(body);
    let rendered_title = crate::render_view::render_view(admin_ssr_text.as_ref().to_owned());
    crate::admin_ssr_html::AdminSsrHtml::try_from(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{}</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css?v=20260917-63\"><link rel=\"stylesheet\" href=\"/admin/assets/rust-ui.css?v=20260906-38\"></head><body>{}</body></html>",
        String::from(rendered_title),
        String::from(rendered_body)
    ))
    .unwrap_or_else(crate::admin_ssr_html::AdminSsrHtml::from)
}
