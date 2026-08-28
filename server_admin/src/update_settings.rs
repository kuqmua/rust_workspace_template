pub(crate) use admin_html_settings_action_route_registry::AdminHtmlSettingsActionRouteRegistry;

#[frontend_contract::domain_types::route_error(AdminHtmlUpdateSettingsError)]
pub(crate) async fn update_settings(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::SettingsForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::AdminError::Csrf);
    };
    let parsed = (
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminMainLogo,
            _,
        >(form.main_logo),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminOrganizationContacts,
            _,
        >(form.organization_contacts),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminOrganizationName,
            _,
        >(form.organization_name),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminPrimaryColor,
            _,
        >(form.primary_color),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminSupportUrl,
            _,
        >(form.support_url),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminTabTitle,
            _,
        >(form.tab_title),
    );
    let (
        Ok(main_logo),
        Ok(organization_contacts),
        Ok(organization_name),
        Ok(primary_color),
        Ok(support_url),
        Ok(tab_title),
    ) = parsed
    else {
        return axum::response::IntoResponse::into_response(crate::AdminError::Validation);
    };
    let mut clear = Vec::new();
    [
        (
            main_logo.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::MainLogo,
        ),
        (
            organization_contacts.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::OrganizationContacts,
        ),
        (
            organization_name.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::OrganizationName,
        ),
        (
            primary_color.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::PrimaryColor,
        ),
        (
            support_url.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::SupportUrl,
        ),
        (
            tab_title.is_none(),
            server_admin_contract::domain_types::AdminOptionalSetting::TabTitle,
        ),
    ]
    .into_iter()
    .filter_map(|(is_empty, setting)| is_empty.then_some(setting))
    .for_each(|setting| clear.push(setting));
    let Ok(clear) = server_admin_contract::domain_types::AdminOptionalSettings::try_from(clear)
    else {
        return axum::response::IntoResponse::into_response(crate::AdminError::Validation);
    };
    let request = server_admin_contract::domain_types::AdminUpdateSettingsReq::new(
        Some(form.default_admin_route),
        main_logo,
        organization_contacts,
        organization_name,
        primary_color,
        Some(form.site_name),
        support_url,
        tab_title,
        clear,
    );
    crate::action_result_impl::action_result_impl(
        crate::settings_update::settings_update(auth, crate::AxumAdminJson(request)).await,
        server_admin_contract::domain_types::AdminFrontendPath::Settings,
    )
}

// Root-owned module compatibility wrappers.
mod admin_html_settings_action_route_registry {
    pub use crate::admin_html_settings_action_route_registry::*;
}
