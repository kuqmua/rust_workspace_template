#[cfg(test)]
use super::super::AdminSsrViewExt;
#[cfg(test)]
use super::{test_admin, test_branding};

#[test]
fn generated_column_metadata_drives_data_table_markup() {
    let columns = server_admin_contract::AdminDataColumns::try_from(vec![
        server_admin_contract::AdminDataColumn::new(
            server_admin_contract::AdminDataFilters::try_from(Vec::new()).expect("2239fb0a"),
            server_admin_contract::AdminDataInputKind::Number,
            server_admin_contract::AdminText::try_from(String::from("User identifier"))
                .expect("f707908b"),
            server_admin_contract::AdminText::try_from(String::from("id")).expect("694184c1"),
        ),
        server_admin_contract::AdminDataColumn::new(
            server_admin_contract::AdminDataFilters::try_from(vec![
                server_admin_contract::AdminDataFilter::from(
                    frontend_contract::FilterOperation::Eq,
                ),
                server_admin_contract::AdminDataFilter::from(
                    frontend_contract::FilterOperation::Regex,
                ),
                server_admin_contract::AdminDataFilter::from(
                    frontend_contract::FilterOperation::Between,
                ),
            ])
            .expect("5ba25cf7"),
            server_admin_contract::AdminDataInputKind::Text,
            server_admin_contract::AdminText::try_from(String::from("Login name"))
                .expect("0336b6ad"),
            server_admin_contract::AdminText::try_from(String::from("login")).expect("fdcaa4d2"),
        ),
    ])
    .expect("57462ad9");
    let values = server_admin_contract::AdminTexts::try_from(vec![
        server_admin_contract::AdminText::try_from(String::from("42")).expect("32862269"),
        server_admin_contract::AdminText::try_from(String::from("alice")).expect("77e6370f"),
    ])
    .expect("58fed1d1");
    let rows = server_admin_contract::AdminDataRows::try_from(vec![
        server_admin_contract::AdminDataRow::new(values),
    ])
    .expect("ac944ccc");
    let view = server_admin_contract::AdminDataTableView::new(
        columns.clone(),
        rows.clone(),
        server_admin_contract::AdminDataTable::Users,
        server_admin_contract::AdminPageTotal::from(1u64),
    );
    let filter_view = server_admin_contract::AdminDataTableView::new(
        columns,
        rows,
        server_admin_contract::AdminDataTable::RolePermissions,
        server_admin_contract::AdminPageTotal::from(1u64),
    );

    let default_query = server_admin_contract::AdminDataTableQuery::default();
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

    let query = server_admin_contract::AdminDataTableQuery::new(
        server_admin_contract::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::AdminFilterField::try_from(String::from("login"))
                    .expect("774bc583"),
            ),
            Some(frontend_contract::FilterOperation::Eq),
            Some(
                server_admin_contract::AdminFilterValue::try_from(String::from("alice"))
                    .expect("63d17f8e"),
            ),
            None,
        ),
        server_admin_contract::AdminTableQuery::default(),
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
        .expect("45b73477");
    let (login_header, _after_login) = login_tail.split_once("</th>").expect("e8120a92");
    assert!(login_header.contains("class=\"table-column-filter\""));
    assert!(login_header.contains("data-name=\"Popover\""));
    assert!(login_header.contains("data-name=\"PopoverContent\""));
    assert!(login_header.contains("data-name=\"RadioButtonGroup\""));
    let (_before_id, id_tail) = filters_html
        .as_ref()
        .split_once("data-field=\"id\"")
        .expect("c8a92ef4");
    let (id_header, _after_id) = id_tail.split_once("</th>").expect("58cdf783");
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
        .expect("10c26d45");
    let close_position = filters_html
        .as_ref()
        .find(">Close</button>")
        .expect("1542a5c3");
    let clear_position = filters_html.as_ref().find(">Clear</a>").expect("58f35e11");
    assert!(close_position > apply_position);
    assert!(clear_position > apply_position);
    assert!(clear_position > close_position);
    assert_eq!(
        filters_html
            .as_ref()
            .matches("action=\"/admin/role_permissions\"")
            .count(),
        1usize
    );
    assert_eq!(
        filters_html.as_ref().matches(">Apply</button>").count(),
        1usize
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
        1usize
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
            .contains(server_admin_contract::AdminHtmlAction::SignOut.get())
    );
}
