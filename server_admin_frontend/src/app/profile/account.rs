#![allow(
    clippy::single_call_fn,
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
    admin: &server_admin_contract::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView + use<> {
    let login = admin.login().to_string();
    let display_name = admin.display_name().to_string();
    let roles = String::from(crate::shared::text::join_txt(
        admin.roles().iter().map(|name| name.as_ref().as_str()),
    ));
    let permissions = String::from(crate::shared::text::join_txt(
        admin
            .permissions()
            .iter()
            .map(|permission| permission.as_ref().as_str()),
    ));
    leptos::view! {
        <crate::ui::card::AdminCard variant=crate::ui::card::AdminCardVariant::Profile>
            <crate::ui::card::AdminCardHeader><crate::ui::card::AdminCardTitle class="profile-card-title">"Account"</crate::ui::card::AdminCardTitle></crate::ui::card::AdminCardHeader>
            <dl>
                <dt>"Login"</dt><dd>{login}</dd>
                <dt>"Display name"</dt><dd>{display_name}</dd>
                <dt>"Roles"</dt><dd>{roles}</dd>
                <dt>"Permissions"</dt><dd>{permissions}</dd>
            </dl>
        </crate::ui::card::AdminCard>
    }
}
