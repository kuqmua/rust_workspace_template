use leptos::prelude::ElementChild;

fn render_owned_view<View>(view: View) -> String
where
    View: leptos::prelude::IntoAny,
{
    leptos::prelude::RenderHtml::to_html(leptos::prelude::IntoAny::into_any(view))
}

#[test]
fn test_sidebar_preserves_accessible_navigation_and_each_nested_item_once() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_sidebar::AdminSidebar>
            <crate::admin_sidebar_item::AdminSidebarItem><span>{constants_str::ROOT}</span></crate::admin_sidebar_item::AdminSidebarItem>
            <crate::admin_sidebar_item::AdminSidebarItem><span>{constants_str::LOGIN}</span></crate::admin_sidebar_item::AdminSidebarItem>
        </crate::admin_sidebar::AdminSidebar>
    });
    assert!(html.contains(constants_str::ADMIN_BUTTON_NAVIGATION));
    assert!(html.contains(constants_str::ADMIN_UI_ADMIN_SECTIONS));
    assert!(html.contains(stringify!(NavigationMenu)));
    assert_eq!(
        html.matches(concat!('<', stringify!(nav), ' ')).count(),
        1usize
    );
    assert_eq!(
        html.matches(concat!('<', stringify!(ul), ' ')).count(),
        1usize
    );
    assert!(
        [constants_str::ROOT, constants_str::LOGIN]
            .into_iter()
            .all(|text| {
                let expected = render_owned_view(leptos::view! { <li><span>{text}</span></li> });
                html.matches(expected.as_str()).count() == 1usize
            })
    );
}

#[test]
fn test_table_action_trigger_preserves_accessibility_and_branch_attributes() {
    assert!(server_admin_contract::admin_route_path::AdminRoutePath::try_from(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get().to_owned(),
    ).is_ok_and(|admin_route_path| {
        let expected_href = admin_route_path.to_string();
        let link = render_owned_view(leptos::view! {
            <crate::admin_table_action_trigger::AdminTableActionTrigger label=constants_str::ADMIN dialog_id=constants_str::ROOT.to_owned() href=admin_route_path><span>{constants_str::X}</span></crate::admin_table_action_trigger::AdminTableActionTrigger>
        });
        let button = render_owned_view(leptos::view! {
            <crate::admin_table_action_trigger::AdminTableActionTrigger label=constants_str::ADMIN dialog_id=constants_str::ROOT.to_owned()><span>{constants_str::X}</span></crate::admin_table_action_trigger::AdminTableActionTrigger>
        });
        [link, button].into_iter().enumerate().all(|(index, html)| {
            let expected_child = render_owned_view(leptos::view! { <span>{constants_str::X}</span> });
            html.matches(expected_child.as_str()).count() == 1usize
                && crate::admin_button_variant::AdminButtonVariant::Secondary.class().split_ascii_whitespace().take(2usize).all(|word|
                    html.split_ascii_whitespace().any(|rendered| rendered == word))
                && [
                    (concat!(stringify!(aria), '-', stringify!(label)), constants_str::ADMIN, true),
                    (stringify!(title), constants_str::ADMIN, true),
                    (stringify!(href), expected_href.as_str(), index == 0usize),
                    (stringify!(commandfor), constants_str::ROOT, index == 1usize),
                    (stringify!(command), concat!(stringify!(show), '-', stringify!(modal)), index == 1usize),
                    (stringify!(type), stringify!(button), index == 1usize),
                ].into_iter().all(|(attribute, expected, present)| {
                    let matched = html.split('"').zip(html.split('"').skip(1usize)).any(|(name, value)| {
                        name.strip_suffix('=').is_some_and(|prefix|
                            prefix.split_ascii_whitespace().last() == Some(attribute)) && value == expected
                    });
                    matched == present
                })
        })
    }));
}

