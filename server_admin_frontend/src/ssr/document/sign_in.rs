#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the sign-in Leptos view requires its local set of document attribute traits"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes, StyleAttribute,
};

pub(super) fn render(
    error: Option<super::super::AdminSsrErrorMessage>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
) -> super::super::AdminSsrHtml {
    let tab_title = branding
        .and_then(server_admin_contract::AdminBrandingView::tab_title)
        .map_or_else(
            || String::from(constants_str::ADMINISTRATOR_SIGN_IN),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    super::render_document(
        &super::super::AdminSsrText::try_from(tab_title)
            .unwrap_or_else(super::super::AdminSsrText::from),
        leptos::view! {
            <main class="auth-layout" style=primary_color>
                <crate::ui::card::AdminCard variant=crate::ui::card::AdminCardVariant::Auth>
                    {error.map(|message| leptos::view! { <crate::ui::alert::AdminAlert>{message.to_string()}</crate::ui::alert::AdminAlert> })}
                    <form method="post" action=server_admin_contract::AdminHtmlAction::SignIn.get()>
                        <crate::ui::field::AdminField label="Login"><crate::ui::input::AdminInput name="login" autocomplete="username" required=true /></crate::ui::field::AdminField>
                        <crate::ui::field::AdminField label="Password"><crate::ui::input::AdminInput name="password" kind=crate::ui::input::AdminInputKind::Password autocomplete="current-password" required=true /></crate::ui::field::AdminField>
                        <crate::ui::button::AdminButton>"Sign in"</crate::ui::button::AdminButton>
                    </form>
                </crate::ui::card::AdminCard>
            </main>
        },
    )
}
