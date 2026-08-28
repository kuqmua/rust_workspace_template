pub(super) fn save(
    signals: crate::domain_types::shared::settings::admin_settings_form_signals::AdminSettingsFormSignals,
) {
    let default_route =
        signals.get(server_admin_contract::domain_types::AdminSetting::DefaultRoute);
    let main_logo = signals.get(server_admin_contract::domain_types::AdminSetting::MainLogo);
    let organization_contacts =
        signals.get(server_admin_contract::domain_types::AdminSetting::OrganizationContacts);
    let organization_name =
        signals.get(server_admin_contract::domain_types::AdminSetting::OrganizationName);
    let primary_color =
        signals.get(server_admin_contract::domain_types::AdminSetting::PrimaryColor);
    let site_name = signals.get(server_admin_contract::domain_types::AdminSetting::SiteName);
    let support_url = signals.get(server_admin_contract::domain_types::AdminSetting::SupportUrl);
    let tab_title = signals.get(server_admin_contract::domain_types::AdminSetting::TabTitle);
    let default_route_value = default_route.value().as_ref().to_owned();
    let main_logo_value = main_logo.value().as_ref().to_owned();
    let organization_contacts_value = organization_contacts.value().as_ref().to_owned();
    let organization_name_value = organization_name.value().as_ref().to_owned();
    let primary_color_value = primary_color.value().as_ref().to_owned();
    let site_name_value = site_name.value().as_ref().to_owned();
    let support_url_value = support_url.value().as_ref().to_owned();
    let tab_title_value = tab_title.value().as_ref().to_owned();
    let clear = server_admin_contract::domain_types::AdminOptionalSettings::try_from(
        server_admin_contract::domain_types::AdminSetting::ALL
            .into_iter()
            .filter_map(|setting| match setting.spec().optionality() {
                server_admin_contract::domain_types::AdminSettingOptionality::Clearable(
                    optional,
                ) if signals.get(setting).value().as_ref().is_empty() => Some(optional),
                server_admin_contract::domain_types::AdminSettingOptionality::Clearable(_)
                | server_admin_contract::domain_types::AdminSettingOptionality::Required => None,
            })
            .collect::<Vec<_>>(),
    );
    let values = (
        server_admin_contract::domain_types::AdminDefaultRoute::try_from(default_route_value),
        (!main_logo_value.is_empty())
            .then(|| server_admin_contract::domain_types::AdminMainLogo::try_from(main_logo_value))
            .transpose(),
        (!organization_contacts_value.is_empty())
            .then(|| {
                server_admin_contract::domain_types::AdminOrganizationContacts::try_from(
                    organization_contacts_value,
                )
            })
            .transpose(),
        (!organization_name_value.is_empty())
            .then(|| {
                server_admin_contract::domain_types::AdminOrganizationName::try_from(
                    organization_name_value,
                )
            })
            .transpose(),
        (!primary_color_value.is_empty())
            .then(|| {
                server_admin_contract::domain_types::AdminPrimaryColor::try_from(
                    primary_color_value,
                )
            })
            .transpose(),
        server_admin_contract::domain_types::AdminSiteName::try_from(site_name_value),
        (!support_url_value.is_empty())
            .then(|| {
                server_admin_contract::domain_types::AdminSupportUrl::try_from(support_url_value)
            })
            .transpose(),
        (!tab_title_value.is_empty())
            .then(|| server_admin_contract::domain_types::AdminTabTitle::try_from(tab_title_value))
            .transpose(),
        clear,
        crate::http::url::admin_api_url(
            server_admin_contract::domain_types::AdminRoute::UpdateSettings,
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
        crate::mutation::reload_after(
            crate::mutation::AdminMutationMethod::Patch,
            path,
            server_admin_contract::domain_types::AdminUpdateSettingsReq::new(
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
}
