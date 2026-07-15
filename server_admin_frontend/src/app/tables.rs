use leptos::prelude::*;
#[component]
fn TableTools(
    state: RwSignal<crate::table_state::TableState>,
    sort_options: &'static [(&'static str, &'static str)],
) -> impl IntoView {
    view! {
        <div class="table-tools">
            <label class="search-field"><span class="sr-only">"Filter rows"</span><input type="search" placeholder="Filter..." aria-label="Filter rows" prop:value=move || state.get().search().0.to_owned() on:input=move |event| { if let Ok(search) = crate::table_state::AdminFrontendTableText::try_from(event_target_value(&event)) { state.update(|value| value.apply_search(search)); } } /></label>
            <label><span>"Sort by"</span><select aria-label="Sort field" on:change=move |event| { if let Ok(sort) = crate::table_state::AdminFrontendTableText::try_from(event_target_value(&event)) { state.update(|value| value.apply_sort(sort)); } }>{sort_options.iter().map(|(value, label)| view! { <option value=*value selected=move || state.get().sort().0 == *value>{*label}</option> }).collect_view()}</select></label>
            <button class="sort-direction" title="Toggle sort direction" aria-label="Toggle sort direction" on:click=move |_| { if let Ok(sort) = crate::table_state::AdminFrontendTableText::try_from(state.get().sort().0.to_owned()) { state.update(|value| value.apply_sort(sort)); } }>{move || match state.get().sort_dir() { crate::table_state::SortDir::Asc => "Asc", crate::table_state::SortDir::Desc => "Desc" }}</button>
            <label><span>"Rows"</span><select aria-label="Rows per page" on:change=move |event| { if let Ok(size) = event_target_value(&event).parse::<usize>() { state.update(|value| value.apply_page_size(crate::table_state::AdminFrontendTableIndex::from(size))); } }><option value="10">"10"</option><option value="20" selected>"20"</option><option value="50">"50"</option><option value="100">"100"</option></select></label>
        </div>
    }
}

#[component]
fn TablePager(
    state: RwSignal<crate::table_state::TableState>,
    total: Signal<usize>,
) -> impl IntoView {
    view! { <div class="table-footer"><span>{move || { let count = total.get(); if count == 0usize { "No results".to_owned() } else { let current = state.get(); format!("{}-{} of {count}", current.start(crate::table_state::AdminFrontendTableIndex::from(count)).0.saturating_add(1usize), current.end(crate::table_state::AdminFrontendTableIndex::from(count)).0) } }}</span><div><button aria-label="Previous page" disabled=move || { state.get().start(crate::table_state::AdminFrontendTableIndex::from(total.get())).0 == 0usize } on:click=move |_| state.update(crate::table_state::TableState::previous)>"Previous"</button><strong>{move || format!("Page {}", state.get().page_number().0)}</strong><button aria-label="Next page" disabled=move || { state.get().end(crate::table_state::AdminFrontendTableIndex::from(total.get())).0 >= total.get() } on:click=move |_| { let count = crate::table_state::AdminFrontendTableIndex::from(total.get()); state.update(|value| value.next(count)); }>"Next"</button></div></div> }
}

