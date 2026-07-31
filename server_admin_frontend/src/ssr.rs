#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the stable SSR facade delegates to screen, document, and table modules; test view rendering requires the named extension trait"
)]

mod data_tables;
mod document;
mod permissions;
mod profile;
mod roles;
mod sessions;
mod settings;
mod table;
mod text_page;
mod users;

const SSR_TEXT_MAX_BYTES: usize = 16_777_216usize;

#[cfg(test)]
trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> AdminSsrHtml;
}
#[cfg(test)]
impl<View> AdminSsrViewExt for View
where
    View: leptos::prelude::IntoAny,
{
    fn render_admin_ssr(self) -> AdminSsrHtml {
        AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(
            leptos::prelude::IntoAny::into_any(self),
        ))
        .unwrap_or_else(AdminSsrHtml::from)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("administrator SSR HTML exceeds the size limit")]
pub struct AdminSsrHtmlTryFromStringError;
impl From<AdminSsrHtmlTryFromStringError> for AdminSsrHtml {
    fn from(value: AdminSsrHtmlTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}", message = str_constants::ADMIN_SSR_TITLE_TOO_LONG)]
pub struct AdminSsrTextTryFromStringError;
impl From<AdminSsrTextTryFromStringError> for AdminSsrText {
    fn from(value: AdminSsrTextTryFromStringError) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, newtype::Display)]
pub struct AdminSsrErrorMessage(to_err_string::ErrorText);
impl TryFrom<String> for AdminSsrErrorMessage {
    type Error = to_err_string::ErrorTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        to_err_string::ErrorText::try_from(value).map(Self)
    }
}

#[derive(
    Clone, Debug, Eq, PartialEq, newtype::AsRefStr, newtype::Display, newtype::IntoInnerFrom,
)]
pub struct AdminSsrText(String);
impl TryFrom<String> for AdminSsrText {
    type Error = AdminSsrTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= SSR_TEXT_MAX_BYTES)
            .then_some(Self(value))
            .ok_or(AdminSsrTextTryFromStringError)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefStr, newtype::IntoInnerFrom)]
pub struct AdminSsrHtml(String);
impl TryFrom<String> for AdminSsrHtml {
    type Error = AdminSsrHtmlTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= SSR_TEXT_MAX_BYTES)
            .then_some(Self(value))
            .ok_or(AdminSsrHtmlTryFromStringError)
    }
}

fn render_document(title: &AdminSsrText, body: impl leptos::prelude::IntoAny) -> AdminSsrHtml {
    document::render_document(title, body)
}

#[must_use]
pub fn render_sign_in(
    error: Option<AdminSsrErrorMessage>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
) -> AdminSsrHtml {
    document::render_sign_in(error, branding)
}

#[must_use]
pub fn render_admin_page(
    page: server_admin_contract::AdminPage,
    content: AdminSsrHtml,
) -> AdminSsrHtml {
    document::render_admin_page(page, content)
}

fn render_admin_page_with_access(
    page: server_admin_contract::AdminPage,
    content: AdminSsrHtml,
    admin: Option<&server_admin_contract::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
) -> AdminSsrHtml {
    document::render_admin_page_with_access(page, content, admin, branding)
}

fn render_admin_page_with_table_access(
    page: server_admin_contract::AdminPage,
    content: AdminSsrHtml,
    admin: Option<&server_admin_contract::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
    active_table: Option<server_admin_contract::AdminDataTable>,
) -> AdminSsrHtml {
    document::render_admin_page_with_table_access(page, content, admin, branding, active_table)
}

fn table_pagination(
    page: server_admin_contract::AdminPage,
    query: &server_admin_contract::AdminTableQuery,
    total: server_admin_contract::AdminPageTotal,
    table: Option<server_admin_contract::AdminDataTable>,
    table_filter: Option<&server_admin_contract::AdminDataTableFilterQuery>,
) -> impl leptos::prelude::IntoView {
    table::table_pagination(page, query, total, table, table_filter)
}

fn data_table_grid(
    view: &server_admin_contract::AdminDataTableView,
    query: &server_admin_contract::AdminDataTableQuery,
) -> impl leptos::prelude::IntoView {
    table::data_table_grid(view, query)
}

