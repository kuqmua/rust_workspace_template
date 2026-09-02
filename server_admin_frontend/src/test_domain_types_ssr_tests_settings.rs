#[test]
fn test_settings_page_uses_centered_layout_container() {
    let settings = server_admin_contract::admin_settings_view::AdminSettingsView::new(
        server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_92B485CF),
        None,
        None,
        None,
        None,
        server_admin_contract::admin_site_name::AdminSiteName::try_from(
            constants_str::ADMIN.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_BBF5F240),
        None,
        None,
    );
    let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(
            constants_str::ADMIN.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_A0EB7DF6),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_9FF62B22),
        server_admin_contract::admin_login::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect(constants_str::DIAGNOSTIC_984553CD),
        server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_86848EB5),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_D3F8287B),
    );
    let branding =
        server_admin_contract::admin_branding_view::AdminBrandingView::from_settings(&settings);
    let html =
        crate::render_admin_settings_page::render_admin_settings_page(&settings, &admin, &branding);
    assert!(html.as_ref().contains(constants_str::VALUE_073C0D6E));
    assert!(html.as_ref().contains(constants_str::VALUE_2BEB20BD));
}

#[test]
fn test_editable_settings_render_every_input_kind_from_the_contract_catalog() {
    let settings = server_admin_contract::admin_settings_view::AdminSettingsView::new(
        server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(String::from(constants_str::VALUE_074B6E5E))
            .expect(constants_str::DIAGNOSTIC_BE6493D0),
        Some(
            server_admin_contract::admin_main_logo::AdminMainLogo::try_from(String::from(
                constants_str::VALUE_A24910BB,
            ))
            .expect(constants_str::DIAGNOSTIC_A5708CB4),
        ),
        Some(
            server_admin_contract::admin_organization_contacts::AdminOrganizationContacts::try_from(String::from(
                constants_str::VALUE_6F4C18D3,
            ))
            .expect(constants_str::DIAGNOSTIC_32DA6E91),
        ),
        Some(
            server_admin_contract::admin_organization_name::AdminOrganizationName::try_from(String::from(constants_str::VALUE_D029F87E))
                .expect(constants_str::DIAGNOSTIC_F4C739A1),
        ),
        Some(
            server_admin_contract::admin_primary_color::AdminPrimaryColor::try_from(String::from(constants_str::VALUE_55F98A52))
                .expect(constants_str::DIAGNOSTIC_C86B50D7),
        ),
        server_admin_contract::admin_site_name::AdminSiteName::try_from(String::from(constants_str::VALUE_AD710C1F))
            .expect(constants_str::DIAGNOSTIC_70AF15DC),
        Some(
            server_admin_contract::admin_support_url::AdminSupportUrl::try_from(String::from(
                constants_str::VALUE_FE4E2333,
            ))
            .expect(constants_str::DIAGNOSTIC_195D8ECA),
        ),
        Some(
            server_admin_contract::admin_tab_title::AdminTabTitle::try_from(String::from(constants_str::VALUE_46BB10C9))
                .expect(constants_str::DIAGNOSTIC_E317C4B8),
        ),
    );
    let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(String::from(
            constants_str::ADMIN,
        ))
        .expect(constants_str::DIAGNOSTIC_6FA15BC0),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_9E80D2C4),
        server_admin_contract::admin_login::AdminLogin::try_from(String::from(constants_str::ROOT))
            .expect(constants_str::DIAGNOSTIC_241B70AE),
        server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(vec![
            server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                server_admin_contract::admin_permission::AdminPermission::SystemSettingsUpdate
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_414056C4),
        ])
        .expect(constants_str::DIAGNOSTIC_DA8504F2),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_480EB7C3),
    );
    let branding =
        server_admin_contract::admin_branding_view::AdminBrandingView::from_settings(&settings);

    let html =
        crate::render_admin_settings_page::render_admin_settings_page(&settings, &admin, &branding);

    assert!(html.as_ref().contains(constants_str::VALUE_58BBA249));
    assert!(html.as_ref().contains(constants_str::VALUE_4951C9D3));
    assert!(html.as_ref().contains(constants_str::VALUE_3F96A519));
    assert!(html.as_ref().contains(constants_str::VALUE_345DE32F));
    assert!(html.as_ref().contains(constants_str::VALUE_28A73F84));
    assert!(html.as_ref().contains(constants_str::VALUE_E1A1A172));
    assert!(html.as_ref().contains(constants_str::VALUE_ADC2C05B));
}
