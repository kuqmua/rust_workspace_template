#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the CSR user field cells are composed once by their user row"
)]

use leptos::prelude::{CustomAttribute, ElementChild, OnAttribute};

pub(super) fn admin_user_fields(
    item: &server_admin_contract::AdminUserSummary,
    login: super::LeptosAdminUserTextSignal,
    display_name: super::LeptosAdminUserTextSignal,
    can_update: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView + use<> {
    let disabled = !bool::from(can_update);
    leptos::view! {
        <td data-label="id">{item.id().to_string()}</td>
        <td data-label="login"><input disabled=disabled value=item.login().to_string() on:input=move |event| leptos::prelude::Set::set(&login.0, leptos::prelude::event_target_value(&event)) /></td>
        <td data-label="display_name"><input disabled=disabled value=item.display_name().to_string() on:input=move |event| leptos::prelude::Set::set(&display_name.0, leptos::prelude::event_target_value(&event)) /></td>
        <td data-label="banned">{item.is_banned().to_string()}</td>
    }
}
