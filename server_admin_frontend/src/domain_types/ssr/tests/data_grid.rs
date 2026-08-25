#[cfg(test)]
use super::super::AdminSsrViewExt;
#[cfg(test)]
use super::{test_admin, test_branding};

#[test]
fn generated_column_metadata_drives_data_table_markup() {
    let columns = server_admin_contract::domain_types::AdminDataColumns::try_from(vec![
        server_admin_contract::domain_types::AdminDataColumn::new(
            server_admin_contract::domain_types::AdminDataFilters::try_from(Vec::new()).expect(
                "2239fb0a generated_column_metadata_drives_data_table_markup invariant must hold",
            ),
            server_admin_contract::domain_types::AdminDataInputKind::Number,
            server_admin_contract::domain_types::AdminText::try_from(String::from("User identifier")).expect(
                "f707908b generated_column_metadata_drives_data_table_markup invariant must hold",
            ),
            server_admin_contract::domain_types::AdminText::try_from(String::from("id")).expect(
                "694184c1 generated_column_metadata_drives_data_table_markup invariant must hold",
            ),
        ),
        server_admin_contract::domain_types::AdminDataColumn::new(
            server_admin_contract::domain_types::AdminDataFilters::try_from(vec![
                server_admin_contract::domain_types::AdminDataFilter::from(
                    frontend_contract::domain_types::FilterOperation::Eq,
                ),
                server_admin_contract::domain_types::AdminDataFilter::from(
                    frontend_contract::domain_types::FilterOperation::Regex,
                ),
                server_admin_contract::domain_types::AdminDataFilter::from(
                    frontend_contract::domain_types::FilterOperation::Between,
                ),
            ])
            .expect(
                "5ba25cf7 generated_column_metadata_drives_data_table_markup invariant must hold",
            ),
            server_admin_contract::domain_types::AdminDataInputKind::Text,
            server_admin_contract::domain_types::AdminText::try_from(String::from("Login name")).expect(
                "0336b6ad generated_column_metadata_drives_data_table_markup invariant must hold",
            ),
            server_admin_contract::domain_types::AdminText::try_from(String::from("login")).expect(
                "fdcaa4d2 generated_column_metadata_drives_data_table_markup invariant must hold",
            ),
        ),
    ])
    .expect("57462ad9 generated_column_metadata_drives_data_table_markup invariant must hold");
    let values = server_admin_contract::domain_types::AdminTexts::try_from(vec![
        server_admin_contract::domain_types::AdminText::try_from(String::from("42")).expect(
            "32862269 generated_column_metadata_drives_data_table_markup invariant must hold",
        ),
        server_admin_contract::domain_types::AdminText::try_from(String::from("alice")).expect(
            "77e6370f generated_column_metadata_drives_data_table_markup invariant must hold",
        ),
    ])
    .expect("58fed1d1 generated_column_metadata_drives_data_table_markup invariant must hold");
    let rows = server_admin_contract::domain_types::AdminDataRows::try_from(vec![
        server_admin_contract::domain_types::AdminDataRow::new(values),
    ])
    .expect("ac944ccc generated_column_metadata_drives_data_table_markup invariant must hold");
    let view = server_admin_contract::domain_types::AdminDataTableView::new(
        columns.clone(),
        rows.clone(),
        server_admin_contract::domain_types::AdminDataTable::Users,
        server_admin_contract::domain_types::AdminPageTotal::from(1u64),
    );
    let filter_view = server_admin_contract::domain_types::AdminDataTableView::new(
        columns,
        rows,
        server_admin_contract::domain_types::AdminDataTable::RolePermissions,
        server_admin_contract::domain_types::AdminPageTotal::from(1u64),
    );

    let default_query = server_admin_contract::domain_types::AdminDataTableQuery::default();
    let html = super::super::data_table_grid(&view, &default_query).render_admin_ssr();

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

    let query = server_admin_contract::domain_types::AdminDataTableQuery::new(
        server_admin_contract::domain_types::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::domain_types::AdminFilterField::try_from(String::from("login"))
                    .expect("774bc583 generated_column_metadata_drives_data_table_markup invariant must hold"),
            ),
            Some(frontend_contract::domain_types::FilterOperation::Eq),
            Some(
                server_admin_contract::domain_types::AdminFilterValue::try_from(String::from("alice"))
                    .expect("63d17f8e generated_column_metadata_drives_data_table_markup invariant must hold"),
            ),
            None,
        ),
        server_admin_contract::domain_types::AdminTableQuery::default(),
    );
    let filters_html = super::super::data_table_grid(&filter_view, &query).render_admin_ssr();
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
        .split_once("data-field=\"login\"")
        .expect("45b73477 generated_column_metadata_drives_data_table_markup invariant must hold");
    let (login_header, _after_login) = login_tail
        .split_once("</th>")
        .expect("e8120a92 generated_column_metadata_drives_data_table_markup invariant must hold");
    assert!(login_header.contains("class=\"table-column-filter\""));
    assert!(login_header.contains("data-name=\"Popover\""));
    assert!(login_header.contains("data-name=\"PopoverContent\""));
    assert!(login_header.contains("data-name=\"RadioButtonGroup\""));
    let (_before_id, id_tail) = filters_html
        .as_ref()
        .split_once("data-field=\"id\"")
        .expect("c8a92ef4 generated_column_metadata_drives_data_table_markup invariant must hold");
    let (id_header, _after_id) = id_tail
        .split_once("</th>")
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
        .find(">Apply</button>")
        .expect("10c26d45 generated_column_metadata_drives_data_table_markup invariant must hold");
    let close_position = filters_html
        .as_ref()
        .find(">Close</button>")
        .expect("1542a5c3 generated_column_metadata_drives_data_table_markup invariant must hold");
    let clear_position = filters_html
        .as_ref()
        .find(">Clear</a>")
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

    let admin = test_admin();
    let branding = test_branding();
    let page_html = super::super::render_data_tables(Some(&filter_view), &query, &admin, &branding);
    assert!(page_html.as_ref().contains("data-field=\"login\""));
    assert!(
        page_html
            .as_ref()
            .contains("action=\"/admin/role_permissions\"")
    );
    let empty_html = super::super::render_data_tables(None, &query, &admin, &branding);
    assert!(!empty_html.as_ref().contains("class=\"table-scroll\""));
    assert!(
        empty_html
            .as_ref()
            .contains(server_admin_contract::domain_types::AdminHtmlAction::SignOut.get())
    );
}
