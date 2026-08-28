fn render_owned_view<View>(view: View) -> String
where
    View: leptos::prelude::IntoAny,
{
    leptos::prelude::RenderHtml::to_html(leptos::prelude::IntoAny::into_any(view))
}

#[test]
fn owned_singlestage_context_renders_without_an_external_owner() {
    let html = render_owned_view(crate::with_owner::with_owner(|| {
        leptos::view! { <singlestage::Popover>"Owned popover"</singlestage::Popover> }
    }));

    assert!(html.contains("Owned popover"));
}

#[test]
fn primitives_render_semantic_accessible_markup() {
    let owned_label = crate::with_owner::field::AdminFieldLabel::from(String::from(
        constants_str::VALUE_9E41A9D1,
    ));
    assert_eq!(owned_label.as_ref(), "Owned label");
    let html = render_owned_view(leptos::view! {
        <crate::with_owner::card::AdminCard variant=crate::with_owner::card::AdminCardVariant::Settings>
            <crate::with_owner::card::AdminCardHeader><crate::with_owner::card::AdminCardTitle>"Settings"</crate::with_owner::card::AdminCardTitle></crate::with_owner::card::AdminCardHeader>
            <crate::with_owner::alert::AdminAlert>"Invalid value"</crate::with_owner::alert::AdminAlert>
        <crate::with_owner::field::AdminField label="Login">
            <crate::with_owner::input::AdminInput name="login" required=true />
            <singlestage::FieldDescription>"Account login"</singlestage::FieldDescription>
            <singlestage::FieldError>"Login is invalid"</singlestage::FieldError>
        </crate::with_owner::field::AdminField>
        <crate::with_owner::field::AdminField label=String::from("Owned label")>
            <crate::with_owner::admin_empty::AdminEmpty>"Owned value"</crate::with_owner::admin_empty::AdminEmpty>
        </crate::with_owner::field::AdminField>
            <crate::with_owner::button::AdminButton kind=crate::with_owner::button::AdminButtonKind::Button>"Save"</crate::with_owner::button::AdminButton>
            <crate::with_owner::badge::AdminBadge variant=crate::with_owner::badge::AdminBadgeVariant::Success>"Active"</crate::with_owner::badge::AdminBadge>
            <crate::with_owner::admin_textarea::AdminTextarea name="notes" />
            <crate::with_owner::admin_alert_dialog::AdminAlertDialog id=String::from("test-alert-dialog") title="Confirm action?" description="This action changes data." trigger="Delete" confirm="Confirm" on_confirm=leptos::prelude::Callback::new(|()| {}) />
        </crate::with_owner::card::AdminCard>
        <crate::with_owner::admin_empty::AdminEmpty>"Nothing here"</crate::with_owner::admin_empty::AdminEmpty>
        <crate::with_owner::admin_spinner::AdminSpinner />
    });

    assert!(html.contains("data-name=\"Card\""));
    assert!(html.contains("class=\"ui-card settings-card "));
    assert!(html.contains("data-name=\"CardContent\" class=\"px-6\""));
    assert!(html.contains("data-name=\"CardHeader\""));
    assert!(html.contains("data-name=\"CardTitle\""));
    assert!(html.contains("data-name=\"Alert\""));
    assert!(html.contains("class=\"ui-alert field-error "));
    assert!(html.contains("role=\"alert\""));
    assert!(html.contains("data-name=\"Field\""));
    assert!(html.contains("ui-field "));
    assert!(html.contains("data-name=\"Label\""));
    assert!(html.contains("singlestage-field-description"));
    assert!(html.contains("singlestage-field-error"));
    assert!(html.contains("<span>Login</span>"));
    assert!(html.contains("<span>Owned label</span>"));
    assert!(html.contains("data-name=\"Input\""));
    assert!(html.contains("class=\"ui-input "));
    assert!(html.contains("name=\"login\""));
    assert!(html.contains("ui-button ui-button-primary "));
    assert!(html.contains("type=\"button\""));
    assert!(html.contains("data-name=\"Badge\""));
    assert!(html.contains("class=\"ui-badge ui-badge-success "));
    assert!(html.contains("data-name=\"Textarea\""));
    assert!(html.contains("class=\"ui-textarea "));
    assert!(html.contains("name=\"notes\""));
    assert!(html.contains("data-name=\"Empty\""));
    assert!(html.contains("data-name=\"EmptyHeader\""));
    assert!(html.contains("data-name=\"EmptyTitle\""));
    assert!(html.contains("class=\"ui-empty empty-state "));
    assert!(html.contains("role=\"status\" aria-live=\"polite\""));
    assert!(html.contains("data-name=\"Spinner\""));
    assert!(html.contains("Loading\u{2026}"));
    assert!(html.contains("data-name=\"AlertDialogContent\""));
    assert!(html.contains("data-name=\"AlertDialogBody\""));
    assert!(html.contains("data-name=\"AlertDialogHeader\""));
    assert!(html.contains("data-name=\"AlertDialogTitle\""));
    assert!(html.contains("data-name=\"AlertDialogDescription\""));
    assert!(html.contains("data-name=\"AlertDialogFooter\""));
}

