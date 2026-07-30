#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the CSR user action controls are composed once by their user row"
)]

mod password;

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

#[allow(
    clippy::too_many_arguments,
    reason = "the action cell receives the typed row identity, state, signals, and permission gates it renders"
)]
pub(super) fn admin_user_buttons(
    user_id: server_admin_contract::AdminUserId,
    is_banned: server_admin_contract::AdminBool,
    expected_roles: &[server_admin_contract::AdminRoleId],
    login: super::LeptosAdminUserTextSignal,
    display_name: super::LeptosAdminUserTextSignal,
    password: super::LeptosAdminUserTextSignal,
    selected_roles: super::LeptosAdminRoleIdsSignal,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
    can_update_roles: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let expected_roles = expected_roles.to_vec();
    let can_delete = bool::from(can_delete);
    let can_update = bool::from(can_update);
    let can_update_roles = bool::from(can_update_roles);
    leptos::view! {
        <td data-label="actions"><div class="table-actions">
            {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                let request = super::super::edit::request(
                    server_admin_contract::AdminDisplayName::try_from(leptos::prelude::Get::get(&display_name.0)).ok(),
                    server_admin_contract::AdminLogin::try_from(leptos::prelude::Get::get(&login.0)).ok(),
                );
                super::super::actions::update(user_id, request);
            }>"Save"</button> })}
            {password::admin_user_password(user_id, password, server_admin_contract::AdminBool::from(can_update))}
            {can_update_roles.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                let expected = server_admin_contract::AdminRoleIds::try_from(expected_roles.clone());
                let selected = server_admin_contract::AdminRoleIds::try_from(leptos::prelude::Get::get(&selected_roles.0));
                if let (Ok(expected), Ok(selected)) = (expected, selected) {
                    super::super::actions::set_roles(user_id, expected, selected);
                }
            }>"Save roles"</button> })}
            {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                super::super::actions::set_ban(user_id, is_banned);
            }>{if bool::from(is_banned) { "Unban" } else { "Ban" }}</button> })}
            {can_delete.then(|| leptos::view! { <button class="danger-button" type="button" on:click=move |_event| {
                super::super::actions::delete(user_id);
            }>"Delete"</button> })}
        </div></td>
    }
}
