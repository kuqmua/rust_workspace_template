#![allow(
    unused_imports,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos row rendering requires grouped attribute traits and event-local values replace domain inputs"
)]

mod buttons;
mod fields;
mod roles;

use leptos::prelude::{CustomAttribute, ElementChild};

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct LeptosAdminUserTextSignal(leptos::prelude::RwSignal<String>);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct LeptosAdminRoleIdsSignal(
    leptos::prelude::RwSignal<Vec<server_admin_contract::AdminRoleId>>,
);

pub(super) fn admin_user_row(
    item: &server_admin_contract::AdminUserSummary,
    page: &server_admin_contract::AdminUsersPage,
    can_delete: server_admin_contract::AdminBool,
    can_update: server_admin_contract::AdminBool,
    can_update_roles: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let login =
        LeptosAdminUserTextSignal::from(leptos::prelude::RwSignal::new(item.login().to_string()));
    let display_name = LeptosAdminUserTextSignal::from(leptos::prelude::RwSignal::new(
        item.display_name().to_string(),
    ));
    let password = LeptosAdminUserTextSignal::from(leptos::prelude::RwSignal::new(String::new()));
    let selected_roles =
        LeptosAdminRoleIdsSignal::from(leptos::prelude::RwSignal::new(item.role_ids().to_vec()));
    leptos::view! {
        <tr>
            {fields::admin_user_fields(item, login, display_name, can_update)}
            {roles::admin_user_roles(item, page, selected_roles, can_update_roles)}
            {buttons::admin_user_buttons(
                item.id(),
                item.is_banned(),
                item.role_ids(),
                login,
                display_name,
                password,
                selected_roles,
                can_delete,
                can_update,
                can_update_roles,
            )}
        </tr>
    }
}
