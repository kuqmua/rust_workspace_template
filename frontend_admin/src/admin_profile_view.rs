#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

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
            <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Security>
                <crate::admin_card_header::AdminCardHeader><crate::admin_card_title::AdminCardTitle>{constants_str::ADMIN_UI_CHANGE_PASSWORD}</crate::admin_card_title::AdminCardTitle></crate::admin_card_header::AdminCardHeader>
                <form novalidate on:submit=move |event| {
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
                    <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_CURRENT_PASSWORD><crate::admin_input::AdminInput admin_input_name="current_password" admin_input_kind=crate::admin_input_kind::AdminInputKind::Password required=true bind_value=current_password /></crate::admin_field::AdminField>
                    <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_NEW_PASSWORD>
                        <crate::admin_input::AdminInput admin_input_name="new_password" admin_input_kind=crate::admin_input_kind::AdminInputKind::Password minlength=server_admin_contract::identity::ADMIN_NEW_PASSWORD_MIN_CHARS maxlength=server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS required=true bind_value=new_password />
                        <singlestage::FieldDescription attr:class="password-policy">{constants_str::ADMIN_UI_NEW_PASSWORDS_MUST_CONTAIN_12_TO_1024_CHARACTERS_INCLUDING_UPPERCASE_LOWERCASE_DIGIT_AND_SPECIAL_CHARACTERS_WITH_NO_WHITESPACE}</singlestage::FieldDescription>
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
                        <crate::admin_button::AdminButton admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button on_click=leptos::prelude::Callback::new(move |_| {
                            leptos::prelude::Update::update(&password_visible, |visible| *visible = !*visible);
                        })>{move || if leptos::prelude::Get::get(&password_visible) { constants_str::ADMIN_HIDE_PASSWORD } else { constants_str::ADMIN_SHOW_PASSWORD }}</crate::admin_button::AdminButton>
                        {move || leptos::prelude::Get::get(&password_visible).then(|| leptos::view! {
                            <p><code>{move || leptos::prelude::Get::get(&new_password)}</code></p>
                        })}
                        {move || leptos::prelude::Get::get(&password_generation_failed).then(|| leptos::view! {
                            <singlestage::FieldError>{constants_str::ADMIN_UI_PASSWORD_GENERATION_FAILED}</singlestage::FieldError>
                        })}
                        {move || leptos::prelude::Get::get(&password_validation_failed).then(|| leptos::view! {
                            <singlestage::FieldError>{constants_str::ADMIN_UI_CHECK_BOTH_PASSWORDS_AND_ENSURE_THE_NEW_PASSWORD_SATISFIES_THE_POLICY}</singlestage::FieldError>
                        })}
                    </crate::admin_field::AdminField>
                    <crate::admin_card_footer::AdminCardFooter><crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_CHANGE_PASSWORD}</crate::admin_button::AdminButton></crate::admin_card_footer::AdminCardFooter>
                </form>
            </crate::admin_card::AdminCard>
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
            <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Profile>
                <crate::admin_card_header::AdminCardHeader><crate::admin_card_title::AdminCardTitle option="profile-card-title">{constants_str::ADMIN_UI_ACCOUNT}</crate::admin_card_title::AdminCardTitle></crate::admin_card_header::AdminCardHeader>
                <dl>
                    <dt>{constants_str::ADMIN_UI_LOGIN}</dt><dd>{login}</dd>
                    <dt>{constants_str::ADMIN_UI_DISPLAY_NAME}</dt><dd>{display_name}</dd>
                    <dt>{constants_str::ADMIN_UI_ROLES}</dt><dd>{roles}</dd>
                    <dt>{constants_str::ADMIN_UI_PERMISSIONS}</dt><dd>{permissions}</dd>
                </dl>
            </crate::admin_card::AdminCard>
        }
    };

    leptos::view! {
        <section class="profile-grid" data-renderer="csr">
            {admin_profile_account()}
            {admin_change_password()}
        </section>
    }
}
