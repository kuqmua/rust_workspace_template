#![allow(
    clippy::unused_trait_names,
    reason = "Leptos view macro expansion requires these attribute traits in lexical scope and repository policy forbids underscore import aliases"
)]
use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, IntoAny, StyleAttribute,
};

const SSR_TEXT_MAX_BYTES: usize = 16_777_216usize;

trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> AdminSsrHtml;
}
impl<View> AdminSsrViewExt for View
where
    View: IntoAny,
{
    fn render_admin_ssr(self) -> AdminSsrHtml {
        AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(IntoAny::into_any(
            self,
        )))
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

fn render_document(title: &AdminSsrText, body: impl IntoAny) -> AdminSsrHtml {
    let rendered_body = body.render_admin_ssr();
    AdminSsrHtml::try_from(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css?v=20260723-23\"></head><body>{}</body></html>",
        rendered_body.0
    ))
    .unwrap_or_else(AdminSsrHtml::from)
}

#[must_use]
pub fn render_sign_in(
    error: Option<AdminSsrErrorMessage>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
) -> AdminSsrHtml {
    let tab_title = branding
        .and_then(server_admin_contract::AdminBrandingView::tab_title)
        .map_or_else(
            || String::from(str_constants::ADMINISTRATOR_SIGN_IN),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    render_document(
        &AdminSsrText::try_from(tab_title).unwrap_or_else(AdminSsrText::from),
        leptos::view! {
            <main class="auth-layout" style=primary_color>
                <section class="auth-card">
                    {error.map(|message| leptos::view! { <p class="field-error" role="alert">{message.0.to_string()}</p> })}
                    <form method="post" action=server_admin_contract::AdminHtmlAction::SignIn.get()>
                        <label><span>"Login"</span><input name="login" autocomplete="username" required /></label>
                        <label><span>"Password"</span><input name="password" type="password" autocomplete="current-password" required /></label>
                        <button type="submit">"Sign in"</button>
                    </form>
                </section>
            </main>
        },
    )
}

#[must_use]
pub fn render_admin_page(
    page: server_admin_contract::AdminPage,
    content: AdminSsrHtml,
) -> AdminSsrHtml {
    render_admin_page_with_access(page, content, None, None)
}

fn render_admin_page_with_access(
    page: server_admin_contract::AdminPage,
    content: AdminSsrHtml,
    admin: Option<&server_admin_contract::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
) -> AdminSsrHtml {
    render_admin_page_with_table_access(page, content, admin, branding, None)
}

fn render_admin_page_with_table_access(
    page: server_admin_contract::AdminPage,
    content: AdminSsrHtml,
    admin: Option<&server_admin_contract::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::AdminBrandingView>,
    active_table: Option<server_admin_contract::AdminDataTable>,
) -> AdminSsrHtml {
    let spec = page.spec();
    let title = spec.title();
    let document_title = branding
        .and_then(server_admin_contract::AdminBrandingView::tab_title)
        .map_or_else(
            || title.as_ref().to_owned(),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    render_document(
        &AdminSsrText::try_from(document_title).unwrap_or_else(AdminSsrText::from),
        leptos::view! {
            <div class="app-shell" style=primary_color>
                <header class="topbar">
                    <nav aria-label="Admin sections">
                        {server_admin_contract::AdminDataTable::PG_ORDER.into_iter().filter(|table| admin.is_none_or(|value| bool::from(value.has_permission(server_admin_contract::AdminPermission::TablesRead)) && bool::from(value.has_permission(table.permission())))).map(|table| {
                            let name = table.to_string();
                            let href = table.frontend_path().to_string();
                            leptos::view! {
                                <a class=(active_table == Some(table)).then_some("active") href=href>{name}</a>
                            }
                        }).collect::<Vec<_>>()}
                        {server_admin_contract::AdminPage::NAV_ORDER.into_iter().filter(|item_page| admin.is_none_or(|value| bool::from(value.can_access(*item_page)))).map(|item_page| {
                            let item = item_page.spec();
                            let href = String::from(item.path());
                            let label = item.route_name().as_ref().to_owned();
                            leptos::view! {
                                <a class=(item_page == page).then_some("active") href=href>{label}</a>
                            }
                        }).collect::<Vec<_>>()}
                        <form method="post" action=server_admin_contract::AdminHtmlAction::SignOut.get()><button type="submit">{server_admin_contract::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</button></form>
                    </nav>
                </header>
                <main class="main-content"><p id="saved" class="flash-success" role="status">"Changes saved."</p><div inner_html=content.0></div></main>
            </div>
        },
    )
}

fn table_pagination(
    page: server_admin_contract::AdminPage,
    query: &server_admin_contract::AdminTableQuery,
    total: server_admin_contract::AdminPageTotal,
    table: Option<server_admin_contract::AdminDataTable>,
    table_filter: Option<&server_admin_contract::AdminDataTableFilterQuery>,
) -> impl leptos::prelude::IntoView {
    let action = table.map_or_else(
        || String::from(page.path()),
        |value| value.frontend_path().to_string(),
    );
    let limit = u16::from(query.limit());
    let offset = u32::from(query.offset());
    let previous_offset = offset.saturating_sub(u32::from(limit));
    let next_offset = offset.saturating_add(u32::from(limit));
    let previous_disabled = offset == 0u32;
    let next_disabled = u64::from(next_offset) >= u64::from(total);
    let filter_operation = table_filter
        .and_then(server_admin_contract::AdminDataTableFilterQuery::operation)
        .map(server_admin_contract::AdminFilterOperationKey::from);
    let filter_field =
        table_filter.and_then(server_admin_contract::AdminDataTableFilterQuery::field);
    let filter_value =
        table_filter.and_then(server_admin_contract::AdminDataTableFilterQuery::value);
    let filter_end = table_filter.and_then(server_admin_contract::AdminDataTableFilterQuery::end);
    leptos::view! {
        <nav class="table-pagination" aria-label="Table pages">
            <form class="table-page-size" method="get" action=action.clone()>
                {crate::shared::admin_table_query_hidden_inputs(query.search(), query.sort(), &crate::shared::AdminTableQueryDirection::Ssr(query.direction()), query.limit())}
                {crate::shared::admin_filter_hidden_inputs(filter_field, filter_operation.as_ref(), filter_value, filter_end)}
                <input type="hidden" name="offset" value="0" />
                <label><span>"Rows"</span><input name="limit" type="number" min="1" max="100" value=limit.to_string() /></label>
                <button type="submit">"Apply"</button>
            </form>
            <form method="get" action=action.clone()>
                {crate::shared::admin_table_query_hidden_inputs(query.search(), query.sort(), &crate::shared::AdminTableQueryDirection::Ssr(query.direction()), query.limit())}
                {crate::shared::admin_filter_hidden_inputs(filter_field, filter_operation.as_ref(), filter_value, filter_end)}
                <input type="hidden" name="offset" value=previous_offset.to_string() /><button type="submit" disabled=previous_disabled>"Previous"</button>
            </form>
            <span>{format!("{}-{} of {}", u64::from(offset).saturating_add(1u64).min(u64::from(total)), u64::from(offset).saturating_add(u64::from(limit)).min(u64::from(total)), total)}</span>
            <form method="get" action=action>
                {crate::shared::admin_table_query_hidden_inputs(query.search(), query.sort(), &crate::shared::AdminTableQueryDirection::Ssr(query.direction()), query.limit())}
                {crate::shared::admin_filter_hidden_inputs(filter_field, filter_operation.as_ref(), filter_value, filter_end)}
                <input type="hidden" name="offset" value=next_offset.to_string() /><button type="submit" disabled=next_disabled>"Next"</button>
            </form>
        </nav>
    }
}

#[allow(clippy::single_call_fn)] // isolates the metadata-driven grid for focused SSR contract testing
fn data_table_grid(
    view: &server_admin_contract::AdminDataTableView,
    query: &server_admin_contract::AdminDataTableQuery,
) -> impl leptos::prelude::IntoView {
    let operation = query
        .filter()
        .operation()
        .map(server_admin_contract::AdminFilterOperationKey::from);
    crate::shared::admin_data_table_grid(
        view,
        query.filter().field(),
        operation.as_ref(),
        query.filter().value(),
        query.filter().end(),
        query.page().limit(),
    )
}

#[must_use]
pub fn render_users(
    page: &server_admin_contract::AdminUsersPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let can_create =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersCreate));
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersDelete));
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersUpdate));
    let can_update_roles =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UserRolesUpdate));
    let content = leptos::view! {
        <section class="table-page">
        {crate::shared::admin_table_filters(server_admin_contract::AdminFrontendPath::Users, query.search(), query.sort(), crate::shared::AdminTableFilterDirection::from(query.direction()), query.limit(), &server_admin_contract::AdminTableSortField::USER, crate::shared::AdminTableFilterPresentation::Ssr)}
        {can_create.then(|| leptos::view! { <details class="mutation-form"><summary>"Create user"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserCreate.get()>
            <label><span>"Login"</span><input name="login" required /></label><label><span>"Display name"</span><input name="display_name" required /></label>
            <label><span>"Password"</span><input name="password" type="password" required /></label><button type="submit">"Create user"</button>
        </form></details> })}
        <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"login"</th><th>"display_name"</th><th>"banned"</th><th>"roles"</th><th>"actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| { let expected_role_ids = item.role_ids().iter().map(ToString::to_string).collect::<Vec<_>>().join(","); leptos::view! {
            <tr><td data-label="id">{item.id().to_string()}</td><td data-label="login">{item.login().to_string()}</td><td data-label="display_name">{item.display_name().to_string()}</td><td data-label="banned">{item.is_banned().to_string()}</td>
            <td data-label="roles">{can_update_roles.then(|| leptos::view! { <form method="post" action=server_admin_contract::AdminHtmlAction::UserRoles.get()><input type="hidden" name="user_id" value=item.id().to_string() />
                <input type="hidden" name="expected_role_ids" value=expected_role_ids />
                {page.roles().iter().map(|role| { let checked = item.role_ids().contains(&role.id()); let name = format!("role_{}", role.id()); leptos::view! { <label><input type="checkbox" name=name value=role.id().to_string() checked=checked />{role.name().to_string()}</label> } }).collect::<Vec<_>>()}
                <button type="submit">"Save roles"</button></form> })}</td>
            <td data-label="actions">{can_update.then(|| leptos::view! { <details><summary>"Edit"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserUpdate.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="login" value=item.login().to_string() required /><input name="display_name" value=item.display_name().to_string() required /><button type="submit">"Save"</button></form></details>
                <details><summary>"Password"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserPassword.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="password" type="password" required /><button type="submit">"Change password"</button></form></details>
                <form method="post" action=server_admin_contract::AdminHtmlAction::UserBan.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input type="hidden" name="is_banned" value=(!bool::from(item.is_banned())).to_string() /><button type="submit">{if bool::from(item.is_banned()) { "Unban" } else { "Ban" }}</button></form> })}
                {can_delete.then(|| leptos::view! { <details><summary>"Delete"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserDelete.get()><input type="hidden" name="user_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm permanent deletion"</label><button class="danger-button" type="submit">"Delete user"</button></form></details> })}</td></tr>
        }}).collect::<Vec<_>>()}</tbody></table></div>
        {table_pagination(server_admin_contract::AdminPage::Users, query, page.total(), None, None)}
        </section>
    }.render_admin_ssr();
    render_admin_page_with_access(
        server_admin_contract::AdminPage::Users,
        content,
        Some(admin),
        Some(branding),
    )
}

