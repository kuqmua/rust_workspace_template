mod crud;
mod data_grid;
mod document;
mod navigation;
mod settings;
mod static_pages;

fn test_admin() -> server_admin_contract::domain_types::AuthenticatedAdmin {
    server_admin_contract::domain_types::AuthenticatedAdmin::new(
        server_admin_contract::domain_types::AdminDisplayName::try_from(String::from(
            "Alice Admin",
        ))
        .expect("21d86f4c test_admin invariant must hold"),
        server_admin_contract::domain_types::AdminUserId::try_from(constants_i64::ONE)
            .expect("3ac90e75 test_admin invariant must hold"),
        server_admin_contract::domain_types::AdminLogin::try_from(String::from("alice"))
            .expect("d5810a3f test_admin invariant must hold"),
        server_admin_contract::domain_types::AdminPermissionValues::try_from(Vec::new())
            .expect("8e2c74b1 test_admin invariant must hold"),
        server_admin_contract::domain_types::AdminRoleNames::try_from(vec![
            server_admin_contract::domain_types::AdminRoleName::try_from(String::from("operator"))
                .expect("f0b31c86 test_admin invariant must hold"),
            server_admin_contract::domain_types::AdminRoleName::try_from(String::from("auditor"))
                .expect("5d94ea20 test_admin invariant must hold"),
        ])
        .expect("c72f0d39 test_admin invariant must hold"),
    )
}

fn test_branding() -> server_admin_contract::domain_types::AdminBrandingView {
    server_admin_contract::domain_types::AdminBrandingView::from_settings(
        &server_admin_contract::domain_types::AdminSettingsView::new(
            server_admin_contract::domain_types::AdminDefaultRoute::try_from(String::from(
                "/admin/users",
            ))
            .expect("143ea69b test_branding invariant must hold"),
            None,
            None,
            None,
            None,
            server_admin_contract::domain_types::AdminSiteName::try_from(String::from(
                "Test Admin",
            ))
            .expect("a82f1d63 test_branding invariant must hold"),
            None,
            None,
        ),
    )
}
