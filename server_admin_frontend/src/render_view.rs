pub(super) fn render_view(
    view: impl leptos::prelude::IntoAny,
) -> crate::admin_ssr_html::AdminSsrHtml {
    crate::admin_ssr_html::AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(
        leptos::prelude::IntoAny::into_any(view),
    ))
    .unwrap_or_else(crate::admin_ssr_html::AdminSsrHtml::from)
}
