#[test]
fn server_rendered_pages_contain_forms_and_no_scripts() {
    let sign_in = super::super::render_sign_in(None, None);
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
    let failed_sign_in = super::super::render_sign_in(
        Some(
            super::super::AdminSsrErrorMessage::try_from(String::from("Invalid credentials"))
                .expect("31b0d69f server_rendered_pages_contain_forms_and_no_scripts invariant must hold"),
        ),
        None,
    );
    assert!(
        failed_sign_in
            .as_ref()
            .contains("Invalid credentials</div>")
    );
    assert!(failed_sign_in.as_ref().contains("role=\"alert\""));

    let page = super::super::render_admin_page(
        server_admin_contract::AdminPage::Users,
        super::super::AdminSsrHtml::try_from(String::from("<p>ready</p>")).expect(
            "c78bd3a1 server_rendered_pages_contain_forms_and_no_scripts invariant must hold",
        ),
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
                server_admin_contract::AdminHtmlAction::SignOut
                    .route_name()
                    .as_ref()
            )
            .as_str()
        )
    );
    assert!(!page.as_ref().contains("<script"));
}

#[test]
fn header_table_labels_match_table_names_and_routes() {
    let page = super::super::render_admin_page(
        server_admin_contract::AdminPage::Users,
        super::super::AdminSsrHtml::try_from(String::new()).expect(
            "5a984c96 header_table_labels_match_table_names_and_routes invariant must hold",
        ),
    );

    assert!(
        server_admin_contract::AdminDataTable::PG_ORDER
            .into_iter()
            .all(|table| {
                let table_name = table.to_string();
                let route = table.frontend_path().to_string();
                let route_name = route.rsplit_once('/').map(|(_prefix, name)| name).expect(
                    "100762f4 header_table_labels_match_table_names_and_routes invariant must hold",
                );
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
fn header_items_stay_stable_between_static_and_table_pages() {
    let metrics = super::super::render_admin_page(
        server_admin_contract::AdminPage::Metrics,
        super::super::AdminSsrHtml::try_from(String::new()).expect(
            "f2d57bb4 header_items_stay_stable_between_static_and_table_pages invariant must hold",
        ),
    );
    let cleanup_status = super::super::render_admin_page_with_table_access(
        server_admin_contract::AdminPage::Tables,
        super::super::AdminSsrHtml::try_from(String::new()).expect(
            "7f46cfd6 header_items_stay_stable_between_static_and_table_pages invariant must hold",
        ),
        None,
        None,
        Some(server_admin_contract::AdminDataTable::CleanupStatus),
    );
    let normalized_header = |html: &super::super::AdminSsrHtml| {
        html.as_ref()
                .split_once("<header")
                .and_then(|(_prefix, header_tail)| header_tail.split_once("</header>"))
                .map_or_else(String::new, |(header, _suffix)| {
                    header
                        .replace(" aria-current=\"page\"", "")
                        .replace(
                            "active inline-flex items-center rounded-sm text-sm font-medium text-foreground transition-colors focus:outline-none",
                            "inline-flex items-center rounded-sm text-sm font-medium text-foreground/70 transition-colors hover:text-foreground focus:outline-none",
                        )
                        .replace(" class=\"\"", "")
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
fn csr_page_contains_only_bootstrap_shell() {
    let admin = server_admin_contract::AuthenticatedAdmin::new(
        server_admin_contract::AdminDisplayName::try_from(constants_str::ADMIN.to_owned())
            .expect("642357a8 csr_page_contains_only_bootstrap_shell invariant must hold"),
        server_admin_contract::AdminUserId::try_from(constants_i64::ONE)
            .expect("41856438 csr_page_contains_only_bootstrap_shell invariant must hold"),
        server_admin_contract::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect("71a3b6e5 csr_page_contains_only_bootstrap_shell invariant must hold"),
        server_admin_contract::AdminPermissionValues::try_from(Vec::new())
            .expect("8e3cf81f csr_page_contains_only_bootstrap_shell invariant must hold"),
        server_admin_contract::AdminRoleNames::try_from(Vec::new())
            .expect("a5677f33 csr_page_contains_only_bootstrap_shell invariant must hold"),
    );
    let settings = server_admin_contract::AdminSettingsView::new(
        server_admin_contract::AdminDefaultRoute::try_from(
            server_admin_contract::AdminFrontendPath::Users
                .get()
                .to_owned(),
        )
        .expect("44758b19 csr_page_contains_only_bootstrap_shell invariant must hold"),
        None,
        None,
        None,
        None,
        server_admin_contract::AdminSiteName::try_from(String::from("Admin"))
            .expect("8ba6b381 csr_page_contains_only_bootstrap_shell invariant must hold"),
        None,
        None,
    );
    let branding = server_admin_contract::AdminBrandingView::from_settings(&settings);
    let html = super::super::render_admin_csr(
        server_admin_contract::AdminPage::Users,
        None,
        &admin,
        &branding,
    );

    assert!(html.as_ref().contains("id=\"admin-csr-root\""));
    assert!(html.as_ref().contains("loading-spinner size-4"));
    assert!(html.as_ref().contains("aria-live=\"polite\""));
    assert!(
        html.as_ref()
            .contains("src=\"/admin/assets/csr_bootstrap.js?v=20260801-37\"")
    );
    assert!(!html.as_ref().contains("<nav"));
    assert!(!html.as_ref().contains("<table"));
    assert!(!html.as_ref().contains("<form"));

    let table_html = super::super::render_data_tables_csr(
        Some(server_admin_contract::AdminDataTable::Users),
        &admin,
        &branding,
    );
    assert!(table_html.as_ref().contains("id=\"admin-csr-root\""));
    assert!(
        table_html
            .as_ref()
            .contains("src=\"/admin/assets/csr_bootstrap.js?v=20260801-37\"")
    );
}
