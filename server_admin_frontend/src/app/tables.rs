use leptos::prelude::*;
#[component]
fn TableTools(
    state: RwSignal<crate::table_state::TableState>,
    sort_options: &'static [server_admin_contract::AdminTableSortField],
    #[prop(optional)] on_change: Option<Callback<crate::table_state::TableState>>,
) -> impl IntoView {
    let on_change = on_change.or_else(use_context::<Callback<crate::table_state::TableState>>);
    view! {
        <div class="table-tools">
            <label class="search-field"><span class="sr-only">"Filter rows"</span><input type="search" placeholder="Filter..." aria-label="Filter rows" prop:value=move || state.get().search().0.to_owned() on:input=move |event| { if let Ok(search) = crate::table_state::AdminFrontendTableText::try_from(event_target_value(&event)) { state.update(|value| value.apply_search(search)); if let Some(callback) = on_change { callback.run(state.get_untracked()); } } } /></label>
            <label><span>"Sort by"</span><select aria-label="Sort field" on:change=move |event| { let event_value = event_target_value(&event); if let Ok(sort) = server_admin_contract::AdminTableSortField::try_from_key(sort_options, server_admin_contract::AdminTableSortKeyRef::from(event_value.as_str())) { state.update(|value| value.apply_sort(sort)); if let Some(callback) = on_change { callback.run(state.get_untracked()); } } }>{sort_options.iter().copied().map(|option| { let value = option.key().to_string(); let label = option.label().to_string(); view! { <option value=value selected=move || state.get().sort() == option>{label}</option> } }).collect_view()}</select></label>
            <button class="sort-direction" title="Toggle sort direction" aria-label="Toggle sort direction" on:click=move |_| { let sort = state.get().sort(); state.update(|value| value.apply_sort(sort)); if let Some(callback) = on_change { callback.run(state.get_untracked()); } }>{move || match state.get().sort_dir() { crate::table_state::SortDir::Asc => "Asc", crate::table_state::SortDir::Desc => "Desc" }}</button>
            <label><span>"Rows"</span><select aria-label="Rows per page" on:change=move |event| { if let Ok(size) = event_target_value(&event).parse::<usize>() { state.update(|value| value.apply_page_size(crate::table_state::AdminFrontendTableIndex::from(size))); if let Some(callback) = on_change { callback.run(state.get_untracked()); } } }><option value="10" selected=move || state.get().query().starts_with("limit=10&")>"10"</option><option value="20" selected=move || state.get().query().starts_with("limit=20&")>"20"</option><option value="50" selected=move || state.get().query().starts_with("limit=50&")>"50"</option><option value="100" selected=move || state.get().query().starts_with("limit=100&")>"100"</option></select></label>
        </div>
    }
}

#[component]
fn TablePager(
    state: RwSignal<crate::table_state::TableState>,
    total: Signal<usize>,
    #[prop(optional)] on_change: Option<Callback<crate::table_state::TableState>>,
) -> impl IntoView {
    let on_change = on_change.or_else(use_context::<Callback<crate::table_state::TableState>>);
    view! { <div class="table-footer"><span>{move || { let count = total.get(); if count == 0usize { "No results".to_owned() } else { let current = state.get(); format!("{}-{} of {count}", current.start(crate::table_state::AdminFrontendTableIndex::from(count)).0.saturating_add(1usize), current.end(crate::table_state::AdminFrontendTableIndex::from(count)).0) } }}</span><div><button aria-label="Previous page" disabled=move || { state.get().start(crate::table_state::AdminFrontendTableIndex::from(total.get())).0 == 0usize } on:click=move |_| { state.update(crate::table_state::TableState::previous); if let Some(callback) = on_change { callback.run(state.get_untracked()); } }>"Previous"</button><strong>{move || format!("Page {}", state.get().page_number().0)}</strong><button aria-label="Next page" disabled=move || { state.get().end(crate::table_state::AdminFrontendTableIndex::from(total.get())).0 >= total.get() } on:click=move |_| { let count = crate::table_state::AdminFrontendTableIndex::from(total.get()); state.update(|value| value.next(count)); if let Some(callback) = on_change { callback.run(state.get_untracked()); } }>"Next"</button></div></div> }
}

fn normalized(value: &str) -> String {
    value.trim().to_lowercase()
}
fn run_mutation<FutureValue>(
    future: FutureValue,
    pending: RwSignal<bool>,
    error: RwSignal<Option<String>>,
    success: &'static str,
    client: super::AdminApiClient,
    loader: super::PageLoader,
) where
    FutureValue: Future<Output = Result<(), super::ApiError>> + 'static,
{
    pending.set(true);
    error.set(None);
    leptos::task::spawn_local(async move {
        match future.await {
            Ok(()) => {
                loader.set_notice(
                    super::state::Text::try_from(success.to_owned()).unwrap_or_default(),
                );
                super::load(client, loader);
            }
            Err(value) => error.set(Some(value.to_string())),
        }
        pending.set(false);
    });
}

