#![allow(
    unreachable_pub,
    reason = "Leptos component expansion and prelude visibility are confined by the private parent module"
)]
pub use leptos::prelude::*;
fn mfa_proof_from_input(value: String) -> Result<server_admin_contract::AdminMfaProof, String> {
    if value.len() == 6usize {
        server_admin_contract::AdminMfaCode::try_from(value)
            .map(server_admin_contract::AdminMfaProof::Totp)
            .map_err(|error| error.to_string())
    } else {
        server_admin_contract::AdminRecoveryCode::try_from(value)
            .map(server_admin_contract::AdminMfaProof::Recovery)
            .map_err(|error| error.to_string())
    }
}
fn mfa_view(
    status: server_admin_contract::AdminMfaStatus,
    client: super::AdminApiClient,
    loader: super::PageLoader,
) -> impl IntoView {
    let password = RwSignal::new(String::new());
    let code = RwSignal::new(String::new());
    let enrollment = RwSignal::<Option<server_admin_contract::AdminMfaEnrollRes>>::new(None);
    let recovery_codes = RwSignal::<Vec<server_admin_contract::AdminRecoveryCode>>::new(Vec::new());
    let error = RwSignal::<Option<String>>::new(None);
    let pending = RwSignal::new(false);
    let enroll_client = client.clone();
    let confirm_client = client.clone();
    let step_up_client = client.clone();
    let disable_client = client;
    view! { <section class="security-card"><h2>"Multi-factor authentication"</h2><p>{format!("Status: {} - unused recovery codes: {}", if bool::from(status.enabled()) { "enabled" } else { "not enabled" }, status.recovery_codes_remaining())}</p><label><span>"Current password for MFA changes"</span><input type="password" aria-label="MFA current password" prop:value=move || password.get() on:input=move |event| password.set(event_target_value(&event)) /></label><button type="button" disabled=move || pending.get() on:click=move |_| { match server_admin_contract::AdminPassword::try_from(password.get_untracked()) { Ok(value) => { pending.set(true); error.set(None); let client = enroll_client.clone(); leptos::task::spawn_local(async move { let request = server_admin_contract::AdminMfaEnrollReq::new(value); match client.send_json_response::<_, server_admin_contract::AdminMfaEnrollRes>(server_admin_contract::AdminRoute::MfaEnroll, request).await { Ok(value) => enrollment.set(Some(value)), Err(value) => error.set(Some(value.to_string())), } pending.set(false); }); }, Err(value) => error.set(Some(value.to_string())), } }>"Start new TOTP enrollment"</button>{move || enrollment.get().map(|value| view! { <div class="enrollment-secret" role="status"><p>"Add this secret to an authenticator supporting TOTP SHA-256:"</p><code>{value.secret().as_ref().to_owned()}</code><details><summary>"Enrollment URI"</summary><code>{value.uri().as_ref().to_owned()}</code></details></div> })}<label><span>"TOTP or recovery code"</span><input aria-label="MFA proof" autocomplete="one-time-code" prop:value=move || code.get() on:input=move |event| code.set(event_target_value(&event)) /></label><div class="form-actions"><button type="button" disabled=move || pending.get() || enrollment.get().is_none() on:click=move |_| { match server_admin_contract::AdminMfaCode::try_from(code.get_untracked()) { Ok(value) => { pending.set(true); error.set(None); let client = confirm_client.clone(); leptos::task::spawn_local(async move { let request = server_admin_contract::AdminMfaConfirmReq::new(value); match client.send_json_response::<_, server_admin_contract::AdminMfaConfirmRes>(server_admin_contract::AdminRoute::MfaConfirm, request).await { Ok(value) => { recovery_codes.set(value.recovery_codes().to_vec()); enrollment.set(None); }, Err(value) => error.set(Some(value.to_string())), } pending.set(false); }); }, Err(value) => error.set(Some(value.to_string())), } }>"Confirm enrollment"</button><button type="button" disabled=move || pending.get() || !bool::from(status.enabled()) on:click=move |_| { match (server_admin_contract::AdminPassword::try_from(password.get_untracked()), mfa_proof_from_input(code.get_untracked())) { (Ok(password), Ok(proof)) => { pending.set(true); error.set(None); let client = step_up_client.clone(); leptos::task::spawn_local(async move { let request = server_admin_contract::AdminMfaStepUpReq::new(password, proof); match client.clone().send_json(server_admin_contract::AdminRoute::MfaStepUp, request).await { Ok(()) => loader.set_notice(super::state::Text::try_from("Step-up authentication valid".to_owned()).unwrap_or_default()), Err(value) => error.set(Some(value.to_string())), } pending.set(false); }); }, (Err(value), _) => error.set(Some(value.to_string())), (_, Err(value)) => error.set(Some(value)), } }>"Verify step-up"</button><button type="button" class="danger-button" disabled=move || pending.get() || !bool::from(status.enabled()) on:click=move |_| { match (server_admin_contract::AdminPassword::try_from(password.get_untracked()), mfa_proof_from_input(code.get_untracked())) { (Ok(password), Ok(proof)) => { pending.set(true); error.set(None); let client = disable_client.clone(); leptos::task::spawn_local(async move { let request = server_admin_contract::AdminMfaDisableReq::new(password, proof); match client.clone().send_json(server_admin_contract::AdminRoute::MfaDisable, request).await { Ok(()) => { loader.set_notice(super::state::Text::try_from("MFA disabled".to_owned()).unwrap_or_default()); super::load(client, loader); }, Err(value) => error.set(Some(value.to_string())), } pending.set(false); }); }, (Err(value), _) => error.set(Some(value.to_string())), (_, Err(value)) => error.set(Some(value)), } }>"Disable MFA"</button></div>{move || { let values = recovery_codes.get(); (!values.is_empty()).then(|| view! { <div class="recovery-codes" role="alert"><strong>"Save these one-time recovery codes now. They will not be shown again."</strong><ul>{values.into_iter().map(|value| view! { <li><code>{value.as_ref().to_owned()}</code></li> }).collect_view()}</ul></div> }) }}{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}</section> }
}
pub(super) fn profile_view(
    mfa_status: server_admin_contract::AdminMfaStatus,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let mfa = mfa_view(mfa_status, client.clone(), loader);
    let current_password = RwSignal::new(String::new());
    let new_password = RwSignal::new(String::new());
    let revoke_other_sessions = RwSignal::new(true);
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    let login = auth
        .as_ref()
        .map(|value| value.login().to_string())
        .unwrap_or_default();
    let display_name = auth
        .as_ref()
        .map(|value| value.display_name().to_string())
        .unwrap_or_default();
    let roles = auth
        .as_ref()
        .map(|value| {
            value
                .roles()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(str_constants::TEXT_ALT_6)
        })
        .unwrap_or_default();
    let audit_href = format!(
        "{}?user_login={}",
        server_admin_contract::AdminFrontendPath::Audit.get(),
        crate::table_state::percent_encode(&login),
    );
    view! { <section><div class="page-heading"><div><p class="eyebrow">"Account security"</p><h1>"Profile"</h1></div></div><dl class="profile-details"><dt>"Login"</dt><dd>{login}</dd><dt>"Display name"</dt><dd>{display_name}</dd><dt>"Roles"</dt><dd>{roles}</dd></dl><nav class="security-links" aria-label="Account security links"><a href=server_admin_contract::AdminFrontendPath::Sessions.get()>"Manage sessions"</a><a href=audit_href>"Recent security events"</a></nav>{mfa}<form class="settings-form" on:submit=move |event| { event.prevent_default(); match (server_admin_contract::AdminPassword::try_from(current_password.get_untracked()), server_admin_contract::AdminNewPassword::try_from(new_password.get_untracked())) { (Ok(current), Ok(new)) => { pending.set(true); error.set(None); let body = server_admin_contract::AdminChangeOwnPasswordReq::new(current, new, server_admin_contract::AdminBool::from(revoke_other_sessions.get_untracked())); let action_client = client.clone(); leptos::task::spawn_local(async move { match action_client.clone().send_json(server_admin_contract::AdminRoute::ChangeOwnPassword, body).await { Ok(()) => { loader.set_notice(super::state::Text::try_from("Password changed".to_owned()).unwrap_or_default()); super::load(action_client, loader); }, Err(value) => error.set(Some(value.to_string())), } pending.set(false); }); }, _ => error.set(Some("Check current password and new password policy".to_owned())), } }><h2>"Change password"</h2><label><span>"Current password"</span><input type="password" aria-label="Current password" prop:value=move || current_password.get() on:input=move |event| current_password.set(event_target_value(&event)) /></label><label><span>"New password"</span><input type="password" aria-label="Profile new password" prop:value=move || new_password.get() on:input=move |event| new_password.set(event_target_value(&event)) /></label><label class="checkbox-field"><input type="checkbox" prop:checked=move || revoke_other_sessions.get() on:change=move |event| revoke_other_sessions.set(event_target_checked(&event)) /><span>"Revoke other sessions and all refresh tokens"</span></label>{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button type="submit" disabled=move || pending.get()>{move || if pending.get() { "Changing..." } else { "Change password" }}</button></form></section> }
}
#[component]
pub(super) fn SignIn(
    client: super::AdminApiClient,
    branding: Option<server_admin_contract::AdminBrandingView>,
) -> impl IntoView {
    let login = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let mfa_proof = RwSignal::new(String::new());
    let error = RwSignal::new(Option::<super::Text>::None);
    let pending = RwSignal::new(false);
    let site_name = branding.as_ref().map_or_else(
        || str_constants::ADMIN_CONSOLE.to_owned(),
        |value| value.site_name().as_ref().to_owned(),
    );
    let logo = branding.and_then(|value| value.main_logo().cloned());
    view! {
        <main class="auth-page"><section class="auth-card"><div class="auth-brand">{logo.map_or_else(|| view! { <span class="brand-mark">"A"</span> }.into_any(), |value| view! { <img class="brand-logo" src=value.as_ref().to_owned() alt="" /> }.into_any())}<div><strong>{site_name}</strong><small>"Workspace control center"</small></div></div><div class="auth-copy"><p class="eyebrow">"Secure access"</p><h1>"Welcome back"</h1><p>"Sign in to manage users, roles and system settings."</p></div>
        <form class="auth-form" on:submit=move |event| {
            event.prevent_default();
            let client = client.clone();
            pending.set(true);
            error.set(None);
            leptos::task::spawn_local(async move {
                let input = match (server_admin_contract::AdminLogin::try_from(login.get()), server_admin_contract::AdminPassword::try_from(password.get())) {
                    (Ok(login), Ok(password)) => { let proof_value = mfa_proof.get(); if proof_value.is_empty() { server_admin_contract::AdminSignInReq::new(login, password) } else if proof_value.len() == 6usize { match server_admin_contract::AdminMfaCode::try_from(proof_value) { Ok(code) => server_admin_contract::AdminSignInReq::with_mfa(login, password, server_admin_contract::AdminMfaProof::Totp(code)), Err(value) => { pending.set(false); error.set(Some(super::Text::try_from(value.to_string()).unwrap_or_default())); return; } } } else { match server_admin_contract::AdminRecoveryCode::try_from(proof_value) { Ok(code) => server_admin_contract::AdminSignInReq::with_mfa(login, password, server_admin_contract::AdminMfaProof::Recovery(code)), Err(value) => { pending.set(false); error.set(Some(super::Text::try_from(value.to_string()).unwrap_or_default())); return; } } } },
                    (Err(value), _) => { pending.set(false); error.set(Some(super::Text::try_from(value.to_string()).unwrap_or_default())); return; },
                    (_, Err(value)) => { pending.set(false); error.set(Some(super::Text::try_from(value.to_string()).unwrap_or_default())); return; },
                };
                match client.sign_in(input).await {
                    Ok(value) => { let _display_name = value.user().display_name(); super::redirect(server_admin_contract::AdminFrontendPath::Root.get()); },
                    Err(value) => { pending.set(false); error.set(Some(super::Text::try_from(value.to_string()).unwrap_or_default())); },
                }
            });
        }>
        <label><span>"Login"</span><input placeholder="Enter your login" autocomplete="username" prop:value=move || login.get() on:input=move |event| login.set(event_target_value(&event)) /></label>
        <label><span>"Password"</span><input type="password" placeholder="Enter your password" autocomplete="current-password" prop:value=move || password.get() on:input=move |event| password.set(event_target_value(&event)) /></label>
        <label><span>"MFA or recovery code (if enabled)"</span><input inputmode="numeric" autocomplete="one-time-code" placeholder="123456 or xxxx-xxxx-xxxx-xxxx" prop:value=move || mfa_proof.get() on:input=move |event| mfa_proof.set(event_target_value(&event)) /></label>
        <button class="primary-button" type="submit" disabled=move || pending.get()>{move || if pending.get() { "Signing in..." } else { "Sign in" }}</button>
        </form>
        {move || error.get().map(|value| view! { <div class="alert error" role="alert"><strong>"Unable to sign in"</strong><span>{value.to_string()}</span></div> })}
        <p class="auth-footnote">"Protected administrator area"</p></section><div class="auth-visual"><div class="visual-orb"></div><div class="visual-card"><span>"SYSTEM STATUS"</span><strong>"All controls in one place"</strong><p>"Fast, typed and secure administration powered by Rust."</p></div></div></main>
    }
}
fn optional_setting<Value, Error>(value: String) -> Result<Option<Value>, Error>
where
    Value: TryFrom<String, Error = Error>,
{
    if value.trim().is_empty() {
        Ok(None)
    } else {
        Value::try_from(value).map(Some)
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
    let initial_site_name = site_name.get_untracked();
    let initial_default_admin_route = default_admin_route.get_untracked();
    let initial_tab_title = tab_title.get_untracked();
    let initial_main_logo = main_logo.get_untracked();
    let initial_primary_color = primary_color.get_untracked();
    let initial_organization_name = organization_name.get_untracked();
    let initial_organization_contacts = organization_contacts.get_untracked();
    let initial_support_url = support_url.get_untracked();
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    view! { <section><p class="eyebrow">"Configuration"</p><h1>"System settings"</h1><div class="branding-preview" style=move || format!("--preview-primary: {}", primary_color.get())><span class="brand-mark">"A"</span><div><strong>{move || site_name.get()}</strong><small>{move || tab_title.get()}</small></div></div><form class="settings-form" on:submit=move |event| { event.prevent_default(); let parsed = (server_admin_contract::AdminSiteName::try_from(site_name.get_untracked()), server_admin_contract::AdminDefaultRoute::try_from(default_admin_route.get_untracked()), optional_setting::<server_admin_contract::AdminTabTitle, _>(tab_title.get_untracked()), optional_setting::<server_admin_contract::AdminMainLogo, _>(main_logo.get_untracked()), optional_setting::<server_admin_contract::AdminPrimaryColor, _>(primary_color.get_untracked()), optional_setting::<server_admin_contract::AdminOrganizationName, _>(organization_name.get_untracked()), optional_setting::<server_admin_contract::AdminOrganizationContacts, _>(organization_contacts.get_untracked()), optional_setting::<server_admin_contract::AdminSupportUrl, _>(support_url.get_untracked())); match parsed { (Ok(site_name), Ok(default_admin_route), Ok(tab_title), Ok(main_logo), Ok(primary_color), Ok(organization_name), Ok(organization_contacts), Ok(support_url)) => { error.set(None); pending.set(true); let mut clear = Vec::new(); if tab_title.is_none() { clear.push(server_admin_contract::AdminOptionalSetting::TabTitle); } if main_logo.is_none() { clear.push(server_admin_contract::AdminOptionalSetting::MainLogo); } if primary_color.is_none() { clear.push(server_admin_contract::AdminOptionalSetting::PrimaryColor); } if organization_name.is_none() { clear.push(server_admin_contract::AdminOptionalSetting::OrganizationName); } if organization_contacts.is_none() { clear.push(server_admin_contract::AdminOptionalSetting::OrganizationContacts); } if support_url.is_none() { clear.push(server_admin_contract::AdminOptionalSetting::SupportUrl); } let body = server_admin_contract::AdminUpdateSettingsReq::new(Some(default_admin_route), main_logo, organization_contacts, organization_name, primary_color, Some(site_name), support_url, tab_title, clear); let action_client = client.clone(); leptos::task::spawn_local(async move { match action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateSettings, body).await { Ok(()) => { loader.set_notice(super::state::Text::try_from("Settings saved".to_owned()).unwrap_or_default()); super::load(action_client, loader); }, Err(value) => error.set(Some(value.to_string())), } pending.set(false); }); }, _ => error.set(Some("Use an existing admin page, #RRGGBB color, and HTTPS URLs".to_owned())), } }>
    <label><span>"Site name"</span><input placeholder="Administration" prop:value=move || site_name.get() on:input=move |event| site_name.set(event_target_value(&event)) /></label>
    <label><span>"Browser tab title"</span><input placeholder="Admin Console" prop:value=move || tab_title.get() on:input=move |event| tab_title.set(event_target_value(&event)) /></label>
    <label><span>"Default admin route"</span><input placeholder=server_admin_contract::AdminFrontendPath::Users.get() prop:value=move || default_admin_route.get() on:input=move |event| default_admin_route.set(event_target_value(&event)) /></label>
    <label><span>"Primary color"</span><input placeholder="#6757e8" prop:value=move || primary_color.get() on:input=move |event| primary_color.set(event_target_value(&event)) /></label>
    <label class="full-field"><span>"Logo URL"</span><input placeholder="https://..." prop:value=move || main_logo.get() on:input=move |event| main_logo.set(event_target_value(&event)) /></label>
    <label><span>"Organization"</span><input placeholder="Organization name" prop:value=move || organization_name.get() on:input=move |event| organization_name.set(event_target_value(&event)) /></label>
    <label><span>"Contacts"</span><input placeholder="support@example.com" prop:value=move || organization_contacts.get() on:input=move |event| organization_contacts.set(event_target_value(&event)) /></label>
    <label class="full-field"><span>"Support URL"</span><input placeholder="https://support.example.com" prop:value=move || support_url.get() on:input=move |event| support_url.set(event_target_value(&event)) /></label>
    {move || error.get().map(|value| view! { <p class="field-error full-field" role="alert">{value}</p> })}<div class="form-actions full-field"><button type="button" class="secondary-button" on:click=move |_| { site_name.set(initial_site_name.clone()); default_admin_route.set(initial_default_admin_route.clone()); tab_title.set(initial_tab_title.clone()); main_logo.set(initial_main_logo.clone()); primary_color.set(initial_primary_color.clone()); organization_name.set(initial_organization_name.clone()); organization_contacts.set(initial_organization_contacts.clone()); support_url.set(initial_support_url.clone()); error.set(None); }>"Reset unsaved changes"</button><button type="button" class="secondary-button" disabled=move || !can_update || pending.get() on:click=move |_| { site_name.set("Admin Console".to_owned()); default_admin_route.set(server_admin_contract::AdminFrontendPath::Dashboard.get().to_owned()); tab_title.set(String::new()); main_logo.set(String::new()); primary_color.set(String::new()); organization_name.set(String::new()); organization_contacts.set(String::new()); support_url.set(String::new()); error.set(None); }>"Restore defaults"</button><button type="submit" disabled=move || !can_update || pending.get()>{move || if pending.get() { "Saving..." } else { "Save changes" }}</button></div></form></section> }
}
