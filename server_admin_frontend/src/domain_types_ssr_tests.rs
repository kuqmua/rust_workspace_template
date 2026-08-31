pub(crate) fn test_admin() -> server_admin_contract::authenticated_admin::AuthenticatedAdmin {
    server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(String::from(
            constants_str::VALUE_A31B31EA,
        ))
        .expect("21d86f4c test_admin invariant must hold"),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect("3ac90e75 test_admin invariant must hold"),
        server_admin_contract::admin_login::AdminLogin::try_from(String::from(
            constants_str::VALUE_2BD806C9,
        ))
        .expect("d5810a3f test_admin invariant must hold"),
        server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(Vec::new())
            .expect("8e2c74b1 test_admin invariant must hold"),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(vec![
            server_admin_contract::admin_role_name::AdminRoleName::try_from(String::from(
                constants_str::PG_CRUD_OPERATOR_FIELD,
            ))
            .expect(constants_str::VALUE_8B4613C7),
            server_admin_contract::admin_role_name::AdminRoleName::try_from(String::from(
                constants_str::VALUE_C5A62CE3,
            ))
            .expect(constants_str::VALUE_E124D275),
        ])
        .expect("c72f0d39 test_admin invariant must hold"),
    )
}

pub(crate) fn test_branding() -> server_admin_contract::admin_branding_view::AdminBrandingView {
    server_admin_contract::admin_branding_view::AdminBrandingView::from_settings(
        &server_admin_contract::admin_settings_view::AdminSettingsView::new(
            server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(String::from(
                constants_str::VALUE_074B6E5E,
            ))
            .expect("143ea69b test_branding invariant must hold"),
            None,
            None,
            None,
            None,
            server_admin_contract::admin_site_name::AdminSiteName::try_from(String::from(
                constants_str::VALUE_B49D7EDE,
            ))
            .expect("a82f1d63 test_branding invariant must hold"),
            None,
            None,
        ),
    )
}

// Root-owned module compatibility wrappers.
pub(crate) mod crud {}
pub(crate) mod data_grid {}
pub(crate) mod document {}
pub(crate) mod navigation {}
pub(crate) mod settings {}
pub(crate) mod static_pages {}
