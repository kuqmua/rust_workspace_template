#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminSettingsView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_settings_view: server_admin_contract::admin_settings_view::AdminSettingsView,
) -> impl leptos::prelude::IntoView {
    let save = |admin_settings_form_signals: crate::admin_settings_form_signals::AdminSettingsFormSignals,| {
    let default_route = admin_settings_form_signals
        .get(server_admin_contract::admin_setting::AdminSetting::DefaultRoute);
    let main_logo = admin_settings_form_signals
        .get(server_admin_contract::admin_setting::AdminSetting::MainLogo);
    let organization_contacts = admin_settings_form_signals
        .get(server_admin_contract::admin_setting::AdminSetting::OrganizationContacts);
    let organization_name = admin_settings_form_signals
        .get(server_admin_contract::admin_setting::AdminSetting::OrganizationName);
    let primary_color = admin_settings_form_signals
        .get(server_admin_contract::admin_setting::AdminSetting::PrimaryColor);
    let site_name = admin_settings_form_signals
        .get(server_admin_contract::admin_setting::AdminSetting::SiteName);
    let support_url = admin_settings_form_signals
        .get(server_admin_contract::admin_setting::AdminSetting::SupportUrl);
    let tab_title = admin_settings_form_signals
        .get(server_admin_contract::admin_setting::AdminSetting::TabTitle);
    let default_route_value = default_route.value().as_ref().to_owned();
    let main_logo_value = main_logo.value().as_ref().to_owned();
    let organization_contacts_value = organization_contacts.value().as_ref().to_owned();
    let organization_name_value = organization_name.value().as_ref().to_owned();
    let primary_color_value = primary_color.value().as_ref().to_owned();
    let site_name_value = site_name.value().as_ref().to_owned();
    let support_url_value = support_url.value().as_ref().to_owned();
    let tab_title_value = tab_title.value().as_ref().to_owned();
    let clear = server_admin_contract::admin_optional_settings::AdminOptionalSettings::try_from(
        server_admin_contract::admin_setting::AdminSetting::ALL
            .into_iter()
            .filter_map(|setting| match setting.spec().optionality() {
                server_admin_contract::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    optional,
                ) if admin_settings_form_signals.get(setting).value().as_ref().is_empty() => Some(optional),
                server_admin_contract::admin_setting_optionality::AdminSettingOptionality::Clearable(_)
                | server_admin_contract::admin_setting_optionality::AdminSettingOptionality::Required => None,
            })
            .collect::<Vec<_>>(),
    );
    let values = (
        server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(default_route_value),
        (!main_logo_value.is_empty())
            .then(|| server_admin_contract::admin_main_logo::AdminMainLogo::try_from(main_logo_value))
            .transpose(),
        (!organization_contacts_value.is_empty())
            .then(|| {
                server_admin_contract::admin_organization_contacts::AdminOrganizationContacts::try_from(
                    organization_contacts_value,
                )
            })
            .transpose(),
        (!organization_name_value.is_empty())
            .then(|| {
                server_admin_contract::admin_organization_name::AdminOrganizationName::try_from(
                    organization_name_value,
                )
            })
            .transpose(),
        (!primary_color_value.is_empty())
            .then(|| {
                server_admin_contract::admin_primary_color::AdminPrimaryColor::try_from(
                    primary_color_value,
                )
            })
            .transpose(),
        server_admin_contract::admin_site_name::AdminSiteName::try_from(site_name_value),
        (!support_url_value.is_empty())
            .then(|| {
                server_admin_contract::admin_support_url::AdminSupportUrl::try_from(support_url_value)
            })
            .transpose(),
        (!tab_title_value.is_empty())
            .then(|| server_admin_contract::admin_tab_title::AdminTabTitle::try_from(tab_title_value))
            .transpose(),
        clear,
        crate::admin_api_url::admin_api_url(
            server_admin_contract::admin_route::AdminRoute::UpdateSettings,
        ),
    );
    if let (
        Ok(request_default_route),
        Ok(request_main_logo),
        Ok(request_organization_contacts),
        Ok(request_organization_name),
        Ok(request_primary_color),
        Ok(request_site_name),
        Ok(request_support_url),
        Ok(request_tab_title),
        Ok(request_clear),
        Ok(path),
    ) = values
    {
        crate::reload_after::reload_after(
            crate::admin_mutation_method::AdminMutationMethod::Patch,
            path,
            server_admin_contract::admin_update_settings_request::AdminUpdateSettingsRequest::new(
                Some(request_default_route),
                request_main_logo,
                request_organization_contacts,
                request_organization_name,
                request_primary_color,
                Some(request_site_name),
                request_support_url,
                request_tab_title,
                request_clear,
            ),
        );
    }
    };

    let reset = || {
        let clear = server_admin_contract::admin_optional_settings::AdminOptionalSettings::try_from(
            server_admin_contract::admin_optional_setting::AdminOptionalSetting::ALL.to_vec(),
        );
        let values = (
            server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users
                    .get()
                    .to_owned(),
            ),
            server_admin_contract::admin_site_name::AdminSiteName::try_from(
                constants_str::ADMIN.to_owned(),
            ),
            clear,
            crate::admin_api_url::admin_api_url(
                server_admin_contract::admin_route::AdminRoute::UpdateSettings,
            ),
        );
        if let (Ok(request_default_route), Ok(request_site_name), Ok(request_clear), Ok(path)) =
            values
        {
            crate::reload_after::reload_after(
            crate::admin_mutation_method::AdminMutationMethod::Patch,
            path,
            server_admin_contract::admin_update_settings_request::AdminUpdateSettingsRequest::new(
                Some(request_default_route),
                None,
                None,
                None,
                None,
                Some(request_site_name),
                None,
                None,
                request_clear,
            ),
        );
        }
    };

    let can_update = bool::from(authenticated_admin.has_permission(
        server_admin_contract::admin_permission::AdminPermission::SystemSettingsUpdate,
    ));
    let values =
        crate::admin_settings_form_values::AdminSettingsFormValues::from(&admin_settings_view);
    let signals = crate::admin_settings_form_signals::AdminSettingsFormSignals::new(&values);
    leptos::view! {
        <section class="settings-grid" data-renderer="csr"><crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Settings><form class="settings-form" on:submit=move |event| {
            event.prevent_default();
            save(signals);
        }>
            {crate::admin_setting_inputs::admin_setting_inputs(signals, crate::admin_setting_disabled::AdminSettingDisabled::from(!can_update))}
            <crate::admin_card_footer::AdminCardFooter>
                <crate::admin_button::AdminButton bool=!can_update>"Save settings"</crate::admin_button::AdminButton>
                <crate::admin_alert_dialog::AdminAlertDialog string=String::from("reset-settings-dialog") title="Reset settings?" description="All administrator settings will return to the template defaults." trigger="Reset to template defaults" confirm="Reset settings" bool=!can_update callback=leptos::prelude::Callback::new(move |()| {
                    reset();
                }) />
            </crate::admin_card_footer::AdminCardFooter>
        </form></crate::admin_card::AdminCard></section>
    }
}
