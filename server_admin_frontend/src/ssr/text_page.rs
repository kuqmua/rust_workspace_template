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
pub(super) fn render_text_page(
    page: server_admin_contract::AdminPage,
    _title: super::AdminSsrText,
    text: super::AdminSsrText,
) -> super::AdminSsrHtml {
    let content = leptos::view! {
        <section><div class="code-card"><pre>{text.0}</pre></div></section>
    }
    .render_admin_ssr();
    super::render_admin_page(page, content)
}

#[must_use]
pub(super) fn render_text_page_with_access(
    page: server_admin_contract::AdminPage,
    _title: super::AdminSsrText,
    text: super::AdminSsrText,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let content = leptos::view! {
        <section><div class="code-card"><pre>{text.0}</pre></div></section>
    }
    .render_admin_ssr();
    super::render_admin_page_with_access(page, content, Some(admin), Some(branding))
}
