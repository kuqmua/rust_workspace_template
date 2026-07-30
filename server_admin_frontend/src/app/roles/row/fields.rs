#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the CSR role field cells are composed once by their role row"
)]

use leptos::prelude::{CustomAttribute, ElementChild, OnAttribute};

pub(super) fn admin_role_fields(
    item: &server_admin_contract::AdminRoleSummary,
    name: super::LeptosAdminRoleNameSignal,
    can_update: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let disabled = !bool::from(can_update);
    leptos::view! {
        <td data-label="id">{item.id().to_string()}</td>
        <td data-label="name"><input disabled=disabled value=item.name().to_string() on:input=move |event| leptos::prelude::Set::set(&name.0, leptos::prelude::event_target_value(&event)) /></td>
        <td data-label="system">{item.is_system().to_string()}</td>
    }
}
