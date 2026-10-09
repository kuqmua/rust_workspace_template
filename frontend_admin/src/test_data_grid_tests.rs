#[test]
fn test_data_grid() {
    let columns = server_admin_contract::admin_data_columns::AdminDataColumns::try_from(vec![
        server_admin_contract::admin_data_column::AdminDataColumn::new(
            server_admin_contract::admin_data_filters::AdminDataFilters::try_from(Vec::new())
                .expect(constants_str::VALUE_D0BD1ECC),
            frontend_contract::input_kind::InputKind::Number,
            server_admin_contract::admin_text::AdminText::try_from(String::from(
                constants_str::VALUE_1D438D9B,
            ))
            .expect(constants_str::VALUE_46CE1BB0),
            server_admin_contract::admin_text::AdminText::try_from(String::from(
                constants_str::SQL_NAMES_ID,
            ))
            .expect(constants_str::VALUE_81310A83),
        ),
        server_admin_contract::admin_data_column::AdminDataColumn::new(
            server_admin_contract::admin_data_filters::AdminDataFilters::try_from(vec![
                server_admin_contract::admin_data_filter::AdminDataFilter::from(
                    frontend_contract::filter_operation::FilterOperation::Eq,
                ),
                server_admin_contract::admin_data_filter::AdminDataFilter::from(
                    frontend_contract::filter_operation::FilterOperation::Regex,
                ),
                server_admin_contract::admin_data_filter::AdminDataFilter::from(
                    frontend_contract::filter_operation::FilterOperation::Between,
                ),
            ])
            .expect(constants_str::VALUE_4C7734E6),
            frontend_contract::input_kind::InputKind::Text,
            server_admin_contract::admin_text::AdminText::try_from(String::from(
                constants_str::VALUE_B2D6201D,
            ))
            .expect(constants_str::VALUE_EC14A0FD),
            server_admin_contract::admin_text::AdminText::try_from(String::from(
                constants_str::LOGIN,
            ))
            .expect(constants_str::VALUE_6A1237E9),
        ),
    ])
    .expect(constants_str::DIAGNOSTIC_57462AD9);
    let values = server_admin_contract::admin_texts::AdminTexts::try_from(vec![
        server_admin_contract::admin_text::AdminText::try_from(String::from(
            constants_str::VALUE_42,
        ))
        .expect(constants_str::VALUE_1DF3FF47),
        server_admin_contract::admin_text::AdminText::try_from(String::from(
            constants_str::VALUE_2BD806C9,
        ))
        .expect(constants_str::VALUE_BED65ED1),
    ])
    .expect(constants_str::DIAGNOSTIC_58FED1D1);
    let rows = server_admin_contract::admin_data_rows::AdminDataRows::try_from(vec![
        server_admin_contract::admin_data_row::AdminDataRow::new(values),
    ])
    .expect(constants_str::DIAGNOSTIC_AC944CCC);
    let view = server_admin_contract::admin_data_table_view::AdminDataTableView::new(
        columns.clone(),
        rows.clone(),
        server_admin_contract::admin_data_table::AdminDataTable::Users,
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let filter_view = server_admin_contract::admin_data_table_view::AdminDataTableView::new(
        columns,
        rows,
        server_admin_contract::admin_data_table::AdminDataTable::RoleRules,
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );

    let default_query =
        server_admin_contract::admin_data_table_query::AdminDataTableQuery::default();
    let html = crate::admin_ssr_view_ext_tests::AdminSsrViewExt::render_admin_ssr(
        crate::data_table_grid::data_table_grid(&view, &default_query),
    );

    let update_path = format!(
        "{}/{}/{}",
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
        constants_str::VALUE_42,
        constants_str::PG_CRUD_UPDATE_RULE_ACTION
    );
    assert!(!html.as_ref().contains(update_path.as_str()));
    let actions_html = crate::admin_ssr_view_ext_tests::AdminSsrViewExt::render_admin_ssr(
        crate::admin_data_table_grid::admin_data_table_grid(
            &view,
            default_query.filter().field(),
            None,
            default_query.filter().value(),
            default_query.filter().end(),
            default_query.page().limit(),
            Some(default_query.page()),
            &view.table().frontend_path(),
            false,
            true,
        ),
    );
    assert!(actions_html.as_ref().contains(update_path.as_str()));

    assert!(html.as_ref().contains(constants_str::VALUE_469219C9));
    assert!(html.as_ref().contains(constants_str::VALUE_846B8D6B));
    assert!(html.as_ref().contains(constants_str::VALUE_31819FEE));
    assert!(html.as_ref().contains(constants_str::VALUE_8886AF1E));
    assert!(html.as_ref().contains(constants_str::VALUE_38C2F107));
    assert!(html.as_ref().contains(constants_str::VALUE_447968F6));
    assert!(html.as_ref().contains(constants_str::VALUE_0702D49A));
    assert!(html.as_ref().contains(constants_str::VALUE_7CA6DA11));
    assert!(html.as_ref().contains(constants_str::VALUE_0F228174));
    assert!(
        html.as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_50E5C8E7)
    );
    assert!(html.as_ref().contains(constants_str::VALUE_80DFAFAE));
    assert!(html.as_ref().contains(constants_str::VALUE_ED7335DF));
    assert!(html.as_ref().contains(constants_str::VALUE_B2F8E281));

    let query = server_admin_contract::admin_data_table_query::AdminDataTableQuery::new(
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::admin_filter_field::AdminFilterField::try_from(
                    String::from(constants_str::LOGIN),
                )
                .expect(constants_str::DIAGNOSTIC_774BC583),
            ),
            Some(frontend_contract::filter_operation::FilterOperation::Eq),
            Some(
                server_admin_contract::admin_filter_value::AdminFilterValue::try_from(
                    String::from(constants_str::VALUE_2BD806C9),
                )
                .expect(constants_str::DIAGNOSTIC_63D17F8E),
            ),
            None,
        ),
        server_admin_contract::admin_table_query::AdminTableQuery::new(
            server_admin_contract::admin_table_search::AdminTableSearch::default(),
            server_admin_contract::admin_table_sort_key::AdminTableSortKey::default(),
            server_admin_contract::admin_page_offset::AdminPageOffset::default(),
            server_admin_contract::admin_page_limit::AdminPageLimit::default(),
            server_admin_contract::admin_sort_direction::AdminSortDirection::Descending,
        ),
    );
    let filters_html = crate::admin_ssr_view_ext_tests::AdminSsrViewExt::render_admin_ssr(
        crate::data_table_grid::data_table_grid(&filter_view, &query),
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_B67A1896)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_B2F8E281)
    );
    assert!(
        !filters_html
            .as_ref()
            .contains(constants_str::VALUE_A01DB716)
    );
    let (_before_login, login_tail) = filters_html
        .as_ref()
        .split_once(constants_str::VALUE_3837854C)
        .expect(constants_str::DIAGNOSTIC_45B73477);
    let (login_header, _after_login) = login_tail
        .split_once(constants_str::VALUE_25C350AC)
        .expect(constants_str::DIAGNOSTIC_E8120A92);
    assert!(login_header.contains(constants_str::VALUE_B2F8E281));
    assert!(login_header.contains(constants_str::VALUE_EDC93DB7));
    assert!(login_header.contains(constants_str::VALUE_68BB5D51));
    assert!(login_header.contains(constants_str::VALUE_802A0142));
    let (_before_id, id_tail) = filters_html
        .as_ref()
        .split_once(constants_str::VALUE_469219C9)
        .expect(constants_str::DIAGNOSTIC_C8A92EF4);
    let (id_header, _after_id) = id_tail
        .split_once(constants_str::VALUE_25C350AC)
        .expect(constants_str::DIAGNOSTIC_58CDF783);
    assert!(!id_header.contains(constants_str::VALUE_B2F8E281));
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_1665AC0E)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_96BBC0EC)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_D641BA27)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_6FB367C6)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_1C61CF88)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_E7A7CF18)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_5CAEF150)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_AC56ED1F)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_C7C36CF4)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_D241380B)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_26B901BB)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_022ECEBF)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_DDF681C4)
    );
    assert!(
        !filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_FF2F6A65)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_FEA2007C)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_D8E97E9A)
    );
    assert!(
        !filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_8C4051D1)
    );
    assert!(
        !filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_69FA33B8)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_E1FD30CF)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::VALUE_F37D548A)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_F969D2DE)
    );
    assert!(
        filters_html
            .as_ref()
            .contains(constants_str::ADMIN_UI_EXPECT_VALUE_BD7A6256)
    );
    let apply_position = filters_html
        .as_ref()
        .find(constants_str::ADMIN_UI_EXPECT_VALUE_38228244)
        .expect(constants_str::DIAGNOSTIC_10C26D45);
    let close_position = filters_html
        .as_ref()
        .find(constants_str::ADMIN_UI_EXPECT_VALUE_0D4379EB)
        .expect(constants_str::DIAGNOSTIC_1542A5C3);
    let clear_position = filters_html
        .as_ref()
        .find(constants_str::ADMIN_UI_EXPECT_VALUE_BD7A6256)
        .expect(constants_str::DIAGNOSTIC_58F35E11);
    assert!(close_position > apply_position);
    assert!(clear_position > apply_position);
    assert!(clear_position > close_position);
    assert_eq!(
        filters_html
            .as_ref()
            .matches(constants_str::VALUE_AC55DE88)
            .count(),
        constants_usize::TWO
    );
    assert_eq!(
        filters_html
            .as_ref()
            .matches(
                server_admin_contract::admin_sort_direction::AdminSortDirection::Descending
                    .as_ref()
            )
            .count(),
        constants_usize::TWO
    );
    assert_eq!(
        filters_html
            .as_ref()
            .matches(constants_str::ADMIN_UI_EXPECT_VALUE_38228244)
            .count(),
        constants_usize::ONE
    );
    assert!(
        !filters_html
            .as_ref()
            .contains(constants_str::VALUE_155A24DE)
    );
    assert!(
        !filters_html
            .as_ref()
            .contains(constants_str::VALUE_5EBA168E)
    );
    assert_eq!(
        filters_html
            .as_ref()
            .matches(constants_str::VALUE_939560A4)
            .count(),
        constants_usize::ONE
    );

    let admin = crate::domain_types_ssr_tests::test_admin();
    let branding = crate::domain_types_ssr_tests::test_branding();
    let page_html = crate::render_data_tables::render_data_tables(
        Some(&filter_view),
        &query,
        &admin,
        &branding,
    );
    assert!(page_html.as_ref().contains(constants_str::VALUE_3837854C));
    assert!(page_html.as_ref().contains(constants_str::VALUE_AC55DE88));
    let empty_html = crate::render_data_tables::render_data_tables(None, &query, &admin, &branding);
    assert!(!empty_html.as_ref().contains(constants_str::VALUE_BBB98D48));
    assert!(
        empty_html
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.get())
    );
}