#[must_use]
pub fn render_roles(
    page: &server_admin_contract::AdminRolesPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let can_create =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesCreate));
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesDelete));
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate));
    let can_update_permissions = bool::from(
        admin.has_permission(server_admin_contract::AdminPermission::RolePermissionsUpdate),
    );
    let content = leptos::view! {
        <section class="table-page">
        {crate::shared::admin_table_filters(server_admin_contract::AdminFrontendPath::Roles, query.search(), query.sort(), crate::shared::AdminTableFilterDirection::from(query.direction()), query.limit(), &server_admin_contract::AdminTableSortField::ROLE, crate::shared::AdminTableFilterPresentation::Ssr)}
        {can_create.then(|| leptos::view! { <details class="mutation-form"><summary>"Create role"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::RoleCreate.get()><label><span>"Name"</span><input name="name" required /></label><button type="submit">"Create role"</button></form></details> })}
        <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"name"</th><th>"system"</th><th>"permissions"</th><th>"actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| { let expected_permission_ids = item.permission_ids().iter().map(ToString::to_string).collect::<Vec<_>>().join(","); leptos::view! {
            <tr><td data-label="id">{item.id().to_string()}</td><td data-label="name">{item.name().to_string()}</td><td data-label="system">{item.is_system().to_string()}</td><td data-label="permissions">{can_update_permissions.then(|| leptos::view! { <form method="post" action=server_admin_contract::AdminHtmlAction::RolePermissions.get()><input type="hidden" name="role_id" value=item.id().to_string() />
                <input type="hidden" name="expected_permission_ids" value=expected_permission_ids />
                {page.permissions().iter().map(|permission| { let checked = item.permission_ids().contains(&permission.id()); let name = format!("permission_{}", permission.id()); leptos::view! { <label><input type="checkbox" name=name value=permission.id().to_string() checked=checked />{permission.name().to_string()}</label> } }).collect::<Vec<_>>()}
                <button type="submit">"Save permissions"</button></form> })}</td><td data-label="actions">
                {can_update.then(|| leptos::view! { <form method="post" action=server_admin_contract::AdminHtmlAction::RoleUpdate.get()><input type="hidden" name="role_id" value=item.id().to_string() /><input name="name" value=item.name().to_string() required /><button type="submit">"Save"</button></form> })}
                {can_delete.then(|| leptos::view! { <details><summary>"Delete"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::RoleDelete.get()><input type="hidden" name="role_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm permanent deletion"</label><button class="danger-button" type="submit" disabled=bool::from(item.is_system())>"Delete role"</button></form></details> })}</td></tr>
        }}).collect::<Vec<_>>()}</tbody></table></div>
        {table_pagination(server_admin_contract::AdminPage::Roles, query, page.total(), None, None)}
        </section>
    }.render_admin_ssr();
    render_admin_page_with_access(
        server_admin_contract::AdminPage::Roles,
        content,
        Some(admin),
        Some(branding),
    )
}