#[test]
fn test_branding_details_pair_labels_and_values_for_every_optional_combination() {
    assert!((0usize..16usize).all(|mask| {
        let main_logo = (mask & 1usize != 0usize).then_some(constants_str::ADMIN_DEFAULT_MAIN_LOGO);
        let primary_color = (mask & 2usize != 0usize).then_some(constants_str::PRIMARY_COLOR_DEFAULT);
        let support_url = (mask & 4usize != 0usize).then_some(constants_str::ADMIN_DEFAULT_SUPPORT_URL);
        let tab_title = (mask & 8usize != 0usize).then_some(constants_str::ADMIN);
        let default_route = server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get();
        leptos::serde_json::from_value::<server_admin_contract::admin_branding_view::AdminBrandingView>(leptos::serde_json::json!({
            (stringify!(default_admin_route)): default_route,
            (stringify!(main_logo)): main_logo,
            (stringify!(primary_color)): primary_color,
            (stringify!(site_name)): constants_str::X,
            (stringify!(support_url)): support_url,
            (stringify!(tab_title)): tab_title,
        })).is_ok_and(|admin_branding_view| {
            let html = render_owned_view(leptos::view! {
                <crate::admin_branding_details::AdminBrandingDetails admin_branding_view=admin_branding_view />
            });
            [
                (constants_str::ADMIN_UI_SITE_NAME, constants_str::X),
                (constants_str::ADMIN_UI_TAB_TITLE, tab_title.unwrap_or(constants_str::EMPTY)),
                (constants_str::ADMIN_UI_MAIN_LOGO_URL, main_logo.unwrap_or(constants_str::EMPTY)),
                (constants_str::ADMIN_UI_PRIMARY_COLOR, primary_color.unwrap_or(constants_str::EMPTY)),
                (constants_str::ADMIN_UI_SUPPORT_URL, support_url.unwrap_or(constants_str::EMPTY)),
                (constants_str::VALUE_ACD40F02, default_route),
            ].into_iter().all(|(label, text)| {
                let expected = render_owned_view(leptos::view! {
                    <span>{label.to_owned()}</span><span>{text.to_owned()}</span>
                });
                html.matches(expected.as_str()).count() == 1usize
            })
        })
    }));
}

#[test]
fn test_card_title_optional_class_preserves_children() {
    let plain = render_owned_view(leptos::view! {
        <crate::admin_card_title::AdminCardTitle><span>{constants_str::ADMIN}</span></crate::admin_card_title::AdminCardTitle>
    });
    let styled = render_owned_view(leptos::view! {
        <crate::admin_card_title::AdminCardTitle option=constants_str::ROOT><span>{constants_str::ADMIN}</span></crate::admin_card_title::AdminCardTitle>
    });
    assert!(
        [&plain, &styled]
            .into_iter()
            .all(|html| html.contains(stringify!(CardTitle))
                && html.matches(constants_str::ADMIN).count() == 1usize)
    );
    assert!(!plain.contains(constants_str::ROOT));
    assert!(styled.split('"').any(|attribute| {
        attribute
            .split_ascii_whitespace()
            .any(|word| word == constants_str::ROOT)
    }));
}

#[test]
fn test_card_description_preserves_nested_children() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_card_description::AdminCardDescription><span>{constants_str::ADMIN}</span></crate::admin_card_description::AdminCardDescription>
    });
    assert!(html.contains(stringify!(CardDescription)));
    assert_eq!(html.matches(constants_str::ADMIN).count(), 1usize);
    assert!(html.contains(stringify!(span)));
}

#[test]
fn test_owned_singlestage_context_renders_without_an_external_owner() {
    let html = render_owned_view(crate::with_owner::with_owner(|| {
        leptos::view! { <singlestage::Popover>"Owned popover"</singlestage::Popover> }
    }));

    assert!(html.contains(constants_str::VALUE_DA70E1B6));
}

