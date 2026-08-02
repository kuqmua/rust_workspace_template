fn render<View>(view: View) -> String
where
    View: leptos::prelude::IntoAny,
{
    leptos::prelude::RenderHtml::to_html(leptos::prelude::IntoAny::into_any(view))
}

#[test]
fn primitives_render_semantic_accessible_markup() {
    let owned_label = super::field::AdminFieldLabel::from(String::from("Owned label"));
    assert_eq!(owned_label.as_ref(), "Owned label");
    let html = render(leptos::view! {
        <super::card::AdminCard variant=super::card::AdminCardVariant::Settings>
            <super::alert::AdminAlert>"Invalid value"</super::alert::AdminAlert>
        <super::field::AdminField label="Login">
            <super::input::AdminInput name="login" required=true />
        </super::field::AdminField>
        <super::field::AdminField label=String::from("Owned label")>
            <super::empty::AdminEmpty>"Owned value"</super::empty::AdminEmpty>
        </super::field::AdminField>
            <super::button::AdminButton kind=super::button::AdminButtonKind::Button>"Save"</super::button::AdminButton>
            <super::badge::AdminBadge variant=super::badge::AdminBadgeVariant::Success>"Active"</super::badge::AdminBadge>
            <super::textarea::AdminTextarea name="notes" />
            <super::alert_dialog::AdminAlertDialog id=String::from("test-alert-dialog") title="Confirm action?" description="This action changes data." trigger="Delete" confirm="Confirm" on_confirm=leptos::prelude::Callback::new(|()| {}) />
        </super::card::AdminCard>
        <super::empty::AdminEmpty>"Nothing here"</super::empty::AdminEmpty>
        <super::spinner::AdminSpinner />
    });

    assert!(html.contains("data-name=\"Card\""));
    assert!(html.contains("class=\"ui-card settings-card "));
    assert!(html.contains("data-name=\"CardContent\" class=\"px-6\""));
    assert!(html.contains("data-name=\"Alert\""));
    assert!(html.contains("class=\"ui-alert field-error "));
    assert!(html.contains("role=\"alert\""));
    assert!(html.contains("data-name=\"Field\" class=\"ui-field "));
    assert!(html.contains("data-name=\"Label\""));
    assert!(html.contains("<span>Login</span>"));
    assert!(html.contains("<span>Owned label</span>"));
    assert!(html.contains("data-name=\"Input\""));
    assert!(html.contains("class=\"ui-input "));
    assert!(html.contains("name=\"login\""));
    assert!(html.contains("data-name=\"Button\""));
    assert!(html.contains("class=\"ui-button ui-button-primary "));
    assert!(html.contains("type=\"button\""));
    assert!(html.contains("data-name=\"Badge\""));
    assert!(html.contains("class=\"ui-badge ui-badge-success "));
    assert!(html.contains("data-name=\"Textarea\""));
    assert!(html.contains("class=\"ui-textarea "));
    assert!(html.contains("name=\"notes\""));
    assert!(html.contains("data-name=\"Empty\""));
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
    let html = render(leptos::view! {
        <super::button::AdminButton disabled=true>"Primary"</super::button::AdminButton>
        <super::button::AdminButton
            variant=super::button::AdminButtonVariant::Secondary
            kind=super::button::AdminButtonKind::Button
            popover_target=String::from("filters")
            popover_target_action="hide"
            aria_label=String::from("Close filters")
            style=String::from("width:100%")
        >
            "Secondary"
        </super::button::AdminButton>
        <super::button::AdminButton
            variant=super::button::AdminButtonVariant::Danger
            kind=super::button::AdminButtonKind::Button
            command_for=String::from("confirmation")
            command="show-modal"
        >
            "Danger"
        </super::button::AdminButton>
    });

    assert!(html.contains("class=\"ui-button ui-button-primary "));
    assert!(html.contains("type=\"submit\" disabled"));
    assert!(html.contains("class=\"ui-button ui-button-secondary "));
    assert!(html.contains("type=\"button\""));
    assert!(html.contains("popovertarget=\"filters\""));
    assert!(html.contains("popovertargetaction=\"hide\""));
    assert!(html.contains("aria-label=\"Close filters\""));
    assert!(html.contains("style=\"width:100%;\""));
    assert!(html.contains("class=\"ui-button ui-button-danger danger-button "));
    assert!(html.contains("commandfor=\"confirmation\""));
    assert!(html.contains("command=\"show-modal\""));
}

