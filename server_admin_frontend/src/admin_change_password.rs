#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the change-password card is composed once by the profile screen"
)]

use leptos::prelude::{AddAnyAttr, ElementChild, OnAttribute};

pub(super) fn admin_change_password() -> impl leptos::prelude::IntoView {
    let current_password = leptos::prelude::RwSignal::new(String::new());
    let new_password = leptos::prelude::RwSignal::new(String::new());
    let password_validation_failed = leptos::prelude::RwSignal::new(false);
    leptos::view! {
        <crate::domain_types::with_owner::card::AdminCard variant=crate::domain_types::with_owner::card::AdminCardVariant::Security>
            <crate::domain_types::with_owner::card::AdminCardHeader><crate::domain_types::with_owner::card::AdminCardTitle>"Change password"</crate::domain_types::with_owner::card::AdminCardTitle></crate::domain_types::with_owner::card::AdminCardHeader>
            <form novalidate on:submit=move |event| {
                event.prevent_default();
                let request = (
                    server_admin_contract::domain_types::AdminPassword::try_from(leptos::prelude::Get::get(&current_password)),
                    server_admin_contract::domain_types::AdminNewPassword::try_from(leptos::prelude::Get::get(&new_password)),
                );
                if let (Ok(current), Ok(new_value), Ok(path)) = (
                    request.0,
                    request.1,
                    crate::domain_types::start::http::url::admin_api_url(
                        server_admin_contract::domain_types::AdminRoute::ChangeOwnPassword,
                    ),
                ) {
                    leptos::prelude::Set::set(&password_validation_failed, false);
                    crate::domain_types::start::mutation::reload_after(
                        crate::domain_types::start::mutation::AdminMutationMethod::Post,
                        path,
                        server_admin_contract::domain_types::AdminChangeOwnPasswordReq::new(current, new_value),
                    );
                } else {
                    leptos::prelude::Set::set(&password_validation_failed, true);
                }
            }>
                <crate::domain_types::with_owner::field::AdminField label="Current password"><crate::domain_types::with_owner::input::AdminInput name="current_password" kind=crate::domain_types::with_owner::input::AdminInputKind::Password required=true bind_value=current_password /></crate::domain_types::with_owner::field::AdminField>
                <crate::domain_types::with_owner::field::AdminField label="New password">
                    <crate::domain_types::with_owner::input::AdminInput name="new_password" kind=crate::domain_types::with_owner::input::AdminInputKind::Password minlength=server_admin_contract::domain_types::ADMIN_NEW_PASSWORD_MIN_CHARS maxlength=server_admin_contract::domain_types::ADMIN_PASSWORD_MAX_CHARS required=true bind_value=new_password />
                    <singlestage::FieldDescription attr:class="password-policy">{constants_str::ADMIN_PASSWORD_POLICY_DESCRIPTION}</singlestage::FieldDescription>
                    {move || leptos::prelude::Get::get(&password_validation_failed).then(|| leptos::view! {
                        <singlestage::FieldError>"Check both passwords and ensure the new password satisfies the policy."</singlestage::FieldError>
                    })}
                </crate::domain_types::with_owner::field::AdminField>
                <crate::domain_types::with_owner::card::AdminCardFooter><crate::domain_types::with_owner::button::AdminButton>"Change password"</crate::domain_types::with_owner::button::AdminButton></crate::domain_types::with_owner::card::AdminCardFooter>
            </form>
        </crate::domain_types::with_owner::card::AdminCard>
    }
}
