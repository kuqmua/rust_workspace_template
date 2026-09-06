#[test]
fn test_table_cell_preview_preserves_the_complete_value() {
    let html = crate::render_view::render_view(leptos::view! {
        <crate::table_cell::TableCell data_label=constants_str::LOGIN>
            {constants_str::ADMIN_REVOKE_ALL_SESSIONS_DESCRIPTION}
        </crate::table_cell::TableCell>
    });
    assert!(
        html.as_ref()
            .contains(constants_str::ADMIN_TABLE_CELL_PREVIEW_CLASS)
    );
    assert!(
        html.as_ref()
            .contains(constants_str::ADMIN_REVOKE_ALL_SESSIONS_DESCRIPTION)
    );
    assert!(html.as_ref().contains(constants_str::VALUE_82A744A6));
}

#[test]
fn test_table_cell_actions_remain_outside_the_value_preview() {
    let html = crate::render_view::render_view(leptos::view! {
        <crate::table_cell::TableCell bool=true>
            <crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_REVOKE_SESSION}</crate::admin_button::AdminButton>
        </crate::table_cell::TableCell>
    });
    assert!(
        !html
            .as_ref()
            .contains(constants_str::ADMIN_TABLE_CELL_PREVIEW_CLASS)
    );
    assert!(
        html.as_ref()
            .contains(constants_str::ADMIN_BUTTON_REVOKE_SESSION)
    );
}