#[test]
fn test_admin_grid_requires_valid_identifier_and_permission_for_actions() {
    assert!(
        server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER
            .into_iter()
            .all(|table| {
                let valid_identifier = constants_str::VALUE_42;
                [
                    (valid_identifier, true, true, true),
                    (valid_identifier, true, false, false),
                    (constants_str::X, true, true, false),
                    (stringify!(0), true, true, false),
                    (stringify!(-1), true, true, false),
                    (stringify!(9223372036854775808), true, true, false),
                    (valid_identifier, false, true, false),
                ]
                .into_iter()
                .all(
                    |(identifier, has_identifier_column, can_update, expected_update)| {
                        let column_name = if has_identifier_column {
                            constants_str::SQL_NAMES_ID
                        } else {
                            constants_str::LOGIN
                        };
                        let Ok(name) = server_admin_contract::admin_text::AdminText::try_from(
                            column_name.to_owned(),
                        ) else {
                            return false;
                        };
                        let Ok(filters) =
                            server_admin_contract::admin_data_filters::AdminDataFilters::try_from(
                                Vec::new(),
                            )
                        else {
                            return false;
                        };
                        let Ok(prefix_name) = server_admin_contract::admin_text::AdminText::try_from(
                            constants_str::LOGIN.to_owned(),
                        ) else {
                            return false;
                        };
                        let Ok(columns) =
                            server_admin_contract::admin_data_columns::AdminDataColumns::try_from(
                                vec![
                                    server_admin_contract::admin_data_column::AdminDataColumn::new(
                                        filters.clone(),
                                        frontend_contract::input_kind::InputKind::Text,
                                        prefix_name.clone(),
                                        prefix_name,
                                    ),
                                    server_admin_contract::admin_data_column::AdminDataColumn::new(
                                        filters,
                                        frontend_contract::input_kind::InputKind::Number,
                                        name.clone(),
                                        name,
                                    ),
                                ],
                            )
                        else {
                            return false;
                        };
                        let Ok(text) = server_admin_contract::admin_text::AdminText::try_from(
                            identifier.to_owned(),
                        ) else {
                            return false;
                        };
                        let Ok(prefix_text) = server_admin_contract::admin_text::AdminText::try_from(
                            constants_str::TEST_FIRST.to_owned(),
                        ) else {
                            return false;
                        };
                        let Ok(values) =
                            server_admin_contract::admin_texts::AdminTexts::try_from(vec![
                                prefix_text,
                                text,
                            ])
                        else {
                            return false;
                        };
                        let Ok(rows) =
                            server_admin_contract::admin_data_rows::AdminDataRows::try_from(vec![
                                server_admin_contract::admin_data_row::AdminDataRow::new(values),
                            ])
                        else {
                            return false;
                        };

                        let view =
                            server_admin_contract::admin_data_table_view::AdminDataTableView::new(
                                columns,
                                rows,
                                table,
                                server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
                            );
                        let query =
                            server_admin_contract::admin_table_query::AdminTableQuery::default();
                        let html =
                            crate::admin_ssr_view_ext_tests::AdminSsrViewExt::render_admin_ssr(
                                crate::admin_data_table_grid::admin_data_table_grid(
                                    &view,
                                    None,
                                    None,
                                    None,
                                    None,
                                    query.limit(),
                                    Some(&query),
                                    &table.frontend_path(),
                                    false,
                                    can_update,
                                ),
                            );
                        let Ok(role) =
                            server_admin_contract::admin_role_id::AdminRoleId::try_from(42i64)
                        else {
                            return false;
                        };
                        let update =
                    server_admin_contract::admin_route_path::AdminRoutePath::role_update_path(role);
                        let expected_role_update = expected_update
                            && table
                                == server_admin_contract::admin_data_table::AdminDataTable::Roles;
                        let Ok(user) = server_admin_contract::admin_user_id::AdminUserId::try_from(42i64) else { return false; };
                        let user_update = server_admin_contract::admin_route_path::AdminRoutePath::user_update_path(user);
                        let expected_user_update = expected_update && table == server_admin_contract::admin_data_table::AdminDataTable::Users;
                        let read_path = if table == server_admin_contract::admin_data_table::AdminDataTable::Rules {
                            let Ok(rule) = server_admin_contract::admin_rule_id::AdminRuleId::try_from(42i64) else { return false; };
                            server_admin_contract::admin_route_path::AdminRoutePath::from(rule).to_string()
                        } else { format!("{}/{}/{}", table.frontend_path(), valid_identifier, constants_str::PG_CRUD_READ_RULE_ACTION) };
                        let read_link = format!("href=\"{read_path}\"");
                        let read_label =
                            format!("aria-label=\"{}\"", constants_str::PG_CRUD_READ_RULE_ACTION);
                        html.as_ref().contains(update.as_ref()) == expected_role_update
                            && html.as_ref().contains(user_update.as_ref()) == expected_user_update
                            && html.as_ref().contains(read_link.as_str()) == (has_identifier_column && identifier == valid_identifier)
                            && html.as_ref().contains(read_label.as_str())
                                == (has_identifier_column && identifier == valid_identifier)
                    },
                )
            })
    );
}

