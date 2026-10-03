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
fn test_optional_settings_enforce_six_clearable_fields() {
    let all = crate::admin_optional_setting::AdminOptionalSetting::ALL.to_vec();
    assert!(
        crate::admin_optional_settings::AdminOptionalSettings::try_from(all)
            .is_ok_and(|settings| settings.as_ref().len() == 6usize)
    );
    let seven = vec![crate::admin_optional_setting::AdminOptionalSetting::MainLogo; 7usize];
    assert!(matches!(
        crate::admin_optional_settings::AdminOptionalSettings::try_from(seven.clone()),
        Err(crate::admin_collection_error::AdminCollectionError::TooLong(
            bounded_types::bounded_value_error::BoundedValueError::AboveMax { actual, max }
        )) if actual.get() == 7usize && max.get() == 6usize
    ));
    assert!(
        serde_json::to_value(seven)
            .and_then(
                serde_json::from_value::<crate::admin_optional_settings::AdminOptionalSettings>
            )
            .err()
            .is_some()
    );
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
        crate::admin_data_table::AdminDataTable::RoleRules
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

#[test]
fn test_settings_update_checks_each_populated_field_and_clear_conflict() {
    assert!(
        crate::admin_setting::AdminSetting::ALL
            .into_iter()
            .all(|setting| {
                let spec = setting.spec();
                let value = match setting {
                    crate::admin_setting::AdminSetting::DefaultRoute => {
                        constants_str::VALUE_074B6E5E
                    }
                    crate::admin_setting::AdminSetting::MainLogo => constants_str::VALUE_A24910BB,
                    crate::admin_setting::AdminSetting::PrimaryColor => {
                        constants_str::VALUE_55F98A52
                    }
                    crate::admin_setting::AdminSetting::SupportUrl => constants_str::VALUE_FE4E2333,
                    crate::admin_setting::AdminSetting::SiteName
                    | crate::admin_setting::AdminSetting::TabTitle
                    | crate::admin_setting::AdminSetting::OrganizationName
                    | crate::admin_setting::AdminSetting::OrganizationContacts => {
                        constants_str::ADMIN
                    }
                };
                let mut fields = serde_json::Map::new();
                let _previous = fields.insert(
                    spec.name().as_ref().to_owned(),
                    serde_json::Value::String(value.to_owned()),
                );
                let _previous_clear = fields.insert(
                    constants_str::VALUE_913A4CB9.to_owned(),
                    serde_json::Value::Array(Vec::new()),
                );
                let Ok(request) = serde_json::from_value::<
                    crate::admin_update_settings_request::AdminUpdateSettingsRequest,
                >(serde_json::Value::Object(fields.clone())) else {
                    return false;
                };
                if !bool::from(request.has_fields()) || !bool::from(request.is_valid()) {
                    return false;
                }
                match spec.optionality() {
                    crate::admin_setting_optionality::AdminSettingOptionality::Required => true,
                    crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                        optional_setting,
                    ) => crate::admin_optional_setting::AdminOptionalSetting::ALL
                        .into_iter()
                        .all(|clear_setting| {
                            let Ok(clear_value) = serde_json::to_value([clear_setting]) else {
                                return false;
                            };
                            let mut request_fields = fields.clone();
                            let _previous_request_clear = request_fields
                                .insert(constants_str::VALUE_913A4CB9.to_owned(), clear_value);
                            serde_json::from_value::<
                                crate::admin_update_settings_request::AdminUpdateSettingsRequest,
                            >(serde_json::Value::Object(request_fields))
                            .is_ok_and(|update| {
                                bool::from(update.has_fields())
                                    && bool::from(update.is_valid())
                                        == (clear_setting != optional_setting)
                            })
                        }),
                }
            })
    );
}

#[test]
fn test_settings_update_rejects_duplicate_clear_fields_and_accepts_all_unique_fields() {
    let deserialize_clear = |optional_settings: Vec<
        crate::admin_optional_setting::AdminOptionalSetting,
    >| {
        let clear = serde_json::to_value(optional_settings)?;
        let mut fields = serde_json::Map::new();
        let _previous = fields.insert(constants_str::VALUE_913A4CB9.to_owned(), clear);
        serde_json::from_value::<crate::admin_update_settings_request::AdminUpdateSettingsRequest>(
            serde_json::Value::Object(fields),
        )
    };
    assert!(deserialize_clear(crate::admin_optional_setting::AdminOptionalSetting::ALL.to_vec())
        .is_ok_and(|request| bool::from(request.has_fields()) && bool::from(request.is_valid())));
    assert!(
        crate::admin_optional_setting::AdminOptionalSetting::ALL
            .into_iter()
            .all(|optional_setting| {
                deserialize_clear(vec![optional_setting, optional_setting]).is_ok_and(|request| {
                    bool::from(request.has_fields()) && !bool::from(request.is_valid())
                })
            })
    );
}

#[test]
fn test_settings_update_into_parts_preserves_values_and_clear_ownership() {
    assert!([true, false].into_iter().all(|populated| {
        let wire = serde_json::json!({
            (stringify!(default_admin_route)): populated.then_some(crate::admin_frontend_path::AdminFrontendPath::Users.get()),
            (stringify!(main_logo)): populated.then_some(constants_str::ADMIN_DEFAULT_MAIN_LOGO),
            (stringify!(organization_contacts)): populated.then_some(constants_str::ADMIN),
            (stringify!(organization_name)): populated.then_some(constants_str::LOGIN),
            (stringify!(primary_color)): populated.then_some(constants_str::PRIMARY_COLOR_DEFAULT),
            (stringify!(site_name)): populated.then_some(constants_str::X),
            (stringify!(support_url)): populated.then_some(constants_str::ADMIN_DEFAULT_SUPPORT_URL),
            (stringify!(tab_title)): populated.then_some(constants_str::ADMIN),
            (stringify!(clear)): if populated { Vec::new() } else { crate::admin_optional_setting::AdminOptionalSetting::ALL.to_vec() },
        });
        let fields = [stringify!(default_admin_route), stringify!(main_logo), stringify!(organization_contacts), stringify!(organization_name), stringify!(primary_color), stringify!(site_name), stringify!(support_url), stringify!(tab_title), stringify!(clear)];
        let expected = fields.into_iter().map(|field| wire.get(field).cloned()).collect::<Option<Vec<_>>>();
        expected.is_some_and(|values| {
            serde_json::from_value::<crate::admin_update_settings_request::AdminUpdateSettingsRequest>(wire).is_ok_and(|request| {
                serde_json::to_value(request.into_parts()).is_ok_and(|parts| parts == serde_json::Value::Array(values))
            })
        })
    }));
}