#[component]
fn CreateUserForm(
    client: super::AdminApiClient,
    loader: super::PageLoader,
    can_create: bool,
) -> impl IntoView {
    let login = RwSignal::new(String::new());
    let display_name = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    view! { <details class="mutation-form"><summary>"Create user"</summary><form on:submit=move |event| {
        event.prevent_default();
        let values = (
            server_admin_contract::AdminLogin::try_from(login.get_untracked()),
            server_admin_contract::AdminDisplayName::try_from(display_name.get_untracked()),
            server_admin_contract::AdminNewPassword::try_from(password.get_untracked()),
        );
        match values {
            (Ok(login), Ok(display_name), Ok(password)) => {
                let body = server_admin_contract::AdminCreateUserReq::new(display_name, login, password);
                let action_client = client.clone();
                run_mutation(action_client.clone().send_json(server_admin_contract::AdminRoute::CreateUser, body), pending, error, "User created", action_client, loader);
            }
            _ => error.set(Some("Check login, display name and password requirements".to_owned())),
        }
    }><label><span>"Login"</span><input aria-label="New user login" prop:value=move || login.get() on:input=move |event| login.set(event_target_value(&event)) /></label><label><span>"Display name"</span><input aria-label="New user display name" prop:value=move || display_name.get() on:input=move |event| display_name.set(event_target_value(&event)) /></label><label><span>"Password"</span><input type="password" aria-label="New user password" prop:value=move || password.get() on:input=move |event| password.set(event_target_value(&event)) /></label>{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button type="submit" disabled=move || !can_create || pending.get()>{move || if pending.get() { "Creating..." } else { "Create user" }}</button></form></details> }
}

#[component]
fn EditUserForm(
    user_id: server_admin_contract::AdminUserId,
    current_login: server_admin_contract::AdminLogin,
    current_display_name: server_admin_contract::AdminDisplayName,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    can_update: bool,
) -> impl IntoView {
    let login = RwSignal::new(current_login.to_string());
    let display_name = RwSignal::new(current_display_name.to_string());
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    view! { <details class="mutation-form compact"><summary>"Edit"</summary><form on:submit:capture=move |event| {
        event.prevent_default();
        match (server_admin_contract::AdminLogin::try_from(login.get_untracked()), server_admin_contract::AdminDisplayName::try_from(display_name.get_untracked())) {
            (Ok(login), Ok(display_name)) => { let body = server_admin_contract::AdminUpdateUserReq::new(Some(display_name), Some(login)); let action_client = client.clone(); run_mutation(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateUser(user_id), body), pending, error, "User updated", action_client, loader); }
            _ => error.set(Some("Check login and display name requirements".to_owned())),
        }
    }><label><span>"Login"</span><input aria-label="Edit user login" prop:value=move || login.get() on:input=move |event| login.set(event_target_value(&event)) /></label><label><span>"Display name"</span><input aria-label="Edit user display name" prop:value=move || display_name.get() on:input=move |event| display_name.set(event_target_value(&event)) /></label>{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button type="submit" disabled=move || !can_update || pending.get()>"Save"</button></form></details> }
}

#[component]
fn UserPasswordForm(
    user_id: server_admin_contract::AdminUserId,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    can_update: bool,
) -> impl IntoView {
    let password = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    view! { <details class="mutation-form compact"><summary>"Password"</summary><form on:submit:capture=move |event| { event.prevent_default(); match server_admin_contract::AdminNewPassword::try_from(password.get_untracked()) { Ok(password) => { let body = server_admin_contract::AdminSetUserPasswordReq::new(password); let action_client = client.clone(); run_mutation(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserPassword(user_id), body), pending, error, "Password updated", action_client, loader); }, Err(_error) => error.set(Some("Password does not meet the policy".to_owned())), } }><label><span>"New password"</span><input type="password" aria-label="Change user password" prop:value=move || password.get() on:input=move |event| password.set(event_target_value(&event)) /></label>{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button type="submit" disabled=move || !can_update || pending.get()>"Change password"</button></form></details> }
}

#[component]
fn BanUserForm(
    user_id: server_admin_contract::AdminUserId,
    is_banned: bool,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    can_update: bool,
) -> impl IntoView {
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    let save = Callback::new(move |()| {
        let body = server_admin_contract::AdminSetUserBanReq::new(
            server_admin_contract::AdminBool::from(!is_banned),
        );
        let action_client = client.clone();
        run_mutation(
            action_client
                .clone()
                .send_json(server_admin_contract::AdminRoute::SetUserBan(user_id), body),
            pending,
            error,
            if is_banned {
                str_constants::USER_UNBANNED_NOTICE
            } else {
                str_constants::USER_BANNED_NOTICE
            },
            action_client,
            loader,
        );
    });
    let save_for_pointer = save;
    let save_for_submit = save;
    view! { <form class="inline-mutation" on:submit:capture=move |event| { event.prevent_default(); save_for_submit.run(()); }>{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button type="submit" disabled=move || !can_update || pending.get() on:pointerdown=move |event| { event.prevent_default(); save_for_pointer.run(()); }>{move || if pending.get() { "Saving..." } else if is_banned { "Unban" } else { "Ban" }}</button></form> }
}

#[component]
fn DeleteUserForm(
    user_id: server_admin_contract::AdminUserId,
    login: server_admin_contract::AdminLogin,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    can_delete: bool,
) -> impl IntoView {
    let confirmation = RwSignal::new(String::new());
    let expected = login.to_string();
    let expected_for_submit = expected.clone();
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    view! { <details class="mutation-form compact danger"><summary>"Delete"</summary><form on:submit:capture=move |event| { event.prevent_default(); if confirmation.get_untracked() == expected_for_submit { let action_client = client.clone(); run_mutation(action_client.clone().send(server_admin_contract::AdminRoute::DeleteUser(user_id)), pending, error, "User deleted", action_client, loader); } else { error.set(Some("Type the login exactly to confirm deletion".to_owned())); } }><p>{format!("This permanently deletes {expected}. Type the login to continue.")}</p><label><span>"Confirm login"</span><input aria-label="Confirm user deletion" prop:value=move || confirmation.get() on:input=move |event| confirmation.set(event_target_value(&event)) /></label>{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button class="danger-button" type="submit" disabled=move || !can_delete || pending.get()>"Delete permanently"</button></form></details> }
}

#[component]
fn RoleForm(
    role_id: Option<server_admin_contract::AdminRoleId>,
    initial_name: String,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    enabled: bool,
) -> impl IntoView {
    let name = RwSignal::new(initial_name);
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    let title = if role_id.is_some() {
        str_constants::EDIT
    } else {
        str_constants::CREATE_ROLE
    };
    view! { <details class="mutation-form compact"><summary>{title}</summary><form on:submit:capture=move |event| { event.prevent_default(); match server_admin_contract::AdminRoleName::try_from(name.get_untracked()) { Ok(name) => { let action_client = client.clone(); if let Some(role_id) = role_id { let body = server_admin_contract::AdminUpdateRoleReq::new(name); run_mutation(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateRole(role_id), body), pending, error, "Role updated", action_client, loader); } else { let body = server_admin_contract::AdminCreateRoleReq::new(name); run_mutation(action_client.clone().send_json(server_admin_contract::AdminRoute::CreateRole, body), pending, error, "Role created", action_client, loader); } }, Err(_error) => error.set(Some("Role name does not meet the requirements".to_owned())), } }><label><span>"Name"</span><input aria-label=if role_id.is_some() { "Edit role name" } else { "New role name" } prop:value=move || name.get() on:input=move |event| name.set(event_target_value(&event)) /></label>{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button type="submit" disabled=move || !enabled || pending.get()>{if role_id.is_some() { "Save" } else { "Create role" }}</button></form></details> }
}

#[component]
fn DeleteRoleForm(
    role_id: server_admin_contract::AdminRoleId,
    name: server_admin_contract::AdminRoleName,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    enabled: bool,
) -> impl IntoView {
    let confirmation = RwSignal::new(String::new());
    let expected = name.to_string();
    let expected_for_submit = expected.clone();
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    view! { <details class="mutation-form compact danger"><summary>"Delete"</summary><form on:submit:capture=move |event| { event.prevent_default(); if confirmation.get_untracked() == expected_for_submit { let action_client = client.clone(); run_mutation(action_client.clone().send(server_admin_contract::AdminRoute::DeleteRole(role_id)), pending, error, "Role deleted", action_client, loader); } else { error.set(Some("Type the role name exactly to confirm deletion".to_owned())); } }><p>{format!("Deleting {expected} removes its assignments. Type the name to continue.")}</p><label><span>"Confirm role name"</span><input aria-label="Confirm role deletion" prop:value=move || confirmation.get() on:input=move |event| confirmation.set(event_target_value(&event)) /></label>{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button class="danger-button" type="submit" disabled=move || !enabled || pending.get()>"Delete permanently"</button></form></details> }
}
pub(super) fn error(value: super::state::Text) -> impl IntoView {
    leptos::view! { <div class="alert error page-alert" role="alert"><strong>"Something went wrong"</strong><span>{value.to_string()}</span></div> }
}
pub(super) fn loading() -> impl IntoView {
    leptos::view! { <div class="loading-state"><span class="spinner"></span><strong>"Loading workspace"</strong><p>"Fetching the latest data..."</p></div> }
}
#[component]
fn UserRoleEditor(
    user_id: server_admin_contract::AdminUserId,
    assigned: Vec<server_admin_contract::AdminRoleId>,
    available: Vec<server_admin_contract::AdminRoleSummary>,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    can_update: bool,
) -> impl IntoView {
    let expected = assigned.clone();
    let expected_for_diff = assigned.clone();
    let available_for_diff = available.clone();
    let selected = RwSignal::new(assigned);
    let search = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    let save = Callback::new(move |()| {
        let body =
            server_admin_contract::AdminSetUserRolesReq::new(expected.clone(), selected.get());
        let action_client = client.clone();
        run_mutation(
            action_client.clone().send_json(
                server_admin_contract::AdminRoute::SetUserRoles(user_id),
                body,
            ),
            pending,
            error,
            str_constants::USER_ROLES_UPDATED_NOTICE,
            action_client,
            loader,
        );
    });
    let save_for_pointer = save;
    let save_for_submit = save;
    view! {
        <details class="assignment-editor">
            <summary>"Roles"</summary>
            <label><span class="sr-only">"Filter roles"</span><input type="search" placeholder="Filter roles..." prop:value=move || search.get() on:input=move |event| search.set(event_target_value(&event)) /></label>
            <form on:submit:capture=move |event| { event.prevent_default(); save_for_submit.run(()); }>
            <fieldset><legend>"Assigned roles"</legend>{available.into_iter().map(|role| { let role_id = role.id(); let role_name = role.name().to_string(); view! {
                <label style:display=move || if role_name.to_lowercase().contains(search.get().to_lowercase().as_str()) { "flex" } else { "none" }><input type="checkbox" prop:checked=move || selected.get().contains(&role_id) on:change=move |event| { let checked = event_target_checked(&event); selected.update(|ids| { ids.retain(|id| *id != role_id); if checked { ids.push(role_id); } }); } />{role_name.clone()}</label>
            } }).collect_view()}</fieldset>
            <p class="assignment-diff" role="status">{move || { let future = selected.get(); let added = available_for_diff.iter().filter(|role| future.contains(&role.id()) && !expected_for_diff.contains(&role.id())).map(|role| role.name().to_string()).collect::<Vec<_>>(); let removed = available_for_diff.iter().filter(|role| !future.contains(&role.id()) && expected_for_diff.contains(&role.id())).map(|role| role.name().to_string()).collect::<Vec<_>>(); format!("Will add: {}; will remove: {}", if added.is_empty() { "none".to_owned() } else { added.join(", ") }, if removed.is_empty() { "none".to_owned() } else { removed.join(", ") }) }}</p>
            {move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button type="submit" disabled=move || !can_update || pending.get() on:pointerdown=move |event| { event.prevent_default(); save_for_pointer.run(()); }>{move || if pending.get() { "Saving..." } else { "Save roles" }}</button>
            </form>
        </details>
    }
}
#[component]
fn RolePermissionEditor(
    role_id: server_admin_contract::AdminRoleId,
    assigned: Vec<server_admin_contract::AdminPermissionId>,
    available: Vec<server_admin_contract::AdminPermissionSummary>,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    can_update: bool,
) -> impl IntoView {
    let expected = assigned.clone();
    let expected_for_diff = assigned.clone();
    let available_for_diff = available.clone();
    let selected = RwSignal::new(assigned);
    let search = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    let save = Callback::new(move |()| {
        let body = server_admin_contract::AdminSetRolePermissionsReq::new(
            expected.clone(),
            selected.get(),
        );
        let action_client = client.clone();
        run_mutation(
            action_client.clone().send_json(
                server_admin_contract::AdminRoute::SetRolePermissions(role_id),
                body,
            ),
            pending,
            error,
            str_constants::ROLE_PERMISSIONS_UPDATED_NOTICE,
            action_client,
            loader,
        );
    });
    let save_for_pointer = save;
    let save_for_submit = save;
    view! {
        <details class="assignment-editor">
            <summary>"Permissions"</summary>
            <label><span class="sr-only">"Filter permissions"</span><input type="search" placeholder="Filter permissions..." prop:value=move || search.get() on:input=move |event| search.set(event_target_value(&event)) /></label>
            <form on:submit:capture=move |event| { event.prevent_default(); save_for_submit.run(()); }>
            <fieldset><legend>"Granted permissions"</legend>{available.into_iter().map(|permission| { let permission_id = permission.id(); let permission_name = permission.name().to_string(); view! {
                <label style:display=move || if permission_name.to_lowercase().contains(search.get().to_lowercase().as_str()) { "flex" } else { "none" }><input type="checkbox" disabled=!can_update prop:checked=move || selected.get().contains(&permission_id) on:change=move |event| { let checked = event_target_checked(&event); selected.update(|ids| { ids.retain(|id| *id != permission_id); if checked { ids.push(permission_id); } }); } />{permission_name.clone()}</label>
            } }).collect_view()}</fieldset>
            <p class="assignment-diff" role="status">{move || { let future = selected.get(); let added = available_for_diff.iter().filter(|permission| future.contains(&permission.id()) && !expected_for_diff.contains(&permission.id())).map(|permission| permission.name().to_string()).collect::<Vec<_>>(); let removed = available_for_diff.iter().filter(|permission| !future.contains(&permission.id()) && expected_for_diff.contains(&permission.id())).map(|permission| permission.name().to_string()).collect::<Vec<_>>(); format!("Will add: {}; will remove: {}", if added.is_empty() { "none".to_owned() } else { added.join(", ") }, if removed.is_empty() { "none".to_owned() } else { removed.join(", ") }) }}</p>
            {move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button type="submit" disabled=move || !can_update || pending.get() on:pointerdown=move |event| { event.prevent_default(); save_for_pointer.run(()); }>{move || if pending.get() { "Saving..." } else { "Save permissions" }}</button>
            </form>
        </details>
    }
}
pub(super) fn users_view(
    values: Vec<server_admin_contract::AdminUserSummary>,
    roles: Vec<server_admin_contract::AdminRoleSummary>,
    server_total: server_admin_contract::AdminPageTotal,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create =
        super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::CreateUser);
    let auth = auth.clone();
    let client_for_create = client.clone();
    let state = RwSignal::new(super::table_state(
        server_admin_contract::AdminTableSortField::UserLogin,
        &server_admin_contract::AdminTableSortField::USER,
    ));
    let table_client = client.clone();
    let on_change = Callback::new(move |state: crate::table_state::TableState| {
        super::apply_table_state_url(server_admin_contract::AdminPage::Users, &state);
        super::load(table_client.clone(), loader);
    });
    let source = StoredValue::new(values);
    let roles = StoredValue::new(roles);
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
            let order = match state_value.sort() {
                server_admin_contract::AdminTableSortField::UserDisplayName => left
                    .display_name()
                    .as_ref()
                    .cmp(right.display_name().as_ref()),
                server_admin_contract::AdminTableSortField::UserId => {
                    i64::from(left.id()).cmp(&i64::from(right.id()))
                }
                server_admin_contract::AdminTableSortField::UserStatus => {
                    bool::from(left.is_banned()).cmp(&bool::from(right.is_banned()))
                }
                server_admin_contract::AdminTableSortField::UserLogin => {
                    left.login().as_ref().cmp(right.login().as_ref())
                }
                _ => std::cmp::Ordering::Equal,
            };
            match state_value.sort_dir() {
                crate::table_state::SortDir::Asc => order,
                crate::table_state::SortDir::Desc => order.reverse(),
            }
        });
        filtered
    };
    let total_value = usize::try_from(u64::from(server_total)).unwrap_or(usize::MAX);
    let total = Signal::derive(move || total_value);
    let content = view! { <div class="crud-content">
    <CreateUserForm client=client_for_create loader can_create />
    <TableTools state sort_options=&server_admin_contract::AdminTableSortField::USER on_change />
    <table><thead><tr><th>"ID"</th><th>"Login"</th><th>"Display name"</th><th>"Banned"</th><th>"Actions"</th></tr></thead><tbody>
    <For each=move || { let all_rows = rows(); let current = state.get(); let start = current.start(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; let end = current.end(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; all_rows[start..end].to_vec() } key=|value| value.id() children=move |value| { let edit_client = client.clone(); let ban_client = client.clone(); let password_client = client.clone(); let roles_client = client.clone(); let delete_client = client.clone(); let id = value.id(); let edit_login = value.login().clone(); let edit_display_name = value.display_name().clone(); let delete_login = value.login().clone(); let is_banned = bool::from(value.is_banned()); let assigned_role_ids = value.role_ids().to_vec(); view! { <tr><td>{id.to_string()}</td><td>{value.login().to_string()}</td><td>{value.display_name().to_string()}</td><td>{is_banned.to_string()}</td><td>
    <EditUserForm user_id=id current_login=edit_login current_display_name=edit_display_name client=edit_client loader can_update=super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::UpdateUser(id)) />
    <BanUserForm user_id=id is_banned client=ban_client loader can_update=super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::SetUserBan(id)) />
    <UserPasswordForm user_id=id client=password_client loader can_update=super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::SetUserPassword(id)) />
    <UserRoleEditor user_id=id assigned=assigned_role_ids available=roles.get_value() client=roles_client loader can_update=super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::SetUserRoles(id)) />
    <DeleteUserForm user_id=id login=delete_login client=delete_client loader can_delete=super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::DeleteUser(id)) />
    </td></tr> } } />
    </tbody></table><TablePager state total on_change /></div> };
    crud_page(server_admin_contract::AdminPage::Users, content)
}
pub(super) fn roles_view(
    values: Vec<server_admin_contract::AdminRoleSummary>,
    permissions: Vec<server_admin_contract::AdminPermissionSummary>,
    server_total: server_admin_contract::AdminPageTotal,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create =
        super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::CreateRole);
    let auth = auth.clone();
    let client_for_create = client.clone();
    let state = RwSignal::new(super::table_state(
        server_admin_contract::AdminTableSortField::RoleName,
        &server_admin_contract::AdminTableSortField::ROLE,
    ));
    let table_client = client.clone();
    let on_change = Callback::new(move |state: crate::table_state::TableState| {
        super::apply_table_state_url(server_admin_contract::AdminPage::Roles, &state);
        super::load(table_client.clone(), loader);
    });
    provide_context(on_change);
    let source = StoredValue::new(values);
    let permissions = StoredValue::new(permissions);
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
            let order = match state_value.sort() {
                server_admin_contract::AdminTableSortField::RoleId => {
                    i64::from(left.id()).cmp(&i64::from(right.id()))
                }
                server_admin_contract::AdminTableSortField::RoleSystem => {
                    bool::from(left.is_system()).cmp(&bool::from(right.is_system()))
                }
                server_admin_contract::AdminTableSortField::RoleName => {
                    left.name().as_ref().cmp(right.name().as_ref())
                }
                _ => std::cmp::Ordering::Equal,
            };
            match state_value.sort_dir() {
                crate::table_state::SortDir::Asc => order,
                crate::table_state::SortDir::Desc => order.reverse(),
            }
        });
        filtered
    };
    let total_value = usize::try_from(u64::from(server_total)).unwrap_or(usize::MAX);
    let total = Signal::derive(move || total_value);
    let content = view! { <section class="crud-content"><RoleForm role_id=None initial_name=String::new() client=client_for_create loader enabled=can_create /><TableTools state sort_options=&server_admin_contract::AdminTableSortField::ROLE />
    <table><thead><tr><th>"ID"</th><th>"Name"</th><th>"System"</th><th>"Actions"</th></tr></thead><tbody><For each=move || { let all_rows = rows(); let current = state.get(); let start = current.start(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; let end = current.end(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; all_rows[start..end].to_vec() } key=|value| value.id() children=move |value| { let edit_client = client.clone(); let permissions_client = client.clone(); let delete_client = client.clone(); let id = value.id(); let edit_name = value.name().clone(); let delete_name = value.name().clone(); let is_system = bool::from(value.is_system()); let assigned_permission_ids = value.permission_ids().to_vec(); let can_edit = !is_system && super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::UpdateRole(id)); let can_delete = !is_system && super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::DeleteRole(id)); view! { <tr><td>{id.to_string()}</td><td>{value.name().to_string()}</td><td>{value.is_system().to_string()}</td><td><RoleForm role_id=Some(id) initial_name=edit_name.to_string() client=edit_client loader enabled=can_edit /><RolePermissionEditor role_id=id assigned=assigned_permission_ids available=permissions.get_value() client=permissions_client loader can_update=!is_system && super::pages::has_route_permission(&auth, server_admin_contract::AdminRoute::SetRolePermissions(id)) /><DeleteRoleForm role_id=id name=delete_name client=delete_client loader enabled=can_delete /></td></tr> } } /></tbody></table><TablePager state total /></section> };
    crud_page(server_admin_contract::AdminPage::Roles, content)
}