#[allow(clippy::single_call_fn)] // isolates the metadata-driven grid for focused SSR contract testing
#[must_use]
pub fn render_users(
    page: &server_admin_contract::AdminUsersPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    users::render(page, query, admin, branding)
}

#[must_use]
pub fn render_roles(
    page: &server_admin_contract::AdminRolesPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    roles::render(page, query, admin, branding)
}

#[must_use]
pub fn render_permissions(
    page: &server_admin_contract::AdminPermissionsPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    permissions::render_permissions(page, query, admin, branding)
}

#[must_use]
pub fn render_data_tables(
    table: Option<&server_admin_contract::AdminDataTableView>,
    query: &server_admin_contract::AdminDataTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    data_tables::ssr::render_data_tables(table, query, admin, branding)
}

#[must_use]
pub fn render_data_tables_csr(
    active_table: Option<server_admin_contract::AdminDataTable>,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    data_tables::csr::render_data_tables_csr(active_table, admin, branding)
}

#[must_use]
pub fn render_admin_csr(
    page: server_admin_contract::AdminPage,
    active_table: Option<server_admin_contract::AdminDataTable>,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    data_tables::csr::render_admin_csr(page, active_table, admin, branding)
}

#[must_use]
pub fn render_sessions(
    page: &server_admin_contract::AdminSessionsPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    sessions::render_sessions(page, query, admin, branding)
}

#[must_use]
pub fn render_profile(
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    profile::render_profile(admin, branding)
}

#[must_use]
pub fn render_settings(
    view: &server_admin_contract::AdminSettingsView,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    settings::render_settings(view, admin, branding)
}

#[must_use]
pub fn render_text_page(
    page: server_admin_contract::AdminPage,
    title: AdminSsrText,
    text: AdminSsrText,
) -> AdminSsrHtml {
    text_page::render_text_page(page, title, text)
}

#[must_use]
pub fn render_text_page_with_access(
    page: server_admin_contract::AdminPage,
    title: AdminSsrText,
    text: AdminSsrText,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    text_page::render_text_page_with_access(page, title, text, admin, branding)
}

#[cfg(test)]
mod tests {
    use super::AdminSsrViewExt;

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
                server_admin_contract::AdminText::try_from(String::from("login"))
                    .expect("fdcaa4d2"),
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
        let html = super::data_table_grid(&view, &default_query).render_admin_ssr();