#[test]
fn form_controls_render_every_supported_kind_and_constraint() {
    let html = render(leptos::view! {
        <super::input::AdminInput
            name="login"
            autocomplete="username"
            required=true
            minlength=2
            maxlength=32
            initial_value=String::from("alice")
        />
        <super::input::AdminInput
            name="password"
            kind=super::input::AdminInputKind::Password
            disabled=true
        />
        <super::input::AdminInput
            name="limit"
            kind=super::input::AdminInputKind::Number
            min=1
            max=100
        />
        <super::input::AdminInput name="url" kind=super::input::AdminInputKind::Url />
        <super::textarea::AdminTextarea name="notes" required=true disabled=true />
        <super::checkbox::AdminCheckbox name="confirmation" value="true" required=true />
    });

    assert!(html.contains("name=\"login\" type=\"text\" autocomplete=\"username\" required"));
    assert!(html.contains("minlength=\"2\" maxlength=\"32\""));
    assert!(html.contains("value=\"alice\""));
    assert!(html.contains("name=\"password\" type=\"password\""));
    assert!(html.contains("name=\"limit\" type=\"number\""));
    assert!(html.contains("min=\"1\" max=\"100\""));
    assert!(html.contains("name=\"url\" type=\"url\""));
    assert!(html.contains("data-name=\"Textarea\""));
    assert!(html.contains("name=\"notes\" required disabled"));
    assert!(html.contains("data-name=\"Checkbox\""));
    assert!(html.contains("type=\"checkbox\" name=\"confirmation\" value=\"true\" required"));
}

#[test]
fn bound_form_controls_render_signal_values() {
    let owner = leptos::prelude::Owner::new();
    let html = owner.with(|| {
        let input = super::input::LeptosAdminInputSignal::from(leptos::prelude::RwSignal::new(
            String::from("bound input"),
        ));
        let textarea = super::input::LeptosAdminInputSignal::from(leptos::prelude::RwSignal::new(
            String::from("bound textarea"),
        ));
        render(leptos::view! {
            <super::input::AdminInput name="bound_input" bind_value=input />
            <super::textarea::AdminTextarea name="bound_textarea" bind_value=textarea />
        })
    });

    assert!(html.contains("name=\"bound_input\" type=\"text\""));
    assert!(html.contains("name=\"bound_textarea\""));
    assert!(html.contains(">bound textarea</textarea>"));
}

#[test]
fn visual_variants_keep_their_rust_ui_contracts() {
    let html = render(leptos::view! {
        <super::alert::AdminAlert variant=super::alert::AdminAlertVariant::Success id="saved">"Saved"</super::alert::AdminAlert>
        <super::badge::AdminBadge>"Neutral"</super::badge::AdminBadge>
        <super::badge::AdminBadge variant=super::badge::AdminBadgeVariant::Success>"Success"</super::badge::AdminBadge>
        <super::card::AdminCard>"Default"</super::card::AdminCard>
        <super::card::AdminCard variant=super::card::AdminCardVariant::Auth>"Auth"</super::card::AdminCard>
        <super::card::AdminCard variant=super::card::AdminCardVariant::Code>"Code"</super::card::AdminCard>
        <super::card::AdminCard variant=super::card::AdminCardVariant::Profile>"Profile"</super::card::AdminCard>
        <super::card::AdminCard variant=super::card::AdminCardVariant::Security>"Security"</super::card::AdminCard>
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
    let html = render(leptos::view! {
        <super::navigation::AdminNavigationLink href=String::from("/admin/users") active=true>"Users"</super::navigation::AdminNavigationLink>
        <super::navigation::AdminNavigationLink href=String::from("/admin/roles") active=false>"Roles"</super::navigation::AdminNavigationLink>
    });

    assert!(html.contains("data-name=\"NavigationMenuLink\""));
    assert!(html.contains("class=\"active "));
    assert!(html.contains("aria-current=\"page\" href=\"/admin/users\""));
    assert!(html.contains("text-foreground/70 transition-colors"));
    assert!(html.contains("href=\"/admin/roles\""));
    assert_eq!(html.matches("aria-current=\"page\"").count(), 1);
}

#[test]
fn alert_dialog_wires_trigger_cancel_and_confirmation_commands() {
    let html = render(leptos::view! {
        <super::alert_dialog::AdminAlertDialog
            id=String::from("delete-dialog")
            title="Delete item?"
            description="The item will be removed."
            trigger="Delete"
            confirm="Confirm"
            disabled=true
            on_confirm=leptos::prelude::Callback::new(|()| {})
        />
    });

    assert!(html.contains("commandfor=\"delete-dialog\" command=\"show-modal\""));
    assert!(html.contains("disabled"));
    assert!(html.contains("id=\"delete-dialog\""));
    assert!(html.contains("aria-label=\"Delete item?\""));
    assert_eq!(html.matches("commandfor=\"delete-dialog\"").count(), 3);
    assert_eq!(html.matches("command=\"close\"").count(), 2);
}
