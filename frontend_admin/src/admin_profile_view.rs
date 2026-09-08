#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminProfileView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView {
    let admin_profile_account = || {
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
                <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_DISPLAY_NAME><span>{display_name}</span></crate::admin_field::AdminField>
                <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_LOGIN><span>{login}</span></crate::admin_field::AdminField>
                <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_ROLES><span>{roles}</span></crate::admin_field::AdminField>
                <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_PERMISSIONS><span>{permissions}</span></crate::admin_field::AdminField>
        }
    };

    leptos::view! {
        <section class="profile-grid profile-fields" data-renderer="csr">
            {admin_profile_account()}
        </section>
    }
}
