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
#[error("administrator SSR error message exceeds the size limit")]
pub struct AdminSsrErrorMessageTryFromStringError;

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
    AdminSsrHtml::try_from(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css?v=20260722-15\"></head><body>{}</body></html>",
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
                            let label = item.title().as_ref().to_ascii_lowercase().replace(' ', "_");
                            leptos::view! {
                                <a class=(item_page == page).then_some("active") href=href>{label}</a>
                            }
                        }).collect::<Vec<_>>()}
                        <form method="post" action=server_admin_contract::AdminHtmlAction::SignOut.get()><button type="submit">{str_constants::SIGN_OUT.to_ascii_lowercase().replace(' ', "_")}</button></form>
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
            <input name="limit" type="hidden" value=limit />
            <input name="offset" type="hidden" value="0" />
            <button type="submit">"Apply"</button>
        </form>
    }
}

fn table_pagination(
    page: server_admin_contract::AdminPage,
    query: &server_admin_contract::AdminTableQuery,
    total: server_admin_contract::AdminPageTotal,
    table: Option<server_admin_contract::AdminDataTable>,
    table_filter: Option<&server_admin_contract::AdminDataTableFilterQuery>,
    audit: Option<&server_admin_contract::AdminAuditHtmlQuery>,
) -> impl leptos::prelude::IntoView {
    let action = table.map_or_else(
        || String::from(page.path()),
        |value| value.frontend_path().to_string(),
    );
    let search = query.search().as_ref().to_owned();
    let sort = query.sort().as_ref().to_owned();
    let direction = query.direction().as_ref().to_owned();
    let limit = u16::from(query.limit());
    let offset = u32::from(query.offset());
    let previous_offset = offset.saturating_sub(u32::from(limit));
    let next_offset = offset.saturating_add(u32::from(limit));
    let previous_disabled = offset == 0u32;
    let next_disabled = u64::from(next_offset) >= u64::from(total);
    let filter_field = table_filter
        .and_then(server_admin_contract::AdminDataTableFilterQuery::field)
        .map(ToString::to_string);
    let filter_operation = table_filter
        .and_then(server_admin_contract::AdminDataTableFilterQuery::operation)
        .map(server_admin_contract::AdminFilterOperationKey::from)
        .map(|value| value.to_string());
    let filter_value = table_filter
        .and_then(server_admin_contract::AdminDataTableFilterQuery::value)
        .map(ToString::to_string);
    let filter_end = table_filter
        .and_then(server_admin_contract::AdminDataTableFilterQuery::end)
        .map(ToString::to_string);
    let audit_action = audit
        .and_then(server_admin_contract::AdminAuditHtmlQuery::action)
        .map(ToString::to_string);
    let audit_resource = audit
        .and_then(server_admin_contract::AdminAuditHtmlQuery::resource)
        .map(ToString::to_string);
    let audit_resource_id = audit
        .and_then(server_admin_contract::AdminAuditHtmlQuery::resource_id)
        .map(ToString::to_string);
    let audit_user_login = audit
        .and_then(server_admin_contract::AdminAuditHtmlQuery::user_login)
        .map(ToString::to_string);
    leptos::view! {
        <nav class="table-pagination" aria-label="Table pages">
            <form class="table-page-size" method="get" action=action.clone()>
                <input type="hidden" name="search" value=search.clone() /><input type="hidden" name="sort" value=sort.clone() />
                <input type="hidden" name="direction" value=direction.clone() />
                {filter_field.clone().map(|value| leptos::view! { <input type="hidden" name="filter_field" value=value /> })}
                {filter_operation.clone().map(|value| leptos::view! { <input type="hidden" name="filter_operation" value=value /> })}
                {filter_value.clone().map(|value| leptos::view! { <input type="hidden" name="filter_value" value=value /> })}
                {filter_end.clone().map(|value| leptos::view! { <input type="hidden" name="filter_end" value=value /> })}
                {audit_action.clone().map(|value| leptos::view! { <input type="hidden" name="action" value=value /> })}
                {audit_resource.clone().map(|value| leptos::view! { <input type="hidden" name="resource" value=value /> })}
                {audit_resource_id.clone().map(|value| leptos::view! { <input type="hidden" name="resource_id" value=value /> })}
                {audit_user_login.clone().map(|value| leptos::view! { <input type="hidden" name="user_login" value=value /> })}
                <input type="hidden" name="offset" value="0" />
                <label><span>"Rows"</span><input name="limit" type="number" min="1" max="100" value=limit.to_string() /></label>
                <button type="submit">"Apply"</button>
            </form>
            <form method="get" action=action.clone()>
                <input type="hidden" name="search" value=search.clone() /><input type="hidden" name="sort" value=sort.clone() />
                <input type="hidden" name="direction" value=direction.clone() /><input type="hidden" name="limit" value=limit.to_string() />
                {filter_field.clone().map(|value| leptos::view! { <input type="hidden" name="filter_field" value=value /> })}
                {filter_operation.clone().map(|value| leptos::view! { <input type="hidden" name="filter_operation" value=value /> })}
                {filter_value.clone().map(|value| leptos::view! { <input type="hidden" name="filter_value" value=value /> })}
                {filter_end.clone().map(|value| leptos::view! { <input type="hidden" name="filter_end" value=value /> })}
                {audit_action.clone().map(|value| leptos::view! { <input type="hidden" name="action" value=value /> })}
                {audit_resource.clone().map(|value| leptos::view! { <input type="hidden" name="resource" value=value /> })}
                {audit_resource_id.clone().map(|value| leptos::view! { <input type="hidden" name="resource_id" value=value /> })}
                {audit_user_login.clone().map(|value| leptos::view! { <input type="hidden" name="user_login" value=value /> })}
                <input type="hidden" name="offset" value=previous_offset.to_string() /><button type="submit" disabled=previous_disabled>"Previous"</button>
            </form>
            <span>{format!("{}-{} of {}", u64::from(offset).saturating_add(1u64).min(u64::from(total)), u64::from(offset).saturating_add(u64::from(limit)).min(u64::from(total)), total)}</span>
            <form method="get" action=action>
                <input type="hidden" name="search" value=search /><input type="hidden" name="sort" value=sort />
                <input type="hidden" name="direction" value=direction /><input type="hidden" name="limit" value=limit.to_string() />
                {filter_field.map(|value| leptos::view! { <input type="hidden" name="filter_field" value=value /> })}
                {filter_operation.map(|value| leptos::view! { <input type="hidden" name="filter_operation" value=value /> })}
                {filter_value.map(|value| leptos::view! { <input type="hidden" name="filter_value" value=value /> })}
                {filter_end.map(|value| leptos::view! { <input type="hidden" name="filter_end" value=value /> })}
                {audit_action.map(|value| leptos::view! { <input type="hidden" name="action" value=value /> })}
                {audit_resource.map(|value| leptos::view! { <input type="hidden" name="resource" value=value /> })}
                {audit_resource_id.map(|value| leptos::view! { <input type="hidden" name="resource_id" value=value /> })}
                {audit_user_login.map(|value| leptos::view! { <input type="hidden" name="user_login" value=value /> })}
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
    let table_path = view.table().frontend_path();
    let action = table_path.to_string();
    let supports_filters = bool::from(view.table().supports_filters());
    let limit = u16::from(query.page().limit()).to_string();
    let active_field = query.filter().field().map(ToString::to_string);
    let active_operation = query.filter().operation();
    let active_value = query.filter().value().map(ToString::to_string);
    let active_end = query.filter().end().map(ToString::to_string);
    let clear_href = table_path.to_string();
    leptos::view! {
        <div class="table-scroll"><table>
            <thead><tr>{view.columns().iter().map(|column| {
                let field = column.name().to_string();
                let label = column.label().to_string();
                let filter_count = column.filters().len().to_string();
                let input_type = match column.input_kind() {
                    server_admin_contract::AdminDataInputKind::Date => "date",
                    server_admin_contract::AdminDataInputKind::DateTime => "datetime-local",
                    server_admin_contract::AdminDataInputKind::Number => "number",
                    server_admin_contract::AdminDataInputKind::Time => "time",
                    server_admin_contract::AdminDataInputKind::Checkbox
                    | server_admin_contract::AdminDataInputKind::Text
                    | server_admin_contract::AdminDataInputKind::Uuid => "text",
                };
                let is_active_field = active_field.as_deref() == Some(field.as_str());
                let filter_label = format!("Filter {label}");
                leptos::view! {
                    <th data-field=field.clone() data-filter-count=filter_count>
                        <div class="table-column-heading">
                            <span>{label}</span>
                            {(supports_filters && !column.filters().is_empty()).then(|| leptos::view! {
                                <details class="table-column-filter" open=is_active_field>
                                    <summary class=("active", is_active_field) aria-label=filter_label>"Filter"</summary>
                                    <div class="table-filter-operations">
                                        {is_active_field.then(|| leptos::view! { <a class="table-filter-clear" href=clear_href.clone()>"Clear"</a> })}
                                        {column.filters().iter().map(|filter| {
                                            let operation = filter.operation();
                                            let operation_key = server_admin_contract::AdminFilterOperationKey::from(operation).to_string();
                                            let is_active = is_active_field && active_operation == Some(operation);
                                            let value = is_active.then(|| active_value.clone()).flatten().unwrap_or_default();
                                            let end = is_active.then(|| active_end.clone()).flatten().unwrap_or_default();
                                            let needs_value = bool::from(filter.requires_value());
                                            let needs_end = bool::from(filter.requires_end());
                                            leptos::view! {
                                                <form class="table-filter-form" method="get" action=action.clone()>
                                                    <input type="hidden" name="filter_field" value=field.clone() />
                                                    <input type="hidden" name="filter_operation" value=operation_key />
                                                    <input type="hidden" name="limit" value=limit.clone() />
                                                    <input type="hidden" name="offset" value="0" />
                                                    <span>{format!("{operation:?}")}</span>
                                                    {needs_value.then(|| leptos::view! { <input name="filter_value" type=input_type value=value required /> })}
                                                    {needs_end.then(|| leptos::view! { <input name="filter_end" type=input_type value=end required /> })}
                                                    <button type="submit">"Apply"</button>
                                                </form>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </details>
                            })}
                        </div>
                    </th>
                }
            }).collect::<Vec<_>>()}
            </tr></thead>
            <tbody>{view.items().iter().map(|row| leptos::view! {
                <tr>{row.values().iter().enumerate().map(|(index, value)| {
                    let column = view.columns().get(index);
                    let label = column.map_or_else(String::new, |item| item.label().to_string());
                    let field = column.map_or_else(String::new, |item| item.name().to_string());
                    let numeric = column.is_some_and(|item| matches!(item.input_kind(), server_admin_contract::AdminDataInputKind::Number));
                    leptos::view! { <td class=("numeric-cell", numeric) data-field=field data-label=label>{value.to_string()}</td> }
                }).collect::<Vec<_>>()}</tr>
            }).collect::<Vec<_>>()}</tbody>
        </table></div>
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
        <section class="table-page">
        {table_filters(server_admin_contract::AdminPage::Users, query, &server_admin_contract::AdminTableSortField::USER)}
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
        {table_pagination(server_admin_contract::AdminPage::Users, query, page.total(), None, None, None)}
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
        {table_filters(server_admin_contract::AdminPage::Roles, query, &server_admin_contract::AdminTableSortField::ROLE)}
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
        {table_pagination(server_admin_contract::AdminPage::Roles, query, page.total(), None, None, None)}
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
        {table_filters(server_admin_contract::AdminPage::Permissions, query, &server_admin_contract::AdminTableSortField::PERMISSION)}
        <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"permission"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td data-label="id">{item.id().to_string()}</td><td data-label="permission">{item.name().to_string()}</td></tr>
        }).collect::<Vec<_>>()}</tbody></table></div>
        {table_pagination(server_admin_contract::AdminPage::Permissions, query, page.total(), None, None, None)}
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
                {table_pagination(server_admin_contract::AdminPage::Tables, query.page(), view.total(), Some(view.table()), bool::from(view.table().supports_filters()).then_some(query.filter()), None)}
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
            <script type="module" src="/admin/assets/csr-bootstrap.js?v=20260722-16"></script>
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
        {table_pagination(server_admin_contract::AdminPage::Sessions, query, page.total(), None, None, None)}
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
    query: &server_admin_contract::AdminTableQuery,
    filters: &server_admin_contract::AdminAuditHtmlQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> AdminSsrHtml {
    let content = leptos::view! {
        <section class="table-page">
        <form class="audit-filters" method="get" action=server_admin_contract::AdminFrontendPath::Audit.get()>
            <label><span>"Action"</span><input name="action" value=filters.action().map(ToString::to_string).unwrap_or_default() /></label><label><span>"Resource"</span><input name="resource" value=filters.resource().map(ToString::to_string).unwrap_or_default() /></label><label><span>"Resource ID"</span><input name="resource_id" value=filters.resource_id().map(ToString::to_string).unwrap_or_default() /></label>
            <label><span>"User login"</span><input name="user_login" value=filters.user_login().map(ToString::to_string).unwrap_or_default() /></label><input name="limit" type="hidden" value=u16::from(query.limit()).to_string() /><input name="offset" type="hidden" value="0" /><button type="submit">"Apply"</button>
        </form>
        <div class="table-scroll"><table><thead><tr><th>"time"</th><th>"user"</th><th>"action"</th><th>"resource"</th><th>"result"</th></tr></thead><tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td data-label="time">{item.created_at().to_string()}</td><td data-label="user">{item.user_login().map(ToString::to_string).unwrap_or_default()}</td><td data-label="action">{item.action().to_string()}</td><td data-label="resource">{item.resource().to_string()}</td><td data-label="result">{item.succeeded().to_string()}</td></tr>
        }).collect::<Vec<_>>()}</tbody></table></div>
        {table_pagination(server_admin_contract::AdminPage::Audit, query, page.total(), None, None, Some(filters))}
        </section>
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
            server_admin_contract::AdminPage::Audit,
            super::AdminSsrHtml::try_from(String::from("<p>ready</p>")).expect("c78bd3a1"),
        );
        assert!(page.as_ref().contains("<p>ready</p>"));
        assert!(!page.as_ref().contains("<h1"));
        assert!(!page.as_ref().contains("<h2"));
        assert!(!page.as_ref().contains("class=\"brand\""));
        assert!(!page.as_ref().contains("nav-dot"));
        assert!(
            page.as_ref().contains(
                format!(
                    "{}</button></form></nav>",
                    str_constants::SIGN_OUT
                        .to_ascii_lowercase()
                        .replace(' ', "_")
                )
                .as_str()
            )
        );
        assert!(!page.as_ref().contains("<script"));
    }

    #[test]
    fn csr_page_contains_only_bootstrap_shell() {
        let admin = server_admin_contract::AuthenticatedAdmin::new(
            server_admin_contract::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
                .expect("642357a8"),
            server_admin_contract::AdminUserId::from(1i64),
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
                .contains("src=\"/admin/assets/csr-bootstrap.js?v=20260722-16\"")
        );
        assert!(!html.as_ref().contains("<nav"));
        assert!(!html.as_ref().contains("<table"));
        assert!(!html.as_ref().contains("<form"));
    }

    #[test]
    fn pagination_preserves_server_side_navigation() {
        let html = super::table_pagination(
            server_admin_contract::AdminPage::Users,
            &server_admin_contract::AdminTableQuery::default(),
            server_admin_contract::AdminPageTotal::from(101u64),
            None,
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
            None,
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

        let filters = server_admin_contract::AdminAuditHtmlQuery::new(
            Some(
                server_admin_contract::AdminText::try_from(
                    str_constants::PG_CRUD_CREATE_PERMISSION_ACTION.to_owned(),
                )
                .expect("3f422443"),
            ),
            None,
            None,
            None,
        );
        let audit_html = super::table_pagination(
            server_admin_contract::AdminPage::Audit,
            &server_admin_contract::AdminTableQuery::default(),
            server_admin_contract::AdminPageTotal::from(21u64),
            None,
            None,
            Some(&filters),
        )
        .render_admin_ssr();
        assert!(
            audit_html
                .as_ref()
                .contains("name=\"action\" value=\"create\"")
        );
    }

    #[test]
    fn navigation_only_contains_accessible_pages() {
        let admin = server_admin_contract::AuthenticatedAdmin::new(
            server_admin_contract::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
                .expect("cdae3e58"),
            server_admin_contract::AdminUserId::from(1i64),
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
                .contains(server_admin_contract::AdminFrontendPath::Audit.get())
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
