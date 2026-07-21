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
        AdminSsrHtml(leptos::prelude::RenderHtml::to_html(IntoAny::into_any(
            self,
        )))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("administrator SSR error message exceeds the size limit")]
pub struct AdminSsrErrorMessageTryFromStringError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("administrator SSR HTML exceeds the size limit")]
pub struct AdminSsrHtmlTryFromStringError;
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{message}", message = str_constants::ADMIN_SSR_TITLE_TOO_LONG)]
pub struct AdminSsrTextTryFromStringError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdminSsrErrorMessage(String);
impl TryFrom<String> for AdminSsrErrorMessage {
    type Error = AdminSsrErrorMessageTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= SSR_TEXT_MAX_BYTES)
            .then_some(Self(value))
            .ok_or(AdminSsrErrorMessageTryFromStringError)
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
    AdminSsrHtml(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css?v=20260721-11\"></head><body>{}</body></html>",
        rendered_body.0
    ))
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
        &AdminSsrText(tab_title),
        leptos::view! {
            <main class="auth-layout" style=primary_color>
                <section class="auth-card">
                    {error.map(|message| leptos::view! { <p class="field-error" role="alert">{message.0}</p> })}
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
        &AdminSsrText(document_title),
        leptos::view! {
            <div class="app-shell" style=primary_color>
                <header class="topbar">
                    <nav aria-label="Admin sections">
                        {server_admin_contract::AdminPage::specs().iter().copied().filter(|item| !matches!(item.page(), server_admin_contract::AdminPage::Audit | server_admin_contract::AdminPage::Permissions | server_admin_contract::AdminPage::Roles | server_admin_contract::AdminPage::Sessions | server_admin_contract::AdminPage::Settings | server_admin_contract::AdminPage::Tables | server_admin_contract::AdminPage::Users) && admin.is_none_or(|value| bool::from(value.can_access(item.page())))).map(|item| {
                            let item_page = item.page();
                            let href = String::from(item.path());
                            let label = String::from(item.title());
                            leptos::view! {
                                <a class=(item_page == page).then_some("active") href=href>{label}</a>
                            }
                        }).collect::<Vec<_>>()}
                        {server_admin_contract::AdminDataTable::ALL.into_iter().filter(|table| admin.is_none_or(|value| bool::from(value.has_permission(server_admin_contract::AdminPermission::TablesRead)) && bool::from(value.has_permission(table.permission())))).map(|table| {
                            let name = table.to_string();
                            let href = format!("{}{}{}", server_admin_contract::AdminFrontendPath::Tables.get(), str_constants::ADMIN_TABLE_QUERY_PREFIX, name);
                            leptos::view! {
                                <a class=(active_table == Some(table)).then_some("active") href=href>{name}</a>
                            }
                        }).collect::<Vec<_>>()}
                        <form method="post" action=server_admin_contract::AdminHtmlAction::SignOut.get()><button type="submit">"Sign out"</button></form>
                    </nav>
                </header>
                <main class="main-content"><p id="saved" class="flash-success" role="status">"Changes saved."</p><div inner_html=content.0></div></main>
            </div>
        },
    )
}

fn table_filters(
    page: server_admin_contract::AdminPage,
    query: &server_admin_contract::AdminTableQuery,
    sort_fields: &[server_admin_contract::AdminTableSortField],
) -> impl leptos::prelude::IntoView {
    let action = String::from(page.path());
    let search = query.search().as_ref().to_owned();
    let selected_sort = query.sort().as_ref().to_owned();
    let ascending = matches!(
        query.direction(),
        server_admin_contract::AdminSortDirection::Asc
    );
    let limit = u16::from(query.limit()).to_string();
    leptos::view! {
        <form class="table-tools" method="get" action=action>
            <label><span>"Search"</span><input name="search" value=search /></label>
            <label><span>"Sort"</span><select name="sort">
                <option value="" selected=selected_sort.is_empty()>"Default"</option>
                {sort_fields.iter().copied().map(|field| { let key = field.key().as_ref().to_owned(); let selected = key == selected_sort; leptos::view! { <option value=key selected=selected>{field.label().as_ref().to_owned()}</option> } }).collect::<Vec<_>>()}
            </select></label>
            <label><span>"Direction"</span><select name="direction"><option value="asc" selected=ascending>"Ascending"</option><option value="desc" selected=!ascending>"Descending"</option></select></label>
            <label><span>"Rows"</span><input name="limit" type="number" min="1" max="100" value=limit /></label>
            <input name="offset" type="hidden" value="0" />
            <button type="submit">"Apply"</button>
        </form>
    }
}

