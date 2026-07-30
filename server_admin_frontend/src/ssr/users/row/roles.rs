#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR user-role form is composed once by its user row"
)]

use leptos::prelude::{CustomAttribute, ElementChild};

pub(super) fn admin_user_roles(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
    can_update_roles: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let expected_role_ids = item
        .role_ids()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(str_constants::COMMA_SPACE.trim());
    let update_allowed = bool::from(can_update_roles);
    leptos::view! {
        <td data-label="roles">{update_allowed.then(|| leptos::view! {
            <form method="post" action=server_admin_contract::AdminHtmlAction::UserRoles.get()>
                <input type="hidden" name="user_id" value=item.id().to_string() />
                <input type="hidden" name="expected_role_ids" value=expected_role_ids />
                {page.roles().iter().map(|role| {
                    let checked = item.role_ids().contains(&role.id());
                    let name = format!("role_{}", role.id());
                    leptos::view! { <label><input type="checkbox" name=name value=role.id().to_string() checked=checked />{role.name().to_string()}</label> }
                }).collect::<Vec<_>>()}
                <button type="submit">"Save roles"</button>
            </form>
        })}</td>
    }
}
