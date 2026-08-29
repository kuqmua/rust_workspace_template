#[test]
fn data_grid() {
    let columns = server_admin_contract::admin_data_columns::AdminDataColumns::try_from(vec![
        server_admin_contract::admin_data_column::AdminDataColumn::new(
            server_admin_contract::admin_data_filters::AdminDataFilters::try_from(Vec::new())
                .expect(constants_str::test_fixtures::VALUE_D0BD1ECC),
            frontend_contract::input_kind::InputKind::Number,
            server_admin_contract::admin_text::AdminText::try_from(String::from(
                constants_str::test_fixtures::VALUE_1D438D9B,
            ))
            .expect(constants_str::test_fixtures::VALUE_46CE1BB0),
            server_admin_contract::admin_text::AdminText::try_from(String::from(
                constants_str::catalog::SQL_NAMES_ID,
            ))
            .expect(constants_str::test_fixtures::VALUE_81310A83),
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
            .expect(constants_str::test_fixtures::VALUE_4C7734E6),
            frontend_contract::input_kind::InputKind::Text,
            server_admin_contract::admin_text::AdminText::try_from(String::from(
                constants_str::test_fixtures::VALUE_B2D6201D,
            ))
            .expect(constants_str::test_fixtures::VALUE_EC14A0FD),
            server_admin_contract::admin_text::AdminText::try_from(String::from(
                constants_str::catalog::LOGIN,
            ))
            .expect(constants_str::test_fixtures::VALUE_6A1237E9),
        ),
    ])
    .expect("57462ad9 generated_column_metadata_drives_data_table_markup invariant must hold");
    let values = server_admin_contract::admin_texts::AdminTexts::try_from(vec![
        server_admin_contract::admin_text::AdminText::try_from(String::from(
            constants_str::catalog::VALUE_42,
        ))
        .expect(constants_str::test_fixtures::VALUE_1DF3FF47),
        server_admin_contract::admin_text::AdminText::try_from(String::from(
            constants_str::test_fixtures::VALUE_2BD806C9,
        ))
        .expect(constants_str::test_fixtures::VALUE_BED65ED1),
    ])
    .expect("58fed1d1 generated_column_metadata_drives_data_table_markup invariant must hold");
    let rows = server_admin_contract::admin_data_rows::AdminDataRows::try_from(vec![
        server_admin_contract::admin_data_row::AdminDataRow::new(values),
    ])
    .expect("ac944ccc generated_column_metadata_drives_data_table_markup invariant must hold");
    let view = server_admin_contract::admin_data_table_view::AdminDataTableView::new(
        columns.clone(),
        rows.clone(),
        server_admin_contract::admin_data_table::AdminDataTable::Users,
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );
    let filter_view = server_admin_contract::admin_data_table_view::AdminDataTableView::new(
        columns,
        rows,
        server_admin_contract::admin_data_table::AdminDataTable::RolePermissions,
        server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
    );

    let default_query =
        server_admin_contract::admin_data_table_query::AdminDataTableQuery::default();
    let html = crate::admin_ssr_view_ext::AdminSsrViewExt::render_admin_ssr(
        crate::data_table_grid::data_table_grid(&view, &default_query),
    );

    assert!(html.as_ref().contains("data-field=\"id\""));
    assert!(html.as_ref().contains("data-name=\"TableWrapper\""));
    assert!(html.as_ref().contains("data-name=\"TableHeader\""));
    assert!(html.as_ref().contains("data-name=\"TableBody\""));
    assert!(html.as_ref().contains("data-name=\"TableRow\""));
    assert!(html.as_ref().contains("data-name=\"TableHead\""));
    assert!(html.as_ref().contains("data-name=\"TableCell\""));
    assert!(html.as_ref().contains("data-filter-count=\"0\""));
    assert!(html.as_ref().contains("data-filter-count=\"3\""));
    assert!(html.as_ref().contains(">User identifier</span>"));
    assert!(html.as_ref().contains("numeric-cell"));
    assert!(html.as_ref().contains("data-label=\"Login name\""));
    assert!(!html.as_ref().contains("class=\"table-column-filter\""));

    let query = server_admin_contract::admin_data_table_query::AdminDataTableQuery::new(
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::admin_filter_field::AdminFilterField::try_from(String::from(constants_str::catalog::LOGIN))
                    .expect("774bc583 generated_column_metadata_drives_data_table_markup invariant must hold"),
            ),
            Some(frontend_contract::filter_operation::FilterOperation::Eq),
            Some(
                server_admin_contract::admin_filter_value::AdminFilterValue::try_from(String::from(constants_str::test_fixtures::VALUE_2BD806C9))
                    .expect("63d17f8e generated_column_metadata_drives_data_table_markup invariant must hold"),
            ),
            None,
        ),
        server_admin_contract::admin_table_query::AdminTableQuery::default(),
    );
    let filters_html = crate::admin_ssr_view_ext::AdminSsrViewExt::render_admin_ssr(
        crate::data_table_grid::data_table_grid(&filter_view, &query),
    );
    assert!(
        filters_html
            .as_ref()
            .contains("class=\"table-column-heading\"")
    );
    assert!(
        filters_html
            .as_ref()
            .contains("class=\"table-column-filter\"")
    );
    assert!(!filters_html.as_ref().contains("table-filter-tools"));
    let (_before_login, login_tail) = filters_html
        .as_ref()
        .split_once(constants_str::test_fixtures::VALUE_3837854C)
        .expect("45b73477 generated_column_metadata_drives_data_table_markup invariant must hold");
    let (login_header, _after_login) = login_tail
        .split_once(constants_str::test_fixtures::VALUE_25C350AC)
        .expect("e8120a92 generated_column_metadata_drives_data_table_markup invariant must hold");
    assert!(login_header.contains("class=\"table-column-filter\""));
    assert!(login_header.contains("data-name=\"Popover\""));
    assert!(login_header.contains("data-name=\"PopoverContent\""));
    assert!(login_header.contains("data-name=\"RadioButtonGroup\""));
    let (_before_id, id_tail) = filters_html
        .as_ref()
        .split_once(constants_str::test_fixtures::VALUE_469219C9)
        .expect("c8a92ef4 generated_column_metadata_drives_data_table_markup invariant must hold");
    let (id_header, _after_id) = id_tail
        .split_once(constants_str::test_fixtures::VALUE_25C350AC)
        .expect("58cdf783 generated_column_metadata_drives_data_table_markup invariant must hold");
    assert!(!id_header.contains("class=\"table-column-filter\""));
    assert!(
        filters_html
            .as_ref()
            .contains("aria-label=\"Filter Login name\"")
    );
    assert!(
        filters_html
            .as_ref()
            .contains("popover=\"auto\" role=\"dialog\"")
    );
    assert!(filters_html.as_ref().contains(">Filter by Login name</h2>"));
    assert!(
        filters_html
            .as_ref()
            .contains("class=\"table-filter-header\"><h2>Filter by Login name</h2></div>")
    );
    assert!(
        filters_html
            .as_ref()
            .contains("popovertargetaction=\"hide\"")
    );
    assert!(
        filters_html
            .as_ref()
            .contains("name=\"filter_field\" value=\"login\"")
    );
    assert!(filters_html.as_ref().contains("name=\"filter_operation\""));
    assert!(filters_html.as_ref().contains("value=\"eq\""));
    assert!(filters_html.as_ref().contains("value=\"regex\""));
    assert!(filters_html.as_ref().contains("name=\"filter_value\""));
    assert!(filters_html.as_ref().contains("type=\"text\""));
    assert!(filters_html.as_ref().contains("value=\"alice\""));
    assert!(
        filters_html
            .as_ref()
            .contains("singlestage-label table-filter-input-label")
    );
    assert!(filters_html.as_ref().contains("<span>Value</span>"));
    assert!(filters_html.as_ref().contains("data-name=\"Input\""));
    assert!(!filters_html.as_ref().contains("placeholder=\"Value\""));
    assert!(filters_html.as_ref().contains("<span>Start</span>"));
    assert!(filters_html.as_ref().contains("<span>End</span>"));
    assert!(filters_html.as_ref().contains("placeholder=\"Start\""));
    assert!(filters_html.as_ref().contains("name=\"filter_end\""));
    assert!(filters_html.as_ref().contains("placeholder=\"End\""));
    assert!(filters_html.as_ref().contains(">Clear</a>"));
    let apply_position = filters_html
        .as_ref()
        .find(constants_str::test_fixtures::VALUE_38228244)
        .expect("10c26d45 generated_column_metadata_drives_data_table_markup invariant must hold");
    let close_position = filters_html
        .as_ref()
        .find(constants_str::test_fixtures::VALUE_0D4379EB)
        .expect("1542a5c3 generated_column_metadata_drives_data_table_markup invariant must hold");
    let clear_position = filters_html
        .as_ref()
        .find(constants_str::test_fixtures::VALUE_BD7A6256)
        .expect("58f35e11 generated_column_metadata_drives_data_table_markup invariant must hold");
    assert!(close_position > apply_position);
    assert!(clear_position > apply_position);
    assert!(clear_position > close_position);
    assert_eq!(
        filters_html
            .as_ref()
            .matches("action=\"/admin/role_permissions\"")
            .count(),
        constants_usize::ONE
    );
    assert_eq!(
        filters_html.as_ref().matches(">Apply</button>").count(),
        constants_usize::ONE
    );
    assert!(
        filters_html
            .as_ref()
            .contains("href=\"/admin/role_permissions\"")
    );
    assert!(!filters_html.as_ref().contains("name=\"table\""));
    assert!(!filters_html.as_ref().contains("?table="));
    assert_eq!(
        filters_html
            .as_ref()
            .matches("class=\"table-filter-form\"")
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
    assert!(page_html.as_ref().contains("data-field=\"login\""));
    assert!(
        page_html
            .as_ref()
            .contains("action=\"/admin/role_permissions\"")
    );
    let empty_html = crate::render_data_tables::render_data_tables(None, &query, &admin, &branding);
    assert!(!empty_html.as_ref().contains("class=\"table-scroll\""));
    assert!(
        empty_html
            .as_ref()
            .contains(server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.get())
    );
}