#[test]
fn test_admin_grid_renders_empty_columns_and_unmatched_cells_without_actions() {
    assert!(
        [None, Some(0usize), Some(2usize)]
            .into_iter()
            .all(|value_count| {
                let Ok(columns) =
                    server_admin_contract::admin_data_columns::AdminDataColumns::try_from(
                        Vec::new(),
                    )
                else {
                    return false;
                };
                let Ok(text) = server_admin_contract::admin_text::AdminText::try_from(
                    constants_str::TEST_FIRST.to_owned(),
                ) else {
                    return false;
                };
                let rows = if let Some(count) = value_count {
                    let Ok(values) =
                        server_admin_contract::admin_texts::AdminTexts::try_from(vec![text; count])
                    else {
                        return false;
                    };
                    vec![server_admin_contract::admin_data_row::AdminDataRow::new(
                        values,
                    )]
                } else {
                    Vec::new()
                };
                let Ok(items) =
                    server_admin_contract::admin_data_rows::AdminDataRows::try_from(rows)
                else {
                    return false;
                };
                let table = server_admin_contract::admin_data_table::AdminDataTable::Users;
                let view = server_admin_contract::admin_data_table_view::AdminDataTableView::new(
                    columns,
                    items,
                    table,
                    server_admin_contract::admin_page_total::AdminPageTotal::from(0u64),
                );
                let query = server_admin_contract::admin_table_query::AdminTableQuery::default();
                let html = crate::admin_ssr_view_ext_tests::AdminSsrViewExt::render_admin_ssr(
                    crate::admin_data_table_grid::admin_data_table_grid(
                        &view,
                        None,
                        None,
                        None,
                        None,
                        query.limit(),
                        Some(&query),
                        &table.frontend_path(),
                        false,
                        true,
                    ),
                );
                let cell_text = format!(">{}<", constants_str::TEST_FIRST);
                let read_label =
                    format!("aria-label=\"{}\"", constants_str::PG_CRUD_READ_RULE_ACTION);
                html.as_ref().contains(stringify!(TableBody))
                    && html.as_ref().matches(cell_text.as_str()).count()
                        == value_count.unwrap_or_default()
                    && !html.as_ref().contains(read_label.as_str())
                    && !html
                        .as_ref()
                        .contains(concat!(stringify!(numeric), "-", stringify!(cell)))
            })
    );
}