fn normalized(value: &str) -> String {
    value.trim().to_lowercase()
}
pub(super) fn error(value: super::state::Text) -> impl IntoView {
    leptos::view! { <div class="alert error page-alert" role="alert"><strong>"Something went wrong"</strong><span>{value.to_string()}</span></div> }
}
pub(super) fn loading() -> impl IntoView {
    leptos::view! { <div class="loading-state"><span class="spinner"></span><strong>"Loading workspace"</strong><p>"Fetching the latest data..."</p></div> }
}
pub(super) fn users_view(
    values: Vec<server_admin_contract::AdminUserSummary>,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create =
        super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::CreateUser);
    let auth = auth.clone();
    let client_for_create = client.clone();
    let state = RwSignal::new(crate::table_state::TableState::new(
        crate::table_state::AdminFrontendTableText::try_from("login".to_owned())
            .unwrap_or_default(),
    ));
    let source = StoredValue::new(values);
    let rows = move || {
        let state_value = state.get();
        let search = normalized(state_value.search().0);
        let mut filtered = source.get_value();
        filtered.retain(|value| {
            search.is_empty()
                || value
                    .login()
                    .as_ref()
                    .to_lowercase()
                    .contains(search.as_str())
                || value
                    .display_name()
                    .as_ref()
                    .to_lowercase()
                    .contains(search.as_str())
                || value.id().to_string().contains(search.as_str())
        });
        filtered.sort_by(|left, right| {
            let order = match state_value.sort().0 {
                "display_name" => left
                    .display_name()
                    .as_ref()
                    .cmp(right.display_name().as_ref()),
                "id" => i64::from(left.id()).cmp(&i64::from(right.id())),
                "status" => bool::from(left.is_banned()).cmp(&bool::from(right.is_banned())),
                _ => left.login().as_ref().cmp(right.login().as_ref()),
            };
            match state_value.sort_dir() {
                crate::table_state::SortDir::Asc => order,
                crate::table_state::SortDir::Desc => order.reverse(),
            }
        });
        filtered
    };
    let rows_for_total = rows.clone();
    let total = Signal::derive(move || rows_for_total().len());
    let content = view! { <div class="crud-content">
    <button disabled=!can_create on:click=move |_| { if let (Some(login), Some(display_name), Some(password)) = (super::prompt("Login", ""), super::prompt("Display name", ""), super::prompt("Password", "")) && let (Ok(login), Ok(display_name), Ok(password)) = (server_admin_contract::AdminLogin::try_from(login.0), server_admin_contract::AdminDisplayName::try_from(display_name.0), server_admin_contract::AdminPassword::try_from(password.0)) { let body = server_admin_contract::AdminCreateUserReq::new(display_name, login, password); let action_client = client_for_create.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::CreateUser, body), action_client, loader); } }>"Create user"</button>
    <TableTools state sort_options=&contract_constants::admin_table::USER_SORTS />
    <table><thead><tr><th>"ID"</th><th>"Login"</th><th>"Display name"</th><th>"Banned"</th><th>"Actions"</th></tr></thead><tbody>
    {move || { let all_rows = rows(); let current = state.get(); let start = current.start(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; let end = current.end(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; all_rows[start..end].iter().cloned().map(|value| { let edit_client = client.clone(); let ban_client = client.clone(); let password_client = client.clone(); let roles_client = client.clone(); let delete_client = client.clone(); let id = value.id(); let edit_login = value.login().clone(); let edit_display_name = value.display_name().clone(); let delete_login = value.login().clone(); let is_banned = bool::from(value.is_banned()); view! { <tr><td>{id.to_string()}</td><td>{value.login().to_string()}</td><td>{value.display_name().to_string()}</td><td>{is_banned.to_string()}</td><td>
    <button disabled=!super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::UpdateUser(id)) on:click=move |_| { if let (Some(login), Some(display_name)) = (super::prompt("Login", edit_login.as_ref()), super::prompt("Display name", edit_display_name.as_ref())) && let (Ok(login), Ok(display_name)) = (server_admin_contract::AdminLogin::try_from(login.0), server_admin_contract::AdminDisplayName::try_from(display_name.0)) { let body = server_admin_contract::AdminUpdateUserReq::new(Some(display_name), Some(login)); let action_client = edit_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateUser(id), body), action_client, loader); } }>"Edit"</button>
    <button disabled=!super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::SetUserBan(id)) on:click=move |_| { let body = server_admin_contract::AdminSetUserBanReq::new(server_admin_contract::AdminBool::from(!is_banned)); let action_client = ban_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserBan(id), body), action_client, loader); }>{if is_banned { "Unban" } else { "Ban" }}</button>
    <button disabled=!super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::SetUserPassword(id)) on:click=move |_| { if let Some(password) = super::prompt("New password", "") && let Ok(password) = server_admin_contract::AdminPassword::try_from(password.0) { let body = server_admin_contract::AdminSetUserPasswordReq::new(password); let action_client = password_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserPassword(id), body), action_client, loader); } }>"Password"</button>
    <button disabled=!super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::SetUserRoles(id)) on:click=move |_| { if let Some(value) = super::prompt("Role IDs separated by commas", "") { let body = server_admin_contract::AdminSetUserRolesReq::from_ids(super::forms::role_ids(&value.0)); let action_client = roles_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserRoles(id), body), action_client, loader); } }>"Roles"</button>
    <button disabled=!super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::DeleteUser(id)) on:click=move |_| { let confirmed = super::browser_window().and_then(|window| window.confirm_with_message(&format!("Delete {delete_login}?")).ok()).unwrap_or(false); if confirmed { let action_client = delete_client.clone(); super::run_action(action_client.clone().send(server_admin_contract::AdminRoute::DeleteUser(id)), action_client, loader); } }>"Delete"</button>
    </td></tr> } }).collect_view() }}
    </tbody></table><TablePager state total /></div> };
    crud_page(server_admin_contract::AdminPage::Users, content)
}
pub(super) fn roles_view(
    values: Vec<server_admin_contract::AdminRoleSummary>,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create =
        super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::CreateRole);
    let auth = auth.clone();
    let client_for_create = client.clone();
    let state = RwSignal::new(crate::table_state::TableState::new(
        crate::table_state::AdminFrontendTableText::try_from("name".to_owned()).unwrap_or_default(),
    ));
    let source = StoredValue::new(values);
    let rows = move || {
        let state_value = state.get();
        let search = normalized(state_value.search().0);
        let mut filtered = source.get_value();
        filtered.retain(|value| {
            search.is_empty()
                || value
                    .name()
                    .as_ref()
                    .to_lowercase()
                    .contains(search.as_str())
                || value.id().to_string().contains(search.as_str())
        });
        filtered.sort_by(|left, right| {
            let order = match state_value.sort().0 {
                "id" => i64::from(left.id()).cmp(&i64::from(right.id())),
                "system" => bool::from(left.is_system()).cmp(&bool::from(right.is_system())),
                _ => left.name().as_ref().cmp(right.name().as_ref()),
            };
            match state_value.sort_dir() {
                crate::table_state::SortDir::Asc => order,
                crate::table_state::SortDir::Desc => order.reverse(),
            }
        });
        filtered
    };
    let rows_for_total = rows.clone();
    let total = Signal::derive(move || rows_for_total().len());
    let content = view! { <section class="crud-content"><button disabled=!can_create on:click=move |_| { if let Some(name) = super::prompt("Name", "") && let Ok(name) = server_admin_contract::AdminRoleName::try_from(name.0) { let body = server_admin_contract::AdminCreateRoleReq::new(name); let action_client = client_for_create.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::CreateRole, body), action_client, loader); } }>"Create role"</button><TableTools state sort_options=&contract_constants::admin_table::ROLE_SORTS />
    <table><thead><tr><th>"ID"</th><th>"Name"</th><th>"System"</th><th>"Actions"</th></tr></thead><tbody>{move || { let all_rows = rows(); let current = state.get(); let start = current.start(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; let end = current.end(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; all_rows[start..end].iter().cloned().map(|value| { let edit_client = client.clone(); let permissions_client = client.clone(); let delete_client = client.clone(); let id = value.id(); let edit_name = value.name().clone(); let delete_name = value.name().clone(); view! { <tr><td>{id.to_string()}</td><td>{value.name().to_string()}</td><td>{value.is_system().to_string()}</td><td><button disabled=!super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::UpdateRole(id)) on:click=move |_| { if let Some(name) = super::prompt("Name", edit_name.as_ref()) && let Ok(name) = server_admin_contract::AdminRoleName::try_from(name.0) { let body = server_admin_contract::AdminUpdateRoleReq::new(name); let action_client = edit_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateRole(id), body), action_client, loader); } }>"Edit"</button><button disabled=!super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::SetRolePermissions(id)) on:click=move |_| { if let Some(value) = super::prompt("Permission IDs separated by commas", "") { let body = server_admin_contract::AdminSetRolePermissionsReq::from_ids(super::forms::permission_ids(&value.0)); let action_client = permissions_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetRolePermissions(id), body), action_client, loader); } }>"Permissions"</button><button disabled=!super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::DeleteRole(id)) on:click=move |_| { let confirmed = super::browser_window().and_then(|window| window.confirm_with_message(&format!("Delete {delete_name}?")).ok()).unwrap_or(false); if confirmed { let action_client = delete_client.clone(); super::run_action(action_client.clone().send(server_admin_contract::AdminRoute::DeleteRole(id)), action_client, loader); } }>"Delete"</button></td></tr> } }).collect_view() }}</tbody></table><TablePager state total /></section> };
    crud_page(server_admin_contract::AdminPage::Roles, content)
}