fn table_pagination(
    page: server_admin_contract::AdminPage,
    query: &server_admin_contract::AdminTableQuery,
    total: server_admin_contract::AdminPageTotal,
) -> impl leptos::prelude::IntoView {
    let action = String::from(page.path());
    let search = query.search().as_ref().to_owned();
    let sort = query.sort().as_ref().to_owned();
    let direction = query.direction().as_ref().to_owned();
    let limit = u16::from(query.limit());
    let offset = u32::from(query.offset());
    let previous_offset = offset.saturating_sub(u32::from(limit));
    let next_offset = offset.saturating_add(u32::from(limit));
    let previous_disabled = offset == 0u32;
    let next_disabled = u64::from(next_offset) >= u64::from(total);
    leptos::view! {
        <nav class="table-pagination" aria-label="Table pages">
            <form method="get" action=action.clone()>
                <input type="hidden" name="search" value=search.clone() /><input type="hidden" name="sort" value=sort.clone() />
                <input type="hidden" name="direction" value=direction.clone() /><input type="hidden" name="limit" value=limit.to_string() />
                <input type="hidden" name="offset" value=previous_offset.to_string() /><button type="submit" disabled=previous_disabled>"Previous"</button>
            </form>
            <span>{format!("{}-{} of {}", u64::from(offset).saturating_add(1u64).min(u64::from(total)), u64::from(offset).saturating_add(u64::from(limit)).min(u64::from(total)), total)}</span>
            <form method="get" action=action>
                <input type="hidden" name="search" value=search /><input type="hidden" name="sort" value=sort />
                <input type="hidden" name="direction" value=direction /><input type="hidden" name="limit" value=limit.to_string() />
                <input type="hidden" name="offset" value=next_offset.to_string() /><button type="submit" disabled=next_disabled>"Next"</button>
            </form>
        </nav>
    }
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
        {table_filters(server_admin_contract::AdminPage::Users, query, &server_admin_contract::AdminTableSortField::USER)}
        {can_create.then(|| leptos::view! { <details class="mutation-form"><summary>"Create user"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserCreate.get()>
            <label><span>"Login"</span><input name="login" required /></label><label><span>"Display name"</span><input name="display_name" required /></label>
            <label><span>"Password"</span><input name="password" type="password" required /></label><button type="submit">"Create user"</button>
        </form></details> })}
        <table><thead><tr><th>"ID"</th><th>"Login"</th><th>"Display name"</th><th>"Banned"</th><th>"Roles"</th><th>"Actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| { let expected_role_ids = item.role_ids().iter().map(ToString::to_string).collect::<Vec<_>>().join(","); leptos::view! {
            <tr><td data-label="ID">{item.id().to_string()}</td><td data-label="Login">{item.login().to_string()}</td><td data-label="Display name">{item.display_name().to_string()}</td><td data-label="Banned">{item.is_banned().to_string()}</td>
            <td data-label="Roles">{can_update_roles.then(|| leptos::view! { <form method="post" action=server_admin_contract::AdminHtmlAction::UserRoles.get()><input type="hidden" name="user_id" value=item.id().to_string() />
                <input type="hidden" name="expected_role_ids" value=expected_role_ids />
                {page.roles().iter().map(|role| { let checked = item.role_ids().contains(&role.id()); let name = format!("role_{}", role.id()); leptos::view! { <label><input type="checkbox" name=name value=role.id().to_string() checked=checked />{role.name().to_string()}</label> } }).collect::<Vec<_>>()}
                <button type="submit">"Save roles"</button></form> })}</td>
            <td data-label="Actions">{can_update.then(|| leptos::view! { <details><summary>"Edit"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserUpdate.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="login" value=item.login().to_string() required /><input name="display_name" value=item.display_name().to_string() required /><button type="submit">"Save"</button></form></details>
                <details><summary>"Password"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserPassword.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="password" type="password" required /><button type="submit">"Change password"</button></form></details>
                <form method="post" action=server_admin_contract::AdminHtmlAction::UserBan.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input type="hidden" name="is_banned" value=(!bool::from(item.is_banned())).to_string() /><button type="submit">{if bool::from(item.is_banned()) { "Unban" } else { "Ban" }}</button></form> })}
                {can_delete.then(|| leptos::view! { <details><summary>"Delete"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserDelete.get()><input type="hidden" name="user_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm permanent deletion"</label><button class="danger-button" type="submit">"Delete user"</button></form></details> })}</td></tr>
        }}).collect::<Vec<_>>()}</tbody></table>
        {table_pagination(server_admin_contract::AdminPage::Users, query, page.total())}
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
        {table_filters(server_admin_contract::AdminPage::Roles, query, &server_admin_contract::AdminTableSortField::ROLE)}
        {can_create.then(|| leptos::view! { <details class="mutation-form"><summary>"Create role"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::RoleCreate.get()><label><span>"Name"</span><input name="name" required /></label><button type="submit">"Create role"</button></form></details> })}
        <table><thead><tr><th>"ID"</th><th>"Name"</th><th>"System"</th><th>"Permissions"</th><th>"Actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| { let expected_permission_ids = item.permission_ids().iter().map(ToString::to_string).collect::<Vec<_>>().join(","); leptos::view! {
            <tr><td data-label="ID">{item.id().to_string()}</td><td data-label="Name">{item.name().to_string()}</td><td data-label="System">{item.is_system().to_string()}</td><td data-label="Permissions">{can_update_permissions.then(|| leptos::view! { <form method="post" action=server_admin_contract::AdminHtmlAction::RolePermissions.get()><input type="hidden" name="role_id" value=item.id().to_string() />
                <input type="hidden" name="expected_permission_ids" value=expected_permission_ids />
                {page.permissions().iter().map(|permission| { let checked = item.permission_ids().contains(&permission.id()); let name = format!("permission_{}", permission.id()); leptos::view! { <label><input type="checkbox" name=name value=permission.id().to_string() checked=checked />{permission.name().to_string()}</label> } }).collect::<Vec<_>>()}
                <button type="submit">"Save permissions"</button></form> })}</td><td data-label="Actions">
                {can_update.then(|| leptos::view! { <form method="post" action=server_admin_contract::AdminHtmlAction::RoleUpdate.get()><input type="hidden" name="role_id" value=item.id().to_string() /><input name="name" value=item.name().to_string() required /><button type="submit">"Save"</button></form> })}
                {can_delete.then(|| leptos::view! { <details><summary>"Delete"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::RoleDelete.get()><input type="hidden" name="role_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm permanent deletion"</label><button class="danger-button" type="submit" disabled=bool::from(item.is_system())>"Delete role"</button></form></details> })}</td></tr>
        }}).collect::<Vec<_>>()}</tbody></table>
        {table_pagination(server_admin_contract::AdminPage::Roles, query, page.total())}
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
        {table_filters(server_admin_contract::AdminPage::Permissions, query, &server_admin_contract::AdminTableSortField::PERMISSION)}
        <table><thead><tr><th>"ID"</th><th>"Permission"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td data-label="ID">{item.id().to_string()}</td><td data-label="Permission">{item.name().to_string()}</td></tr>
        }).collect::<Vec<_>>()}</tbody></table>
        {table_pagination(server_admin_contract::AdminPage::Permissions, query, page.total())}
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
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let content = leptos::view! {
        {table.map(|view| leptos::view! {
            <section>
                <div class="table-scroll"><table>
                    <thead><tr>{view.columns().iter().map(|column| leptos::view! { <th>{column.to_string()}</th> }).collect::<Vec<_>>()}</tr></thead>
                    <tbody>{view.items().iter().map(|row| leptos::view! {
                        <tr>{row.values().iter().enumerate().map(|(index, value)| {
                            let label = view.columns().get(index).map_or_else(String::new, ToString::to_string);
                            leptos::view! { <td data-label=label>{value.to_string()}</td> }
                        }).collect::<Vec<_>>()}</tr>
                    }).collect::<Vec<_>>()}</tbody>
                </table></div>
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
pub fn render_sessions(
    items: &[server_admin_contract::AdminSessionView],
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let content = leptos::view! {
        <table><thead><tr><th>"Session"</th><th>"Created"</th><th>"Expires"</th><th>"Current"</th><th>"Actions"</th></tr></thead>
        <tbody>{items.iter().map(|item| leptos::view! {
            <tr><td data-label="Session">{item.id().to_string()}</td><td data-label="Created">{item.created_at().to_string()}</td><td data-label="Expires">{item.expires_at().to_string()}</td><td data-label="Current">{item.is_current().to_string()}</td><td data-label="Actions"><details><summary>"Revoke"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::SessionRevoke.get()><input type="hidden" name="session_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm session revocation"</label><button class="danger-button" type="submit">"Revoke session"</button></form></details></td></tr>
        }).collect::<Vec<_>>()}</tbody></table>
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
            <label><input name="revoke_other_sessions" type="checkbox" value="true" />"Revoke other sessions"</label><button type="submit">"Change password"</button>
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
    let site_name = AsRef::<str>::as_ref(view.site_name()).to_owned();
    let default_admin_route = AsRef::<str>::as_ref(view.default_admin_route()).to_owned();
    let tab_title = view
        .tab_title()
        .map(|value| AsRef::<str>::as_ref(value).to_owned())
        .unwrap_or_default();
    let main_logo = view
        .main_logo()
        .map(|value| AsRef::<str>::as_ref(value).to_owned())
        .unwrap_or_default();
    let primary_color = view
        .primary_color()
        .map(|value| AsRef::<str>::as_ref(value).to_owned())
        .unwrap_or_default();
    let organization_name = view
        .organization_name()
        .map(|value| AsRef::<str>::as_ref(value).to_owned())
        .unwrap_or_default();
    let organization_contacts = view
        .organization_contacts()
        .map(|value| AsRef::<str>::as_ref(value).to_owned())
        .unwrap_or_default();
    let support_url = view
        .support_url()
        .map(|value| AsRef::<str>::as_ref(value).to_owned())
        .unwrap_or_default();
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::AdminPermission::SystemSettingsUpdate),
    );
    let content = leptos::view! {
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
    }.render_admin_ssr();
    render_admin_page_with_access(
        server_admin_contract::AdminPage::Settings,
        content,
        Some(admin),
        Some(branding),
    )
}

#[must_use]
pub fn render_audit(
    page: &server_admin_contract::AdminAuditPage,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let content = leptos::view! {
        <form class="audit-filters" method="get" action=server_admin_contract::AdminFrontendPath::Audit.get()>
            <label><span>"Action"</span><input name="action" /></label><label><span>"Resource"</span><input name="resource" /></label><label><span>"Resource ID"</span><input name="resource_id" /></label>
            <label><span>"User login"</span><input name="user_login" /></label><label><span>"Limit"</span><input name="limit" type="number" min="1" max="100" value="50" /></label><button type="submit">"Apply"</button>
        </form>
        <table><thead><tr><th>"Time"</th><th>"User"</th><th>"Action"</th><th>"Resource"</th><th>"Result"</th></tr></thead><tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td data-label="Time">{item.created_at().to_string()}</td><td data-label="User">{item.user_login().map(ToString::to_string).unwrap_or_default()}</td><td data-label="Action">{item.action().to_string()}</td><td data-label="Resource">{item.resource().to_string()}</td><td data-label="Result">{item.succeeded().to_string()}</td></tr>
        }).collect::<Vec<_>>()}</tbody></table>
    }.render_admin_ssr();
    render_admin_page_with_access(
        server_admin_contract::AdminPage::Audit,
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
            server_admin_contract::AdminPage::Audit,
            super::AdminSsrHtml::try_from(String::from("<p>ready</p>")).expect("c78bd3a1"),
        );
        assert!(page.as_ref().contains("<p>ready</p>"));
        assert!(!page.as_ref().contains("<h1"));
        assert!(!page.as_ref().contains("<h2"));
        assert!(!page.as_ref().contains("class=\"brand\""));
        assert!(!page.as_ref().contains("nav-dot"));
        assert!(page.as_ref().contains("Sign out</button></form></nav>"));
        assert!(!page.as_ref().contains("<script"));
    }

    #[test]
    fn pagination_preserves_server_side_navigation() {
        let html = super::table_pagination(
            server_admin_contract::AdminPage::Users,
            &server_admin_contract::AdminTableQuery::default(),
            server_admin_contract::AdminPageTotal::from(101u64),
        )
        .render_admin_ssr();
        assert!(html.as_ref().contains("name=\"offset\" value=\"20\""));
        assert!(html.as_ref().contains("disabled>Previous"));
        assert!(!html.as_ref().contains("<script"));
    }

    #[test]
    fn navigation_only_contains_accessible_pages() {
        let admin = server_admin_contract::AuthenticatedAdmin::new(
            server_admin_contract::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
                .expect("cdae3e58"),
            server_admin_contract::AdminUserId::from(1i64),
            server_admin_contract::AdminLogin::try_from(str_constants::ROOT.to_owned())
                .expect("9ae5b850"),
            vec![
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
            ],
            Vec::new(),
        );
        let html = super::render_admin_page_with_access(
            server_admin_contract::AdminPage::Users,
            super::AdminSsrHtml::try_from(String::new()).expect("aa3fa21e"),
            Some(&admin),
            None,
        );
        assert!(
            !html
                .as_ref()
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
                .contains(server_admin_contract::AdminFrontendPath::Audit.get())
        );
        assert!(
            !html
                .as_ref()
                .contains(server_admin_contract::AdminFrontendPath::Settings.get())
        );
        assert!(
            !html
                .as_ref()
                .contains(server_admin_contract::AdminFrontendPath::Sessions.get())
        );
        assert!(
            html.as_ref()
                .contains(server_admin_contract::AdminFrontendPath::Profile.get())
        );
        assert!(
            html.as_ref().contains(
                format!(
                    "{}{}{}",
                    server_admin_contract::AdminFrontendPath::Tables.get(),
                    str_constants::ADMIN_TABLE_QUERY_PREFIX,
                    server_admin_contract::AdminDataTable::AccessSessions
                )
                .as_str()
            )
        );
        assert!(
            html.as_ref().contains(
                format!(
                    "{}{}{}",
                    server_admin_contract::AdminFrontendPath::Tables.get(),
                    str_constants::ADMIN_TABLE_QUERY_PREFIX,
                    server_admin_contract::AdminDataTable::Users
                )
                .as_str()
            )
        );
        assert!(!html.as_ref().contains("href=\"/admin/tables\""));
    }

    #[test]
    fn sign_in_uses_server_side_color_without_logo() {
        let settings = server_admin_contract::AdminSettingsView::new(
            server_admin_contract::AdminDefaultRoute::try_from(
                server_admin_contract::AdminFrontendPath::Audit
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
