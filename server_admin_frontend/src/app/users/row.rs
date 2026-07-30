#![allow(
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos row rendering requires named attribute traits and event-local values replace domain inputs"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

pub(super) fn admin_user_row(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
    can_update_roles: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let login = leptos::prelude::RwSignal::new(item.login().to_string());
    let display_name = leptos::prelude::RwSignal::new(item.display_name().to_string());
    let password = leptos::prelude::RwSignal::new(String::new());
    let selected_roles = leptos::prelude::RwSignal::new(item.role_ids().to_vec());
    let expected_roles = item.role_ids().to_vec();
    let update_user_id = item.id();
    let password_user_id = item.id();
    let roles_user_id = item.id();
    let ban_user_id = item.id();
    let delete_user_id = item.id();
    let is_banned = item.is_banned();
    let item_id = item.id().to_string();
    let item_login = item.login().to_string();
    let item_display_name = item.display_name().to_string();
    let role_ids = item.role_ids().to_vec();
    let can_delete = bool::from(can_delete);
    let can_update = bool::from(can_update);
    let can_update_roles = bool::from(can_update_roles);
    leptos::view! {
        <tr>
            <td data-label="id">{item_id}</td>
            <td data-label="login"><input disabled=!can_update value=item_login on:input=move |event| leptos::prelude::Set::set(&login, leptos::prelude::event_target_value(&event)) /></td>
            <td data-label="display_name"><input disabled=!can_update value=item_display_name on:input=move |event| leptos::prelude::Set::set(&display_name, leptos::prelude::event_target_value(&event)) /></td>
            <td data-label="banned">{is_banned.to_string()}</td>
            <td data-label="roles"><div class="table-options">{page.roles().iter().map(|role| {
                let role_id = role.id();
                let checked = role_ids.contains(&role_id);
                leptos::view! { <label><input type="checkbox" checked=checked disabled=!can_update_roles on:change=move |event| {
                    leptos::prelude::Update::update(&selected_roles, |ids| {
                        if leptos::prelude::event_target_checked(&event) {
                            if !ids.contains(&role_id) { ids.push(role_id); }
                        } else { ids.retain(|value| *value != role_id); }
                    });
                } />{role.name().to_string()}</label> }
            }).collect::<Vec<_>>()}</div></td>
            <td data-label="actions"><div class="table-actions">
                {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                    let request = super::edit::request(
                        server_admin_contract::AdminDisplayName::try_from(leptos::prelude::Get::get(&display_name)).ok(),
                        server_admin_contract::AdminLogin::try_from(leptos::prelude::Get::get(&login)).ok(),
                    );
                    super::actions::update(update_user_id, request);
                }>"Save"</button> })}
                {can_update.then(|| leptos::view! { <><input type="password" placeholder="New password" on:input=move |event| leptos::prelude::Set::set(&password, leptos::prelude::event_target_value(&event)) />
                <button type="button" on:click=move |_event| {
                    if let Ok(value) = server_admin_contract::AdminNewPassword::try_from(leptos::prelude::Get::get(&password)) {
                        super::actions::set_password(password_user_id, value);
                    }
                }>"Change password"</button></> })}
                {can_update_roles.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                    let expected = server_admin_contract::AdminRoleIds::try_from(expected_roles.clone());
                    let selected = server_admin_contract::AdminRoleIds::try_from(leptos::prelude::Get::get(&selected_roles));
                    if let (Ok(expected), Ok(selected)) = (expected, selected) {
                        super::actions::set_roles(roles_user_id, expected, selected);
                    }
                }>"Save roles"</button> })}
                {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                    super::actions::set_ban(ban_user_id, is_banned);
                }>{if bool::from(is_banned) { "Unban" } else { "Ban" }}</button> })}
                {can_delete.then(|| leptos::view! { <button class="danger-button" type="button" on:click=move |_event| {
                    super::actions::delete(delete_user_id);
                }>"Delete"</button> })}
            </div></td>
        </tr>
    }
}