#[test]
fn test_primitives_render_semantic_accessible_markup() {
    let owned_label = crate::admin_field_label::AdminFieldLabel::from(String::from(
        constants_str::VALUE_9E41A9D1,
    ));
    assert_eq!(owned_label.as_ref(), constants_str::VALUE_9E41A9D1);
    let html = render_owned_view(leptos::view! {
        <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Settings>
            <crate::admin_card_header::AdminCardHeader><crate::admin_card_title::AdminCardTitle>"Settings"</crate::admin_card_title::AdminCardTitle></crate::admin_card_header::AdminCardHeader>
            <crate::admin_alert::AdminAlert>"Invalid value"</crate::admin_alert::AdminAlert>
        <crate::admin_field::AdminField admin_field_label="Login">
            <crate::admin_input::AdminInput admin_input_name="login" required=true />
            <singlestage::FieldDescription>"Account login"</singlestage::FieldDescription>
            <singlestage::FieldError>"Login is invalid"</singlestage::FieldError>
        </crate::admin_field::AdminField>
        <crate::admin_field::AdminField admin_field_label=String::from("Owned label")>
            <crate::admin_empty::AdminEmpty>"Owned value"</crate::admin_empty::AdminEmpty>
        </crate::admin_field::AdminField>
            <crate::admin_button::AdminButton admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button>"Save"</crate::admin_button::AdminButton>
            <crate::admin_badge::AdminBadge admin_badge_variant=crate::admin_badge_variant::AdminBadgeVariant::Success>"Active"</crate::admin_badge::AdminBadge>
            <crate::admin_textarea::AdminTextarea admin_input_name="notes" />
            <crate::admin_alert_dialog::AdminAlertDialog string=String::from("test-alert-dialog") title="Confirm action?" description="This action changes data." trigger="Delete" confirm="Confirm" callback=leptos::prelude::Callback::new(|()| {}) />
        </crate::admin_card::AdminCard>
        <crate::admin_empty::AdminEmpty>"Nothing here"</crate::admin_empty::AdminEmpty>
        <crate::admin_spinner::AdminSpinner />
    });

    assert!(html.contains(constants_str::VALUE_F1BAB7A5));
    assert!(html.contains(constants_str::VALUE_2BEB20BD));
    assert!(html.contains(constants_str::VALUE_591C6255));
    assert!(html.contains(constants_str::VALUE_747256CE));
    assert!(html.contains(constants_str::VALUE_BB0F0FC0));
    assert!(html.contains(constants_str::VALUE_118DDD9C));
    assert!(html.contains(constants_str::VALUE_DA7417C3));
    assert!(html.contains(constants_str::VALUE_88B7E010));
    assert!(html.contains(constants_str::VALUE_C849F665));
    assert!(html.contains(constants_str::VALUE_327E27AA));
    assert!(html.contains(constants_str::VALUE_8A98943D));
    assert!(html.contains(constants_str::VALUE_FA2E248C));
    assert!(html.contains(constants_str::VALUE_882C5512));
    assert!(html.contains(constants_str::VALUE_BE03D0C6));
    assert!(html.contains(constants_str::VALUE_875B5A65));
    assert!(html.contains(constants_str::VALUE_FEA2007C));
    assert!(html.contains(constants_str::VALUE_021512E6));
    assert!(html.contains(constants_str::VALUE_AAD09AEC));
    assert!(html.contains(constants_str::VALUE_67CBA746));
    assert!(html.contains(constants_str::VALUE_82A744A6));
    assert!(html.contains(constants_str::VALUE_1BEF2C87));
    assert!(html.contains(constants_str::VALUE_3B0D4158));
    assert!(html.contains(constants_str::VALUE_345DE32F));
    assert!(html.contains(constants_str::VALUE_BB10EDD8));
    assert!(html.contains(constants_str::VALUE_F7F92547));
    assert!(html.contains(constants_str::VALUE_4BE79CDF));
    assert!(html.contains(constants_str::VALUE_E10EAC29));
    assert!(html.contains(constants_str::VALUE_469D8B78));
    assert!(html.contains(constants_str::VALUE_407E5FF2));
    assert!(html.contains(constants_str::VALUE_FAE48E86));
    assert!(html.contains(constants_str::VALUE_F8CB664C));
    assert!(html.contains(constants_str::ADMIN_UI_LOADING));
    assert!(html.contains(constants_str::VALUE_706A5FC3));
}

