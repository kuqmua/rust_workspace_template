#[test]
fn empty_settings_update_has_no_fields_and_is_valid() {
    let request = super::AdminUpdateSettingsReq::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        crate::domain_types::AdminOptionalSettings::try_from(Vec::new()).expect(
            "d1846f3a empty_settings_update_has_no_fields_and_is_valid invariant must hold",
        ),
    );
    assert!(!bool::from(request.has_fields()));
    assert!(bool::from(request.is_valid()));
}

#[test]
fn setting_types_match_database_constraints() {
    let Err(_empty_site_name_error) = crate::domain_types::AdminSiteName::try_from(String::new())
    else {
        panic!("4cfb6820");
    };
    let Err(_blank_site_name_error) =
        crate::domain_types::AdminSiteName::try_from(constants_str::SPACE.to_owned())
    else {
        panic!("b5fba19e");
    };
    let _site_name = crate::domain_types::AdminSiteName::try_from(constants_str::ADMIN.to_owned())
        .expect("adb58327 setting_types_match_database_constraints invariant must hold");
    let _default_route = crate::domain_types::AdminDefaultRoute::try_from(
        crate::domain_types::AdminFrontendPath::Users
            .get()
            .to_owned(),
    )
    .expect("3582a0ec setting_types_match_database_constraints invariant must hold");
    let _table_default_route = crate::domain_types::AdminDefaultRoute::try_from(
        crate::domain_types::AdminDataTable::RolePermissions
            .frontend_path()
            .to_string(),
    )
    .expect("e3d42017 setting_types_match_database_constraints invariant must hold");
    let Err(_invalid_route_error) =
        crate::domain_types::AdminDefaultRoute::try_from(constants_str::ROUTE.to_owned())
    else {
        panic!("bb0d454a");
    };
}

#[test]
fn update_reports_whether_it_contains_a_field() {
    let empty = super::AdminUpdateSettingsReq::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        crate::domain_types::AdminOptionalSettings::try_from(Vec::new())
            .expect("c4a1e2d3 update_reports_whether_it_contains_a_field invariant must hold"),
    );
    assert!(!bool::from(empty.has_fields()));
    let with_site_name = super::AdminUpdateSettingsReq::new(
        None,
        None,
        None,
        None,
        None,
        Some(
            crate::domain_types::AdminSiteName::try_from(constants_str::ADMIN.to_owned())
                .expect("5db76a91 update_reports_whether_it_contains_a_field invariant must hold"),
        ),
        None,
        None,
        crate::domain_types::AdminOptionalSettings::try_from(Vec::new())
            .expect("32e4e74d update_reports_whether_it_contains_a_field invariant must hold"),
    );
    assert!(bool::from(with_site_name.has_fields()));
    assert!(bool::from(with_site_name.is_valid()));
    let clear_logo = super::AdminUpdateSettingsReq::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        crate::domain_types::AdminOptionalSettings::try_from(vec![
            crate::domain_types::AdminOptionalSetting::MainLogo,
        ])
        .expect("96e94562 update_reports_whether_it_contains_a_field invariant must hold"),
    );
    assert!(bool::from(clear_logo.has_fields()));
    assert!(bool::from(clear_logo.is_valid()));
}

#[test]
fn catalog_covers_read_and_update_wire_fields() {
    let empty_clear = crate::domain_types::AdminOptionalSettings::try_from(Vec::new())
        .expect("7f3a9c2e catalog_covers_read_and_update_wire_fields invariant must hold");
    let update = super::AdminUpdateSettingsReq::new(
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
        .expect("c84d1e6a catalog_covers_read_and_update_wire_fields invariant must hold")
        .as_object()
        .expect("49b2e7c1 catalog_covers_read_and_update_wire_fields invariant must hold")
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let setting_fields = super::AdminSetting::ALL
        .into_iter()
        .map(|setting| setting.spec().name().as_ref().to_owned())
        .collect::<std::collections::BTreeSet<_>>();
    let mut expected_update_fields = setting_fields.clone();
    let _inserted = expected_update_fields.insert(String::from("clear"));
    assert_eq!(update_fields, expected_update_fields);

    let view = super::AdminSettingsView::new(
        crate::domain_types::AdminDefaultRoute::try_from(String::from("/admin/users"))
            .expect("b6831fd4 catalog_covers_read_and_update_wire_fields invariant must hold"),
        None,
        None,
        None,
        None,
        crate::domain_types::AdminSiteName::try_from(String::from("Admin"))
            .expect("e15c7a93 catalog_covers_read_and_update_wire_fields invariant must hold"),
        None,
        None,
    );
    let view_fields = serde_json::to_value(view)
        .expect("86d4a2f9 catalog_covers_read_and_update_wire_fields invariant must hold")
        .as_object()
        .expect("21c9e5b7 catalog_covers_read_and_update_wire_fields invariant must hold")
        .keys()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(view_fields, setting_fields);
}
