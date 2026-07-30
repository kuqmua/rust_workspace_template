#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR role action forms are composed once by their role row"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

pub(super) fn admin_role_actions(
    item: &server_admin_contract::AdminRoleSummary,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let delete_allowed = bool::from(can_delete);
    let update_allowed = bool::from(can_update);
    leptos::view! {
        <td data-label="actions">
            {update_allowed.then(|| leptos::view! {
                <form method="post" action=server_admin_contract::AdminHtmlAction::RoleUpdate.get()><input type="hidden" name="role_id" value=item.id().to_string() /><input name="name" value=item.name().to_string() required /><button type="submit">"Save"</button></form>
            })}
            {delete_allowed.then(|| leptos::view! {
                <details><summary>"Delete"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::RoleDelete.get()><input type="hidden" name="role_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm permanent deletion"</label><button class="danger-button" type="submit" disabled=bool::from(item.is_system())>"Delete role"</button></form></details>
            })}
        </td>
    }
}
