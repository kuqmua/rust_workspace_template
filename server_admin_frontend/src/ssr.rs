#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the stable SSR facade delegates to screen, document, and table modules; test view rendering requires the named extension trait"
)]

#[path = "domain_types_ssr_crud_render_role_create.rs"]
mod crud_render_role_create;
#[path = "domain_types_ssr_crud_render_role_manage.rs"]
mod crud_render_role_manage;
#[path = "domain_types_ssr_crud_render_shell.rs"]
mod crud_render_shell;
#[path = "domain_types_ssr_crud_render_user_create.rs"]
mod crud_render_user_create;
#[path = "domain_types_ssr_crud_render_user_manage.rs"]
mod crud_render_user_manage;
#[path = "domain_types_ssr_table_data_table_grid.rs"]
mod data_table_grid;
#[path = "data_tables.rs"]
mod data_tables;
#[path = "domain_types_ssr_document.rs"]
mod document;
#[path = "render_permissions.rs"]
mod render_permissions;
#[path = "render_profile.rs"]
mod render_profile;
#[path = "render_roles.rs"]
mod render_roles;
#[path = "render_sessions.rs"]
mod render_sessions;
#[path = "render_settings.rs"]
mod render_settings;
#[path = "render_users.rs"]
mod render_users;
#[path = "domain_types_ssr_table_table_pagination.rs"]
mod table_pagination;
#[path = "text_page.rs"]
mod text_page;

#[cfg(test)]
#[path = "domain_types_ssr_tests.rs"]
mod tests;
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
        render_view(self)
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("administrator SSR HTML exceeds the size limit")]
pub struct AdminSsrHtmlTryFromStringError;
impl From<AdminSsrHtmlTryFromStringError> for AdminSsrHtml {
    fn from(value: AdminSsrHtmlTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{message}", message = constants_str::ADMIN_SSR_TITLE_TOO_LONG)]
pub struct AdminSsrTextTryFromStringError;
impl From<AdminSsrTextTryFromStringError> for AdminSsrText {
    fn from(value: AdminSsrTextTryFromStringError) -> Self {
        Self(value.to_string())
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::Display,
)]
pub struct AdminSsrErrorMessage(to_err_string::domain_types::ErrorText);
impl TryFrom<String> for AdminSsrErrorMessage {
    type Error = to_err_string::domain_types::ErrorTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        to_err_string::domain_types::ErrorText::try_from(value).map(Self)
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::Display,
    newtype::IntoInnerFrom,
)]
pub struct AdminSsrText(String);
impl TryFrom<String> for AdminSsrText {
    type Error = AdminSsrTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= constants_usize::VALUE_16_777_216)
            .then_some(Self(value))
            .ok_or(AdminSsrTextTryFromStringError)
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::IntoInnerFrom,
)]
pub struct AdminSsrHtml(String);
impl TryFrom<String> for AdminSsrHtml {
    type Error = AdminSsrHtmlTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= constants_usize::VALUE_16_777_216)
            .then_some(Self(value))
            .ok_or(AdminSsrHtmlTryFromStringError)
    }
}

fn render_view(view: impl leptos::prelude::IntoAny) -> AdminSsrHtml {
    AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(
        leptos::prelude::IntoAny::into_any(view),
    ))
    .unwrap_or_else(AdminSsrHtml::from)
}

fn render_document(title: &AdminSsrText, body: impl leptos::prelude::IntoAny) -> AdminSsrHtml {
    document::render_document::render_document(title, body)
}

#[must_use]
pub fn render_sign_in(
    error: Option<AdminSsrErrorMessage>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
) -> AdminSsrHtml {
    document::render_sign_in::render_sign_in(error, branding)
}

#[must_use]
fn render_admin_page(
    page: server_admin_contract::domain_types::AdminPage,
    content: AdminSsrHtml,
) -> AdminSsrHtml {
    document::render_admin_page::render_admin_page(page, content)
}

