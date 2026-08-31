#[test]
fn test_settings_page_uses_centered_layout_container() {
    let settings = server_admin_contract::admin_settings_view::AdminSettingsView::new(
        server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect("92b485cf settings_page_uses_centered_layout_container invariant must hold"),
        None,
        None,
        None,
        None,
        server_admin_contract::admin_site_name::AdminSiteName::try_from(
            constants_str::ADMIN.to_owned(),
        )
        .expect("bbf5f240 settings_page_uses_centered_layout_container invariant must hold"),
        None,
        None,
    );
    let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(
            constants_str::ADMIN.to_owned(),
        )
        .expect("a0eb7df6 settings_page_uses_centered_layout_container invariant must hold"),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect("9ff62b22 settings_page_uses_centered_layout_container invariant must hold"),
        server_admin_contract::admin_login::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect("984553cd settings_page_uses_centered_layout_container invariant must hold"),
        server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(Vec::new())
            .expect("86848eb5 settings_page_uses_centered_layout_container invariant must hold"),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new())
            .expect("d3f8287b settings_page_uses_centered_layout_container invariant must hold"),
    );
    let branding =
        server_admin_contract::admin_branding_view::AdminBrandingView::from_settings(&settings);
    let html =
        crate::render_admin_settings_page::render_admin_settings_page(&settings, &admin, &branding);
    assert!(
        html.as_ref()
            .contains("<section class=\"settings-grid\"><div data-name=\"Card\"")
    );
    assert!(html.as_ref().contains("class=\"ui-card settings-card "));
}

#[test]
fn test_editable_settings_render_every_input_kind_from_the_contract_catalog() {
    let settings = server_admin_contract::admin_settings_view::AdminSettingsView::new(
        server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(String::from(constants_str::VALUE_074B6E5E))
            .expect("be6493d0 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        Some(
            server_admin_contract::admin_main_logo::AdminMainLogo::try_from(String::from(
                constants_str::VALUE_A24910BB,
            ))
            .expect("a5708cb4 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        Some(
            server_admin_contract::admin_organization_contacts::AdminOrganizationContacts::try_from(String::from(
                constants_str::VALUE_6F4C18D3,
            ))
            .expect("32da6e91 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        Some(
            server_admin_contract::admin_organization_name::AdminOrganizationName::try_from(String::from(constants_str::VALUE_D029F87E))
                .expect("f4c739a1 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        Some(
            server_admin_contract::admin_primary_color::AdminPrimaryColor::try_from(String::from(constants_str::VALUE_55F98A52))
                .expect("c86b50d7 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        server_admin_contract::admin_site_name::AdminSiteName::try_from(String::from(constants_str::VALUE_AD710C1F))
            .expect("70af15dc editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        Some(
            server_admin_contract::admin_support_url::AdminSupportUrl::try_from(String::from(
                constants_str::VALUE_FE4E2333,
            ))
            .expect("195d8eca editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
        Some(
            server_admin_contract::admin_tab_title::AdminTabTitle::try_from(String::from(constants_str::VALUE_46BB10C9))
                .expect("e317c4b8 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        ),
    );
    let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(String::from(constants_str::ADMIN)).expect("6fa15bc0 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE).expect("9e80d2c4 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        server_admin_contract::admin_login::AdminLogin::try_from(String::from(constants_str::ROOT)).expect("241b70ae editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(vec![
            server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                server_admin_contract::admin_permission::AdminPermission::SystemSettingsUpdate
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_414056C4),
        ])
        .expect("da8504f2 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new()).expect("480eb7c3 editable_settings_render_every_input_kind_from_the_contract_catalog invariant must hold"),
    );
    let branding =
        server_admin_contract::admin_branding_view::AdminBrandingView::from_settings(&settings);

    let html =
        crate::render_admin_settings_page::render_admin_settings_page(&settings, &admin, &branding);

    assert!(html.as_ref().contains("name=\"site_name\""));
    assert!(html.as_ref().contains("name=\"main_logo\""));
    assert!(html.as_ref().contains("type=\"url\""));
    assert!(html.as_ref().contains("data-name=\"Textarea\""));
    assert!(html.as_ref().contains("name=\"organization_contacts\""));
    assert!(html.as_ref().contains(">Support desk</textarea>"));
    assert!(html.as_ref().contains(">Save settings</button>"));
}