#[test]
fn button_variants_preserve_native_control_attributes() {
    let html = render_owned_view(leptos::view! {
        <crate::with_owner::button::AdminButton disabled=true>"Primary"</crate::with_owner::button::AdminButton>
        <crate::with_owner::button::AdminButton
            variant=crate::with_owner::button::AdminButtonVariant::Secondary
            kind=crate::with_owner::button::AdminButtonKind::Button
            popover_target=String::from("filters")
            popover_target_action="hide"
            aria_label=String::from("Close filters")
            style=String::from("width:100%")
        >
            "Secondary"
        </crate::with_owner::button::AdminButton>
        <crate::with_owner::button::AdminButton
            variant=crate::with_owner::button::AdminButtonVariant::Danger
            kind=crate::with_owner::button::AdminButtonKind::Button
            command_for=String::from("confirmation")
            command="show-modal"
        >
            "Danger"
        </crate::with_owner::button::AdminButton>
    });

    assert!(html.contains("ui-button ui-button-primary "));
    assert!(html.contains("disabled type=\"submit\""));
    assert!(html.contains("ui-button ui-button-secondary "));
    assert!(html.contains("type=\"button\""));
    assert!(html.contains("popovertarget=\"filters\""));
    assert!(html.contains("popovertargetaction=\"hide\""));
    assert!(html.contains("aria-label=\"Close filters\""));
    assert!(html.contains("style=\"width:100%;\""));
    assert!(html.contains("ui-button ui-button-danger danger-button "));
    assert!(html.contains("commandfor=\"confirmation\""));
    assert!(html.contains("command=\"show-modal\""));
}

#[test]
fn form_controls_render_every_supported_kind_and_constraint() {
    let html = render_owned_view(leptos::view! {
        <crate::with_owner::input::AdminInput
            name="login"
            autocomplete="username"
            required=true
            minlength=2
            maxlength=32
            initial_value=String::from("alice")
        />
        <crate::with_owner::input::AdminInput
            name="password"
            kind=crate::with_owner::input::AdminInputKind::Password
            disabled=true
        />
        <crate::with_owner::input::AdminInput
            name="limit"
            kind=crate::with_owner::input::AdminInputKind::Number
            min=1
            max=100
        />
        <crate::with_owner::input::AdminInput name="url" kind=crate::with_owner::input::AdminInputKind::Url />
        <crate::with_owner::admin_textarea::AdminTextarea name="notes" required=true disabled=true />
        <crate::with_owner::admin_checkbox::AdminCheckbox name="confirmation" value="true" required=true />
    });

    assert!(html.contains("name=\"login\""));
    assert!(html.contains("type=\"text\""));
    assert!(html.contains("autocomplete=\"username\""));
    assert!(html.contains("minlength=\"2\""));
    assert!(html.contains("maxlength=\"32\""));
    assert!(html.contains("value=\"alice\""));
    assert!(html.contains("name=\"password\""));
    assert!(html.contains("type=\"password\""));
    assert!(html.contains("name=\"limit\""));
    assert!(html.contains("type=\"number\""));
    assert!(html.contains("min=\"1\""));
    assert!(html.contains("max=\"100\""));
    assert!(html.contains("name=\"url\""));
    assert!(html.contains("type=\"url\""));
    assert!(html.contains("data-name=\"Textarea\""));
    assert!(html.contains("name=\"notes\""));
    assert!(html.contains("data-name=\"Checkbox\""));
    assert!(html.contains("name=\"confirmation\""));
    assert!(html.contains("value=\"true\""));
}

