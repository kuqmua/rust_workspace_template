pub(super) fn reset() {
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
        crate::http::url::admin_api_url(
            server_admin_contract::admin_route::AdminRoute::UpdateSettings,
        ),
    );
    if let (Ok(request_default_route), Ok(request_site_name), Ok(request_clear), Ok(path)) = values
    {
        crate::mutation::reload_after(
            crate::admin_mutation_method::AdminMutationMethod::Patch,
            path,
            server_admin_contract::admin_update_settings_req::AdminUpdateSettingsReq::new(
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
}