#[must_use]
pub fn render_permissions(
    page: &server_admin_contract::AdminPermissionsPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let content = leptos::view! {
        <section class="table-page">
        {crate::shared::admin_table_filters(server_admin_contract::AdminFrontendPath::Permissions, query.search(), query.sort(), crate::shared::AdminTableFilterDirection::from(query.direction()), query.limit(), &server_admin_contract::AdminTableSortField::PERMISSION, crate::shared::AdminTableFilterPresentation::Ssr)}
        <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"permission"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td data-label="id">{item.id().to_string()}</td><td data-label="permission">{item.name().to_string()}</td></tr>
        }).collect::<Vec<_>>()}</tbody></table></div>
        {table_pagination(server_admin_contract::AdminPage::Permissions, query, page.total(), None, None)}
        </section>
    }
    .render_admin_ssr();
    render_admin_page_with_access(
        server_admin_contract::AdminPage::Permissions,
        content,
        Some(admin),
        Some(branding),
    )
}

#[must_use]
pub fn render_data_tables(
    table: Option<&server_admin_contract::AdminDataTableView>,
    query: &server_admin_contract::AdminDataTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let content = leptos::view! {
        {table.map(|view| leptos::view! {
            <section class="table-page">
                {data_table_grid(view, query)}
                {table_pagination(server_admin_contract::AdminPage::Tables, query.page(), view.total(), Some(view.table()), bool::from(view.table().supports_filters()).then_some(query.filter()))}
            </section>
        })}
    }
    .render_admin_ssr();
    render_admin_page_with_table_access(
        server_admin_contract::AdminPage::Tables,
        content,
        Some(admin),
        Some(branding),
        table.map(server_admin_contract::AdminDataTableView::table),
    )
}