#[test]
fn bound_form_controls_render_signal_values() {
    let owner = leptos::prelude::Owner::new();
    let html = owner.with(|| {
        let input = crate::with_owner::input::LeptosAdminInputSignal::from(leptos::prelude::RwSignal::new(
            String::from(constants_str::VALUE_14527724),
        ));
        let textarea = crate::with_owner::input::LeptosAdminInputSignal::from(leptos::prelude::RwSignal::new(
            String::from(constants_str::VALUE_F013164D),
        ));
        render_owned_view(leptos::view! {
            <crate::with_owner::input::AdminInput name="bound_input" bind_value=input />
            <crate::with_owner::admin_textarea::AdminTextarea name="bound_textarea" bind_value=textarea />
        })
    });

    assert!(html.contains("name=\"bound_input\" type=\"text\""));
    assert!(html.contains("name=\"bound_textarea\""));
    assert!(html.contains(">bound textarea</textarea>"));
}

#[test]
fn visual_variants_keep_their_rust_ui_contracts() {
    let html = render_owned_view(leptos::view! {
        <crate::with_owner::alert::AdminAlert variant=crate::with_owner::alert::AdminAlertVariant::Success id="saved">"Saved"</crate::with_owner::alert::AdminAlert>
        <crate::with_owner::badge::AdminBadge>"Neutral"</crate::with_owner::badge::AdminBadge>
        <crate::with_owner::badge::AdminBadge variant=crate::with_owner::badge::AdminBadgeVariant::Success>"Success"</crate::with_owner::badge::AdminBadge>
        <crate::with_owner::card::AdminCard>"Default"</crate::with_owner::card::AdminCard>
        <crate::with_owner::card::AdminCard variant=crate::with_owner::card::AdminCardVariant::Auth>"Auth"</crate::with_owner::card::AdminCard>
        <crate::with_owner::card::AdminCard variant=crate::with_owner::card::AdminCardVariant::Code>"Code"</crate::with_owner::card::AdminCard>
        <crate::with_owner::card::AdminCard variant=crate::with_owner::card::AdminCardVariant::Profile>"Profile"</crate::with_owner::card::AdminCard>
        <crate::with_owner::card::AdminCard variant=crate::with_owner::card::AdminCardVariant::Security>"Security"</crate::with_owner::card::AdminCard>
    });

    assert!(html.contains("id=\"saved\""));
    assert!(html.contains("class=\"ui-alert flash-success "));
    assert!(html.contains("role=\"status\""));
    assert!(html.contains("class=\"ui-badge ui-badge-neutral "));
    assert!(html.contains("class=\"ui-badge ui-badge-success "));
    assert!(html.contains("class=\"ui-card flex "));
    assert!(html.contains("class=\"ui-card auth-card "));
    assert!(html.contains("class=\"ui-card code-card "));
    assert!(html.contains("class=\"ui-card profile-card "));
    assert!(html.contains("class=\"ui-card security-card "));
}

