#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the CSR password control is composed once by its user action cell"
)]

use leptos::prelude::{ElementChild, OnAttribute};

pub(super) fn admin_user_password(
    user_id: server_admin_contract::AdminUserId,
    password: super::super::LeptosAdminUserTextSignal,
    can_update: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView {
    bool::from(can_update).then(|| leptos::view! {
        <>
            <input type="password" placeholder="New password" on:input=move |event| leptos::prelude::Set::set(&password.0, leptos::prelude::event_target_value(&event)) />
            <button type="button" on:click=move |_event| {
                if let Ok(value) = server_admin_contract::AdminNewPassword::try_from(leptos::prelude::Get::get(&password.0)) {
                    super::super::super::actions::set_password(user_id, value);
                }
            }>"Change password"</button>
        </>
    })
}