#[test]
fn test_column_filter_excludes_membership_for_checkboxes_and_preserves_text_operations() {
    let field_result =
        server_admin_contract::admin_text::AdminText::try_from(String::from(constants_str::LOGIN));
    assert_eq!(field_result.as_ref().err(), None);
    let Ok(field) = field_result else {
        return;
    };
    let path = server_admin_contract::admin_data_table::AdminDataTable::Users.frontend_path();
    let filters = [
        server_admin_contract::admin_data_filter::AdminDataFilter::from(
            frontend_contract::filter_operation::FilterOperation::Eq,
        ),
        server_admin_contract::admin_data_filter::AdminDataFilter::from(
            frontend_contract::filter_operation::FilterOperation::In,
        ),
    ];
    assert!(
        [
            (frontend_contract::input_kind::InputKind::Checkbox, false),
            (frontend_contract::input_kind::InputKind::Text, true),
        ]
        .into_iter()
        .all(|(input_kind, includes_membership)| {
            let html = crate::admin_ssr_view_ext_tests::AdminSsrViewExt::render_admin_ssr(
                crate::with_owner::with_owner(|| {
                    crate::admin_column_filter::admin_column_filter(
                        &path,
                        &field,
                        input_kind,
                        &filters,
                        None,
                        None,
                        None,
                        None,
                        server_admin_contract::admin_page_limit::AdminPageLimit::default(),
                        None,
                    )
                }),
            );
            html.as_ref()
                .contains(constants_str::ADMIN_FILTER_EQ_OPTION_VALUE_FIXTURE)
                && html
                    .as_ref()
                    .contains(constants_str::ADMIN_FILTER_IN_OPTION_VALUE_FIXTURE)
                    == includes_membership
        })
    );
}

