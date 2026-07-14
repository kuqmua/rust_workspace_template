#![allow(
    unreachable_pub,
    reason = "Leptos component expansion and prelude visibility are confined by the private parent module"
)]
pub(super) fn permission_ids(value: &str) -> Vec<server_admin_contract::AdminPermissionId> {
    value
        .split(',')
        .filter_map(|item| item.trim().parse::<i64>().ok())
        .map(server_admin_contract::AdminPermissionId::from)
        .collect()
}
pub(super) fn role_ids(value: &str) -> Vec<server_admin_contract::AdminRoleId> {
    value
        .split(',')
        .filter_map(|item| item.trim().parse::<i64>().ok())
        .map(server_admin_contract::AdminRoleId::from)
        .collect()
}
#[cfg(test)]
mod tests {
    #[test]
    fn identifier_lists_ignore_invalid_and_empty_items() {
        assert_eq!(
            super::role_ids("1, invalid, , 2"),
            vec![
                server_admin_contract::AdminRoleId::from(1i64),
                server_admin_contract::AdminRoleId::from(2i64)
            ]
        );
        assert_eq!(
            super::permission_ids("3, invalid, , 4"),
            vec![
                server_admin_contract::AdminPermissionId::from(3i64),
                server_admin_contract::AdminPermissionId::from(4i64)
            ]
        );
    }
}
pub use leptos::prelude::*;
#[component]
pub(super) fn SignIn(client: super::AdminApiClient) -> impl IntoView {
    let login = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<super::Text>::None);
    let pending = RwSignal::new(false);
    view! {
        <main class="auth-page"><section class="auth-card"><div class="auth-brand"><span class="brand-mark">"A"</span><div><strong>"Admin Console"</strong><small>"Workspace control center"</small></div></div><div class="auth-copy"><p class="eyebrow">"Secure access"</p><h1>"Welcome back"</h1><p>"Sign in to manage users, roles and system settings."</p></div>
        <form class="auth-form" on:submit=move |event| {
            event.prevent_default();
            let client = client.clone();
            pending.set(true);
            error.set(None);
            leptos::task::spawn_local(async move {
                let input = match (server_admin_contract::AdminLogin::try_from(login.get()), server_admin_contract::AdminPassword::try_from(password.get())) {
                    (Ok(login), Ok(password)) => server_admin_contract::AdminSignInReq::new(login, password),
                    (Err(value), _) => { pending.set(false); error.set(Some(super::Text::from(value.to_string()))); return; },
                    (_, Err(value)) => { pending.set(false); error.set(Some(super::Text::from(value.to_string()))); return; },
                };
                match client.sign_in(input).await {
                    Ok(value) => { let _display_name = value.user().display_name(); super::redirect(server_admin_contract::admin_page_paths::ROOT); },
                    Err(value) => { pending.set(false); error.set(Some(super::Text::from(value.to_string()))); },
                }
            });
        }>
        <label><span>"Login"</span><input placeholder="Enter your login" autocomplete="username" prop:value=move || login.get() on:input=move |event| login.set(event_target_value(&event)) /></label>
        <label><span>"Password"</span><input type="password" placeholder="Enter your password" autocomplete="current-password" prop:value=move || password.get() on:input=move |event| password.set(event_target_value(&event)) /></label>
        <button class="primary-button" type="submit" disabled=move || pending.get()>{move || if pending.get() { "Signing in..." } else { "Sign in" }}</button>
        </form>
        {move || error.get().map(|value| view! { <div class="alert error" role="alert"><strong>"Unable to sign in"</strong><span>{value.to_string()}</span></div> })}
        <p class="auth-footnote">"Protected administrator area"</p></section><div class="auth-visual"><div class="visual-orb"></div><div class="visual-card"><span>"SYSTEM STATUS"</span><strong>"All controls in one place"</strong><p>"Fast, typed and secure administration powered by Rust."</p></div></div></main>
    }
}
pub(super) fn settings_view(
    value: server_admin_contract::AdminSettingsView,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let site_name = RwSignal::new(value.site_name().as_ref().to_owned());
    let default_admin_route = RwSignal::new(value.default_admin_route().as_ref().to_owned());
    let tab_title = RwSignal::new(
        value
            .tab_title()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let main_logo = RwSignal::new(
        value
            .main_logo()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let primary_color = RwSignal::new(
        value
            .primary_color()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let organization_name = RwSignal::new(
        value
            .organization_name()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let organization_contacts = RwSignal::new(
        value
            .organization_contacts()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let support_url = RwSignal::new(
        value
            .support_url()
            .map_or_else(String::new, |item| item.as_ref().to_owned()),
    );
    let can_update =
        super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::UpdateSettings);
    view! { <section><p class="eyebrow">"Configuration"</p><h1>"System settings"</h1><form class="settings-form" on:submit=move |event| { event.prevent_default(); if let (Ok(site_name), Ok(default_admin_route), Ok(tab_title), Ok(main_logo), Ok(primary_color), Ok(organization_name), Ok(organization_contacts), Ok(support_url)) = (server_admin_contract::AdminSettingText::try_from(site_name.get()), server_admin_contract::AdminSettingText::try_from(default_admin_route.get()), server_admin_contract::AdminSettingText::try_from(tab_title.get()), server_admin_contract::AdminSettingText::try_from(main_logo.get()), server_admin_contract::AdminSettingText::try_from(primary_color.get()), server_admin_contract::AdminSettingText::try_from(organization_name.get()), server_admin_contract::AdminSettingText::try_from(organization_contacts.get()), server_admin_contract::AdminSettingText::try_from(support_url.get())) { let body = server_admin_contract::AdminUpdateSettingsReq::new(Some(default_admin_route), Some(main_logo), Some(organization_contacts), Some(organization_name), Some(primary_color), Some(site_name), Some(support_url), Some(tab_title)); let action_client = client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateSettings, body), action_client, loader); } }>
    <label><span>"Site name"</span><input placeholder="Administration" prop:value=move || site_name.get() on:input=move |event| site_name.set(event_target_value(&event)) /></label>
    <label><span>"Browser tab title"</span><input placeholder="Admin Console" prop:value=move || tab_title.get() on:input=move |event| tab_title.set(event_target_value(&event)) /></label>
    <label><span>"Default admin route"</span><input placeholder=server_admin_contract::admin_page_paths::USERS prop:value=move || default_admin_route.get() on:input=move |event| default_admin_route.set(event_target_value(&event)) /></label>
    <label><span>"Primary color"</span><input placeholder="#6757e8" prop:value=move || primary_color.get() on:input=move |event| primary_color.set(event_target_value(&event)) /></label>
    <label class="full-field"><span>"Logo URL"</span><input placeholder="https://..." prop:value=move || main_logo.get() on:input=move |event| main_logo.set(event_target_value(&event)) /></label>
    <label><span>"Organization"</span><input placeholder="Organization name" prop:value=move || organization_name.get() on:input=move |event| organization_name.set(event_target_value(&event)) /></label>
    <label><span>"Contacts"</span><input placeholder="support@example.com" prop:value=move || organization_contacts.get() on:input=move |event| organization_contacts.set(event_target_value(&event)) /></label>
    <label class="full-field"><span>"Support URL"</span><input placeholder="https://support.example.com" prop:value=move || support_url.get() on:input=move |event| support_url.set(event_target_value(&event)) /></label>
    <button type="submit" disabled=!can_update>"Save changes"</button></form></section> }
}
