#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view requires attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub(in crate::domain_types::ssr) fn render_text_page_with_access(
    page: server_admin_contract::domain_types::AdminPage,
    _title: super::super::AdminSsrText,
    text: super::super::AdminSsrText,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::super::AdminSsrHtml {
    let content_view = leptos::view! {
        <section><crate::domain_types::with_owner::card::AdminCard variant=crate::domain_types::with_owner::card::AdminCardVariant::Code><singlestage::ScrollArea attr:data-name="CodeScrollArea" class="max-h-[70vh] overflow-auto"><pre>{text.0}</pre></singlestage::ScrollArea></crate::domain_types::with_owner::card::AdminCard></section>
    };
    let content = super::super::render_view(content_view);
    super::super::render_admin_page_with_access(page, content, Some(admin), Some(branding))
}
