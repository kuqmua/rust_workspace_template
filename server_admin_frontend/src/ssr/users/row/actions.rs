#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR user action forms are composed once by their user row"
)]

use leptos::prelude::{CustomAttribute, ElementChild};

pub(super) fn admin_user_actions(
    item: &server_admin_contract::AdminUserSummary,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let is_banned = bool::from(item.is_banned());
    let delete_allowed = bool::from(can_delete);
    let update_allowed = bool::from(can_update);
    leptos::view! {
        <td data-label="actions">
            {update_allowed.then(|| leptos::view! {
                <details><summary>"Edit"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserUpdate.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="login" value=item.login().to_string() required /><input name="display_name" value=item.display_name().to_string() required /><button type="submit">"Save"</button></form></details>
                <details><summary>"Password"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserPassword.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="password" type="password" required /><button type="submit">"Change password"</button></form></details>
                <form method="post" action=server_admin_contract::AdminHtmlAction::UserBan.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input type="hidden" name="is_banned" value=(!is_banned).to_string() /><button type="submit">{if is_banned { "Unban" } else { "Ban" }}</button></form>
            })}
            {delete_allowed.then(|| leptos::view! {
                <details><summary>"Delete"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserDelete.get()><input type="hidden" name="user_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm permanent deletion"</label><button class="danger-button" type="submit">"Delete user"</button></form></details>
            })}
        </td>
    }
}