pub(super) fn permissions_view(
    values: Vec<server_admin_contract::AdminPermissionSummary>,
    server_total: server_admin_contract::AdminPageTotal,
    client: super::AdminApiClient,
    loader: super::PageLoader,
) -> impl IntoView {
    let state = RwSignal::new(super::table_state(
        server_admin_contract::AdminTableSortField::PermissionName,
        &server_admin_contract::AdminTableSortField::PERMISSION,
    ));
    let on_change = Callback::new(move |state: crate::table_state::TableState| {
        super::apply_table_state_url(server_admin_contract::AdminPage::Permissions, &state);
        super::load(client.clone(), loader);
    });
    provide_context(on_change);
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
            let order =
                if state_value.sort() == server_admin_contract::AdminTableSortField::PermissionId {
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
    let total_value = usize::try_from(u64::from(server_total)).unwrap_or(usize::MAX);
    let total = Signal::derive(move || total_value);
    let content = view! { <div class="crud-content"><TableTools state sort_options=&server_admin_contract::AdminTableSortField::PERMISSION /><table><thead><tr><th>"ID"</th><th>"Name"</th></tr></thead><tbody>{move || { let all_rows = rows(); let current = state.get(); let start = current.start(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; let end = current.end(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; all_rows[start..end].iter().cloned().map(|value| view! { <tr><td>{value.id().to_string()}</td><td>{value.name().to_string()}</td></tr> }).collect_view() }}</tbody></table><TablePager state total /></div> };
    crud_page(server_admin_contract::AdminPage::Permissions, content)
}

#[component]
fn AuditFilters(
    next_cursor: Option<server_admin_contract::AdminAuditCursor>,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    can_export: bool,
) -> impl IntoView {
    let created_after = RwSignal::new(super::query_value(str_constants::CREATED_AFTER));
    let created_before = RwSignal::new(super::query_value(str_constants::CREATED_BEFORE));
    let user_login = RwSignal::new(super::query_value(str_constants::USER_LOGIN));
    let action = RwSignal::new(super::query_value(str_constants::ACTION));
    let resource = RwSignal::new(super::query_value(str_constants::RESOURCE));
    let resource_id = RwSignal::new(super::query_value(str_constants::RESOURCE_ID));
    let succeeded = RwSignal::new(super::query_value(str_constants::SUCCEEDED));
    let limit = RwSignal::new(match super::query_value(str_constants::LIMIT).as_str() {
        str_constants::VALUE_10 | str_constants::VALUE_50 | str_constants::VALUE_100 => {
            super::query_value(str_constants::LIMIT)
        }
        _ => str_constants::VALUE_20.to_owned(),
    });
    let next_client = client.clone();
    let export_client = client.clone();
    let export_href = RwSignal::<Option<String>>::new(None);
    let export_error = RwSignal::<Option<String>>::new(None);
    let export_pending = RwSignal::new(false);
    view! {
        <form class="audit-filters" on:submit=move |event| {
            event.prevent_default();
            let values = [
                ("created_after", created_after.get_untracked()),
                ("created_before", created_before.get_untracked()),
                ("user_login", user_login.get_untracked()),
                ("action", action.get_untracked()),
                ("resource", resource.get_untracked()),
                ("resource_id", resource_id.get_untracked()),
                ("succeeded", succeeded.get_untracked()),
                ("limit", limit.get_untracked()),
            ];
            let query = values.into_iter().filter(|(_key, value)| !value.is_empty()).map(|(key, value)| format!("{key}={}", crate::table_state::percent_encode(value.as_str()))).collect::<Vec<_>>().join("&");
            super::replace_path(format!("{}?{query}", server_admin_contract::AdminPage::Audit.path()).as_str());
            super::load(client.clone(), loader);
        }>
            <label><span>"From"</span><input aria-label="Audit created after" placeholder="2026-07-17T09:00:00Z" prop:value=move || created_after.get() on:input=move |event| created_after.set(event_target_value(&event)) /></label>
            <label><span>"To"</span><input aria-label="Audit created before" placeholder="2026-07-17T10:00:00Z" prop:value=move || created_before.get() on:input=move |event| created_before.set(event_target_value(&event)) /></label>
            <label><span>"User login"</span><input aria-label="Audit user login" prop:value=move || user_login.get() on:input=move |event| user_login.set(event_target_value(&event)) /></label>
            <label><span>"Action"</span><input aria-label="Audit action" prop:value=move || action.get() on:input=move |event| action.set(event_target_value(&event)) /></label>
            <label><span>"Resource"</span><input aria-label="Audit resource" prop:value=move || resource.get() on:input=move |event| resource.set(event_target_value(&event)) /></label>
            <label><span>"Resource ID"</span><input aria-label="Audit resource ID" prop:value=move || resource_id.get() on:input=move |event| resource_id.set(event_target_value(&event)) /></label>
            <label><span>"Result"</span><select aria-label="Audit result" on:change=move |event| succeeded.set(event_target_value(&event))><option value="" selected=move || succeeded.get().is_empty()>"Any"</option><option value="true" selected=move || succeeded.get() == "true">"Succeeded"</option><option value="false" selected=move || succeeded.get() == "false">"Failed"</option></select></label>
            <label><span>"Rows"</span><select aria-label="Audit rows" on:change=move |event| limit.set(event_target_value(&event))><option value="10" selected=move || limit.get() == "10">"10"</option><option value="20" selected=move || limit.get() == "20">"20"</option><option value="50" selected=move || limit.get() == "50">"50"</option><option value="100" selected=move || limit.get() == "100">"100"</option></select></label>
            <button type="submit">"Apply filters"</button>
            <button type="button" disabled=move || !can_export || export_pending.get() on:click=move |_| { export_pending.set(true); export_error.set(None); let client = export_client.clone(); leptos::task::spawn_local(async move { match client.audit_export().await { Ok(value) => export_href.set(Some(format!("data:text/csv;charset=utf-8,{}", crate::table_state::percent_encode(value.csv().as_ref())))), Err(error) => export_error.set(Some(error.to_string())), } export_pending.set(false); }); }>{move || if export_pending.get() { "Exporting..." } else { "Prepare CSV" }}</button>
            {move || export_error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}
            {move || export_href.get().map(|href| view! { <a class="button" href=href download="admin-audit.csv">"Download CSV"</a> })}
        </form>
        <div class="audit-pagination">
            <button disabled=next_cursor.is_none() on:click=move |_| {
                if let Some(cursor) = next_cursor.as_ref() {
                    let base = super::search_query().trim_start_matches('?').split('&').filter(|part| !part.starts_with("cursor_created_at=") && !part.starts_with("cursor_id=")).collect::<Vec<_>>().join("&");
                    let separator = if base.is_empty() { "" } else { "&" };
                    let query = format!("{base}{separator}cursor_created_at={}&cursor_id={}", crate::table_state::percent_encode(cursor.created_at().as_ref()), cursor.id());
                    super::push_path(format!("{}?{query}", server_admin_contract::AdminPage::Audit.path()).as_str());
                    super::load(next_client.clone(), loader);
                }
            }>"Older events"</button>
        </div>
    }
}

pub(super) fn audit_view(
    values: Vec<server_admin_contract::AdminAuditView>,
    next_cursor: Option<server_admin_contract::AdminAuditCursor>,
    client: super::AdminApiClient,
    loader: super::PageLoader,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let can_export =
        super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::AuditExport);
    provide_context(Callback::new(|_state: crate::table_state::TableState| {}));
    let state = RwSignal::new(crate::table_state::TableState::new(
        server_admin_contract::AdminTableSortField::AuditCreatedAt,
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
            let order = match state_value.sort() {
                server_admin_contract::AdminTableSortField::AuditUserId => left
                    .user_id()
                    .map(i64::from)
                    .cmp(&right.user_id().map(i64::from)),
                server_admin_contract::AdminTableSortField::AuditAction => {
                    left.action().to_string().cmp(&right.action().to_string())
                }
                server_admin_contract::AdminTableSortField::AuditResource => left
                    .resource()
                    .to_string()
                    .cmp(&right.resource().to_string()),
                server_admin_contract::AdminTableSortField::AuditSucceeded => {
                    bool::from(left.succeeded()).cmp(&bool::from(right.succeeded()))
                }
                server_admin_contract::AdminTableSortField::AuditCreatedAt => {
                    left.created_at().as_ref().cmp(right.created_at().as_ref())
                }
                _ => std::cmp::Ordering::Equal,
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
    let content = view! { <div class="crud-content"><TableTools state sort_options=&server_admin_contract::AdminTableSortField::AUDIT /><table><thead><tr><th>"Time"</th><th>"User"</th><th>"Action"</th><th>"Resource"</th><th>"Result"</th><th>"Details"</th></tr></thead><tbody>{move || { let all_rows = rows(); let current = state.get(); let start = current.start(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; let end = current.end(crate::table_state::AdminFrontendTableIndex::from(all_rows.len())).0; all_rows[start..end].iter().cloned().map(|value| {
        let user = match (value.user_login(), value.user_id()) {
            (Some(login), Some(id)) => format!("{login} (#{id})"),
            (Some(login), None) => login.to_string(),
            (None, Some(id)) => format!("#{id}"),
            (None, None) => "System".to_owned(),
        };
        let user_view = value.user_id().map_or_else(
            || view! { <span>{user.clone()}</span> }.into_any(),
            |id| view! { <a href=format!("{}?search={id}", server_admin_contract::AdminPage::Users.path())>{user.clone()}</a> }.into_any(),
        );
        let resource = value.resource_id().map_or_else(|| value.resource().to_string(), |id| format!("{} #{id}", value.resource()));
        let resource_view = match (value.resource().as_ref().as_str(), value.resource_id()) {
            (str_constants::USER, Some(id)) => view! { <a href=format!("{}?search={}", server_admin_contract::AdminPage::Users.path(), crate::table_state::percent_encode(id.as_ref()))>{resource.clone()}</a> }.into_any(),
            (str_constants::ROLE, Some(id)) => view! { <a href=format!("{}?search={}", server_admin_contract::AdminPage::Roles.path(), crate::table_state::percent_encode(id.as_ref()))>{resource.clone()}</a> }.into_any(),
            _ => view! { <span>{resource.clone()}</span> }.into_any(),
        };
        let event_id = value.id().to_string();
        let details = value.details().map_or_else(|| "No additional details".to_owned(), |details| serde_json::to_string_pretty(details.as_ref()).unwrap_or_else(|_error| details.to_string()));
        view! { <tr><td>{value.created_at().to_string()}</td><td>{user_view}</td><td>{value.action().to_string()}</td><td>{resource_view}</td><td>{value.succeeded().to_string()}</td><td><details class="audit-event"><summary>{format!("Event #{event_id}")}</summary><pre>{details}</pre></details></td></tr> }
    }).collect_view() }}</tbody></table><TablePager state total /></div> };
    let content =
        view! { <div><AuditFilters next_cursor client loader can_export />{content}</div> };
    crud_page(server_admin_contract::AdminPage::Audit, content)
}
pub(super) fn sessions_view(
    values: Vec<server_admin_contract::AdminSessionView>,
    client: super::AdminApiClient,
    loader: super::PageLoader,
) -> impl IntoView {
    let client_for_all = client.clone();
    let confirmation = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let error = RwSignal::<Option<String>>::new(None);
    let content = view! {
        <div class="crud-content">
            <details class="mutation-form danger"><summary>"Revoke all sessions"</summary><form on:submit=move |event| {
                event.prevent_default();
                if confirmation.get_untracked() == "REVOKE" {
                    let action_client = client_for_all.clone();
                    run_mutation(
                        action_client.clone().send(server_admin_contract::AdminRoute::RevokeAllSessions),
                        pending,
                        error,
                        "All sessions revoked",
                        action_client,
                        loader,
                    );
                } else {
                    error.set(Some("Type REVOKE to confirm signing out every active session".to_owned()));
                }
            }><p>"Every active administrator session, including this one, will be revoked."</p><label><span>"Type REVOKE"</span><input aria-label="Confirm all session revocation" prop:value=move || confirmation.get() on:input=move |event| confirmation.set(event_target_value(&event)) /></label>{move || error.get().map(|value| view! { <p class="field-error" role="alert">{value}</p> })}<button class="danger-button" type="submit" disabled=move || pending.get()>"Revoke every session"</button></form></details>
            <table><thead><tr><th>"Session"</th><th>"Created"</th><th>"Expires"</th><th>"Actions"</th></tr></thead><tbody>{values.into_iter().map(|value| {
                let session_client = client.clone();
                let session_id = value.id().clone();
                let session_id_for_action = session_id.clone();
                let is_current = bool::from(value.is_current());
                view! { <tr><td>{session_id.to_string()}{is_current.then_some(" (current)")}</td><td>{value.created_at().to_string()}</td><td>{value.expires_at().to_string()}</td><td><button disabled=is_current on:click=move |_| {
                    let action_client = session_client.clone();
                    super::run_action(
                        action_client.clone().revoke_session(session_id_for_action.clone()),
                        action_client,
                        loader,
                    );
                }>"Revoke"</button></td></tr> }
            }).collect_view()}</tbody></table>
        </div>
    };
    crud_page(server_admin_contract::AdminPage::Sessions, content)
}
fn crud_page(page: server_admin_contract::AdminPage, content: impl IntoView) -> impl IntoView {
    view! { <section><div class="page-heading"><div><p class="eyebrow">"Administration"</p><h1>{page.title().as_ref().to_owned()}</h1></div></div>{content}</section> }
}