#[must_use]
pub fn render_data_tables_csr(
    active_table: Option<server_admin_contract::AdminDataTable>,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    render_admin_csr(
        server_admin_contract::AdminPage::Tables,
        active_table,
        admin,
        branding,
    )
}

#[must_use]
pub fn render_admin_csr(
    page: server_admin_contract::AdminPage,
    _active_table: Option<server_admin_contract::AdminDataTable>,
    _admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let title = branding.tab_title().map_or_else(
        || page.spec().title().as_ref().to_owned(),
        |value| value.as_ref().to_owned(),
    );
    let primary_color = branding
        .primary_color()
        .map(|value| format!("--accent:{}", value.as_ref()));
    render_document(
        &AdminSsrText::try_from(title).unwrap_or_else(AdminSsrText::from),
        leptos::view! {
            <div id=str_constants::ADMIN_CSR_ROOT_ID style=primary_color><p class="loading-state" role="status">"Loading\u{2026}"</p></div>
            <script type="module" src="/admin/assets/csr_bootstrap.js?v=20260723-01"></script>
        },
    )
}

#[must_use]
pub fn render_sessions(
    page: &server_admin_contract::AdminSessionsPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let content = leptos::view! {
        <section class="table-page">
        <div class="table-scroll"><table><thead><tr><th>"session"</th><th>"created"</th><th>"expires"</th><th>"current"</th><th>"actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td data-label="session">{item.id().to_string()}</td><td data-label="created">{item.created_at().to_string()}</td><td data-label="expires">{item.expires_at().to_string()}</td><td data-label="current">{item.is_current().to_string()}</td><td data-label="actions"><details><summary>"Revoke"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::SessionRevoke.get()><input type="hidden" name="session_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm session revocation"</label><button class="danger-button" type="submit">"Revoke session"</button></form></details></td></tr>
        }).collect::<Vec<_>>()}</tbody></table></div>
        {table_pagination(server_admin_contract::AdminPage::Sessions, query, page.total(), None, None)}
        </section>
    }.render_admin_ssr();
    render_admin_page_with_access(
        server_admin_contract::AdminPage::Sessions,
        content,
        Some(admin),
        Some(branding),
    )
}

