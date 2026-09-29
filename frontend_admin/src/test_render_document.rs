#[test]
fn test_document_escapes_configured_tab_title() {
    let title = crate::admin_ssr_text::AdminSsrText::try_from(String::from(
        constants_str::ADMIN_DOCUMENT_UNSAFE_TITLE_FIXTURE,
    ))
    .unwrap_or_else(crate::admin_ssr_text::AdminSsrText::from);
    let html = crate::render_document::render_document(&title, leptos::view! { <main></main> });

    assert!(
        html.as_ref()
            .contains(constants_str::ADMIN_DOCUMENT_ESCAPED_TITLE_FIXTURE)
    );
    assert!(
        !html
            .as_ref()
            .contains(constants_str::ADMIN_DOCUMENT_UNSAFE_SCRIPT_TAG)
    );
}
