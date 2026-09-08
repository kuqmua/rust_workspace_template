#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, OnAttribute,
    StyleAttribute,
};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminProfileView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView {
    let admin_change_password = || {
        let current_password = leptos::prelude::RwSignal::new(String::new());
        let new_password = leptos::prelude::RwSignal::new(String::new());
        let password_validation_failed = leptos::prelude::RwSignal::new(false);
        let password_generation_failed = leptos::prelude::RwSignal::new(false);
        let password_visible = leptos::prelude::RwSignal::new(false);
        let generate_password = || {
            let window = web_sys::window()
                .ok_or(crate::admin_password_generation_error::AdminPasswordGenerationError::BrowserUnavailable)?;
            let randomness_error = |exception| {
                crate::admin_password_generation_error::AdminPasswordGenerationError::Randomness(
                    crate::wasm_bindgen_password_generation_exception::WasmBindgenPasswordGenerationException::from(exception),
                )
            };
            let crypto = window.crypto().map_err(randomness_error)?;
            let mut entropy = [0u8; 32];
            let _random_values = crypto
                .get_random_values_with_u8_array(&mut entropy)
                .map_err(randomness_error)?;
            server_admin_contract::admin_new_password::AdminNewPassword::try_from(
                server_admin_contract::admin_password_entropy::AdminPasswordEntropy::from(entropy),
            )
            .map_err(crate::admin_password_generation_error::AdminPasswordGenerationError::Policy)
        };
        leptos::view! {
                <form class="security-card" novalidate on:submit=move |event| {
                    event.prevent_default();
                    let request = (
                        server_admin_contract::admin_password::AdminPassword::try_from(leptos::prelude::Get::get(&current_password)),
                        server_admin_contract::admin_new_password::AdminNewPassword::try_from(leptos::prelude::Get::get(&new_password)),
                    );
                    if let (Ok(current), Ok(new_value), Ok(path)) = (
                        request.0,
                        request.1,
                        crate::admin_api_url::admin_api_url(
                            server_admin_contract::admin_route::AdminRoute::ChangeOwnPassword,
                        ),
                    ) {
                        leptos::prelude::Set::set(&password_validation_failed, false);
                        crate::reload_after::reload_after(
                            crate::admin_mutation_method::AdminMutationMethod::Post,
                            path,
                            server_admin_contract::admin_change_own_password_request::AdminChangeOwnPasswordRequest::new(current, new_value),
                        );
                    } else {
                        leptos::prelude::Set::set(&password_validation_failed, true);
                    }
                }>
                    <label class="ui-field flex flex-col gap-2" data-name="Label"><span>{constants_str::ADMIN_UI_CURRENT_PASSWORD}</span><crate::admin_input::AdminInput admin_input_name="current_password" admin_input_kind=crate::admin_input_kind::AdminInputKind::Password required=true bind_value=current_password /></label>
                    <label class="ui-field flex flex-col gap-2" data-name="Label"><span>{constants_str::ADMIN_UI_NEW_PASSWORD}{constants_str::SPACE}{'('}{constants_str::ADMIN_UI_CONTAIN_12_TO_1024_CHARACTERS_INCLUDING_UPPERCASE_LOWERCASE_DIGIT_AND_SPECIAL_CHARACTERS_WITH_NO_WHITESPACE}{')'}</span>
                        <span class="password-input">
                        <crate::admin_input::AdminInput admin_input_name="new_password" attr:r#type=move || if leptos::prelude::Get::get(&password_visible) { crate::admin_input_kind::AdminInputKind::Text.value() } else { crate::admin_input_kind::AdminInputKind::Password.value() } admin_input_kind=crate::admin_input_kind::AdminInputKind::Password minlength=server_admin_contract::identity::ADMIN_NEW_PASSWORD_MIN_CHARS maxlength=server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS required=true bind_value=new_password />
                            <button type="button" class="password-visibility" aria-label=move || if leptos::prelude::Get::get(&password_visible) { constants_str::ADMIN_HIDE_PASSWORD } else { constants_str::ADMIN_SHOW_PASSWORD } on:click=move |_| {
                                leptos::prelude::Update::update(&password_visible, |visible| *visible = !*visible);
                            }>
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
                                    <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12Z" />
                                    <circle cx="12" cy="12" r="3" />
                                    <path d="m3 3 18 18" style:display=move || if leptos::prelude::Get::get(&password_visible) { "block" } else { "none" } />
                                </svg>
                            </button>
                        </span>
                        <crate::admin_button::AdminButton admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button on_click=leptos::prelude::Callback::new(move |_| {
                            match generate_password() {
                                Ok(password) => {
                                    leptos::prelude::Set::set(&new_password, password.as_ref().to_owned());
                                    leptos::prelude::Set::set(&password_visible, false);
                                    leptos::prelude::Set::set(&password_generation_failed, false);
                                    leptos::prelude::Set::set(&password_validation_failed, false);
                                }
                                Err(_error) => leptos::prelude::Set::set(&password_generation_failed, true),
                            }
                        })>{constants_str::ADMIN_GENERATE_PASSWORD}</crate::admin_button::AdminButton>

                        {move || leptos::prelude::Get::get(&password_generation_failed).then(|| leptos::view! {
                            <singlestage::FieldError>{constants_str::ADMIN_UI_PASSWORD_GENERATION_FAILED}</singlestage::FieldError>
                        })}
                        {move || leptos::prelude::Get::get(&password_validation_failed).then(|| leptos::view! {
                            <singlestage::FieldError>{constants_str::ADMIN_UI_CHECK_BOTH_PASSWORDS_AND_ENSURE_THE_NEW_PASSWORD_SATISFIES_THE_POLICY}</singlestage::FieldError>
                        })}
                    </label>
                    <crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_CHANGE_PASSWORD}</crate::admin_button::AdminButton>
                </form>
        }
    };

    let admin_profile_account = || {
        let login = authenticated_admin.login().to_string();
        let display_name = authenticated_admin.display_name().to_string();
        let roles = String::from(crate::join_text::join_text(
            authenticated_admin
                .roles()
                .iter()
                .map(|name| name.as_ref().as_str()),
        ));
        let permissions = String::from(crate::join_text::join_text(
            authenticated_admin
                .permissions()
                .iter()
                .map(|permission| permission.as_ref().as_str()),
        ));
        leptos::view! {
                <dl class="profile-card">
                    <dt>{constants_str::ADMIN_UI_DISPLAY_NAME}</dt><dd>{display_name}</dd>
                    <dt>{constants_str::ADMIN_UI_LOGIN}</dt><dd>{login}</dd>
                    <dt>{constants_str::ADMIN_UI_ROLES}</dt><dd>{roles}</dd>
                    <dt>{constants_str::ADMIN_UI_PERMISSIONS}</dt><dd>{permissions}</dd>
                </dl>
        }
    };

    leptos::view! {
        <section class="profile-grid" data-renderer="csr">
            {admin_profile_account()}
            {admin_change_password()}
        </section>
    }
}
