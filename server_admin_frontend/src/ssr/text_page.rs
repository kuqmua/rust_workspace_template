#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub(super) fn render_text_page(
    page: server_admin_contract::AdminPage,
    _title: super::AdminSsrText,
    text: super::AdminSsrText,
) -> super::AdminSsrHtml {
    let content_view = leptos::view! {
        <section><crate::ui::card::AdminCard variant=crate::ui::card::AdminCardVariant::Code><singlestage::ScrollArea attr:data-name="CodeScrollArea" class="max-h-[70vh] overflow-auto"><pre>{text.0}</pre></singlestage::ScrollArea></crate::ui::card::AdminCard></section>
    };
    let content = super::render_view(content_view);
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
    let content_view = leptos::view! {
        <section><crate::ui::card::AdminCard variant=crate::ui::card::AdminCardVariant::Code><singlestage::ScrollArea attr:data-name="CodeScrollArea" class="max-h-[70vh] overflow-auto"><pre>{text.0}</pre></singlestage::ScrollArea></crate::ui::card::AdminCard></section>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(page, content, Some(admin), Some(branding))
}
