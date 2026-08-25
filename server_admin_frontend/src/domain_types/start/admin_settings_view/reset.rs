pub(super) fn reset() {
    let clear = server_admin_contract::domain_types::AdminOptionalSettings::try_from(
        server_admin_contract::domain_types::AdminOptionalSetting::ALL.to_vec(),
    );
    let values = (
        server_admin_contract::domain_types::AdminDefaultRoute::try_from(
            server_admin_contract::domain_types::AdminFrontendPath::Users
                .get()
                .to_owned(),
        ),
        server_admin_contract::domain_types::AdminSiteName::try_from(
            constants_str::ADMIN.to_owned(),
        ),
        clear,
        super::super::http::url::admin_api_url(
            server_admin_contract::domain_types::AdminRoute::UpdateSettings,
        ),
    );
    if let (Ok(request_default_route), Ok(request_site_name), Ok(request_clear), Ok(path)) = values
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Patch,
            path,
            server_admin_contract::domain_types::AdminUpdateSettingsReq::new(
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
