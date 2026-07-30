use leptos::prelude::{
    ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes, OnAttribute,
};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::app) fn AdminProfileView(
    admin: server_admin_contract::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView {
    let current_password = leptos::prelude::RwSignal::new(String::new());
    let new_password = leptos::prelude::RwSignal::new(String::new());
    let password_validation_failed = leptos::prelude::RwSignal::new(false);
    leptos::view! {
        <section class="profile-grid" data-renderer="csr">
            <article class="profile-card"><h2>"Account"</h2><dl>
                <dt>"Login"</dt><dd>{admin.login().to_string()}</dd>
                <dt>"Display name"</dt><dd>{admin.display_name().to_string()}</dd>
                <dt>"Roles"</dt><dd>{admin.roles().iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")}</dd>
                <dt>"Permissions"</dt><dd>{admin.permissions().iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")}</dd>
            </dl></article>
            <article class="security-card"><h2>"Change password"</h2><form novalidate on:submit=move |event| {
                event.prevent_default();
                let request = (
                    server_admin_contract::AdminPassword::try_from(leptos::prelude::Get::get(&current_password)),
                    server_admin_contract::AdminNewPassword::try_from(leptos::prelude::Get::get(&new_password)),
                );
                if let (Ok(current), Ok(new_value), Ok(path)) = (
                    request.0,
                    request.1,
                    super::http::admin_api_url(server_admin_contract::AdminRoute::ChangeOwnPassword),
                ) {
                    leptos::prelude::Set::set(&password_validation_failed, false);
                    super::mutation::reload_after(
                        super::mutation::AdminMutationMethod::Post,
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
            </form></article>
        </section>
    }
}