#[test]
fn navigation_distinguishes_current_and_inactive_destinations() {
    let html = render_owned_view(leptos::view! {
        <crate::with_owner::navigation::admin_navigation_link::AdminNavigationLink href=String::from("/admin/users") active=true>"Users"</crate::with_owner::navigation::admin_navigation_link::AdminNavigationLink>
        <crate::with_owner::navigation::admin_navigation_link::AdminNavigationLink href=String::from("/admin/roles") active=false>"Roles"</crate::with_owner::navigation::admin_navigation_link::AdminNavigationLink>
    });

    assert!(html.contains("data-name=\"NavigationMenuLink\""));
    assert!(html.contains("singlestage-link active "));
    assert!(html.contains("aria-current=\"page\""));
    assert!(html.contains("href=\"/admin/users\""));
    assert!(html.contains("text-foreground/70 transition-colors"));
    assert!(html.contains("href=\"/admin/roles\""));
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1);
}

#[test]
fn table_primitives_preserve_structure_and_class_merging() {
    let html = render_owned_view(leptos::view! {
        <crate::with_owner::tables::table_wrapper::TableWrapper>
            <crate::with_owner::tables::table::Table>
                <crate::with_owner::tables::table_caption::TableCaption>"Identifiers"</crate::with_owner::tables::table_caption::TableCaption>
                <crate::with_owner::tables::table_header::TableHeader>
                    <crate::with_owner::tables::table_row::TableRow>
                        <crate::with_owner::tables::table_head::TableHead>"Identifier"</crate::with_owner::tables::table_head::TableHead>
                    </crate::with_owner::tables::table_row::TableRow>
                </crate::with_owner::tables::table_header::TableHeader>
                <crate::with_owner::tables::table_body::TableBody>
                    <crate::with_owner::tables::table_row::TableRow>
                        <crate::with_owner::tables::table_cell::TableCell class="numeric-cell">"42"</crate::with_owner::tables::table_cell::TableCell>
                    </crate::with_owner::tables::table_row::TableRow>
                </crate::with_owner::tables::table_body::TableBody>
                <crate::with_owner::tables::table_footer::TableFooter>
                    <crate::with_owner::tables::table_row::TableRow><crate::with_owner::tables::table_cell::TableCell>"1"</crate::with_owner::tables::table_cell::TableCell></crate::with_owner::tables::table_row::TableRow>
                </crate::with_owner::tables::table_footer::TableFooter>
            </crate::with_owner::tables::table::Table>
        </crate::with_owner::tables::table_wrapper::TableWrapper>
    });

    assert!(html.contains("data-name=\"TableWrapper\""));
    assert!(html.contains("data-name=\"Table\""));
    assert!(html.contains("data-name=\"TableHeader\""));
    assert!(html.contains("data-name=\"TableBody\""));
    assert!(html.contains("data-name=\"TableCaption\""));
    assert!(html.contains("data-name=\"TableFooter\""));
    assert_eq!(html.matches("data-name=\"TableRow\"").count(), 3);
    assert!(html.contains("numeric-cell"));
}

#[test]
fn alert_dialog_wires_singlestage_trigger_and_dialog_forms() {
    let html = render_owned_view(leptos::view! {
        <crate::with_owner::admin_alert_dialog::AdminAlertDialog
            id=String::from("delete-dialog")
            title="Delete item?"
            description="The item will be removed."
            trigger="Delete"
            confirm="Confirm"
            disabled=true
            on_confirm=leptos::prelude::Callback::new(|()| {})
        />
    });

    assert!(html.contains("disabled"));
    assert!(html.contains("id=\"delete-dialog\""));
    assert!(html.contains("data-name=\"AlertDialogContent\""));
    assert!(html.contains("Delete item?"));
    assert_eq!(html.matches("method=\"dialog\"").count(), 1);
    assert!(html.contains("data-name=\"AlertDialogHeader\""));
    assert!(html.contains("data-name=\"AlertDialogTitle\""));
    assert!(html.contains("data-name=\"AlertDialogDescription\""));
    assert!(html.contains("data-name=\"AlertDialogFooter\""));
    assert!(!html.contains("commandfor="));
}
