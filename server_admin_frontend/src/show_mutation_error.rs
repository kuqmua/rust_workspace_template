pub(crate) fn show_mutation_error(error: &crate::admin_table_load_error::AdminTableLoadError) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Some(root) = document.get_element_by_id(constants_str::test_fixtures::ADMIN_CSR_ROOT_ID)
    else {
        return;
    };
    let Ok(alert) = document.create_element(constants_str::test_fixtures::VALUE_148DE9C5) else {
        return;
    };
    if alert
        .set_attribute(
            constants_str::catalog::ROLE,
            constants_str::test_fixtures::HTML_ALERT_ROLE,
        )
        .is_err()
    {
        return;
    }
    if alert
        .set_attribute(
            constants_str::test_fixtures::HTML_DATA_NAME,
            constants_str::test_fixtures::ADMIN_ALERT_DATA_NAME,
        )
        .is_err()
    {
        return;
    }
    alert.set_text_content(Some(&error.to_string()));
    alert.set_class_name(constants_str::test_fixtures::ADMIN_FIELD_ERROR_CLASS);
    if root.append_child(&alert).is_err() {
        root.set_text_content(Some(&error.to_string()));
        root.set_class_name(constants_str::test_fixtures::ADMIN_FIELD_ERROR_CLASS);
    }
}
