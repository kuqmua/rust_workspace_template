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
    page: server_admin_contract::admin_page::AdminPage,
    _title: crate::admin_ssr_text::AdminSsrText,
    text: crate::admin_ssr_text::AdminSsrText,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let content_view = leptos::view! {
        <section><crate::admin_card::AdminCard variant=crate::admin_card_variant::AdminCardVariant::Code><singlestage::ScrollArea attr:data-name="CodeScrollArea" class="max-h-[70vh] overflow-auto"><pre>{text.0}</pre></singlestage::ScrollArea></crate::admin_card::AdminCard></section>
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page::render_admin_page(page, content)
}
