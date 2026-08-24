#[test]
fn settings_page_uses_centered_layout_container() {
    let settings = server_admin_contract::AdminSettingsView::new(
        server_admin_contract::AdminDefaultRoute::try_from(
            server_admin_contract::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect("92b485cf"),
        None,
        None,
        None,
        None,
        server_admin_contract::AdminSiteName::try_from(str_constants::ADMIN.to_owned())
            .expect("bbf5f240"),
        None,
        None,
    );
    let admin = server_admin_contract::AuthenticatedAdmin::new(
        server_admin_contract::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
            .expect("a0eb7df6"),
        server_admin_contract::AdminUserId::try_from(1i64).expect("9ff62b22"),
        server_admin_contract::AdminLogin::try_from(str_constants::ROOT.to_owned())
            .expect("984553cd"),
        server_admin_contract::AdminPermissionValues::try_from(Vec::new()).expect("86848eb5"),
        server_admin_contract::AdminRoleNames::try_from(Vec::new()).expect("d3f8287b"),
    );
    let branding = server_admin_contract::AdminBrandingView::from_settings(&settings);
    let html = super::super::render_settings(&settings, &admin, &branding);
    assert!(
        html.as_ref()
            .contains("<section class=\"settings-grid\"><div data-name=\"Card\"")
    );
    assert!(html.as_ref().contains("class=\"ui-card settings-card "));
}

#[test]
fn editable_settings_render_every_input_kind_from_the_contract_catalog() {
    let settings = server_admin_contract::AdminSettingsView::new(
        server_admin_contract::AdminDefaultRoute::try_from(String::from("/admin/users"))
            .expect("be6493d0"),
        Some(
            server_admin_contract::AdminMainLogo::try_from(String::from(
                "https://example.test/logo.svg",
            ))
            .expect("a5708cb4"),
        ),
        Some(
            server_admin_contract::AdminOrganizationContacts::try_from(String::from(
                "Support desk",
            ))
            .expect("32da6e91"),
        ),
        Some(
            server_admin_contract::AdminOrganizationName::try_from(String::from("Example"))
                .expect("f4c739a1"),
        ),
        Some(
            server_admin_contract::AdminPrimaryColor::try_from(String::from("#123456"))
                .expect("c86b50d7"),
        ),
        server_admin_contract::AdminSiteName::try_from(String::from("Example Admin"))
            .expect("70af15dc"),
        Some(
            server_admin_contract::AdminSupportUrl::try_from(String::from(
                "https://example.test/support",
            ))
            .expect("195d8eca"),
        ),
        Some(
            server_admin_contract::AdminTabTitle::try_from(String::from("Control Panel"))
                .expect("e317c4b8"),
        ),
    );
    let admin = server_admin_contract::AuthenticatedAdmin::new(
        server_admin_contract::AdminDisplayName::try_from(String::from("Admin")).expect("6fa15bc0"),
        server_admin_contract::AdminUserId::try_from(1i64).expect("9e80d2c4"),
        server_admin_contract::AdminLogin::try_from(String::from("root")).expect("241b70ae"),
        server_admin_contract::AdminPermissionValues::try_from(vec![
            server_admin_contract::AdminPermissionValue::try_from(
                server_admin_contract::AdminPermission::SystemSettingsUpdate
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect("b73c60e9"),
        ])
        .expect("da8504f2"),
        server_admin_contract::AdminRoleNames::try_from(Vec::new()).expect("480eb7c3"),
    );
    let branding = server_admin_contract::AdminBrandingView::from_settings(&settings);

    let html = super::super::render_settings(&settings, &admin, &branding);

    assert!(html.as_ref().contains("name=\"site_name\""));
    assert!(html.as_ref().contains("name=\"main_logo\""));
    assert!(html.as_ref().contains("type=\"url\""));
    assert!(html.as_ref().contains("data-name=\"Textarea\""));
    assert!(html.as_ref().contains("name=\"organization_contacts\""));
    assert!(html.as_ref().contains(">Support desk</textarea>"));
    assert!(html.as_ref().contains(">Save settings</button>"));
}
