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
                    let isolated = server_admin_contract::admin_setting::AdminSetting::ALL.into_iter().all(|setting| {
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
                    });
                    isolated && [false, true].into_iter().all(|disabled| {
                        let html = leptos::prelude::RenderHtml::to_html(leptos::prelude::IntoAny::into_any(
                            crate::admin_setting_inputs::admin_setting_inputs(signals,
                                crate::admin_setting_disabled::AdminSettingDisabled::from(disabled)),
                        ));
                        server_admin_contract::admin_setting::AdminSetting::ALL.into_iter().all(|setting| {
                            let spec = setting.spec();
                            let setting_name = spec.name();
                            let mut tags = html.split('<').filter_map(|fragment| fragment.split_once('>').map(|(tag, _rest)| tag))
                                .filter(|tag| tag.starts_with(concat!(stringify!(input), ' ')) || tag.starts_with(concat!(stringify!(textarea), ' ')))
                                .filter(|tag| tag.split('"').zip(tag.split('"').skip(1usize)).any(|(attribute, value)|
                                    attribute.strip_suffix('=').is_some_and(|prefix| prefix.split_ascii_whitespace().last() == Some(stringify!(name)))
                                        && value == setting_name.as_ref()));
                            tags.next().is_some_and(|tag| {
                                [
                                    (stringify!(disabled), disabled),
                                    (constants_str::REQUIRED, bool::from(spec.required())),
                                ].into_iter().all(|(flag_name, expected)|
                                    tag.split_ascii_whitespace().any(|attribute|
                                        attribute.split_once('=').map_or(attribute, |(attribute_name, _value)| attribute_name) == flag_name) == expected)
                            }) && tags.next().is_none()
                        })
                    })
                })
            })
    }));
}
