#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the stable SSR facade delegates to screen, document, and table modules; test view rendering requires the named extension trait"
)]

mod crud;
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
#[error("{message}", message = str_constants::ADMIN_SSR_TITLE_TOO_LONG)]
pub struct AdminSsrTextTryFromStringError;
impl From<AdminSsrTextTryFromStringError> for AdminSsrText {
    fn from(value: AdminSsrTextTryFromStringError) -> Self {
        Self(value.to_string())
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::Display,
)]
pub struct AdminSsrErrorMessage(to_err_string::ErrorText);
impl TryFrom<String> for AdminSsrErrorMessage {
    type Error = to_err_string::ErrorTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        to_err_string::ErrorText::try_from(value).map(Self)
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
        (value.len() <= SSR_TEXT_MAX_BYTES)
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
        (value.len() <= SSR_TEXT_MAX_BYTES)
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
fn render_admin_page(
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
pub fn render_user_create(
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    crud::render_user_create(admin, branding)
}

#[must_use]
pub fn render_user_manage(
    page: &server_admin_contract::AdminUsersPage,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    crud::render_user_manage(page, admin, branding)
}

#[must_use]
pub fn render_role_create(
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    crud::render_role_create(admin, branding)
}

#[must_use]
pub fn render_role_manage(
    page: &server_admin_contract::AdminRolesPage,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    crud::render_role_manage(page, admin, branding)
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