#[test]
fn test_button_callback_preserves_server_markup_without_running_during_render() {
    let callback_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0usize));
    let render_button = |bool| {
        render_owned_view(crate::with_owner::with_owner(|| {
            let callback_count = std::sync::Arc::clone(&callback_count);
            let callback = leptos::prelude::Callback::new(move |_event| {
                let _previous =
                    callback_count.fetch_add(1usize, std::sync::atomic::Ordering::Relaxed);
            });
            if bool {
                leptos::view! {
                <crate::admin_button::AdminButton
                    admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Danger
                    bool=true
                    form=constants_str::LOGIN.to_owned()
                    on_click=callback
                >{constants_str::ADMIN_BUTTON_SAVE_CHANGES}</crate::admin_button::AdminButton>
                }
            } else {
                leptos::view! {
                    <crate::admin_button::AdminButton
                        admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Danger
                        bool=true
                        form=constants_str::LOGIN.to_owned()
                    >{constants_str::ADMIN_BUTTON_SAVE_CHANGES}</crate::admin_button::AdminButton>
                }
            }
        }))
    };
    assert_eq!(render_button(true), render_button(false));
    assert_eq!(
        callback_count.load(std::sync::atomic::Ordering::Relaxed),
        0usize
    );
}

#[test]
fn test_button_variants_preserve_native_control_attributes() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_button::AdminButton bool=true>"Primary"</crate::admin_button::AdminButton>
        <crate::admin_button::AdminButton
            admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary
            admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button
            popover_target=String::from("filters")
            popover_target_action="hide"
            aria_label=String::from("Close filters")
            style=String::from("width:100%")
        >
            "Secondary"
        </crate::admin_button::AdminButton>
        <crate::admin_button::AdminButton
            admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Danger
            admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button
            command_for=String::from("confirmation")
            command="show-modal"
        >
            "Danger"
        </crate::admin_button::AdminButton>
    });

    assert!(html.contains(constants_str::VALUE_67CBA746));
    assert!(html.contains(constants_str::VALUE_97EF114C));
    assert!(html.contains(constants_str::VALUE_24B9818D));
    assert!(html.contains(constants_str::VALUE_82A744A6));
    assert!(html.contains(constants_str::VALUE_6CBC6F44));
    assert!(html.contains(constants_str::VALUE_1C61CF88));
    assert!(html.contains(constants_str::VALUE_0BA00E46));
    assert!(html.contains(constants_str::VALUE_C6E0E94D));
    assert!(html.contains(constants_str::VALUE_00F2810E));
    assert!(html.contains(constants_str::VALUE_EC530B9C));
    assert!(html.contains(constants_str::VALUE_3B0143B5));
}

#[test]
fn test_form_controls_render_every_supported_kind_and_constraint() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_input::AdminInput
            admin_input_name="login"
            autocomplete="username"
            required=true
            minlength=2
            maxlength=32
            initial_value=String::from("alice")
        />
        <crate::admin_input::AdminInput
            admin_input_name="password"
            admin_input_kind=crate::admin_input_kind::AdminInputKind::Password
            required=true
        />
        <crate::admin_input::AdminInput
            admin_input_name="limit"
            admin_input_kind=crate::admin_input_kind::AdminInputKind::Number
            min=1
            max=100
        />
        <crate::admin_input::AdminInput admin_input_name="url" admin_input_kind=crate::admin_input_kind::AdminInputKind::Url />
        <crate::admin_textarea::AdminTextarea admin_input_name="notes" required=true disabled=true />
        <crate::admin_checkbox::AdminCheckbox name="confirmation" value="true" bool=true />
    });

    assert!(html.contains(constants_str::VALUE_AAD09AEC));
    assert!(html.contains(constants_str::VALUE_26B901BB));
    assert!(html.contains(constants_str::VALUE_7679AE45));
    assert!(html.contains(constants_str::VALUE_5E04A048));
    assert!(html.contains(constants_str::VALUE_CD633D03));
    assert!(html.contains(constants_str::VALUE_022ECEBF));
    assert!(html.contains(constants_str::VALUE_AFCBB462));
    assert!(html.contains(constants_str::VALUE_75D9FED9));
    assert!(html.contains(constants_str::VALUE_0AA8ABD0));
    assert!(html.contains(constants_str::VALUE_C7A9349A));
    assert!(html.contains(constants_str::VALUE_3901EFC3));
    assert!(html.contains(constants_str::VALUE_B1CE91DB));
    assert!(html.contains(constants_str::VALUE_416538A8));
    assert!(html.contains(constants_str::VALUE_3F96A519));
    assert!(html.contains(constants_str::VALUE_345DE32F));
    assert!(html.contains(constants_str::VALUE_F7F92547));
    assert!(html.contains(constants_str::VALUE_94160202));
    assert!(html.contains(constants_str::VALUE_7A05DAEA));
    assert!(html.contains(constants_str::VALUE_97F214A2));
}

