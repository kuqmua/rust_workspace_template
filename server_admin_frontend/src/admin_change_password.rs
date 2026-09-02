#![allow(
    clippy::unused_trait_names,
    reason = "the change-password card is composed once by the profile screen"
)]

use leptos::prelude::{AddAnyAttr, ElementChild, OnAttribute};

pub(super) fn admin_change_password() -> impl leptos::prelude::IntoView {
    let current_password = leptos::prelude::RwSignal::new(String::new());
    let new_password = leptos::prelude::RwSignal::new(String::new());
    let password_validation_failed = leptos::prelude::RwSignal::new(false);
    leptos::view! {
        <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Security>
            <crate::admin_card_header::AdminCardHeader><crate::admin_card_title::AdminCardTitle>"Change password"</crate::admin_card_title::AdminCardTitle></crate::admin_card_header::AdminCardHeader>
            <form novalidate on:submit=move |event| {
                event.prevent_default();
                let request = (
                    server_admin_contract::admin_password::AdminPassword::try_from(leptos::prelude::Get::get(&current_password)),
                    server_admin_contract::admin_new_password::AdminNewPassword::try_from(leptos::prelude::Get::get(&new_password)),
                );
                if let (Ok(current), Ok(new_value), Ok(path)) = (
                    request.0,
                    request.1,
                    crate::domain_types::start::http::url::admin_api_url(
                        server_admin_contract::admin_route::AdminRoute::ChangeOwnPassword,
                    ),
                ) {
                    leptos::prelude::Set::set(&password_validation_failed, false);
                    crate::domain_types::start::mutation::reload_after(
                        crate::admin_mutation_method::AdminMutationMethod::Post,
                        path,
                        server_admin_contract::admin_change_own_password_request::AdminChangeOwnPasswordRequest::new(current, new_value),
                    );
                } else {
                    leptos::prelude::Set::set(&password_validation_failed, true);
                }
            }>
                <crate::admin_field::AdminField admin_field_label="Current password"><crate::admin_input::AdminInput admin_input_name="current_password" admin_input_kind=crate::admin_input_kind::AdminInputKind::Password required=true bind_value=current_password /></crate::admin_field::AdminField>
                <crate::admin_field::AdminField admin_field_label="New password">
                    <crate::admin_input::AdminInput admin_input_name="new_password" admin_input_kind=crate::admin_input_kind::AdminInputKind::Password minlength=server_admin_contract::identity::ADMIN_NEW_PASSWORD_MIN_CHARS maxlength=server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS required=true bind_value=new_password />
                    <singlestage::FieldDescription attr:class="password-policy">{constants_str::ADMIN_PASSWORD_POLICY_DESCRIPTION}</singlestage::FieldDescription>
                    {move || leptos::prelude::Get::get(&password_validation_failed).then(|| leptos::view! {
                        <singlestage::FieldError>"Check both passwords and ensure the new password satisfies the policy."</singlestage::FieldError>
                    })}
                </crate::admin_field::AdminField>
                <crate::admin_card_footer::AdminCardFooter><crate::admin_button::AdminButton>"Change password"</crate::admin_button::AdminButton></crate::admin_card_footer::AdminCardFooter>
            </form>
        </crate::admin_card::AdminCard>
    }
}
