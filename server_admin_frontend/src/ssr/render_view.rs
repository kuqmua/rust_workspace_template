use super::AdminSsrHtml;

pub(super) fn render_view(view: impl leptos::prelude::IntoAny) -> AdminSsrHtml {
    AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(
        leptos::prelude::IntoAny::into_any(view),
    ))
    .unwrap_or_else(AdminSsrHtml::from)
}