#[test]
fn test_bound_form_controls_render_signal_values() {
    let owner = leptos::prelude::Owner::new();
    let html = owner.with(|| {
        let input = crate::leptos_admin_input_signal::LeptosAdminInputSignal::from(
            leptos::prelude::RwSignal::new(String::from(constants_str::VALUE_14527724)),
        );
        let textarea = crate::leptos_admin_input_signal::LeptosAdminInputSignal::from(
            leptos::prelude::RwSignal::new(String::from(constants_str::VALUE_F013164D)),
        );
        render_owned_view(leptos::view! {
            <crate::admin_input::AdminInput admin_input_name="bound_input" bind_value=input min=1u16 max=10u16 />
            <crate::admin_textarea::AdminTextarea admin_input_name="bound_textarea" option=textarea />
        })
    });

    assert!(html.contains(constants_str::VALUE_89410775));
    assert!(html.contains(constants_str::VALUE_14CE4117));
    assert!(html.contains(constants_str::VALUE_C34F2EC8));
    assert!(html.contains(constants_str::VALUE_3901EFC3));
    assert!(html.contains(constants_str::ADMIN_INPUT_MAXIMUM_TEN_ATTRIBUTE_FIXTURE));
}

#[test]
fn test_visual_variants_keep_their_rust_ui_contracts() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_alert::AdminAlert admin_alert_variant=crate::admin_alert_variant::AdminAlertVariant::Success option="saved">"Saved"</crate::admin_alert::AdminAlert>
        <crate::admin_badge::AdminBadge>"Neutral"</crate::admin_badge::AdminBadge>
        <crate::admin_badge::AdminBadge admin_badge_variant=crate::admin_badge_variant::AdminBadgeVariant::Success>"Success"</crate::admin_badge::AdminBadge>
        <crate::admin_card::AdminCard>"Default"</crate::admin_card::AdminCard>
        <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Auth>"Auth"</crate::admin_card::AdminCard>
        <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Code>"Code"</crate::admin_card::AdminCard>
        <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Profile>"Profile"</crate::admin_card::AdminCard>
        <crate::admin_card::AdminCard admin_card_variant=crate::admin_card_variant::AdminCardVariant::Security>"Security"</crate::admin_card::AdminCard>
    });

    assert!(html.contains(constants_str::VALUE_6F0EA044));
    assert!(html.contains(constants_str::VALUE_1030993B));
    assert!(html.contains(constants_str::VALUE_5EB5CE93));
    assert!(html.contains(constants_str::VALUE_26205B45));
    assert!(html.contains(constants_str::VALUE_3B0D4158));
    assert!(html.contains(constants_str::VALUE_9F36484B));
    assert!(html.contains(constants_str::VALUE_45162E60));
    assert!(html.contains(constants_str::VALUE_DDC1D093));
    assert!(html.contains(constants_str::VALUE_E3D56718));
    assert!(html.contains(constants_str::VALUE_ABED9301));
}