        assert!(html.as_ref().contains("data-field=\"id\""));
        assert!(html.as_ref().contains("data-filter-count=\"0\""));
        assert!(html.as_ref().contains("data-filter-count=\"3\""));
        assert!(html.as_ref().contains(">User identifier</span>"));
        assert!(html.as_ref().contains("class=\"numeric-cell\""));
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
        let filters_html = super::data_table_grid(&filter_view, &query).render_admin_ssr();
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
            .split_once("<th data-field=\"login\"")
            .expect("45b73477");
        let (login_header, _after_login) = login_tail.split_once("</th>").expect("e8120a92");
        assert!(login_header.contains("class=\"table-column-filter\""));
        assert!(!login_header.contains("<details class=\"table-column-filter\" open"));
        let (_before_id, id_tail) = filters_html
            .as_ref()
            .split_once("<th data-field=\"id\"")
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
                .contains("role=\"dialog\" aria-modal=\"true\"")
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
                .contains("class=\"table-filter-close-label\">Close</span>")
        );
        assert!(
            filters_html
                .as_ref()
                .contains("name=\"filter_field\" value=\"login\"")
        );
        assert!(
            filters_html
                .as_ref()
                .contains("name=\"filter_operation\" value=\"eq\"")
        );
        assert!(
            filters_html
                .as_ref()
                .contains("name=\"filter_operation\" value=\"regex\"")
        );
        assert!(
            filters_html
                .as_ref()
                .contains("name=\"filter_value\" type=\"text\" value=\"alice\"")
        );
        assert!(filters_html.as_ref().contains(
            "class=\"table-filter-input-label\"><span>Value</span><input name=\"filter_value\""
        ));
        assert!(!filters_html.as_ref().contains("placeholder=\"Value\""));
        assert!(filters_html.as_ref().contains(
            "class=\"table-filter-input-label\"><span>Start</span><input name=\"filter_value\""
        ));
        assert!(filters_html.as_ref().contains(
            "class=\"table-filter-input-label\"><span>End</span><input name=\"filter_end\""
        ));
        assert!(
            filters_html
                .as_ref()
                .contains("name=\"filter_value\" type=\"text\" value=\"\" placeholder=\"Start\"")
        );
        assert!(
            filters_html
                .as_ref()
                .contains("name=\"filter_end\" type=\"text\" value=\"\" placeholder=\"End\"")
        );
        assert!(filters_html.as_ref().contains(">Clear</a>"));
        let apply_position = filters_html
            .as_ref()
            .find(">Apply</button>")
            .expect("10c26d45");
        let close_position = filters_html
            .as_ref()
            .find("class=\"table-filter-close\"")
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
    }

    #[test]
    fn server_rendered_pages_contain_forms_and_no_scripts() {
        let sign_in = super::render_sign_in(None, None);
        assert!(sign_in.as_ref().contains("<form method=\"post\""));
        assert!(!sign_in.as_ref().contains("TOTP"));
        assert!(!sign_in.as_ref().contains("recovery code"));
        assert_eq!(
            sign_in.as_ref().matches("<form method=\"post\"").count(),
            1usize
        );
        assert!(!sign_in.as_ref().contains("<h1"));
        assert!(!sign_in.as_ref().contains("<h2"));
        assert!(!sign_in.as_ref().contains("<script"));
        assert!(!sign_in.as_ref().contains(".wasm"));

        let page = super::render_admin_page(
            server_admin_contract::AdminPage::Users,
            super::AdminSsrHtml::try_from(String::from("<p>ready</p>")).expect("c78bd3a1"),
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
                    "{}</button></form></nav>",
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
        let page = super::render_admin_page(
            server_admin_contract::AdminPage::Users,
            super::AdminSsrHtml::try_from(String::new()).expect("5a984c96"),
        );

        assert!(
            server_admin_contract::AdminDataTable::PG_ORDER
                .into_iter()
                .all(|table| {
                    let table_name = table.to_string();
                    let route = table.frontend_path().to_string();
                    let route_name = route
                        .rsplit_once('/')
                        .map(|(_prefix, name)| name)
                        .expect("100762f4");
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
        let metrics = super::render_admin_page(
            server_admin_contract::AdminPage::Metrics,
            super::AdminSsrHtml::try_from(String::new()).expect("f2d57bb4"),
        );
        let cleanup_status = super::render_admin_page_with_table_access(
            server_admin_contract::AdminPage::Tables,
            super::AdminSsrHtml::try_from(String::new()).expect("7f46cfd6"),
            None,
            None,
            Some(server_admin_contract::AdminDataTable::CleanupStatus),
        );
        let normalized_header = |html: &super::AdminSsrHtml| {
            html.as_ref()
                .split_once("<header")
                .and_then(|(_prefix, header_tail)| header_tail.split_once("</header>"))
                .map_or_else(String::new, |(header, _suffix)| {
                    header
                        .replace(" class=\"active\"", "")
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
            server_admin_contract::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
                .expect("642357a8"),
            server_admin_contract::AdminUserId::try_from(1i64).expect("41856438"),
            server_admin_contract::AdminLogin::try_from(str_constants::ROOT.to_owned())
                .expect("71a3b6e5"),
            server_admin_contract::AdminPermissionValues::try_from(Vec::new()).expect("8e3cf81f"),
            server_admin_contract::AdminRoleNames::try_from(Vec::new()).expect("a5677f33"),
        );
        let settings = server_admin_contract::AdminSettingsView::new(
            server_admin_contract::AdminDefaultRoute::try_from(
                server_admin_contract::AdminFrontendPath::Users
                    .get()
                    .to_owned(),
            )
            .expect("44758b19"),
            None,
            None,
            None,
            None,
            server_admin_contract::AdminSiteName::try_from(String::from("Admin"))
                .expect("8ba6b381"),
            None,
            None,
        );
        let branding = server_admin_contract::AdminBrandingView::from_settings(&settings);
        let html = super::render_admin_csr(
            server_admin_contract::AdminPage::Users,
            None,
            &admin,
            &branding,
        );

        assert!(html.as_ref().contains("id=\"admin-csr-root\""));
        assert!(html.as_ref().contains("class=\"loading-spinner\""));
        assert!(html.as_ref().contains("aria-live=\"polite\""));
        assert!(
            html.as_ref()
                .contains("src=\"/admin/assets/csr_bootstrap.js?v=20260730-36\"")
        );
        assert!(!html.as_ref().contains("<nav"));
        assert!(!html.as_ref().contains("<table"));
        assert!(!html.as_ref().contains("<form"));
    }

    #[test]
    fn settings_page_uses_centered_layout_container() {
        let settings = server_admin_contract::AdminSettingsView::new(
            server_admin_contract::AdminDefaultRoute::try_from(
                server_admin_contract::AdminFrontendPath::Users
                    .get()
                    .to_owned(),
            )
            .expect("92b485cf"),
            None,
            None,
            None,
            None,
            server_admin_contract::AdminSiteName::try_from(str_constants::ADMIN.to_owned())
                .expect("bbf5f240"),
            None,
            None,
        );
        let admin = server_admin_contract::AuthenticatedAdmin::new(
            server_admin_contract::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
                .expect("a0eb7df6"),
            server_admin_contract::AdminUserId::try_from(1i64).expect("9ff62b22"),
            server_admin_contract::AdminLogin::try_from(str_constants::ROOT.to_owned())
                .expect("984553cd"),
            server_admin_contract::AdminPermissionValues::try_from(Vec::new()).expect("86848eb5"),
            server_admin_contract::AdminRoleNames::try_from(Vec::new()).expect("d3f8287b"),
        );
        let branding = server_admin_contract::AdminBrandingView::from_settings(&settings);
        let html = super::render_settings(&settings, &admin, &branding);
        assert!(
            html.as_ref()
                .contains("<section class=\"settings-grid\"><article class=\"settings-card\">")
        );
    }

    #[test]
    fn pagination_preserves_server_side_navigation() {
        let html = super::table_pagination(
            server_admin_contract::AdminPage::Users,
            &server_admin_contract::AdminTableQuery::default(),
            server_admin_contract::AdminPageTotal::from(101u64),
            None,
            None,
        )
        .render_admin_ssr();
        assert!(html.as_ref().contains("class=\"table-page-size\""));
        assert!(
            html.as_ref()
                .contains("<span>Rows</span><input name=\"limit\" type=\"number\"")
        );
        assert!(html.as_ref().contains("name=\"offset\" value=\"20\""));
        assert!(html.as_ref().contains("disabled>Previous"));
        assert!(!html.as_ref().contains("<script"));

        let table_filter = server_admin_contract::AdminDataTableFilterQuery::new(
            Some(
                server_admin_contract::AdminFilterField::try_from(String::from("login"))
                    .expect("7eb9a214"),
            ),
            Some(frontend_contract::FilterOperation::Eq),
            Some(
                server_admin_contract::AdminFilterValue::try_from(String::from("alice"))
                    .expect("2629c095"),
            ),
            None,
        );
        let filtered_html = super::table_pagination(
            server_admin_contract::AdminPage::Tables,
            &server_admin_contract::AdminTableQuery::default(),
            server_admin_contract::AdminPageTotal::from(101u64),
            Some(server_admin_contract::AdminDataTable::RolePermissions),
            Some(&table_filter),
        )
        .render_admin_ssr();
        assert_eq!(
            filtered_html
                .as_ref()
                .matches("name=\"filter_field\" value=\"login\"")
                .count(),
            3usize
        );
        assert_eq!(
            filtered_html
                .as_ref()
                .matches("name=\"filter_operation\" value=\"eq\"")
                .count(),
            3usize
        );
        assert_eq!(
            filtered_html
                .as_ref()
                .matches("name=\"filter_value\" value=\"alice\"")
                .count(),
            3usize
        );
        assert_eq!(
            filtered_html
                .as_ref()
                .matches("action=\"/admin/role_permissions\"")
                .count(),
            3usize
        );
        assert!(!filtered_html.as_ref().contains("name=\"table\""));
        assert!(!filtered_html.as_ref().contains("?table="));
    }

    #[test]
    fn navigation_only_contains_accessible_pages() {
        let admin = server_admin_contract::AuthenticatedAdmin::new(
            server_admin_contract::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
                .expect("cdae3e58"),
            server_admin_contract::AdminUserId::try_from(1i64).expect("4ff30835"),
            server_admin_contract::AdminLogin::try_from(str_constants::ROOT.to_owned())
                .expect("9ae5b850"),
            server_admin_contract::AdminPermissionValues::try_from(vec![
                server_admin_contract::AdminPermissionValue::try_from(
                    server_admin_contract::AdminPermission::UsersRead
                        .as_str()
                        .get()
                        .to_owned(),
                )
                .expect("6afb4194"),
                server_admin_contract::AdminPermissionValue::try_from(
                    server_admin_contract::AdminPermission::TablesRead
                        .as_str()
                        .get()
                        .to_owned(),
                )
                .expect("2c507520"),
                server_admin_contract::AdminPermissionValue::try_from(
                    server_admin_contract::AdminPermission::AccessSessionsRead
                        .as_str()
                        .get()
                        .to_owned(),
                )
                .expect("7e7147f6"),
            ])
            .expect("e05ce0b9"),
            server_admin_contract::AdminRoleNames::try_from(Vec::new()).expect("f1ec0093"),
        );
        let html = super::render_admin_page_with_access(
            server_admin_contract::AdminPage::Users,
            super::AdminSsrHtml::try_from(String::new()).expect("aa3fa21e"),
            Some(&admin),
            None,
        );
        assert!(
            html.as_ref()
                .contains(server_admin_contract::AdminFrontendPath::Users.get())
        );
        assert!(
            !html
                .as_ref()
                .contains(server_admin_contract::AdminFrontendPath::Roles.get())
        );
        assert!(
            !html
                .as_ref()
                .contains(server_admin_contract::AdminFrontendPath::Permissions.get())
        );
        assert!(
            !html
                .as_ref()
                .contains(server_admin_contract::AdminFrontendPath::Settings.get())
        );
        assert!(
            html.as_ref()
                .contains(server_admin_contract::AdminFrontendPath::Sessions.get())
        );
        assert!(
            html.as_ref()
                .contains(server_admin_contract::AdminFrontendPath::Profile.get())
        );
        assert!(
            html.as_ref().contains(
                server_admin_contract::AdminDataTable::AccessSessions
                    .frontend_path()
                    .as_ref()
            )
        );
        let users_table = html
            .as_ref()
            .find("href=\"/admin/users\"")
            .expect("7017fe5d");
        let sessions_table = html
            .as_ref()
            .find("href=\"/admin/access_sessions\"")
            .expect("9510971f");
        let profile_page = html
            .as_ref()
            .find("href=\"/admin/profile\"")
            .expect("21570a0c");
        let sessions_page = html
            .as_ref()
            .find("href=\"/admin/sessions\"")
            .expect("ba431a21");
        let sign_out = html
            .as_ref()
            .find(server_admin_contract::AdminHtmlAction::SignOut.get())
            .expect("46d23e89");
        assert!(users_table < sessions_table);
        assert!(sessions_table < profile_page);
        assert!(profile_page < sessions_page);
        assert!(sessions_page < sign_out);
        assert!(
            html.as_ref().contains(
                server_admin_contract::AdminDataTable::Users
                    .frontend_path()
                    .as_ref()
            )
        );
        assert!(!html.as_ref().contains("?table="));
    }

    #[test]
    fn sign_in_uses_server_side_color_without_logo() {
        let settings = server_admin_contract::AdminSettingsView::new(
            server_admin_contract::AdminDefaultRoute::try_from(
                server_admin_contract::AdminFrontendPath::Users
                    .get()
                    .to_owned(),
            )
            .expect("50ffe2fc"),
            None,
            None,
            None,
            Some(
                server_admin_contract::AdminPrimaryColor::try_from(String::from("#123456"))
                    .expect("9c08c954"),
            ),
            server_admin_contract::AdminSiteName::try_from(String::from("Custom Admin"))
                .expect("0a28fdd7"),
            None,
            None,
        );
        let branding = server_admin_contract::AdminBrandingView::from_settings(&settings);
        let html = super::render_sign_in(None, Some(&branding));
        assert!(!html.as_ref().contains("Custom Admin"));
        assert!(!html.as_ref().contains("auth-brand"));
        assert!(!html.as_ref().contains("brand-mark"));
        assert!(!html.as_ref().contains("brand-logo"));
        assert!(html.as_ref().contains("--accent:#123456"));
        assert!(!html.as_ref().contains("<script"));
    }
}
