#[test]
fn settings_page_uses_centered_layout_container() {
    let settings = server_admin_contract::domain_types::AdminSettingsView::new(
        server_admin_contract::domain_types::AdminDefaultRoute::try_from(
            server_admin_contract::domain_types::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect("92b485cf settings_page_uses_centered_layout_container invariant must hold"),
        None,
        None,
        None,
        None,
        server_admin_contract::domain_types::AdminSiteName::try_from(
            constants_str::ADMIN.to_owned(),
        )
        .expect("bbf5f240 settings_page_uses_centered_layout_container invariant must hold"),
        None,
        None,
    );
    let admin = server_admin_contract::domain_types::AuthenticatedAdmin::new(
        server_admin_contract::domain_types::AdminDisplayName::try_from(
            constants_str::ADMIN.to_owned(),
        )
        .expect("a0eb7df6 settings_page_uses_centered_layout_container invariant must hold"),
        server_admin_contract::domain_types::AdminUserId::try_from(constants_i64::ONE)
            .expect("9ff62b22 settings_page_uses_centered_layout_container invariant must hold"),
        server_admin_contract::domain_types::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect("984553cd settings_page_uses_centered_layout_container invariant must hold"),
        server_admin_contract::domain_types::AdminPermissionValues::try_from(Vec::new())
            .expect("86848eb5 settings_page_uses_centered_layout_container invariant must hold"),
        server_admin_contract::domain_types::AdminRoleNames::try_from(Vec::new())
            .expect("d3f8287b settings_page_uses_centered_layout_container invariant must hold"),
    );
    let branding = server_admin_contract::domain_types::AdminBrandingView::from_settings(&settings);
    let html = super::super::render_settings(&settings, &admin, &branding);
    assert!(
        html.as_ref()
            .contains("<section class=\"settings-grid\"><div data-name=\"Card\"")
    );
    assert!(html.as_ref().contains("class=\"ui-card settings-card "));
}

#[test]
fn editable_settings_render_every_input_kind_from_the_contract_catalog() {
    let settings = server_admin_contract::domain_types::AdminSettingsView::new(
        server_admin_contract::domain_types::AdminDefaultRoute::try_from(String::from("/admin/users"))
            .expect("be6493d0 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        Some(
            server_admin_contract::domain_types::AdminMainLogo::try_from(String::from(
                "https://example.test/logo.svg",
            ))
            .expect("a5708cb4 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        Some(
            server_admin_contract::domain_types::AdminOrganizationContacts::try_from(String::from(
                "Support desk",
            ))
            .expect("32da6e91 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        Some(
            server_admin_contract::domain_types::AdminOrganizationName::try_from(String::from("Example"))
                .expect("f4c739a1 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        Some(
            server_admin_contract::domain_types::AdminPrimaryColor::try_from(String::from("#123456"))
                .expect("c86b50d7 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        server_admin_contract::domain_types::AdminSiteName::try_from(String::from("Example Admin"))
            .expect("70af15dc editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        Some(
            server_admin_contract::domain_types::AdminSupportUrl::try_from(String::from(
                "https://example.test/support",
            ))
            .expect("195d8eca editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        Some(
            server_admin_contract::domain_types::AdminTabTitle::try_from(String::from("Control Panel"))
                .expect("e317c4b8 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
    );
    let admin = server_admin_contract::domain_types::AuthenticatedAdmin::new(
        server_admin_contract::domain_types::AdminDisplayName::try_from(String::from("Admin")).expect("6fa15bc0 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        server_admin_contract::domain_types::AdminUserId::try_from(constants_i64::ONE).expect("9e80d2c4 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        server_admin_contract::domain_types::AdminLogin::try_from(String::from("root")).expect("241b70ae editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        server_admin_contract::domain_types::AdminPermissionValues::try_from(vec![
            server_admin_contract::domain_types::AdminPermissionValue::try_from(
                server_admin_contract::domain_types::AdminPermission::SystemSettingsUpdate
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect("b73c60e9 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ])
        .expect("da8504f2 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        server_admin_contract::domain_types::AdminRoleNames::try_from(Vec::new()).expect("480eb7c3 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
    );
    let branding = server_admin_contract::domain_types::AdminBrandingView::from_settings(&settings);

    let html = super::super::render_settings(&settings, &admin, &branding);

    assert!(html.as_ref().contains("name=\"site_name\""));
    assert!(html.as_ref().contains("name=\"main_logo\""));
    assert!(html.as_ref().contains("type=\"url\""));
    assert!(html.as_ref().contains("data-name=\"Textarea\""));
    assert!(html.as_ref().contains("name=\"organization_contacts\""));
    assert!(html.as_ref().contains(">Support desk</textarea>"));
    assert!(html.as_ref().contains(">Save settings</button>"));
}