#[test]
fn test_navigation_distinguishes_current_and_inactive_destinations() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_navigation_link::AdminNavigationLink string=String::from("/admin/users") bool=true>"Users"</crate::admin_navigation_link::AdminNavigationLink>
        <crate::admin_navigation_link::AdminNavigationLink string=String::from("/admin/roles") bool=false>"Roles"</crate::admin_navigation_link::AdminNavigationLink>
    });

    assert!(html.contains(constants_str::VALUE_9938B0AD));
    assert!(html.contains(constants_str::VALUE_A8416C94));
    assert!(html.contains(constants_str::VALUE_5850635E));
    assert!(html.contains(constants_str::VALUE_A6A17075));
    assert!(html.contains(constants_str::VALUE_2977CF92));
    assert!(html.contains(constants_str::VALUE_BB931721));
    assert_eq!(html.matches(constants_str::VALUE_5850635E).count(), 1);
}

#[test]
fn test_table_primitives_preserve_structure_and_class_merging() {
    let html = render_owned_view(leptos::view! {
        <crate::table_wrapper::TableWrapper>
            <crate::table::Table>
                <crate::table_caption::TableCaption>"Identifiers"</crate::table_caption::TableCaption>
                <crate::table_header::TableHeader>
                    <crate::table_row::TableRow>
                        <crate::table_head::TableHead>"Identifier"</crate::table_head::TableHead>
                    </crate::table_row::TableRow>
                </crate::table_header::TableHeader>
                <crate::table_body::TableBody>
                    <crate::table_row::TableRow>
                        <crate::table_cell::TableCell class="numeric-cell">"42"</crate::table_cell::TableCell>
                    </crate::table_row::TableRow>
                </crate::table_body::TableBody>
                <crate::table_footer::TableFooter>
                    <crate::table_row::TableRow><crate::table_cell::TableCell>"1"</crate::table_cell::TableCell></crate::table_row::TableRow>
                </crate::table_footer::TableFooter>
            </crate::table::Table>
        </crate::table_wrapper::TableWrapper>
    });

    let markers = [
        constants_str::VALUE_846B8D6B,
        constants_str::VALUE_6A98499E,
        constants_str::VALUE_737E03AE,
        constants_str::VALUE_31819FEE,
        constants_str::VALUE_8886AF1E,
        constants_str::VALUE_8925FFE7,
    ];
    assert!(
        markers
            .into_iter()
            .all(|marker| html.matches(marker).count() == 1usize)
    );
    let positions = markers.map(|marker| html.find(marker));
    assert!(
        positions
            .windows(2usize)
            .all(|pair| { matches!(pair, [Some(left), Some(right)] if left < right) })
    );
    assert_eq!(html.matches(constants_str::VALUE_38C2F107).count(), 3);
    assert!(html.contains(constants_str::VALUE_80DFAFAE));
}

#[test]
fn test_alert_dialog_wires_singlestage_trigger_and_dialog_forms() {
    let render_dialog = |disabled, dialog_only| {
        render_owned_view(leptos::view! {
            <crate::admin_alert_dialog::AdminAlertDialog
                string=String::from("delete-dialog")
                title="Delete item?"
                description="The item will be removed."
                trigger="Delete"
                confirm="Confirm"
                bool=disabled
                dialog_only=dialog_only
                callback=leptos::prelude::Callback::new(|()| {})
            />
        })
    };

    let disabled = render_dialog(true, false);
    assert!(disabled.contains(constants_str::VALUE_17EB3C01));
    assert!(!disabled.contains(constants_str::VALUE_64474E4B));
    assert!(!disabled.contains(constants_str::VALUE_AB29C21D));

    let html = render_dialog(false, false);
    assert!(html.contains(constants_str::VALUE_AB29C21D));
    assert!(html.contains(constants_str::VALUE_5AADB989));
    assert_eq!(html.matches(constants_str::VALUE_65D07A5E).count(), 1);
    assert!(html.contains(constants_str::VALUE_706A5FC3));
    assert!(html.contains(constants_str::VALUE_C1451BBC));
    assert!(html.contains(constants_str::VALUE_3B0143B5));

    let dialog_only = render_dialog(false, true);
    assert!(dialog_only.contains(constants_str::VALUE_AB29C21D));
    assert_eq!(
        dialog_only.matches(constants_str::VALUE_65D07A5E).count(),
        1
    );
    assert!(dialog_only.contains(constants_str::VALUE_706A5FC3));
    assert!(!dialog_only.contains(constants_str::VALUE_C1451BBC));
    assert!(dialog_only.contains(constants_str::ADMIN_BUTTON_CANCEL));
    assert!(!dialog_only.contains(constants_str::VALUE_3B0143B5));

    assert_eq!(render_dialog(true, true), disabled);
}

