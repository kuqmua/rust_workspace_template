#[test]
fn test_server_rendered_pages_contain_forms_and_no_scripts() {
    let sign_in = crate::render_sign_in::render_sign_in(None, None);
    assert!(sign_in.as_ref().contains("<form method=\"post\""));
    assert!(!sign_in.as_ref().contains("TOTP"));
    assert!(!sign_in.as_ref().contains("recovery code"));
    assert_eq!(
        sign_in.as_ref().matches("<form method=\"post\"").count(),
        constants_usize::ONE
    );
    assert!(!sign_in.as_ref().contains("<h1"));
    assert!(!sign_in.as_ref().contains("<h2"));
    assert!(!sign_in.as_ref().contains("<script"));
    assert!(!sign_in.as_ref().contains(".wasm"));
    let failed_sign_in = crate::render_sign_in::render_sign_in(
        Some(
            crate::admin_ssr_error_message::AdminSsrErrorMessage::try_from(String::from(
                constants_str::VALUE_7EC371A9,
            ))
            .expect(constants_str::DIAGNOSTIC_31B0D69F),
        ),
        None,
    );
    assert!(
        failed_sign_in
            .as_ref()
            .contains("Invalid credentials</div>")
    );
    assert!(failed_sign_in.as_ref().contains("role=\"alert\""));

    let page = crate::render_admin_page::render_admin_page(
        server_admin_contract::admin_page::AdminPage::Users,
        crate::admin_ssr_html::AdminSsrHtml::try_from(String::from(constants_str::VALUE_91B66961))
            .expect(constants_str::DIAGNOSTIC_C78BD3A1),
    );
    assert!(page.as_ref().contains("<p>ready</p>"));
    assert!(!page.as_ref().contains("<h1"));
    assert!(!page.as_ref().contains("<h2"));
    assert!(!page.as_ref().contains("class=\"brand\""));
    assert!(!page.as_ref().contains("nav-dot"));
    assert!(page.as_ref().contains(">swagger_ui</a>"));
    assert!(page.as_ref().contains(">settings</a>"));
    assert!(!page.as_ref().contains(">api</a>"));
    assert!(
        page.as_ref().contains(
            format!(
                "{}</button></form></li></ul></nav>",
                server_admin_contract::admin_html_action::AdminHtmlAction::SignOut
                    .route_name()
                    .as_ref()
            )
            .as_str()
        )
    );
    assert!(!page.as_ref().contains("<script"));
}

#[test]
fn test_header_table_labels_match_table_names_and_routes() {
    let page = crate::render_admin_page::render_admin_page(
        server_admin_contract::admin_page::AdminPage::Users,
        crate::admin_ssr_html::AdminSsrHtml::try_from(String::new())
            .expect(constants_str::DIAGNOSTIC_5A984C96),
    );

    assert!(
        server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER
            .into_iter()
            .all(|table| {
                let table_name = table.to_string();
                let route = table.frontend_path().to_string();
                let route_name = route
                    .rsplit_once('/')
                    .map(|(_prefix, name)| name)
                    .expect(constants_str::DIAGNOSTIC_100762F4);
                let href = format!("href=\"{route}\"");
                let header_label = page
                    .as_ref()
                    .split_once(href.as_str())
                    .and_then(|(_prefix, link_tail)| link_tail.split_once('>'))
                    .and_then(|(_attributes, label_tail)| label_tail.split_once("</a>"))
                    .map_or("", |(label, _suffix)| label);

                route_name == table_name && header_label == table_name
            })
    );
}

#[test]
fn test_header_items_stay_stable_between_static_and_table_pages() {
    let metrics = crate::render_admin_page::render_admin_page(
        server_admin_contract::admin_page::AdminPage::Metrics,
        crate::admin_ssr_html::AdminSsrHtml::try_from(String::new())
            .expect(constants_str::DIAGNOSTIC_F2D57BB4),
    );
    let cleanup_status =
        crate::render_admin_page_with_table_access::render_admin_page_with_table_access(
            server_admin_contract::admin_page::AdminPage::Tables,
            crate::admin_ssr_html::AdminSsrHtml::try_from(String::new())
                .expect(constants_str::DIAGNOSTIC_7F46CFD6),
            None,
            None,
            Some(server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus),
        );
    let normalized_header = |html: &crate::admin_ssr_html::AdminSsrHtml| {
        html.as_ref()
            .split_once(constants_str::VALUE_75322EEF)
            .and_then(|(_prefix, header_tail)| {
                header_tail.split_once(constants_str::VALUE_5034F288)
            })
            .map_or_else(String::new, |(header, _suffix)| {
                header
                    .replace(
                        constants_str::VALUE_C067F6CF,
                        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
                    )
                    .replace(constants_str::VALUE_80E35525, constants_str::VALUE_A5C068D6)
                    .replace(
                        constants_str::VALUE_319B0378,
                        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
                    )
            })
    };
    let metrics_header = normalized_header(&metrics);
    let cleanup_status_header = normalized_header(&cleanup_status);

    assert!(!metrics_header.is_empty());
    assert_eq!(metrics_header, cleanup_status_header);
    assert!(metrics_header.contains(">swagger_ui</a>"));
    assert!(!metrics_header.contains(">api</a>"));
}

#[test]
fn test_csr_page_contains_only_csr_application_shell() {
    let admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(
            constants_str::ADMIN.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_642357A8),
        server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_41856438),
        server_admin_contract::admin_login::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect(constants_str::DIAGNOSTIC_71A3B6E5),
        server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_8E3CF81F),
        server_admin_contract::admin_role_names::AdminRoleNames::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_A5677F33),
    );
    let settings = server_admin_contract::admin_settings_view::AdminSettingsView::new(
        server_admin_contract::admin_default_route::AdminDefaultRoute::try_from(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_44758B19),
        None,
        None,
        None,
        None,
        server_admin_contract::admin_site_name::AdminSiteName::try_from(String::from(
            constants_str::ADMIN,
        ))
        .expect(constants_str::DIAGNOSTIC_8BA6B381),
        None,
        None,
    );
    let branding =
        server_admin_contract::admin_branding_view::AdminBrandingView::from_settings(&settings);
    let html = crate::render_admin_csr::render_admin_csr(
        server_admin_contract::admin_page::AdminPage::Users,
        None,
        &admin,
        &branding,
    );

    assert!(html.as_ref().contains("id=\"admin-csr-root\""));
    assert!(html.as_ref().contains("loading-spinner size-4"));
    assert!(html.as_ref().contains("aria-live=\"polite\""));
    assert!(
        html.as_ref()
            .contains("src=\"/admin/assets/admin_csr_application.js?v=20260801-37\"")
    );
    assert!(!html.as_ref().contains("<nav"));
    assert!(!html.as_ref().contains("<table"));
    assert!(!html.as_ref().contains("<form"));

    let table_html = crate::render_data_tables_csr::render_data_tables_csr(
        Some(server_admin_contract::admin_data_table::AdminDataTable::Users),
        &admin,
        &branding,
    );
    assert!(table_html.as_ref().contains("id=\"admin-csr-root\""));
    assert!(
        table_html
            .as_ref()
            .contains("src=\"/admin/assets/admin_csr_application.js?v=20260801-37\"")
    );
}
