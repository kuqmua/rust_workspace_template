#[test]
fn test_settings_signals_preserve_defaults_and_isolate_edits() {
    assert!(server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get().to_owned(),
    ).is_ok_and(|default_admin_route| {
        server_admin_contract::admin_site_name::AdminSiteName::try_from(constants_str::ADMIN.to_owned())
            .is_ok_and(|admin_site_name| {
                let settings = server_admin_contract::admin_settings_view::AdminSettingsView::new(
                    default_admin_route, None, None, None, None, admin_site_name, None, None,
                );
                let values = crate::admin_settings_form_values::AdminSettingsFormValues::from(&settings);
                let owner = leptos::prelude::Owner::new();
                owner.with(|| {
                    let signals = crate::admin_settings_form_signals::AdminSettingsFormSignals::new(&values);
                    let independent = crate::admin_settings_form_signals::AdminSettingsFormSignals::new(&values);
                    server_admin_contract::admin_setting::AdminSetting::ALL.into_iter().all(|setting| {
                        let expected = match setting {
                            server_admin_contract::admin_setting::AdminSetting::DefaultRoute => settings.default_admin_route().as_ref(),
                            server_admin_contract::admin_setting::AdminSetting::SiteName => constants_str::ADMIN,
                            server_admin_contract::admin_setting::AdminSetting::MainLogo
                            | server_admin_contract::admin_setting::AdminSetting::OrganizationContacts
                            | server_admin_contract::admin_setting::AdminSetting::OrganizationName
                            | server_admin_contract::admin_setting::AdminSetting::PrimaryColor
                            | server_admin_contract::admin_setting::AdminSetting::SupportUrl
                            | server_admin_contract::admin_setting::AdminSetting::TabTitle => constants_str::EMPTY,
                        };
                        assert_eq!(values.get(setting).as_ref(), expected);
                        assert_eq!(leptos::prelude::Get::get(&signals.get(setting).signal()), expected);
                        leptos::prelude::Set::set(&signals.get(setting).signal(), constants_str::ROOT.to_owned());
                        let isolated = server_admin_contract::admin_setting::AdminSetting::ALL.into_iter().all(|other_setting| {
                            leptos::prelude::Get::get(&independent.get(other_setting).signal()) == values.get(other_setting).as_ref()
                                && leptos::prelude::Get::get(&signals.get(other_setting).signal()) == if other_setting == setting {
                                    constants_str::ROOT
                                } else {
                                    values.get(other_setting).as_ref()
                                }
                        });
                        leptos::prelude::Set::set(&signals.get(setting).signal(), expected.to_owned());
                        isolated && values.get(setting).as_ref() == expected
                    })
                })
            })
    }));
}