#[test]
fn test_table_actions_render_read_and_revoke_in_one_row() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_table_actions::AdminTableActions
            read_action=server_admin_contract::admin_route_path::AdminRoutePath::try_from(String::from("/admin/access_sessions/example")).ok().map(|read_path| leptos::prelude::IntoAny::into_any(leptos::view! {
                <crate::admin_read_action::AdminReadAction read_path=read_path><span>"Session details"</span></crate::admin_read_action::AdminReadAction>
            }))
            command_for=String::from("session-action-dialog")
        >
            <span>{constants_str::ADMIN_BUTTON_CANCEL}</span>
        </crate::admin_table_actions::AdminTableActions>
    });

    assert!(html.contains(constants_str::PG_CRUD_READ_RULE_ACTION));
    assert!(html.contains(constants_str::ADMIN_BUTTON_REVOKE_SESSION));
    assert!(
        html.find(constants_str::PG_CRUD_READ_RULE_ACTION)
            < html.find(constants_str::ADMIN_BUTTON_REVOKE_SESSION)
    );
    assert_eq!(html.matches(constants_str::VALUE_24B9818D).count(), 2);
    assert!(html.contains(constants_str::ADMIN_BUTTON_CLOSE));
    let revoke_only = render_owned_view(leptos::view! {
        <crate::admin_table_actions::AdminTableActions read_action=None command_for=constants_str::ROOT.to_owned()>
            <span>{constants_str::ADMIN_BUTTON_CANCEL}</span>
        </crate::admin_table_actions::AdminTableActions>
    });
    assert!(!revoke_only.contains(constants_str::PG_CRUD_READ_RULE_ACTION));
    assert_eq!(
        revoke_only
            .matches(concat!('<', stringify!(button), ' '))
            .count(),
        1usize
    );
    assert_eq!(
        revoke_only
            .matches(constants_str::ADMIN_BUTTON_REVOKE_SESSION)
            .count(),
        2usize
    );
    assert_eq!(
        revoke_only
            .matches(constants_str::ADMIN_BUTTON_CANCEL)
            .count(),
        1usize
    );
    assert!(
        revoke_only
            .split('"')
            .zip(revoke_only.split('"').skip(1usize))
            .any(|(name, value)| {
                name.strip_suffix('=').is_some_and(|prefix| {
                    prefix.split_ascii_whitespace().last() == Some(stringify!(commandfor))
                }) && value == constants_str::ROOT
            })
    );
}

#[test]
fn test_numeric_input_preserves_optional_minimum_and_maximum_attributes() {
    let html = render_owned_view(crate::with_owner::with_owner(|| {
        leptos::view! {
            <crate::admin_input::AdminInput
                admin_input_name=crate::admin_input_name::AdminInputName::from(constants_str::LOGIN)
                admin_input_kind=crate::admin_input_kind::AdminInputKind::Number
                min=1u16
                max=10u16
            />
        }
    }));
    assert!(html.contains(constants_str::VALUE_3901EFC3));
    assert!(html.contains(constants_str::ADMIN_INPUT_MAXIMUM_TEN_ATTRIBUTE_FIXTURE));
}

