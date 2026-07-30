pub(super) fn reset() {
    if !bool::from(super::super::mutation::mutation_confirmed(
        super::super::mutation::MutationConfirmationMessageRef::from(
            "Reset administrator settings to template defaults?",
        ),
    )) {
        return;
    }
    let clear = server_admin_contract::AdminOptionalSettings::try_from(
        server_admin_contract::AdminOptionalSetting::ALL.to_vec(),
    );
    let values = (
        server_admin_contract::AdminDefaultRoute::try_from(
            server_admin_contract::AdminFrontendPath::Users
                .get()
                .to_owned(),
        ),
        server_admin_contract::AdminSiteName::try_from(str_constants::ADMIN.to_owned()),
        clear,
        super::super::http::url::admin_api_url(server_admin_contract::AdminRoute::UpdateSettings),
    );
    if let (Ok(request_default_route), Ok(request_site_name), Ok(request_clear), Ok(path)) = values
    {
        super::super::mutation::reload_after(
            super::super::mutation::AdminMutationMethod::Patch,
            path,
            server_admin_contract::AdminUpdateSettingsReq::new(
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