#[must_use]
pub fn render_profile(
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let roles = admin
        .roles()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(str_constants::COMMA_SPACE);
    let content = leptos::view! {
        <section class="security-card"><p><strong>{admin.display_name().to_string()}</strong></p><p>{admin.login().to_string()}</p><p>{roles}</p></section>
        <section class="security-card"><form method="post" action=server_admin_contract::AdminHtmlAction::ProfilePassword.get()>
            <label><span>"Current password"</span><input name="current_password" type="password" required /></label>
            <label><span>"New password"</span><input name="new_password" type="password" required /></label>
            <button type="submit">"Change password"</button>
        </form></section>
    }.render_admin_ssr();
    render_admin_page_with_access(
        server_admin_contract::AdminPage::Profile,
        content,
        Some(admin),
        Some(branding),
    )
}

#[must_use]
pub fn render_settings(
    view: &server_admin_contract::AdminSettingsView,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let values = crate::shared::AdminSettingsFormValues::from(view);
    let site_name = values.site_name().as_ref().to_owned();
    let default_admin_route = values.default_route().as_ref().to_owned();
    let tab_title = values.tab_title().as_ref().to_owned();
    let main_logo = values.main_logo().as_ref().to_owned();
    let primary_color = values.primary_color().as_ref().to_owned();
    let organization_name = values.organization_name().as_ref().to_owned();
    let organization_contacts = values.organization_contacts().as_ref().to_owned();
    let support_url = values.support_url().as_ref().to_owned();
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::AdminPermission::SystemSettingsUpdate),
    );
    let content = leptos::view! {
        <section class="settings-grid"><article class="settings-card">
        {can_update.then(|| leptos::view! { <form class="settings-form" method="post" action=server_admin_contract::AdminHtmlAction::SettingsUpdate.get()>
            <label><span>"Site name"</span><input name="site_name" value=site_name required /></label>
            <label><span>"Default route"</span><input name="default_admin_route" value=default_admin_route required /></label>
            <label><span>"Tab title"</span><input name="tab_title" value=tab_title /></label>
            <label><span>"Main logo URL"</span><input name="main_logo" value=main_logo /></label>
            <label><span>"Primary color"</span><input name="primary_color" value=primary_color /></label>
            <label><span>"Organization"</span><input name="organization_name" value=organization_name /></label>
            <label><span>"Organization contacts"</span><textarea name="organization_contacts">{organization_contacts}</textarea></label>
            <label><span>"Support URL"</span><input name="support_url" value=support_url /></label>
            <button type="submit">"Save settings"</button>
        </form> })}
        {(!can_update).then(|| leptos::view! { <p>"Settings are read-only for this account."</p> })}
        </article></section>
    }.render_admin_ssr();
    render_admin_page_with_access(
        server_admin_contract::AdminPage::Settings,
        content,
        Some(admin),
        Some(branding),
    )
}

