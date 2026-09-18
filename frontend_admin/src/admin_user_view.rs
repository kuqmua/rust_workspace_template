#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the application shell"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it into the view"
)]
pub(crate) fn AdminUserView(
    admin_users_page: server_admin_contract::admin_users_page::AdminUsersPage,
) -> impl leptos::prelude::IntoView {
    let content = admin_users_page.items().first().map_or_else(
        || {
            leptos::prelude::IntoAny::into_any(leptos::view! {
                <crate::admin_alert::AdminAlert>{constants_str::RESOURCE_NOT_FOUND}</crate::admin_alert::AdminAlert>
            })
        },
        |admin_user_summary| {
            let roles = String::from(crate::join_text::join_text(
                admin_users_page
                    .roles()
                    .iter()
                    .filter(|role| admin_user_summary.role_ids().contains(&role.id()))
                    .map(server_admin_contract::admin_role_summary::AdminRoleSummary::name)
                    .map(|name| name.as_ref().as_str()),
            ));
            let id = admin_user_summary.id().to_string();
            let login = admin_user_summary.login().to_string();
            let display_name = admin_user_summary.display_name().to_string();
            let is_banned = admin_user_summary.is_banned().to_string();
            leptos::prelude::IntoAny::into_any(leptos::view! {
                <div class="health-label">{constants_str::SQL_NAMES_ID}</div>
                <div class="health-result">{id}</div>
                <div class="health-label">{constants_str::LOGIN}</div>
                <div class="health-result">{login}</div>
                <div class="health-label">{constants_str::DISPLAY_NAME}</div>
                <div class="health-result">{display_name}</div>
                <div class="health-label">{constants_str::IS_BANNED}</div>
                <div class="health-result">{is_banned}</div>
                <div class="health-label">{constants_str::ROLES}</div>
                <div class="health-result">{roles}</div>
            })
        },
    );
    leptos::view! {
        <section class="profile-grid profile-fields" data-renderer="csr" data-page="user-read">
            {content}
        </section>
    }
}