#[test]
fn test_column_filter_preserves_active_range_and_rejects_stale_operation_values() {
    let setup = (
        server_admin_contract::admin_text::AdminText::try_from(String::from(constants_str::LOGIN)),
        server_admin_contract::admin_filter_field::AdminFilterField::try_from(String::from(
            constants_str::LOGIN,
        )),
        server_admin_contract::admin_filter_field::AdminFilterField::try_from(String::from(
            constants_str::SQL_NAMES_ID,
        )),
        server_admin_contract::admin_filter_value::AdminFilterValue::try_from(String::from(
            constants_str::VALUE_2BD806C9,
        )),
        server_admin_contract::admin_filter_value::AdminFilterValue::try_from(String::from(
            constants_str::VALUE_42,
        )),
    );
    assert!(matches!(&setup, (Ok(_), Ok(_), Ok(_), Ok(_), Ok(_))));
    let (Ok(column), Ok(field), Ok(other_field), Ok(value), Ok(end)) = setup else {
        return;
    };
    let path = server_admin_contract::admin_data_table::AdminDataTable::Users.frontend_path();
    let filters = [
        server_admin_contract::admin_data_filter::AdminDataFilter::from(
            frontend_contract::filter_operation::FilterOperation::Eq,
        ),
        server_admin_contract::admin_data_filter::AdminDataFilter::from(
            frontend_contract::filter_operation::FilterOperation::Between,
        ),
    ];
    let between = server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(
        frontend_contract::filter_operation::FilterOperation::Between,
    );
    let regex = server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(
        frontend_contract::filter_operation::FilterOperation::Regex,
    );
    let input_prefix = format!("{} ", stringify!(input));
    let radio_name = format!("name=\"{}\"", stringify!(filter_operation));
    let value_name = format!("name=\"{}\"", stringify!(filter_value));
    let end_name = format!("name=\"{}\"", stringify!(filter_end));
    let value_attribute = format!("{}=", stringify!(value));
    let retained_value = format!("value=\"{}\"", constants_str::VALUE_2BD806C9);
    assert!(
        [
            (&filters[..], None, Some(&between), false, false, 1usize),
            (
                &filters[..],
                Some(&field),
                Some(&between),
                true,
                true,
                1usize
            ),
            (
                &filters[..],
                Some(&field),
                Some(&regex),
                true,
                false,
                1usize
            ),
            (
                &filters[..],
                Some(&other_field),
                Some(&between),
                false,
                false,
                1usize
            ),
            (&filters[..], Some(&field), None, true, false, 1usize),
            (
                &filters[..0usize],
                Some(&field),
                Some(&between),
                true,
                false,
                0usize
            ),
        ]
        .into_iter()
        .all(
            |(offered_filters, active_field, active_operation, active, range, controls)| {
                let html = crate::admin_ssr_view_ext_tests::AdminSsrViewExt::render_admin_ssr(
                    crate::with_owner::with_owner(|| {
                        crate::admin_column_filter::admin_column_filter(
                            &path,
                            &column,
                            frontend_contract::input_kind::InputKind::Text,
                            offered_filters,
                            active_field,
                            active_operation,
                            Some(&value),
                            Some(&end),
                            server_admin_contract::admin_page_limit::AdminPageLimit::default(),
                            None,
                        )
                    }),
                );
                let inputs = html
                    .as_ref()
                    .split('<')
                    .filter(|text| text.starts_with(input_prefix.as_str()))
                    .filter_map(|text| text.split_once('>').map(|(tag, _tail)| tag));
                let selected_value = format!(
                    "value=\"{}\"",
                    if range {
                        constants_str::ADMIN_FILTER_OPERATION_BETWEEN
                    } else {
                        constants_str::ADMIN_FILTER_OPERATION_EQ
                    }
                );
                let checked = inputs
                    .clone()
                    .filter(|tag| {
                        tag.contains(radio_name.as_str())
                            && tag.split_ascii_whitespace().any(|attribute| {
                                attribute
                                    .split_once('=')
                                    .map_or(attribute, |(name, _value)| name)
                                    == stringify!(checked)
                            })
                    })
                    .fold((0usize, true), |(count, valid), tag| {
                        (
                            count + 1usize,
                            valid && tag.contains(selected_value.as_str()),
                        )
                    });
                let matches = checked == (controls, true)
                    && inputs
                        .clone()
                        .filter(|tag| tag.contains(value_name.as_str()))
                        .count()
                        == controls * 2usize
                    && inputs
                        .clone()
                        .filter(|tag| {
                            tag.contains(value_name.as_str())
                                && tag.contains(retained_value.as_str())
                        })
                        .count()
                        == usize::from(range)
                    && inputs.filter(|tag| tag.contains(end_name.as_str())).fold(
                        (0usize, true),
                        |(count, valid), tag| {
                            let disabled = tag.split_ascii_whitespace().any(|attribute| {
                                attribute
                                    .split_once('=')
                                    .map_or(attribute, |(name, _value)| name)
                                    == stringify!(disabled)
                            });
                            (
                                count + 1usize,
                                valid
                                    && disabled != range
                                    && if range {
                                        tag.contains(
                                            format!("value=\"{}\"", constants_str::VALUE_42)
                                                .as_str(),
                                        )
                                    } else {
                                        tag.split_ascii_whitespace()
                                            .find_map(|attribute| {
                                                attribute.strip_prefix(value_attribute.as_str())
                                            })
                                            .is_none_or(|attribute| {
                                                attribute == format!("\"{}\"", constants_str::EMPTY)
                                            })
                                    },
                            )
                        },
                    ) == (controls, true)
                    && html.as_ref().contains(
                        format!(
                            "{}=\"{}\"",
                            stringify!(data_filter_active).replace('_', constants_str::HYPHEN),
                            active
                        )
                        .as_str(),
                    )
                    && html
                        .as_ref()
                        .matches(
                            format!(
                                "class=\"{}\"",
                                stringify!(table_filter_clear).replace('_', constants_str::HYPHEN)
                            )
                            .as_str(),
                        )
                        .count()
                        == usize::from(active);
                assert!(matches, "{active} {range} {controls}");
                matches
            }
        )
    );
}
