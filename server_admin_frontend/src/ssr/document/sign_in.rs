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
            || String::from(str_constants::ADMINISTRATOR_SIGN_IN),
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
                <section class="auth-card">
                    {error.map(|message| leptos::view! { <p class="field-error" role="alert">{message.to_string()}</p> })}
                    <p class="password-policy">{str_constants::ADMIN_PASSWORD_POLICY_DESCRIPTION}</p>
                    <form method="post" action=server_admin_contract::AdminHtmlAction::SignIn.get()>
                        <label><span>"Login"</span><input name="login" autocomplete="username" required /></label>
                        <label><span>"Password"</span><input name="password" type="password" autocomplete="current-password" required /></label>
                        <button type="submit">"Sign in"</button>
                    </form>
                </section>
            </main>
        },
    )
}
