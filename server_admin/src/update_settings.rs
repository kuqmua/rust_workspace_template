#[proc_macro_frontend_contract::route_error(AdminHtmlUpdateSettingsError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn update_settings(
    auth: crate::admin_auth_req::AdminAuthReq,
    form: crate::axum_admin_form::AxumAdminForm<crate::settings_form::SettingsForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Csrf);
    };
    let (
        default_admin_route,
        main_logo_form,
        organization_contacts_form,
        organization_name_form,
        primary_color_form,
        site_name,
        support_url_form,
        tab_title_form,
    ) = form.into_inner().into_parts();
    let parsed = (
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::admin_main_logo::AdminMainLogo,
            _,
        >(main_logo_form),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::admin_organization_contacts::AdminOrganizationContacts,
            _,
        >(organization_contacts_form),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::admin_organization_name::AdminOrganizationName,
            _,
        >(organization_name_form),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::admin_primary_color::AdminPrimaryColor,
            _,
        >(primary_color_form),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::admin_support_url::AdminSupportUrl,
            _,
        >(support_url_form),
        crate::optional_setting_impl::optional_setting_impl::<
            server_admin_contract::admin_tab_title::AdminTabTitle,
            _,
        >(tab_title_form),
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
            crate::admin_error::AdminError::Validation,
        );
    };
    let mut clear = Vec::new();
    [
        (
            main_logo.is_none(),
            server_admin_contract::admin_optional_setting::AdminOptionalSetting::MainLogo,
        ),
        (
            organization_contacts.is_none(),
            server_admin_contract::admin_optional_setting::AdminOptionalSetting::OrganizationContacts,
        ),
        (
            organization_name.is_none(),
            server_admin_contract::admin_optional_setting::AdminOptionalSetting::OrganizationName,
        ),
        (
            primary_color.is_none(),
            server_admin_contract::admin_optional_setting::AdminOptionalSetting::PrimaryColor,
        ),
        (
            support_url.is_none(),
            server_admin_contract::admin_optional_setting::AdminOptionalSetting::SupportUrl,
        ),
        (
            tab_title.is_none(),
            server_admin_contract::admin_optional_setting::AdminOptionalSetting::TabTitle,
        ),
    ]
    .into_iter()
    .filter_map(|(is_empty, setting)| is_empty.then_some(setting))
    .for_each(|setting| clear.push(setting));
    let Ok(clear) =
        server_admin_contract::admin_optional_settings::AdminOptionalSettings::try_from(clear)
    else {
        return axum::response::IntoResponse::into_response(
            crate::admin_error::AdminError::Validation,
        );
    };
    let request = server_admin_contract::admin_update_settings_req::AdminUpdateSettingsReq::new(
        Some(default_admin_route),
        main_logo,
        organization_contacts,
        organization_name,
        primary_color,
        Some(site_name),
        support_url,
        tab_title,
        clear,
    );
    crate::action_result_impl::action_result_impl(
        crate::settings_update::settings_update(
            auth,
            crate::axum_admin_json::AxumAdminJson::from(request),
        )
        .await,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Settings,
    )
}

// Root-owned module compatibility wrappers.
