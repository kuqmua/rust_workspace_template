#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the CSR role action controls are composed once by their role row"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

#[allow(
    clippy::too_many_arguments,
    reason = "the action cell receives the typed row identity, state, signals, and permission gates it renders"
)]
pub(super) fn admin_role_buttons(
    role_id: server_admin_contract::AdminRoleId,
    is_system: server_admin_contract::AdminBool,
    expected_permissions: &[server_admin_contract::AdminPermissionId],
    name: super::LeptosAdminRoleNameSignal,
    selected_permissions: super::LeptosAdminPermissionIdsSignal,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
    can_update_permissions: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let expected_permissions = expected_permissions.to_vec();
    let can_delete = bool::from(can_delete);
    let can_update = bool::from(can_update);
    let can_update_permissions = bool::from(can_update_permissions);
    leptos::view! {
        <td data-label="actions"><div class="table-actions">
            {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                if let Ok(value) = server_admin_contract::AdminRoleName::try_from(leptos::prelude::Get::get(&name.0)) {
                    super::super::actions::update(role_id, super::super::edit::request(value));
                }
            }>"Save"</button> })}
            {can_update_permissions.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                let expected = server_admin_contract::AdminPermissionIds::try_from(expected_permissions.clone());
                let selected = server_admin_contract::AdminPermissionIds::try_from(leptos::prelude::Get::get(&selected_permissions.0));
                if let (Ok(expected), Ok(selected)) = (expected, selected) {
                    super::super::actions::set_permissions(role_id, expected, selected);
                }
            }>"Save permissions"</button> })}
            {can_delete.then(|| leptos::view! { <button class="danger-button" type="button" disabled=bool::from(is_system) on:click=move |_event| {
                super::super::actions::delete(role_id);
            }>"Delete"</button> })}
        </div></td>
    }
}
