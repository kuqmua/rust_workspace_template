#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the change-password card is composed once by the profile screen"
)]

use leptos::prelude::{ClassAttribute, ElementChild, GlobalAttributes, OnAttribute};

pub(super) fn admin_change_password() -> impl leptos::prelude::IntoView {
    let current_password = leptos::prelude::RwSignal::new(String::new());
    let new_password = leptos::prelude::RwSignal::new(String::new());
    let password_validation_failed = leptos::prelude::RwSignal::new(false);
    leptos::view! {
        <article class="security-card">
            <h2>"Change password"</h2>
            <form novalidate on:submit=move |event| {
                event.prevent_default();
                let request = (
                    server_admin_contract::AdminPassword::try_from(leptos::prelude::Get::get(&current_password)),
                    server_admin_contract::AdminNewPassword::try_from(leptos::prelude::Get::get(&new_password)),
                );
                if let (Ok(current), Ok(new_value), Ok(path)) = (
                    request.0,
                    request.1,
                    crate::app::http::url::admin_api_url(
                        server_admin_contract::AdminRoute::ChangeOwnPassword,
                    ),
                ) {
                    leptos::prelude::Set::set(&password_validation_failed, false);
                    crate::app::mutation::reload_after(
                        crate::app::mutation::AdminMutationMethod::Post,
                        path,
                        server_admin_contract::AdminChangeOwnPasswordReq::new(current, new_value),
                    );
                } else {
                    leptos::prelude::Set::set(&password_validation_failed, true);
                }
            }>
                <p class="password-policy">{str_constants::ADMIN_PASSWORD_POLICY_DESCRIPTION}</p>
                {move || leptos::prelude::Get::get(&password_validation_failed).then(|| leptos::view! {
                    <p class="field-error" role="alert">"Check both passwords and ensure the new password satisfies the policy."</p>
                })}
                <label><span>"Current password"</span><input type="password" required on:input=move |event| leptos::prelude::Set::set(&current_password, leptos::prelude::event_target_value(&event)) /></label>
                <label><span>"New password"</span><input type="password" minlength=server_admin_contract::ADMIN_NEW_PASSWORD_MIN_CHARS maxlength=server_admin_contract::ADMIN_PASSWORD_MAX_CHARS required on:input=move |event| leptos::prelude::Set::set(&new_password, leptos::prelude::event_target_value(&event)) /></label>
                <button type="submit">"Change password"</button>
            </form>
        </article>
    }
}
