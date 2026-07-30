#![allow(
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos row rendering requires named attribute traits and event-local values replace domain inputs"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

pub(super) fn admin_role_row(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
    can_update_permissions: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let name = leptos::prelude::RwSignal::new(item.name().to_string());
    let selected_permissions = leptos::prelude::RwSignal::new(item.permission_ids().to_vec());
    let expected_permissions = item.permission_ids().to_vec();
    let update_role_id = item.id();
    let permissions_role_id = item.id();
    let delete_role_id = item.id();
    let item_id = item.id().to_string();
    let item_name = item.name().to_string();
    let is_system = item.is_system();
    let permission_ids = item.permission_ids().to_vec();
    let can_delete = bool::from(can_delete);
    let can_update = bool::from(can_update);
    let can_update_permissions = bool::from(can_update_permissions);
    leptos::view! {
        <tr>
            <td data-label="id">{item_id}</td>
            <td data-label="name"><input disabled=!can_update value=item_name on:input=move |event| leptos::prelude::Set::set(&name, leptos::prelude::event_target_value(&event)) /></td>
            <td data-label="system">{is_system.to_string()}</td>
            <td data-label="permissions"><div class="table-options">{page.permissions().iter().map(|permission| {
                let permission_id = permission.id();
                let checked = permission_ids.contains(&permission_id);
                leptos::view! { <label><input type="checkbox" checked=checked disabled=!can_update_permissions on:change=move |event| {
                    leptos::prelude::Update::update(&selected_permissions, |ids| {
                        if leptos::prelude::event_target_checked(&event) {
                            if !ids.contains(&permission_id) { ids.push(permission_id); }
                        } else { ids.retain(|value| *value != permission_id); }
                    });
                } />{permission.name().to_string()}</label> }
            }).collect::<Vec<_>>()}</div></td>
            <td data-label="actions"><div class="table-actions">
                {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                    if let Ok(value) = server_admin_contract::AdminRoleName::try_from(leptos::prelude::Get::get(&name)) {
                        super::actions::update(update_role_id, super::edit::request(value));
                    }
                }>"Save"</button> })}
                {can_update_permissions.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                    let expected = server_admin_contract::AdminPermissionIds::try_from(expected_permissions.clone());
                    let selected = server_admin_contract::AdminPermissionIds::try_from(leptos::prelude::Get::get(&selected_permissions));
                    if let (Ok(expected), Ok(selected)) = (expected, selected) {
                        super::actions::set_permissions(permissions_role_id, expected, selected);
                    }
                }>"Save permissions"</button> })}
                {can_delete.then(|| leptos::view! { <button class="danger-button" type="button" disabled=bool::from(is_system) on:click=move |_event| {
                    super::actions::delete(delete_role_id);
                }>"Delete"</button> })}
            </div></td>
        </tr>
    }
}
