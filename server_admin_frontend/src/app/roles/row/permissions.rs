#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the CSR role-permission editor is composed once by its role row"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

pub(super) fn admin_role_permissions(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
    selected_permissions: super::LeptosAdminPermissionIdsSignal,
    can_update_permissions: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let permission_ids = item.permission_ids().to_vec();
    let disabled = !bool::from(can_update_permissions);
    leptos::view! {
        <td data-label="permissions"><div class="table-options">{page.permissions().iter().map(|permission| {
            let permission_id = permission.id();
            let checked = permission_ids.contains(&permission_id);
            leptos::view! { <label><input type="checkbox" checked=checked disabled=disabled on:change=move |event| {
                leptos::prelude::Update::update(&selected_permissions.0, |ids| {
                    if leptos::prelude::event_target_checked(&event) {
                        if !ids.contains(&permission_id) { ids.push(permission_id); }
                    } else { ids.retain(|value| *value != permission_id); }
                });
            } />{permission.name().to_string()}</label> }
        }).collect::<Vec<_>>()}</div></td>
    }
}
