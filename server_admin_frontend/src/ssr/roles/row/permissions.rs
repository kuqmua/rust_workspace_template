#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR role-permission form is composed once by its role row"
)]

use leptos::prelude::{CustomAttribute, ElementChild};

pub(super) fn admin_role_permissions(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
    can_update_permissions: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let expected_permission_ids = item
        .permission_ids()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(str_constants::COMMA_SPACE.trim());
    let update_allowed = bool::from(can_update_permissions);
    leptos::view! {
        <td data-label="permissions">{update_allowed.then(|| leptos::view! {
            <form method="post" action=server_admin_contract::AdminHtmlAction::RolePermissions.get()>
                <input type="hidden" name="role_id" value=item.id().to_string() />
                <input type="hidden" name="expected_permission_ids" value=expected_permission_ids />
                {page.permissions().iter().map(|permission| {
                    let checked = item.permission_ids().contains(&permission.id());
                    let name = format!("permission_{}", permission.id());
                    leptos::view! { <label><input type="checkbox" name=name value=permission.id().to_string() checked=checked />{permission.name().to_string()}</label> }
                }).collect::<Vec<_>>()}
                <button type="submit">"Save permissions"</button>
            </form>
        })}</td>
    }
}
