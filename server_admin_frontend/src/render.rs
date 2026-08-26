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
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
) -> super::super::AdminSsrHtml {
    let tab_title = branding
        .and_then(server_admin_contract::domain_types::AdminBrandingView::tab_title)
        .map_or_else(
            || String::from(constants_str::ADMINISTRATOR_SIGN_IN),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::domain_types::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    super::render_document::render_document(
        &super::super::AdminSsrText::try_from(tab_title)
            .unwrap_or_else(super::super::AdminSsrText::from),
        leptos::view! {
            <main class="auth-layout" style=primary_color>
                <crate::domain_types::with_owner::card::AdminCard variant=crate::domain_types::with_owner::card::AdminCardVariant::Auth>
                    {error.map(|message| leptos::view! { <crate::domain_types::with_owner::alert::AdminAlert>{message.to_string()}</crate::domain_types::with_owner::alert::AdminAlert> })}
                    <form method="post" action=server_admin_contract::domain_types::AdminHtmlAction::SignIn.get()>
                        <crate::domain_types::with_owner::field::AdminField label="Login"><crate::domain_types::with_owner::input::AdminInput name="login" autocomplete="username" required=true /></crate::domain_types::with_owner::field::AdminField>
                        <crate::domain_types::with_owner::field::AdminField label="Password"><crate::domain_types::with_owner::input::AdminInput name="password" kind=crate::domain_types::with_owner::input::AdminInputKind::Password autocomplete="current-password" required=true /></crate::domain_types::with_owner::field::AdminField>
                        <crate::domain_types::with_owner::button::AdminButton>"Sign in"</crate::domain_types::with_owner::button::AdminButton>
                    </form>
                </crate::domain_types::with_owner::card::AdminCard>
            </main>
        },
    )
}
