#![allow(
    clippy::unused_trait_names,
    reason = "the account card is composed once by the profile screen"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{ElementChild};

pub(super) fn admin_profile_account(
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView + use<> {
    let login = authenticated_admin.login().to_string();
    let display_name = authenticated_admin.display_name().to_string();
    let roles = String::from(crate::join_text::join_text(
        authenticated_admin
            .roles()
            .iter()
            .map(|name| name.as_ref().as_str()),
    ));
    let permissions = String::from(crate::join_text::join_text(
        authenticated_admin
            .permissions()
            .iter()
            .map(|permission| permission.as_ref().as_str()),
    ));
    leptos::view! {
        <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Profile>
            <crate::admin_card_header::AdminCardHeader><crate::admin_card_title::AdminCardTitle option="profile-card-title">"Account"</crate::admin_card_title::AdminCardTitle></crate::admin_card_header::AdminCardHeader>
            <dl>
                <dt>"Login"</dt><dd>{login}</dd>
                <dt>"Display name"</dt><dd>{display_name}</dd>
                <dt>"Roles"</dt><dd>{roles}</dd>
                <dt>"Permissions"</dt><dd>{permissions}</dd>
            </dl>
        </crate::admin_card::AdminCard>
    }
}
