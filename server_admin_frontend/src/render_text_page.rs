#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub fn render_text_page(
    page: server_admin_contract::domain_types::AdminPage,
    _title: crate::AdminSsrText,
    text: crate::AdminSsrText,
) -> crate::AdminSsrHtml {
    let content_view = leptos::view! {
        <section><crate::domain_types::with_owner::card::AdminCard variant=crate::domain_types::with_owner::card::AdminCardVariant::Code><singlestage::ScrollArea attr:data-name="CodeScrollArea" class="max-h-[70vh] overflow-auto"><pre>{text.0}</pre></singlestage::ScrollArea></crate::domain_types::with_owner::card::AdminCard></section>
    };
    let content = crate::render_view(content_view);
    crate::render_admin_page(page, content)
}