pub(super) fn permissions_view(
    values: Vec<server_admin_contract::AdminPermissionSummary>,
) -> impl IntoView {
    let state = RwSignal::new(crate::table_state::TableState::new(
        crate::table_state::AdminFrontendTableText::try_from("name".to_owned()).unwrap_or_default(),
    ));
    let source = StoredValue::new(values);
    let rows = move || {
        let state_value = state.get();
        let search = normalized(state_value.search().0);
        let mut filtered = source.get_value();
        filtered.retain(|value| {
            search.is_empty()
                || value
                    .name()
                    .as_ref()
                    .to_lowercase()
                    .contains(search.as_str())
                || value.id().to_string().contains(search.as_str())
        });
        filtered.sort_by(|left, right| {
            let order = if state_value.sort().0 == "id" {
                i64::from(left.id()).cmp(&i64::from(right.id()))
            } else {
                left.name().as_ref().cmp(right.name().as_ref())
            };
            match state_value.sort_dir() {
                crate::table_state::SortDir::Asc => order,
                crate::table_state::SortDir::Desc => order.reverse(),
            }
        });
        filtered
    };
    let rows_for_total = rows.clone();
    let total = Signal::derive(move || rows_for_total().len());
    let content = view! { <div class="crud-content"><TableTools state sort_options=&contract_constants::admin_table::PERMISSION_SORTS /><table><thead><tr><th>"ID"</th><th>"Name"</th></tr></thead><tbody>{move || { let all_rows = rows(); let current = state.get(); let start = current.start(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; let end = current.end(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; all_rows[start..end].iter().cloned().map(|value| view! { <tr><td>{value.id().to_string()}</td><td>{value.name().to_string()}</td></tr> }).collect_view() }}</tbody></table><TablePager state total /></div> };
    crud_page(server_admin_contract::AdminPage::Permissions, content)
}

pub(super) fn audit_view(values: Vec<server_admin_contract::AdminAuditView>) -> impl IntoView {
    let state = RwSignal::new(crate::table_state::TableState::new(
        crate::table_state::AdminFrontendTableText::try_from("created_at".to_owned())
            .unwrap_or_default(),
    ));
    let source = StoredValue::new(values);
    let rows = move || {
        let state_value = state.get();
        let search = normalized(state_value.search().0);
        let mut filtered = source.get_value();
        filtered.retain(|value| {
            search.is_empty()
                || value
                    .action()
                    .to_string()
                    .to_lowercase()
                    .contains(search.as_str())
                || value
                    .resource()
                    .to_string()
                    .to_lowercase()
                    .contains(search.as_str())
                || value
                    .user_id()
                    .map(|id| id.to_string().contains(search.as_str()))
                    .unwrap_or(false)
        });
        filtered.sort_by(|left, right| {
            let order = match state_value.sort().0 {
                "user_id" => left
                    .user_id()
                    .map(i64::from)
                    .cmp(&right.user_id().map(i64::from)),
                "action" => left.action().to_string().cmp(&right.action().to_string()),
                "resource" => left
                    .resource()
                    .to_string()
                    .cmp(&right.resource().to_string()),
                "succeeded" => bool::from(left.succeeded()).cmp(&bool::from(right.succeeded())),
                _ => left.created_at().as_ref().cmp(right.created_at().as_ref()),
            };
            match state_value.sort_dir() {
                crate::table_state::SortDir::Asc => order,
                crate::table_state::SortDir::Desc => order.reverse(),
            }
        });
        filtered
    };
    let rows_for_total = rows.clone();
    let total = Signal::derive(move || rows_for_total().len());
    let content = view! { <div class="crud-content"><TableTools state sort_options=&contract_constants::admin_table::AUDIT_SORTS /><table><thead><tr><th>"Time"</th><th>"User"</th><th>"Action"</th><th>"Resource"</th><th>"Result"</th></tr></thead><tbody>{move || { let all_rows = rows(); let current = state.get(); let start = current.start(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; let end = current.end(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; all_rows[start..end].iter().cloned().map(|value| view! { <tr><td>{value.created_at().to_string()}</td><td>{value.user_id().map(|id| id.to_string()).unwrap_or_default()}</td><td>{value.action().to_string()}</td><td>{value.resource().to_string()}</td><td>{value.succeeded().to_string()}</td></tr> }).collect_view() }}</tbody></table><TablePager state total /></div> };
    crud_page(server_admin_contract::AdminPage::Audit, content)
}
fn crud_page(page: server_admin_contract::AdminPage, content: impl IntoView) -> impl IntoView {
    view! { <section><div class="page-heading"><div><p class="eyebrow">"Administration"</p><h1>{page.title().as_ref().to_owned()}</h1></div></div>{content}</section> }
}
