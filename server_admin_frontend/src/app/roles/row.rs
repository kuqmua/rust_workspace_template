#![allow(
    unused_imports,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos row rendering requires grouped attribute traits and event-local values replace domain inputs"
)]

mod buttons;
mod fields;
mod permissions;

use leptos::prelude::{CustomAttribute, ElementChild};

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct LeptosAdminRoleNameSignal(leptos::prelude::RwSignal<String>);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct LeptosAdminPermissionIdsSignal(
    leptos::prelude::RwSignal<Vec<server_admin_contract::AdminPermissionId>>,
);

pub(super) fn admin_role_row(
    item: &server_admin_contract::AdminRoleSummary,
    page: &server_admin_contract::AdminRolesPage,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
    can_update_permissions: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let name =
        LeptosAdminRoleNameSignal::from(leptos::prelude::RwSignal::new(item.name().to_string()));
    let selected_permissions = LeptosAdminPermissionIdsSignal::from(
        leptos::prelude::RwSignal::new(item.permission_ids().to_vec()),
    );
    leptos::view! {
        <tr>
            {fields::admin_role_fields(item, name, can_update)}
            {permissions::admin_role_permissions(item, page, selected_permissions, can_update_permissions)}
            {buttons::admin_role_buttons(
                item.id(),
                item.is_system(),
                item.permission_ids(),
                name,
                selected_permissions,
                can_delete,
                can_update,
                can_update_permissions,
            )}
        </tr>
    }
}
