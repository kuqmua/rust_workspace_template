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

#[test]
fn test_table_cell_optional_attributes_preserve_both_content_modes() {
    assert!((0usize..8usize).all(|mask| {
        let actions = mask & 4usize != 0usize;
        [false, true].into_iter().all(|custom_class| {
            let class = if custom_class { constants_str::ROOT } else { constants_str::EMPTY };
            let html = match mask & 3usize {
                0usize => crate::render_view::render_view(leptos::view! {
                    <crate::table_cell::TableCell class=class bool=actions>{constants_str::X}</crate::table_cell::TableCell>
                }),
                1usize => crate::render_view::render_view(leptos::view! {
                    <crate::table_cell::TableCell data_label=constants_str::LOGIN class=class bool=actions>{constants_str::X}</crate::table_cell::TableCell>
                }),
                2usize => crate::render_view::render_view(leptos::view! {
                    <crate::table_cell::TableCell data_field=constants_str::ROOT class=class bool=actions>{constants_str::X}</crate::table_cell::TableCell>
                }),
                3usize => crate::render_view::render_view(leptos::view! {
                    <crate::table_cell::TableCell data_label=constants_str::LOGIN data_field=constants_str::ROOT class=class bool=actions>{constants_str::X}</crate::table_cell::TableCell>
                }),
                _ => return false,
            };
            let prefix = concat!('<', stringify!(td), ' ');
            html.as_ref().split('>').find(|tag| tag.starts_with(prefix)).is_some_and(|cell| {
                [
                    (concat!(stringify!(data), '-', stringify!(label)), constants_str::LOGIN, mask & 1usize != 0usize),
                    (concat!(stringify!(data), '-', stringify!(field)), constants_str::ROOT, mask & 2usize != 0usize),
                ].into_iter().all(|(attribute, expected, present)| {
                    cell.split('"').zip(cell.split('"').skip(1usize)).any(|(name, value)| {
                        name.strip_suffix('=').is_some_and(|name_prefix| name_prefix.split_ascii_whitespace().last() == Some(attribute)) && value == expected
                    }) == present
                }) && cell.split('"').zip(cell.split('"').skip(1usize)).any(|(name, value)| {
                    name.strip_suffix('=').is_some_and(|name_prefix| name_prefix.split_ascii_whitespace().last() == Some(stringify!(class)))
                        && constants_str::VALUE_19AB4EBD.split_ascii_whitespace().take(2usize).all(|word| value.split_ascii_whitespace().any(|part| part == word))
                        && value.split_ascii_whitespace().any(|part| part == constants_str::ROOT) == custom_class
                })
            }) && html.as_ref().matches(concat!('<', stringify!(button), ' ')).count() == usize::from(!actions)
                && html.as_ref().matches(format!(">{}<", constants_str::X).as_str()).count() == 1usize
        })
    }));
}
