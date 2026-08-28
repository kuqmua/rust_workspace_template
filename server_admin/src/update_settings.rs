#[path = "admin_html_settings_action_route_registry.rs"]
mod admin_html_settings_action_route_registry;

pub(in crate::domain_types::auth::html::actions) use admin_html_settings_action_route_registry::AdminHtmlSettingsActionRouteRegistry;

#[frontend_contract::domain_types::route_error(AdminHtmlUpdateSettingsError)]
pub(in crate::domain_types::auth::html::actions) async fn update_settings(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::SettingsForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = super::super::super::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Csrf,
        );
    };
    let parsed = (
        super::super::super::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminMainLogo,
            _,
        >(form.main_logo),
        super::super::super::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminOrganizationContacts,
            _,
        >(form.organization_contacts),
        super::super::super::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminOrganizationName,
            _,
        >(form.organization_name),
        super::super::super::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminPrimaryColor,
            _,
        >(form.primary_color),
        super::super::super::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::domain_types::AdminSupportUrl,
            _,
        >(form.support_url),
        super::super::super::optional_setting_impl::optional_setting_impl::<
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
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Validation,
        );
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
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Validation,
        );
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
    super::super::super::action_result_impl::action_result_impl(
        super::super::super::super::settings_update::settings_update(
            auth,
            super::super::super::super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Settings,
    )
}