#[must_use]
pub fn render_text_page(
    page: server_admin_contract::AdminPage,
    _title: AdminSsrText,
    text: AdminSsrText,
) -> AdminSsrHtml {
    let content = leptos::view! {
        <section><div class="code-card"><pre>{text.0}</pre></div></section>
    }
    .render_admin_ssr();
    render_admin_page(page, content)
}

#[must_use]
pub fn render_text_page_with_access(
    page: server_admin_contract::AdminPage,
    _title: AdminSsrText,
    text: AdminSsrText,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let content = leptos::view! {
        <section><div class="code-card"><pre>{text.0}</pre></div></section>
    }
    .render_admin_ssr();
    render_admin_page_with_access(page, content, Some(admin), Some(branding))
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
        assert!(html.as_ref().contains("data-filter-count=\"2\""));
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
        assert!(filters_html.as_ref().contains(">Clear</a>"));
        assert_eq!(
            filters_html
                .as_ref()
                .matches("action=\"/admin/role_permissions\"")
                .count(),
            2usize
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
            2usize
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
        assert!(
            html.as_ref()
                .contains("src=\"/admin/assets/csr_bootstrap.js?v=20260723-01\"")
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
    fn shared_table_filters_preserve_ssr_limit_submission() {
        let html = crate::shared::admin_table_filters(
            server_admin_contract::AdminFrontendPath::Users,
            server_admin_contract::AdminTableQuery::default().search(),
            server_admin_contract::AdminTableQuery::default().sort(),
            crate::shared::AdminTableFilterDirection::from(
                server_admin_contract::AdminSortDirection::Asc,
            ),
            server_admin_contract::AdminPageLimit::default(),
            &server_admin_contract::AdminTableSortField::USER,
            crate::shared::AdminTableFilterPresentation::Ssr,
        )
        .render_admin_ssr();
        assert!(
            html.as_ref()
                .contains("<input name=\"limit\" type=\"hidden\"")
        );
        assert!(!html.as_ref().contains("name=\"limit\" type=\"number\""));
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