#[test]
fn test_button_links_preserve_typed_destinations_and_child_text_as_anchors() {
    assert!([
        (
            server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersCreate,
            crate::admin_button_variant::AdminButtonVariant::Primary,
            constants_str::PG_CRUD_CREATE_RULE_ACTION,
        ),
        (
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
            crate::admin_button_variant::AdminButtonVariant::Secondary,
            constants_str::ADMIN_BUTTON_BACK_TO_ROLES,
        ),
    ].into_iter().all(|(admin_frontend_path, admin_button_variant, text)| {
        let route = admin_frontend_path.get();
        let html = render_owned_view(leptos::view! {
            <crate::admin_button_link::AdminButtonLink str=route admin_button_variant=admin_button_variant>{text}</crate::admin_button_link::AdminButtonLink>
        });
        html.contains(format!("href=\"{route}\"").as_str())
            && html.contains(text)
            && html.ends_with(constants_str::VALUE_ECD5B806)
    }));
}

#[test]
fn test_checkbox_default_and_explicit_required_flags_preserve_native_input_attributes() {
    assert!([None, Some(false), Some(true)].into_iter().all(|option| {
        let html = option.map_or_else(
            || render_owned_view(leptos::view! {
                <crate::admin_checkbox::AdminCheckbox name=stringify!(confirmation) value=constants_str::TRUE />
            }),
            |bool| render_owned_view(leptos::view! {
                <crate::admin_checkbox::AdminCheckbox name=stringify!(confirmation) value=constants_str::TRUE bool=bool />
            }),
        );
        let prefix = format!("<{}", stringify!(input));
        html.split('>').find(|text| text.starts_with(prefix.as_str())).is_some_and(|input| {
            input.contains(constants_str::VALUE_7A05DAEA)
                && input.contains(constants_str::VALUE_97F214A2)
                && input.contains(format!("type=\"{}\"", stringify!(checkbox)).as_str())
                && input.split_ascii_whitespace().any(|attribute| {
                    attribute.split_once('=').map_or(attribute, |(name, _value)| name) == constants_str::REQUIRED
                }) == (option == Some(true))
        })
    }));
}

#[test]
fn test_read_action_selects_record_link_or_fallback_dialog() {
    assert!(server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
        .is_ok_and(|admin_user_id| {
            let record = server_admin_contract::admin_route_path::AdminRoutePath::from(admin_user_id);
            server_admin_contract::admin_route_path::AdminRoutePath::try_from(constants_str::ROOT.to_owned())
                .is_ok_and(|fallback| {
                    [record, fallback].into_iter().enumerate().all(|(index, read_path)| {
                        let expected_path = read_path.to_string();
                        let expected_dialog = format!("{}-{expected_path}", constants_str::PG_CRUD_READ_RULE_ACTION);
                        let html = render_owned_view(leptos::view! {
                            <crate::admin_read_action::AdminReadAction read_path=read_path><span>{constants_str::X}</span></crate::admin_read_action::AdminReadAction>
                        });
                        let expected_child = render_owned_view(leptos::view! { <span>{constants_str::X}</span> });
                        html.matches(expected_child.as_str()).count() == index
                            && html.matches(concat!('<', stringify!(dialog), ' ')).count() == index
                            && [
                                (stringify!(href), expected_path.as_str(), index == 0usize),
                                (stringify!(id), expected_dialog.as_str(), index == 1usize),
                                (stringify!(commandfor), expected_dialog.as_str(), index == 1usize),
                                (stringify!(command), concat!(stringify!(show), '-', stringify!(modal)), index == 1usize),
                                (stringify!(command), constants_str::ADMIN_BUTTON_CLOSE, index == 1usize),
                                (concat!(stringify!(aria), '-', stringify!(label)), constants_str::PG_CRUD_READ_RULE_ACTION, true),
                            ].into_iter().all(|(attribute, expected, present)| {
                                html.split('"').zip(html.split('"').skip(1usize)).any(|(name, value)| {
                                    name.strip_suffix('=').is_some_and(|prefix|
                                        prefix.split_ascii_whitespace().last() == Some(attribute)) && value == expected
                                }) == present
                            })
                    })
                })
        }));
}