fn render_admin_page_with_access(
    page: server_admin_contract::domain_types::AdminPage,
    content: AdminSsrHtml,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
) -> AdminSsrHtml {
    document::render_admin_page_with_access::render_admin_page_with_access(
        page, content, admin, branding,
    )
}

fn render_admin_page_with_table_access(
    page: server_admin_contract::domain_types::AdminPage,
    content: AdminSsrHtml,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
) -> AdminSsrHtml {
    document::render_admin_page_with_table_access::render_admin_page_with_table_access(
        page,
        content,
        admin,
        branding,
        active_table,
    )
}

fn table_pagination(
    page: server_admin_contract::domain_types::AdminPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    total: server_admin_contract::domain_types::AdminPageTotal,
    table: Option<server_admin_contract::domain_types::AdminDataTable>,
    table_filter: Option<&server_admin_contract::domain_types::AdminDataTableFilterQuery>,
) -> impl leptos::prelude::IntoView {
    table_pagination::table_pagination(page, query, total, table, table_filter)
}

fn data_table_grid(
    view: &server_admin_contract::domain_types::AdminDataTableView,
    query: &server_admin_contract::domain_types::AdminDataTableQuery,
) -> impl leptos::prelude::IntoView {
    data_table_grid::data_table_grid(view, query)
}

#[allow(clippy::single_call_fn)] // isolates the metadata-driven grid for focused SSR contract testing
#[must_use]
pub fn render_users(
    page: &server_admin_contract::domain_types::AdminUsersPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_users::render_users(page, query, admin, branding)
}

#[must_use]
pub fn render_roles(
    page: &server_admin_contract::domain_types::AdminRolesPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_roles::render_roles(page, query, admin, branding)
}

#[must_use]
pub fn render_user_create(
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    crud_render_user_create::render_user_create(admin, branding)
}

#[must_use]
pub fn render_user_manage(
    page: &server_admin_contract::domain_types::AdminUsersPage,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    crud_render_user_manage::render_user_manage(page, admin, branding)
}

#[must_use]
pub fn render_role_create(
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    crud_render_role_create::render_role_create(admin, branding)
}

#[must_use]
pub fn render_role_manage(
    page: &server_admin_contract::domain_types::AdminRolesPage,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    crud_render_role_manage::render_role_manage(page, admin, branding)
}

#[must_use]
pub fn render_admin_permissions_page(
    page: &server_admin_contract::domain_types::AdminPermissionsPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_permissions::render_permissions(page, query, admin, branding)
}

#[must_use]
pub fn render_data_tables(
    table: Option<&server_admin_contract::domain_types::AdminDataTableView>,
    query: &server_admin_contract::domain_types::AdminDataTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    data_tables::render_data_tables::render_data_tables(table, query, admin, branding)
}

#[must_use]
pub fn render_data_tables_csr(
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    data_tables::render_data_tables_csr::render_data_tables_csr(active_table, admin, branding)
}

#[must_use]
pub fn render_admin_csr(
    page: server_admin_contract::domain_types::AdminPage,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    data_tables::render_admin_csr::render_admin_csr(page, active_table, admin, branding)
}

#[must_use]
pub fn render_admin_sessions_page(
    page: &server_admin_contract::domain_types::AdminSessionsPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_sessions::render_sessions(page, query, admin, branding)
}

#[must_use]
pub fn render_admin_profile_page(
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_profile::render_profile(admin, branding)
}

#[must_use]
pub fn render_admin_settings_page(
    view: &server_admin_contract::domain_types::AdminSettingsView,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_settings::render_settings(view, admin, branding)
}

#[must_use]
pub fn render_text_page(
    page: server_admin_contract::domain_types::AdminPage,
    title: AdminSsrText,
    text: AdminSsrText,
) -> AdminSsrHtml {
    text_page::render_text_page::render_text_page(page, title, text)
}

#[must_use]
pub fn render_text_page_with_access(
    page: server_admin_contract::domain_types::AdminPage,
    title: AdminSsrText,
    text: AdminSsrText,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    text_page::render_text_page_with_access::render_text_page_with_access(
        page, title, text, admin, branding,
    )
}
