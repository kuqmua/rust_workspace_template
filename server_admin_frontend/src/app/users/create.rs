use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent users module"
)]
pub(super) fn AdminCreateUser(
    can_create: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView {
    let create_display_name = leptos::prelude::RwSignal::new(String::new());
    let create_login = leptos::prelude::RwSignal::new(String::new());
    let create_password = leptos::prelude::RwSignal::new(String::new());
    bool::from(can_create).then(|| leptos::view! {
        <form class="mutation-form" on:submit=move |event| {
            event.prevent_default();
            let request = (
                server_admin_contract::AdminDisplayName::try_from(leptos::prelude::Get::get(&create_display_name)),
                server_admin_contract::AdminLogin::try_from(leptos::prelude::Get::get(&create_login)),
                server_admin_contract::AdminNewPassword::try_from(leptos::prelude::Get::get(&create_password)),
            );
            if let (Ok(display_name), Ok(login), Ok(password), Ok(path)) = (
                request.0,
                request.1,
                request.2,
                super::super::http::url::admin_api_url(
                    server_admin_contract::AdminRoute::CreateUser,
                ),
            ) {
                super::super::mutation::reload_after(
                    super::super::mutation::AdminMutationMethod::Post,
                    path,
                    server_admin_contract::AdminCreateUserReq::new(display_name, login, password),
                );
            }
        }>
            <input placeholder="Login" required on:input=move |event| leptos::prelude::Set::set(&create_login, leptos::prelude::event_target_value(&event)) />
            <input placeholder="Display name" required on:input=move |event| leptos::prelude::Set::set(&create_display_name, leptos::prelude::event_target_value(&event)) />
            <input type="password" placeholder="Password" required on:input=move |event| leptos::prelude::Set::set(&create_password, leptos::prelude::event_target_value(&event)) />
            <button type="submit">"Create user"</button>
        </form>
    })
}
