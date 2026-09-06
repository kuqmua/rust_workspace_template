#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the sign-in Leptos view requires its local set of document attribute traits"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes, StyleAttribute,
};

#[must_use]
pub fn render_sign_in(
    error: Option<crate::admin_ssr_error_message::AdminSsrErrorMessage>,
    branding: Option<&server_admin_contract::admin_branding_view::AdminBrandingView>,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let tab_title = branding
        .and_then(server_admin_contract::admin_branding_view::AdminBrandingView::tab_title)
        .map_or_else(
            || String::from(constants_str::ADMINISTRATOR_SIGN_IN),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::admin_branding_view::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    crate::render_document::render_document(
        &crate::admin_ssr_text::AdminSsrText::try_from(tab_title)
            .unwrap_or_else(crate::admin_ssr_text::AdminSsrText::from),
        leptos::view! {
            <main class="auth-layout" style=primary_color>
                <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Auth>
                    {error.map(|message| leptos::view! { <crate::admin_alert::AdminAlert>{message.to_string()}</crate::admin_alert::AdminAlert> })}
                    <form method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::SignIn.get()>
                        <crate::admin_field::AdminField admin_field_label="Login"><crate::admin_input::AdminInput admin_input_name="login" autocomplete="username" required=true /></crate::admin_field::AdminField>
                        <crate::admin_field::AdminField admin_field_label="Password"><crate::admin_input::AdminInput admin_input_name="password" admin_input_kind=crate::admin_input_kind::AdminInputKind::Password autocomplete="current-password" required=true /></crate::admin_field::AdminField>
                        <crate::admin_button::AdminButton>{constants_str::SIGN_IN}</crate::admin_button::AdminButton>
                    </form>
                </crate::admin_card::AdminCard>
            </main>
        },
    )
}
