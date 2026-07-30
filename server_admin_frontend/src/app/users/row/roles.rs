#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the CSR user-role editor is composed once by its user row"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

pub(super) fn admin_user_roles(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
    selected_roles: super::LeptosAdminRoleIdsSignal,
    can_update_roles: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let role_ids = item.role_ids().to_vec();
    let disabled = !bool::from(can_update_roles);
    leptos::view! {
        <td data-label="roles"><div class="table-options">{page.roles().iter().map(|role| {
            let role_id = role.id();
            let checked = role_ids.contains(&role_id);
            leptos::view! { <label><input type="checkbox" checked=checked disabled=disabled on:change=move |event| {
                leptos::prelude::Update::update(&selected_roles.0, |ids| {
                    if leptos::prelude::event_target_checked(&event) {
                        if !ids.contains(&role_id) { ids.push(role_id); }
                    } else { ids.retain(|value| *value != role_id); }
                });
            } />{role.name().to_string()}</label> }
        }).collect::<Vec<_>>()}</div></td>
    }
}
