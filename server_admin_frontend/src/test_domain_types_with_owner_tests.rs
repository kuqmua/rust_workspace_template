fn render_owned_view<View>(view: View) -> String
where
    View: leptos::prelude::IntoAny,
{
    leptos::prelude::RenderHtml::to_html(leptos::prelude::IntoAny::into_any(view))
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
        <crate::admin_card::AdminCard variant=crate::admin_card_variant::AdminCardVariant::Settings>
            <crate::admin_card_header::AdminCardHeader><crate::admin_card_title::AdminCardTitle>"Settings"</crate::admin_card_title::AdminCardTitle></crate::admin_card_header::AdminCardHeader>
            <crate::admin_alert::AdminAlert>"Invalid value"</crate::admin_alert::AdminAlert>
        <crate::admin_field::AdminField label="Login">
            <crate::admin_input::AdminInput name="login" required=true />
            <singlestage::FieldDescription>"Account login"</singlestage::FieldDescription>
            <singlestage::FieldError>"Login is invalid"</singlestage::FieldError>
        </crate::admin_field::AdminField>
        <crate::admin_field::AdminField label=String::from("Owned label")>
            <crate::admin_empty::AdminEmpty>"Owned value"</crate::admin_empty::AdminEmpty>
        </crate::admin_field::AdminField>
            <crate::admin_button::AdminButton kind=crate::admin_button_kind::AdminButtonKind::Button>"Save"</crate::admin_button::AdminButton>
            <crate::admin_badge::AdminBadge variant=crate::admin_badge_variant::AdminBadgeVariant::Success>"Active"</crate::admin_badge::AdminBadge>
            <crate::admin_textarea::AdminTextarea name="notes" />
            <crate::admin_alert_dialog::AdminAlertDialog id=String::from("test-alert-dialog") title="Confirm action?" description="This action changes data." trigger="Delete" confirm="Confirm" on_confirm=leptos::prelude::Callback::new(|()| {}) />
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
    assert!(html.contains(constants_str::VALUE_BA3BBBE1));
    assert!(html.contains(constants_str::VALUE_64474E4B));
    assert!(html.contains(constants_str::VALUE_6BE3FB1C));
    assert!(html.contains(constants_str::VALUE_67B26491));
    assert!(html.contains(constants_str::VALUE_762CF6CB));
    assert!(html.contains(constants_str::VALUE_48CAB863));
    assert!(html.contains(constants_str::VALUE_1D79EA4F));
}

#[test]
fn test_button_variants_preserve_native_control_attributes() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_button::AdminButton disabled=true>"Primary"</crate::admin_button::AdminButton>
        <crate::admin_button::AdminButton
            variant=crate::admin_button_variant::AdminButtonVariant::Secondary
            kind=crate::admin_button_kind::AdminButtonKind::Button
            popover_target=String::from("filters")
            popover_target_action="hide"
            aria_label=String::from("Close filters")
            style=String::from("width:100%")
        >
            "Secondary"
        </crate::admin_button::AdminButton>
        <crate::admin_button::AdminButton
            variant=crate::admin_button_variant::AdminButtonVariant::Danger
            kind=crate::admin_button_kind::AdminButtonKind::Button
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
            name="login"
            autocomplete="username"
            required=true
            minlength=2
            maxlength=32
            initial_value=String::from("alice")
        />
        <crate::admin_input::AdminInput
            name="password"
            kind=crate::admin_input_kind::AdminInputKind::Password
            disabled=true
        />
        <crate::admin_input::AdminInput
            name="limit"
            kind=crate::admin_input_kind::AdminInputKind::Number
            min=1
            max=100
        />
        <crate::admin_input::AdminInput name="url" kind=crate::admin_input_kind::AdminInputKind::Url />
        <crate::admin_textarea::AdminTextarea name="notes" required=true disabled=true />
        <crate::admin_checkbox::AdminCheckbox name="confirmation" value="true" required=true />
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
            <crate::admin_input::AdminInput name="bound_input" bind_value=input />
            <crate::admin_textarea::AdminTextarea name="bound_textarea" bind_value=textarea />
        })
    });

    assert!(html.contains(constants_str::VALUE_89410775));
    assert!(html.contains(constants_str::VALUE_14CE4117));
    assert!(html.contains(constants_str::VALUE_C34F2EC8));
}

#[test]
fn test_visual_variants_keep_their_rust_ui_contracts() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_alert::AdminAlert variant=crate::admin_alert_variant::AdminAlertVariant::Success id="saved">"Saved"</crate::admin_alert::AdminAlert>
        <crate::admin_badge::AdminBadge>"Neutral"</crate::admin_badge::AdminBadge>
        <crate::admin_badge::AdminBadge variant=crate::admin_badge_variant::AdminBadgeVariant::Success>"Success"</crate::admin_badge::AdminBadge>
        <crate::admin_card::AdminCard>"Default"</crate::admin_card::AdminCard>
        <crate::admin_card::AdminCard variant=crate::admin_card_variant::AdminCardVariant::Auth>"Auth"</crate::admin_card::AdminCard>
        <crate::admin_card::AdminCard variant=crate::admin_card_variant::AdminCardVariant::Code>"Code"</crate::admin_card::AdminCard>
        <crate::admin_card::AdminCard variant=crate::admin_card_variant::AdminCardVariant::Profile>"Profile"</crate::admin_card::AdminCard>
        <crate::admin_card::AdminCard variant=crate::admin_card_variant::AdminCardVariant::Security>"Security"</crate::admin_card::AdminCard>
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
        <crate::admin_navigation_link::AdminNavigationLink href=String::from("/admin/users") active=true>"Users"</crate::admin_navigation_link::AdminNavigationLink>
        <crate::admin_navigation_link::AdminNavigationLink href=String::from("/admin/roles") active=false>"Roles"</crate::admin_navigation_link::AdminNavigationLink>
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

    assert!(html.contains(constants_str::VALUE_846B8D6B));
    assert!(html.contains(constants_str::VALUE_6A98499E));
    assert!(html.contains(constants_str::VALUE_31819FEE));
    assert!(html.contains(constants_str::VALUE_8886AF1E));
    assert!(html.contains(constants_str::VALUE_737E03AE));
    assert!(html.contains(constants_str::VALUE_8925FFE7));
    assert_eq!(html.matches(constants_str::VALUE_38C2F107).count(), 3);
    assert!(html.contains(constants_str::VALUE_80DFAFAE));
}

#[test]
fn test_alert_dialog_wires_singlestage_trigger_and_dialog_forms() {
    let html = render_owned_view(leptos::view! {
        <crate::admin_alert_dialog::AdminAlertDialog
            id=String::from("delete-dialog")
            title="Delete item?"
            description="The item will be removed."
            trigger="Delete"
            confirm="Confirm"
            disabled=true
            on_confirm=leptos::prelude::Callback::new(|()| {})
        />
    });

    assert!(html.contains(constants_str::VALUE_17EB3C01));
    assert!(html.contains(constants_str::VALUE_AB29C21D));
    assert!(html.contains(constants_str::VALUE_64474E4B));
    assert!(html.contains(constants_str::VALUE_5AADB989));
    assert_eq!(html.matches(constants_str::VALUE_65D07A5E).count(), 1);
    assert!(html.contains(constants_str::VALUE_67B26491));
    assert!(html.contains(constants_str::VALUE_762CF6CB));
    assert!(html.contains(constants_str::VALUE_48CAB863));
    assert!(html.contains(constants_str::VALUE_1D79EA4F));
    assert!(!html.contains(constants_str::VALUE_C1451BBC));
}
