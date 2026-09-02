#[test]
fn test_empty_settings_update_has_no_fields_and_is_valid() {
    let request = crate::admin_update_settings_request::AdminUpdateSettingsRequest::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        crate::admin_optional_settings::AdminOptionalSettings::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_D1846F3A),
    );
    assert!(!bool::from(request.has_fields()));
    assert!(bool::from(request.is_valid()));
}

#[test]
fn test_setting_types_match_database_constraints() {
    let Err(_empty_site_name_error) =
        crate::admin_site_name::AdminSiteName::try_from(String::new())
    else {
        std::panic::panic_any(constants_str::PANIC_4CFB6820);
    };
    let Err(_blank_site_name_error) =
        crate::admin_site_name::AdminSiteName::try_from(constants_str::SPACE.to_owned())
    else {
        std::panic::panic_any(constants_str::PANIC_B5FBA19E);
    };
    let _site_name =
        crate::admin_site_name::AdminSiteName::try_from(constants_str::ADMIN.to_owned())
            .expect(constants_str::DIAGNOSTIC_ADB58327);
    let _default_route = crate::admin_default_route::AdminDefaultRoute::try_from(
        crate::admin_frontend_path::AdminFrontendPath::Users
            .get()
            .to_owned(),
    )
    .expect(constants_str::DIAGNOSTIC_3582A0EC);
    let _table_default_route = crate::admin_default_route::AdminDefaultRoute::try_from(
        crate::admin_data_table::AdminDataTable::RolePermissions
            .frontend_path()
            .to_string(),
    )
    .expect(constants_str::DIAGNOSTIC_E3D42017);
    let Err(_invalid_route_error) =
        crate::admin_default_route::AdminDefaultRoute::try_from(constants_str::ROUTE.to_owned())
    else {
        std::panic::panic_any(constants_str::PANIC_BB0D454A);
    };
}

#[test]
fn test_update_reports_whether_it_contains_a_field() {
    let empty = crate::admin_update_settings_request::AdminUpdateSettingsRequest::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        crate::admin_optional_settings::AdminOptionalSettings::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_C4A1E2D3),
    );
    assert!(!bool::from(empty.has_fields()));
    let with_site_name = crate::admin_update_settings_request::AdminUpdateSettingsRequest::new(
        None,
        None,
        None,
        None,
        None,
        Some(
            crate::admin_site_name::AdminSiteName::try_from(constants_str::ADMIN.to_owned())
                .expect(constants_str::DIAGNOSTIC_5DB76A91),
        ),
        None,
        None,
        crate::admin_optional_settings::AdminOptionalSettings::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_32E4E74D),
    );
    assert!(bool::from(with_site_name.has_fields()));
    assert!(bool::from(with_site_name.is_valid()));
    let clear_logo = crate::admin_update_settings_request::AdminUpdateSettingsRequest::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        crate::admin_optional_settings::AdminOptionalSettings::try_from(vec![
            crate::admin_optional_setting::AdminOptionalSetting::MainLogo,
        ])
        .expect(constants_str::DIAGNOSTIC_96E94562),
    );
    assert!(bool::from(clear_logo.has_fields()));
    assert!(bool::from(clear_logo.is_valid()));
}

#[test]
fn test_catalog_covers_read_and_update_wire_fields() {
    let empty_clear = crate::admin_optional_settings::AdminOptionalSettings::try_from(Vec::new())
        .expect(constants_str::DIAGNOSTIC_7F3A9C2E);
    let update = crate::admin_update_settings_request::AdminUpdateSettingsRequest::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        empty_clear,
    );
    let update_fields = serde_json::to_value(update)
        .expect(constants_str::DIAGNOSTIC_C84D1E6A)
        .as_object()
        .expect(constants_str::DIAGNOSTIC_49B2E7C1)
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let setting_fields = crate::admin_setting::AdminSetting::ALL
        .into_iter()
        .map(|setting| setting.spec().name().as_ref().to_owned())
        .collect::<std::collections::BTreeSet<_>>();
    let mut expected_update_fields = setting_fields.clone();
    let _inserted = expected_update_fields.insert(String::from(constants_str::VALUE_913A4CB9));
    assert_eq!(update_fields, expected_update_fields);

    let view = crate::admin_settings_view::AdminSettingsView::new(
        crate::admin_default_route::AdminDefaultRoute::try_from(String::from(
            constants_str::VALUE_074B6E5E,
        ))
        .expect(constants_str::DIAGNOSTIC_B6831FD4),
        None,
        None,
        None,
        None,
        crate::admin_site_name::AdminSiteName::try_from(String::from(constants_str::ADMIN))
            .expect(constants_str::DIAGNOSTIC_E15C7A93),
        None,
        None,
    );
    let view_fields = serde_json::to_value(view)
        .expect(constants_str::DIAGNOSTIC_86D4A2F9)
        .as_object()
        .expect(constants_str::DIAGNOSTIC_21C9E5B7)
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(view_fields, setting_fields);
}
