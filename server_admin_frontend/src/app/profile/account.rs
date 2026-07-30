#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the account card is composed once by the profile screen"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

pub(super) fn admin_profile_account(
    admin: &server_admin_contract::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView + use<> {
    let login = admin.login().to_string();
    let display_name = admin.display_name().to_string();
    let roles = admin
        .roles()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(str_constants::COMMA_SPACE);
    let permissions = admin
        .permissions()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(str_constants::COMMA_SPACE);
    leptos::view! {
        <article class="profile-card">
            <h2>"Account"</h2>
            <dl>
                <dt>"Login"</dt><dd>{login}</dd>
                <dt>"Display name"</dt><dd>{display_name}</dd>
                <dt>"Roles"</dt><dd>{roles}</dd>
                <dt>"Permissions"</dt><dd>{permissions}</dd>
            </dl>
        </article>
    }
}
