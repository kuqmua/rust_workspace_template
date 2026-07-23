#![allow(
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos component and entry-point macro expansion produces these patterns"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes, OnAttribute,
};

#[derive(Clone, Debug)]
enum AdminLoadState {
    Empty(server_admin_contract::AuthenticatedAdmin),
    Error(AdminTableLoadError),
    Loading,
    Permissions(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminPermissionsPage,
    ),
    Profile(server_admin_contract::AuthenticatedAdmin),
    Roles(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminRolesPage,
    ),
    Sessions(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminSessionsPage,
    ),
    Settings(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminSettingsView,
    ),
    Table(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminDataTableView,
    ),
    Users(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminUsersPage,
    ),
}

#[derive(Clone, Copy, Debug)]
enum AdminCsrPage {
    Permissions,
    Profile,
    Roles,
    Sessions,
    Settings,
    Tables,
    Users,
}

impl AdminLoadState {
    const fn admin(&self) -> Option<&server_admin_contract::AuthenticatedAdmin> {
        match self {
            Self::Permissions(admin, _)
            | Self::Roles(admin, _)
            | Self::Sessions(admin, _)
            | Self::Settings(admin, _)
            | Self::Table(admin, _)
            | Self::Users(admin, _)
            | Self::Empty(admin)
            | Self::Profile(admin) => Some(admin),
            Self::Error(_) | Self::Loading => None,
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
enum AdminTableLoadError {
    #[error("The table request failed.")]
    Fetch,
    #[error("The server returned status {0} for {1}.")]
    Http(AdminHttpStatus, AdminCsrApiUrl),
    #[error("The table query is invalid.")]
    Query,
    #[error("The table response was invalid.")]
    Response,
}

#[derive(Clone, Copy, Debug, newtype::Display, newtype::FromInner)]
struct AdminHttpStatus(u16);

#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString, newtype::Display)]
#[bounded_string(max = 16_384usize, chars)]
struct AdminCsrApiUrl(String);

#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 8_192usize, chars)]
struct AdminCsrfToken(String);

#[derive(Clone, Copy, Debug)]
enum AdminMutationMethod {
    Delete,
    Patch,
    Post,
    Put,
}
#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct MutationConfirmationMessageRef<'message_lt>(&'message_lt str);

#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
struct MutationConfirmed(bool);

impl AdminMutationMethod {
    const fn get(self) -> &'static str {
        match self {
            Self::Delete => str_constants::DELETE,
            Self::Patch => str_constants::PATCH,
            Self::Post => str_constants::POST,
            Self::Put => str_constants::HTTP_METHOD_PUT_LABEL,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct AdminCsrQuery {
    direction: Option<server_admin_contract::AdminText>,
    filter_end: Option<server_admin_contract::AdminFilterValue>,
    filter_field: Option<server_admin_contract::AdminFilterField>,
    filter_operation: Option<server_admin_contract::AdminFilterOperationKey>,
    filter_value: Option<server_admin_contract::AdminFilterValue>,
    limit: server_admin_contract::AdminPageLimit,
    offset: server_admin_contract::AdminPageOffset,
    search: server_admin_contract::AdminTableSearch,
    sort: server_admin_contract::AdminTableSortKey,
    table: Option<server_admin_contract::AdminDataTable>,
}

impl AdminCsrQuery {
    fn api_url(&self) -> Result<Option<AdminCsrApiUrl>, AdminTableLoadError> {
        let Some(table) = self.table else {
            return Ok(None);
        };
        let search = web_sys::window()
            .ok_or(AdminTableLoadError::Fetch)?
            .location()
            .search()
            .map_err(|_error| AdminTableLoadError::Fetch)?;
        AdminCsrApiUrl::try_from(format!(
            "{}/admin/tables/{table}{search}",
            str_constants::API_V1
        ))
        .map(Some)
        .map_err(|_error| AdminTableLoadError::Query)
    }

    fn from_location() -> Result<Self, AdminTableLoadError> {
        let window = web_sys::window().ok_or(AdminTableLoadError::Fetch)?;
        let search = window
            .location()
            .search()
            .map_err(|_error| AdminTableLoadError::Fetch)?;
        let params = web_sys::UrlSearchParams::new_with_str(&search)
            .map_err(|_error| AdminTableLoadError::Fetch)?;
        let pathname = window
            .location()
            .pathname()
            .map_err(|_error| AdminTableLoadError::Fetch)?;
        let table = server_admin_contract::AdminDataTable::from_frontend_path(
            server_admin_contract::AdminPagePathRef::from(pathname.as_str()),
        );
        Ok(Self {
            direction: params
                .get(str_constants::ADMIN_DIRECTION_QUERY_KEY)
                .map(server_admin_contract::AdminText::try_from)
                .transpose()
                .map_err(|_error| AdminTableLoadError::Query)?,
            filter_end: params
                .get(str_constants::ADMIN_FILTER_END_QUERY_KEY)
                .map(server_admin_contract::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| AdminTableLoadError::Query)?,
            filter_field: params
                .get(str_constants::ADMIN_FILTER_FIELD_QUERY_KEY)
                .map(server_admin_contract::AdminFilterField::try_from)
                .transpose()
                .map_err(|_error| AdminTableLoadError::Query)?,
            filter_operation: params
                .get(str_constants::ADMIN_FILTER_OPERATION_QUERY_KEY)
                .map(server_admin_contract::AdminFilterOperationKey::try_from)
                .transpose()
                .map_err(|_error| AdminTableLoadError::Query)?,
            filter_value: params
                .get(str_constants::ADMIN_FILTER_VALUE_QUERY_KEY)
                .map(server_admin_contract::AdminFilterValue::try_from)
                .transpose()
                .map_err(|_error| AdminTableLoadError::Query)?,
            limit: params
                .get(str_constants::ADMIN_LIMIT_QUERY_KEY)
                .and_then(|value| value.parse::<u16>().ok())
                .and_then(|value| server_admin_contract::AdminPageLimit::try_from(value).ok())
                .unwrap_or_default(),
            offset: params
                .get(str_constants::ADMIN_OFFSET_QUERY_KEY)
                .and_then(|value| value.parse::<u32>().ok())
                .map_or_else(
                    server_admin_contract::AdminPageOffset::default,
                    server_admin_contract::AdminPageOffset::from,
                ),
            search: params
                .get(str_constants::ADMIN_SEARCH_QUERY_KEY)
                .map(server_admin_contract::AdminTableSearch::try_from)
                .transpose()
                .map_err(|_error| AdminTableLoadError::Query)?
                .unwrap_or_default(),
            sort: params
                .get(str_constants::ADMIN_SORT_QUERY_KEY)
                .map(server_admin_contract::AdminTableSortKey::try_from)
                .transpose()
                .map_err(|_error| AdminTableLoadError::Query)?
                .unwrap_or_default(),
            table,
        })
    }
}

impl AdminCsrPage {
    fn from_location() -> Result<Self, AdminTableLoadError> {
        let pathname = web_sys::window()
            .ok_or(AdminTableLoadError::Fetch)?
            .location()
            .pathname()
            .map_err(|_error| AdminTableLoadError::Fetch)?;
        if server_admin_contract::AdminDataTable::from_frontend_path(
            server_admin_contract::AdminPagePathRef::from(pathname.as_str()),
        )
        .is_some()
        {
            return Ok(Self::Tables);
        }
        server_admin_contract::AdminPage::specs()
            .iter()
            .find(|spec| spec.path().as_ref() == pathname)
            .map(|spec| spec.page())
            .and_then(|page| match page {
                server_admin_contract::AdminPage::Permissions => Some(Self::Permissions),
                server_admin_contract::AdminPage::Profile => Some(Self::Profile),
                server_admin_contract::AdminPage::Roles => Some(Self::Roles),
                server_admin_contract::AdminPage::Sessions => Some(Self::Sessions),
                server_admin_contract::AdminPage::Settings => Some(Self::Settings),
                server_admin_contract::AdminPage::Tables => Some(Self::Tables),
                server_admin_contract::AdminPage::Users => Some(Self::Users),
                server_admin_contract::AdminPage::Metrics
                | server_admin_contract::AdminPage::OpenApi
                | server_admin_contract::AdminPage::Version => None,
            })
            .ok_or(AdminTableLoadError::Query)
    }
}

#[allow(
    clippy::future_not_send,
    reason = "browser fetch futures run exclusively on wasm_bindgen_futures::spawn_local"
)]
async fn fetch_json<Response>(url: &AdminCsrApiUrl) -> Result<Response, AdminTableLoadError>
where
    Response: serde::de::DeserializeOwned,
{
    let window = web_sys::window().ok_or(AdminTableLoadError::Fetch)?;
    let response_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(url.as_ref()))
        .await
        .map_err(|_error| AdminTableLoadError::Fetch)?;
    let response = wasm_bindgen::JsCast::dyn_into::<web_sys::Response>(response_value)
        .map_err(|_error| AdminTableLoadError::Response)?;
    if !response.ok() {
        return Err(AdminTableLoadError::Http(
            AdminHttpStatus::from(response.status()),
            url.clone(),
        ));
    }
    let text_promise =
        web_sys::Response::text(&response).map_err(|_error| AdminTableLoadError::Response)?;
    let text_value = wasm_bindgen_futures::JsFuture::from(text_promise)
        .await
        .map_err(|_error| AdminTableLoadError::Response)?;
    let text = text_value
        .as_string()
        .ok_or(AdminTableLoadError::Response)?;
    serde_json::from_str(&text).map_err(|_error| AdminTableLoadError::Response)
}

fn csrf_token() -> Result<AdminCsrfToken, AdminTableLoadError> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or(AdminTableLoadError::Fetch)?;
    let document = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDocument>(document)
        .map_err(|_error| AdminTableLoadError::Fetch)?;
    document
        .cookie()
        .map_err(|_error| AdminTableLoadError::Fetch)?
        .split(';')
        .map(str::trim)
        .find_map(|cookie| cookie.strip_prefix(str_constants::ADMIN_CSRF_TOKEN_ALT))
        .map(str::to_owned)
        .map(AdminCsrfToken::try_from)
        .transpose()
        .map_err(|_error| AdminTableLoadError::Query)?
        .ok_or(AdminTableLoadError::Fetch)
}

#[allow(
    clippy::future_not_send,
    reason = "browser mutation requests run exclusively on wasm_bindgen_futures::spawn_local"
)]
async fn send_json<RequestBody>(
    method: AdminMutationMethod,
    path: &AdminCsrApiUrl,
    request_body: &RequestBody,
) -> Result<(), AdminTableLoadError>
where
    RequestBody: serde::Serialize,
{
    let body = serde_json::to_string(request_body).map_err(|_error| AdminTableLoadError::Query)?;
    let options = web_sys::RequestInit::new();
    options.set_method(method.get());
    options.set_body(&wasm_bindgen::JsValue::from_str(&body));
    let request = web_sys::Request::new_with_str_and_init(path.as_ref(), &options)
        .map_err(|_error| AdminTableLoadError::Fetch)?;
    request
        .headers()
        .set(
            str_constants::CONTENT_TYPE,
            str_constants::HTTP_APPLICATION_JSON,
        )
        .map_err(|_error| AdminTableLoadError::Fetch)?;
    request
        .headers()
        .set(str_constants::X_CSRF_TOKEN, csrf_token()?.as_ref())
        .map_err(|_error| AdminTableLoadError::Fetch)?;
    let response_value = wasm_bindgen_futures::JsFuture::from(
        web_sys::window()
            .ok_or(AdminTableLoadError::Fetch)?
            .fetch_with_request(&request),
    )
    .await
    .map_err(|_error| AdminTableLoadError::Fetch)?;
    let response = wasm_bindgen::JsCast::dyn_into::<web_sys::Response>(response_value)
        .map_err(|_error| AdminTableLoadError::Response)?;
    response.ok().then_some(()).ok_or_else(|| {
        AdminTableLoadError::Http(AdminHttpStatus::from(response.status()), path.clone())
    })
}

fn reload_after<RequestBody>(
    method: AdminMutationMethod,
    path: AdminCsrApiUrl,
    request_body: RequestBody,
) where
    RequestBody: serde::Serialize + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        match send_json(method, &path, &request_body).await {
            Ok(()) => match web_sys::window() {
                Some(window) if window.location().reload().is_ok() => {}
                Some(_) | None => show_mutation_error(&AdminTableLoadError::Fetch),
            },
            Err(error) => show_mutation_error(&error),
        }
    });
}

fn show_mutation_error(error: &AdminTableLoadError) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Some(root) = document.get_element_by_id(str_constants::ADMIN_CSR_ROOT_ID) else {
        return;
    };
    root.set_text_content(Some(&error.to_string()));
    root.set_class_name(str_constants::ADMIN_FIELD_ERROR_CLASS);
}

fn mutation_confirmed(message: MutationConfirmationMessageRef<'_>) -> MutationConfirmed {
    if let Some(Ok(confirmed)) =
        web_sys::window().map(|window| window.confirm_with_message(message.0))
    {
        MutationConfirmed::from(confirmed)
    } else {
        show_mutation_error(&AdminTableLoadError::Fetch);
        MutationConfirmed::from(false)
    }
}

#[allow(
    clippy::future_not_send,
    reason = "browser page loads run exclusively on wasm_bindgen_futures::spawn_local"
)]
async fn fetch_page(
    page: AdminCsrPage,
    query: &AdminCsrQuery,
) -> Result<AdminLoadState, AdminTableLoadError> {
    let search = web_sys::window()
        .ok_or(AdminTableLoadError::Fetch)?
        .location()
        .search()
        .map_err(|_error| AdminTableLoadError::Fetch)?;
    let me_url = AdminCsrApiUrl::try_from(format!(
        "{}{}",
        str_constants::API_V1,
        str_constants::ADMIN_API_ME_PATH
    ))
    .map_err(|_error| AdminTableLoadError::Query)?;
    let admin = fetch_json::<server_admin_contract::AuthenticatedAdmin>(&me_url).await?;
    let path = match page {
        AdminCsrPage::Permissions => str_constants::ADMIN_API_PERMISSIONS_PATH,
        AdminCsrPage::Profile => return Ok(AdminLoadState::Profile(admin)),
        AdminCsrPage::Roles => str_constants::ADMIN_API_ROLES_PATH,
        AdminCsrPage::Sessions => str_constants::ADMIN_API_SESSIONS_PATH,
        AdminCsrPage::Settings => str_constants::ADMIN_API_SETTINGS_PATH,
        AdminCsrPage::Tables => {
            let Some(url) = query.api_url()? else {
                return Ok(AdminLoadState::Empty(admin));
            };
            return fetch_json(&url)
                .await
                .map(|value| AdminLoadState::Table(admin, value));
        }
        AdminCsrPage::Users => str_constants::ADMIN_API_USERS_PATH,
    };
    let suffix = match page {
        AdminCsrPage::Permissions | AdminCsrPage::Roles | AdminCsrPage::Users => search,
        AdminCsrPage::Profile
        | AdminCsrPage::Sessions
        | AdminCsrPage::Settings
        | AdminCsrPage::Tables => String::new(),
    };
    let url = AdminCsrApiUrl::try_from(format!("{}{path}{suffix}", str_constants::API_V1))
        .map_err(|_error| AdminTableLoadError::Query)?;
    match page {
        AdminCsrPage::Permissions => fetch_json(&url)
            .await
            .map(|value| AdminLoadState::Permissions(admin, value)),
        AdminCsrPage::Profile => Ok(AdminLoadState::Profile(admin)),
        AdminCsrPage::Roles => fetch_json(&url)
            .await
            .map(|value| AdminLoadState::Roles(admin, value)),
        AdminCsrPage::Sessions => fetch_json(&url)
            .await
            .map(|value| AdminLoadState::Sessions(admin, value)),
        AdminCsrPage::Settings => fetch_json(&url)
            .await
            .map(|value| AdminLoadState::Settings(admin, value)),
        AdminCsrPage::Tables => Ok(AdminLoadState::Empty(admin)),
        AdminCsrPage::Users => fetch_json(&url)
            .await
            .map(|value| AdminLoadState::Users(admin, value)),
    }
}

#[leptos::component]
fn AdminDataGrid(
    query: AdminCsrQuery,
    view: server_admin_contract::AdminDataTableView,
) -> impl leptos::prelude::IntoView {
    let supports_filters = bool::from(view.table().supports_filters());
    let table_path = view.table().frontend_path();
    let total = u64::from(view.total());
    let limit = u16::from(query.limit);
    let limit_text = limit.to_string();
    let offset = u32::from(query.offset);
    let previous_offset = offset.saturating_sub(u32::from(limit));
    let next_offset = offset.saturating_add(u32::from(limit));
    let previous_disabled = offset == 0u32;
    let next_disabled = u64::from(next_offset) >= total;
    let range_start = u64::from(offset).saturating_add(1u64).min(total);
    let range_end = u64::from(offset)
        .saturating_add(u64::from(limit))
        .min(total);
    let filter_field = supports_filters
        .then_some(query.filter_field.as_ref())
        .flatten();
    let filter_operation = supports_filters
        .then_some(query.filter_operation.as_ref())
        .flatten();
    let filter_value = supports_filters
        .then_some(query.filter_value.as_ref())
        .flatten();
    let filter_end = supports_filters
        .then_some(query.filter_end.as_ref())
        .flatten();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {crate::shared::admin_data_table_grid(
                &view,
                query.filter_field.as_ref(),
                query.filter_operation.as_ref(),
                query.filter_value.as_ref(),
                query.filter_end.as_ref(),
                query.limit,
            )}
            <nav class="table-pagination" aria-label="Table pages">
                <form class="table-page-size" method="get" action=table_path.as_ref().to_owned()>
                    {crate::shared::admin_filter_hidden_inputs(filter_field, filter_operation, filter_value, filter_end)}
                    <input type="hidden" name="offset" value="0" />
                    <label><span>"Rows"</span><input name="limit" type="number" min="1" max="100" value=limit_text.clone() /></label>
                    <button type="submit">"Apply"</button>
                </form>
                <form method="get" action=table_path.as_ref().to_owned()>
                    <input type="hidden" name="limit" value=limit_text.clone() />
                    {crate::shared::admin_filter_hidden_inputs(filter_field, filter_operation, filter_value, filter_end)}
                    <input type="hidden" name="offset" value=previous_offset.to_string() /><button type="submit" disabled=previous_disabled>"Previous"</button>
                </form>
                <span>{format!("{range_start}-{range_end} of {total}")}</span>
                <form method="get" action=table_path.as_ref().to_owned()>
                    <input type="hidden" name="limit" value=limit_text />
                    {crate::shared::admin_filter_hidden_inputs(filter_field, filter_operation, filter_value, filter_end)}
                    <input type="hidden" name="offset" value=next_offset.to_string() /><button type="submit" disabled=next_disabled>"Next"</button>
                </form>
            </nav>
        </section>
    }
}

#[leptos::component]
fn AdminPagination(
    action: server_admin_contract::AdminFrontendPath,
    query: AdminCsrQuery,
    total: server_admin_contract::AdminPageTotal,
) -> impl leptos::prelude::IntoView {
    let limit = u16::from(query.limit);
    let offset = u32::from(query.offset);
    let previous_offset = offset.saturating_sub(u32::from(limit));
    let next_offset = offset.saturating_add(u32::from(limit));
    let total_value = u64::from(total);
    let next_disabled = u64::from(next_offset) >= total_value;
    leptos::view! {
        <nav class="table-pagination" aria-label="Table pages">
            <form method="get" action=action.get()>
                {crate::shared::admin_table_query_hidden_inputs(&query.search, &query.sort, &crate::shared::AdminTableQueryDirection::Csr(query.direction.clone()), query.limit)}
                <input type="hidden" name="offset" value=previous_offset.to_string() /><button type="submit" disabled=offset == 0u32>"Previous"</button>
            </form>
            <span>{format!("{}-{} of {}", u64::from(offset).saturating_add(1u64).min(total_value), u64::from(offset).saturating_add(u64::from(limit)).min(total_value), total_value)}</span>
            <form method="get" action=action.get()>
                {crate::shared::admin_table_query_hidden_inputs(&query.search, &query.sort, &crate::shared::AdminTableQueryDirection::Csr(query.direction), query.limit)}
                <input type="hidden" name="offset" value=next_offset.to_string() /><button type="submit" disabled=next_disabled>"Next"</button>
            </form>
        </nav>
    }
}

#[leptos::component]
fn AdminUsersView(
    admin: server_admin_contract::AuthenticatedAdmin,
    page: server_admin_contract::AdminUsersPage,
    query: AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let create_display_name = leptos::prelude::RwSignal::new(String::new());
    let create_login = leptos::prelude::RwSignal::new(String::new());
    let create_password = leptos::prelude::RwSignal::new(String::new());
    let roles = page.roles().to_vec();
    let can_create =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersCreate));
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersDelete));
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersUpdate));
    let can_update_roles =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UserRolesUpdate));
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {crate::shared::admin_table_filters(server_admin_contract::AdminFrontendPath::Users, &query.search, &query.sort, crate::shared::AdminTableFilterDirection::from_csr(query.direction.as_ref()), query.limit, &server_admin_contract::AdminTableSortField::USER, crate::shared::AdminTableFilterPresentation::Csr)}
            {can_create.then(|| leptos::view! { <form class="mutation-form" on:submit=move |event| {
                event.prevent_default();
                let request = (
                    server_admin_contract::AdminDisplayName::try_from(leptos::prelude::Get::get(&create_display_name)),
                    server_admin_contract::AdminLogin::try_from(leptos::prelude::Get::get(&create_login)),
                    server_admin_contract::AdminNewPassword::try_from(leptos::prelude::Get::get(&create_password)),
                );
                if let (Ok(display_name), Ok(login), Ok(password), Ok(path)) = (
                    request.0, request.1, request.2,
                    AdminCsrApiUrl::try_from(format!("{}{}", str_constants::API_V1, str_constants::ADMIN_API_USERS_PATH)),
                ) {
                    reload_after(AdminMutationMethod::Post, path, server_admin_contract::AdminCreateUserReq::new(display_name, login, password));
                }
            }>
                <input placeholder="Login" required on:input=move |event| leptos::prelude::Set::set(&create_login, leptos::prelude::event_target_value(&event)) />
                <input placeholder="Display name" required on:input=move |event| leptos::prelude::Set::set(&create_display_name, leptos::prelude::event_target_value(&event)) />
                <input type="password" placeholder="Password" required on:input=move |event| leptos::prelude::Set::set(&create_password, leptos::prelude::event_target_value(&event)) />
                <button type="submit">"Create user"</button>
            </form> })}
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"login"</th><th>"display_name"</th><th>"banned"</th><th>"roles"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                let login = leptos::prelude::RwSignal::new(item.login().to_string());
                let display_name = leptos::prelude::RwSignal::new(item.display_name().to_string());
                let password = leptos::prelude::RwSignal::new(String::new());
                let selected_roles = leptos::prelude::RwSignal::new(item.role_ids().to_vec());
                let expected_roles = item.role_ids().to_vec();
                let update_user_id = item.id();
                let password_user_id = item.id();
                let roles_user_id = item.id();
                let ban_user_id = item.id();
                let delete_user_id = item.id();
                let is_banned = item.is_banned();
                leptos::view! {
                <tr>
                    <td data-label="id">{item.id().to_string()}</td>
                    <td data-label="login"><input disabled=!can_update value=item.login().to_string() on:input=move |event| leptos::prelude::Set::set(&login, leptos::prelude::event_target_value(&event)) /></td>
                    <td data-label="display_name"><input disabled=!can_update value=item.display_name().to_string() on:input=move |event| leptos::prelude::Set::set(&display_name, leptos::prelude::event_target_value(&event)) /></td>
                    <td data-label="banned">{is_banned.to_string()}</td>
                    <td data-label="roles"><div class="table-options">{roles.iter().map(|role| {
                        let role_id = role.id();
                        let checked = item.role_ids().contains(&role_id);
                        leptos::view! { <label><input type="checkbox" checked=checked disabled=!can_update_roles on:change=move |event| {
                            leptos::prelude::Update::update(&selected_roles, |ids| {
                                if leptos::prelude::event_target_checked(&event) {
                                    if !ids.contains(&role_id) { ids.push(role_id); }
                                } else { ids.retain(|value| *value != role_id); }
                            });
                        } />{role.name().to_string()}</label> }
                    }).collect::<Vec<_>>()}</div></td>
                    <td data-label="actions"><div class="table-actions">
                        {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            let request = server_admin_contract::AdminUpdateUserReq::new(
                                server_admin_contract::AdminDisplayName::try_from(leptos::prelude::Get::get(&display_name)).ok(),
                                server_admin_contract::AdminLogin::try_from(leptos::prelude::Get::get(&login)).ok(),
                            );
                            if let Ok(path) = AdminCsrApiUrl::try_from(format!("{}{}/{}", str_constants::API_V1, str_constants::ADMIN_API_USERS_PATH, update_user_id)) {
                                reload_after(AdminMutationMethod::Patch, path, request);
                            }
                        }>"Save"</button> })}
                        {can_update.then(|| leptos::view! { <><input type="password" placeholder="New password" on:input=move |event| leptos::prelude::Set::set(&password, leptos::prelude::event_target_value(&event)) />
                        <button type="button" on:click=move |_event| {
                            if let (Ok(value), Ok(path)) = (
                                server_admin_contract::AdminNewPassword::try_from(leptos::prelude::Get::get(&password)),
                                AdminCsrApiUrl::try_from(format!("{}{}/{}/password", str_constants::API_V1, str_constants::ADMIN_API_USERS_PATH, password_user_id)),
                            ) {
                                reload_after(AdminMutationMethod::Post, path, server_admin_contract::AdminSetUserPasswordReq::new(value));
                            }
                        }>"Change password"</button></> })}
                        {can_update_roles.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            let expected = server_admin_contract::AdminRoleIds::try_from(expected_roles.clone());
                            let selected = server_admin_contract::AdminRoleIds::try_from(leptos::prelude::Get::get(&selected_roles));
                            if let (Ok(expected), Ok(selected), Ok(path)) = (
                                expected,
                                selected,
                                AdminCsrApiUrl::try_from(format!("{}{}/{}/roles", str_constants::API_V1, str_constants::ADMIN_API_USERS_PATH, roles_user_id)),
                            ) {
                                reload_after(AdminMutationMethod::Put, path, server_admin_contract::AdminSetUserRolesReq::new(expected, selected));
                            }
                        }>"Save roles"</button> })}
                        {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            if let Ok(path) = AdminCsrApiUrl::try_from(format!("{}{}/{}/ban", str_constants::API_V1, str_constants::ADMIN_API_USERS_PATH, ban_user_id)) {
                                reload_after(AdminMutationMethod::Post, path, server_admin_contract::AdminSetUserBanReq::new(server_admin_contract::AdminBool::from(!bool::from(is_banned))));
                            }
                        }>{if bool::from(is_banned) { "Unban" } else { "Ban" }}</button> })}
                        {can_delete.then(|| leptos::view! { <button class="danger-button" type="button" on:click=move |_event| {
                            if bool::from(mutation_confirmed(MutationConfirmationMessageRef::from("Delete this user?"))) && let Ok(path) = AdminCsrApiUrl::try_from(format!("{}{}/{}", str_constants::API_V1, str_constants::ADMIN_API_USERS_PATH, delete_user_id)) {
                                reload_after(AdminMutationMethod::Delete, path, server_admin_contract::AdminNoBody);
                            }
                        }>"Delete"</button> })}
                    </div></td>
                </tr>
            }}).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
            <AdminPagination action=server_admin_contract::AdminFrontendPath::Users query=query total=page.total() />
        </section>
    }
}

#[leptos::component]
fn AdminRolesView(
    admin: server_admin_contract::AuthenticatedAdmin,
    page: server_admin_contract::AdminRolesPage,
    query: AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let create_name = leptos::prelude::RwSignal::new(String::new());
    let permissions = page.permissions().to_vec();
    let can_create =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesCreate));
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesDelete));
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate));
    let can_update_permissions = bool::from(
        admin.has_permission(server_admin_contract::AdminPermission::RolePermissionsUpdate),
    );
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {crate::shared::admin_table_filters(server_admin_contract::AdminFrontendPath::Roles, &query.search, &query.sort, crate::shared::AdminTableFilterDirection::from_csr(query.direction.as_ref()), query.limit, &server_admin_contract::AdminTableSortField::ROLE, crate::shared::AdminTableFilterPresentation::Csr)}
            {can_create.then(|| leptos::view! { <form class="mutation-form" on:submit=move |event| {
                event.prevent_default();
                if let (Ok(name), Ok(path)) = (
                    server_admin_contract::AdminRoleName::try_from(leptos::prelude::Get::get(&create_name)),
                    AdminCsrApiUrl::try_from(format!("{}{}", str_constants::API_V1, str_constants::ADMIN_API_ROLES_PATH)),
                ) {
                    reload_after(AdminMutationMethod::Post, path, server_admin_contract::AdminCreateRoleReq::new(name));
                }
            }><input placeholder="Role name" required on:input=move |event| leptos::prelude::Set::set(&create_name, leptos::prelude::event_target_value(&event)) /><button type="submit">"Create role"</button></form> })}
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"name"</th><th>"system"</th><th>"permissions"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                let name = leptos::prelude::RwSignal::new(item.name().to_string());
                let selected_permissions = leptos::prelude::RwSignal::new(item.permission_ids().to_vec());
                let expected_permissions = item.permission_ids().to_vec();
                let update_role_id = item.id();
                let permissions_role_id = item.id();
                let delete_role_id = item.id();
                leptos::view! {
                <tr>
                    <td data-label="id">{item.id().to_string()}</td>
                    <td data-label="name"><input disabled=!can_update value=item.name().to_string() on:input=move |event| leptos::prelude::Set::set(&name, leptos::prelude::event_target_value(&event)) /></td>
                    <td data-label="system">{item.is_system().to_string()}</td>
                    <td data-label="permissions"><div class="table-options">{permissions.iter().map(|permission| {
                        let permission_id = permission.id();
                        let checked = item.permission_ids().contains(&permission_id);
                        leptos::view! { <label><input type="checkbox" checked=checked disabled=!can_update_permissions on:change=move |event| {
                            leptos::prelude::Update::update(&selected_permissions, |ids| {
                                if leptos::prelude::event_target_checked(&event) {
                                    if !ids.contains(&permission_id) { ids.push(permission_id); }
                                } else { ids.retain(|value| *value != permission_id); }
                            });
                        } />{permission.name().to_string()}</label> }
                    }).collect::<Vec<_>>()}</div></td>
                    <td data-label="actions"><div class="table-actions">
                        {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            if let (Ok(value), Ok(path)) = (
                                server_admin_contract::AdminRoleName::try_from(leptos::prelude::Get::get(&name)),
                                AdminCsrApiUrl::try_from(format!("{}{}/{}", str_constants::API_V1, str_constants::ADMIN_API_ROLES_PATH, update_role_id)),
                            ) {
                                reload_after(AdminMutationMethod::Patch, path, server_admin_contract::AdminUpdateRoleReq::new(value));
                            }
                        }>"Save"</button> })}
                        {can_update_permissions.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            let expected = server_admin_contract::AdminPermissionIds::try_from(expected_permissions.clone());
                            let selected = server_admin_contract::AdminPermissionIds::try_from(leptos::prelude::Get::get(&selected_permissions));
                            if let (Ok(expected), Ok(selected), Ok(path)) = (
                                expected,
                                selected,
                                AdminCsrApiUrl::try_from(format!("{}{}/{}/permissions", str_constants::API_V1, str_constants::ADMIN_API_ROLES_PATH, permissions_role_id)),
                            ) {
                                reload_after(AdminMutationMethod::Put, path, server_admin_contract::AdminSetRolePermissionsReq::new(expected, selected));
                            }
                        }>"Save permissions"</button> })}
                        {can_delete.then(|| leptos::view! { <button class="danger-button" type="button" disabled=bool::from(item.is_system()) on:click=move |_event| {
                            if bool::from(mutation_confirmed(MutationConfirmationMessageRef::from("Delete this role?"))) && let Ok(path) = AdminCsrApiUrl::try_from(format!("{}{}/{}", str_constants::API_V1, str_constants::ADMIN_API_ROLES_PATH, delete_role_id)) {
                                reload_after(AdminMutationMethod::Delete, path, server_admin_contract::AdminNoBody);
                            }
                        }>"Delete"</button> })}
                    </div></td>
                </tr>
            }}).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
            <AdminPagination action=server_admin_contract::AdminFrontendPath::Roles query=query total=page.total() />
        </section>
    }
}

#[leptos::component]
fn AdminPermissionsView(
    page: server_admin_contract::AdminPermissionsPage,
    query: AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {crate::shared::admin_table_filters(server_admin_contract::AdminFrontendPath::Permissions, &query.search, &query.sort, crate::shared::AdminTableFilterDirection::from_csr(query.direction.as_ref()), query.limit, &server_admin_contract::AdminTableSortField::PERMISSION, crate::shared::AdminTableFilterPresentation::Csr)}
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"permission"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| leptos::view! {
                <tr><td data-label="id">{item.id().to_string()}</td><td data-label="permission">{item.name().to_string()}</td></tr>
            }).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
            <AdminPagination action=server_admin_contract::AdminFrontendPath::Permissions query=query total=page.total() />
        </section>
    }
}

#[leptos::component]
fn AdminSessionsView(
    page: server_admin_contract::AdminSessionsPage,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div class="table-scroll"><table><thead><tr><th>"session"</th><th>"created"</th><th>"expires"</th><th>"current"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                let session_id = item.id().to_string();
                let revoke_session_id = session_id.clone();
                leptos::view! {
                <tr>
                    <td data-label="session">{session_id}</td>
                    <td data-label="created">{item.created_at().to_string()}</td>
                    <td data-label="expires">{item.expires_at().to_string()}</td>
                    <td data-label="current">{item.is_current().to_string()}</td>
                    <td data-label="actions"><div class="table-actions"><button type="button" on:click=move |_event| {
                        if bool::from(mutation_confirmed(MutationConfirmationMessageRef::from("Revoke this session?"))) && let Ok(path) = AdminCsrApiUrl::try_from(format!("{}{}/{}", str_constants::API_V1, str_constants::ADMIN_API_SESSIONS_PATH, revoke_session_id)) {
                            reload_after(AdminMutationMethod::Delete, path, server_admin_contract::AdminNoBody);
                        }
                    }>"Revoke session"</button></div></td>
                </tr>
            }}).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
        </section>
    }
}

#[leptos::component]
fn AdminProfileView(
    admin: server_admin_contract::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView {
    let current_password = leptos::prelude::RwSignal::new(String::new());
    let new_password = leptos::prelude::RwSignal::new(String::new());
    leptos::view! {
        <section class="profile-grid" data-renderer="csr">
            <article class="profile-card"><h2>"Account"</h2><dl>
                <dt>"Login"</dt><dd>{admin.login().to_string()}</dd>
                <dt>"Display name"</dt><dd>{admin.display_name().to_string()}</dd>
                <dt>"Roles"</dt><dd>{admin.roles().iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")}</dd>
                <dt>"Permissions"</dt><dd>{admin.permissions().iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")}</dd>
            </dl></article>
            <article class="security-card"><h2>"Change password"</h2><form on:submit=move |event| {
                event.prevent_default();
                let request = (
                    server_admin_contract::AdminPassword::try_from(leptos::prelude::Get::get(&current_password)),
                    server_admin_contract::AdminNewPassword::try_from(leptos::prelude::Get::get(&new_password)),
                );
                if let (Ok(current), Ok(new_value), Ok(path)) = (
                    request.0,
                    request.1,
                    AdminCsrApiUrl::try_from(format!("{}{}", str_constants::API_V1, str_constants::ADMIN_API_PASSWORD_PATH)),
                ) {
                    reload_after(
                        AdminMutationMethod::Post,
                        path,
                        server_admin_contract::AdminChangeOwnPasswordReq::new(
                            current,
                            new_value,
                        ),
                    );
                }
            }>
                <label><span>"Current password"</span><input type="password" required on:input=move |event| leptos::prelude::Set::set(&current_password, leptos::prelude::event_target_value(&event)) /></label>
                <label><span>"New password"</span><input type="password" required on:input=move |event| leptos::prelude::Set::set(&new_password, leptos::prelude::event_target_value(&event)) /></label>
                <button type="submit">"Change password"</button>
            </form></article>
        </section>
    }
}

#[leptos::component]
fn AdminSettingsView(
    admin: server_admin_contract::AuthenticatedAdmin,
    page: server_admin_contract::AdminSettingsView,
) -> impl leptos::prelude::IntoView {
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::AdminPermission::SystemSettingsUpdate),
    );
    let default_route =
        leptos::prelude::RwSignal::new(page.default_admin_route().as_ref().to_owned());
    let main_logo = leptos::prelude::RwSignal::new(
        page.main_logo()
            .map(|value| value.as_ref().to_owned())
            .unwrap_or_default(),
    );
    let organization_contacts = leptos::prelude::RwSignal::new(
        page.organization_contacts()
            .map(|value| value.as_ref().to_owned())
            .unwrap_or_default(),
    );
    let organization_name = leptos::prelude::RwSignal::new(
        page.organization_name()
            .map(|value| value.as_ref().to_owned())
            .unwrap_or_default(),
    );
    let primary_color = leptos::prelude::RwSignal::new(
        page.primary_color()
            .map(|value| value.as_ref().to_owned())
            .unwrap_or_default(),
    );
    let site_name = leptos::prelude::RwSignal::new(page.site_name().as_ref().to_owned());
    let support_url = leptos::prelude::RwSignal::new(
        page.support_url()
            .map(|value| value.as_ref().to_owned())
            .unwrap_or_default(),
    );
    let tab_title = leptos::prelude::RwSignal::new(
        page.tab_title()
            .map(|value| value.as_ref().to_owned())
            .unwrap_or_default(),
    );
    leptos::view! {
        <section class="settings-grid" data-renderer="csr"><article class="settings-card"><form class="settings-form" on:submit=move |event| {
            event.prevent_default();
            let default_route_value = leptos::prelude::Get::get(&default_route);
            let main_logo_value = leptos::prelude::Get::get(&main_logo);
            let organization_contacts_value = leptos::prelude::Get::get(&organization_contacts);
            let organization_name_value = leptos::prelude::Get::get(&organization_name);
            let primary_color_value = leptos::prelude::Get::get(&primary_color);
            let site_name_value = leptos::prelude::Get::get(&site_name);
            let support_url_value = leptos::prelude::Get::get(&support_url);
            let tab_title_value = leptos::prelude::Get::get(&tab_title);
            let clear = [
                (main_logo_value.is_empty(), server_admin_contract::AdminOptionalSetting::MainLogo),
                (organization_contacts_value.is_empty(), server_admin_contract::AdminOptionalSetting::OrganizationContacts),
                (organization_name_value.is_empty(), server_admin_contract::AdminOptionalSetting::OrganizationName),
                (primary_color_value.is_empty(), server_admin_contract::AdminOptionalSetting::PrimaryColor),
                (support_url_value.is_empty(), server_admin_contract::AdminOptionalSetting::SupportUrl),
                (tab_title_value.is_empty(), server_admin_contract::AdminOptionalSetting::TabTitle),
            ].into_iter().filter_map(|(should_clear, setting)| should_clear.then_some(setting)).collect::<Vec<_>>();
            let values = (
                server_admin_contract::AdminDefaultRoute::try_from(default_route_value),
                (!main_logo_value.is_empty()).then(|| server_admin_contract::AdminMainLogo::try_from(main_logo_value)).transpose(),
                (!organization_contacts_value.is_empty()).then(|| server_admin_contract::AdminOrganizationContacts::try_from(organization_contacts_value)).transpose(),
                (!organization_name_value.is_empty()).then(|| server_admin_contract::AdminOrganizationName::try_from(organization_name_value)).transpose(),
                (!primary_color_value.is_empty()).then(|| server_admin_contract::AdminPrimaryColor::try_from(primary_color_value)).transpose(),
                server_admin_contract::AdminSiteName::try_from(site_name_value),
                (!support_url_value.is_empty()).then(|| server_admin_contract::AdminSupportUrl::try_from(support_url_value)).transpose(),
                (!tab_title_value.is_empty()).then(|| server_admin_contract::AdminTabTitle::try_from(tab_title_value)).transpose(),
                server_admin_contract::AdminOptionalSettings::try_from(clear),
                AdminCsrApiUrl::try_from(format!("{}{}", str_constants::API_V1, str_constants::ADMIN_API_SETTINGS_PATH)),
            );
            if let (Ok(request_default_route), Ok(request_main_logo), Ok(request_organization_contacts), Ok(request_organization_name), Ok(request_primary_color), Ok(request_site_name), Ok(request_support_url), Ok(request_tab_title), Ok(request_clear), Ok(path)) = values {
                reload_after(AdminMutationMethod::Patch, path, server_admin_contract::AdminUpdateSettingsReq::new(Some(request_default_route), request_main_logo, request_organization_contacts, request_organization_name, request_primary_color, Some(request_site_name), request_support_url, request_tab_title, request_clear));
            }
        }>
            <label><span>"Default route"</span><input disabled=!can_update value=page.default_admin_route().as_ref().to_owned() on:input=move |event| leptos::prelude::Set::set(&default_route, leptos::prelude::event_target_value(&event)) /></label>
            <label><span>"Site name"</span><input disabled=!can_update value=page.site_name().as_ref().to_owned() on:input=move |event| leptos::prelude::Set::set(&site_name, leptos::prelude::event_target_value(&event)) /></label>
            <label><span>"Tab title"</span><input disabled=!can_update value=page.tab_title().map(|value| value.as_ref().to_owned()).unwrap_or_default() on:input=move |event| leptos::prelude::Set::set(&tab_title, leptos::prelude::event_target_value(&event)) /></label>
            <label><span>"Organization"</span><input disabled=!can_update value=page.organization_name().map(|value| value.as_ref().to_owned()).unwrap_or_default() on:input=move |event| leptos::prelude::Set::set(&organization_name, leptos::prelude::event_target_value(&event)) /></label>
            <label><span>"Contacts"</span><input disabled=!can_update value=page.organization_contacts().map(|value| value.as_ref().to_owned()).unwrap_or_default() on:input=move |event| leptos::prelude::Set::set(&organization_contacts, leptos::prelude::event_target_value(&event)) /></label>
            <label><span>"Support URL"</span><input disabled=!can_update value=page.support_url().map(|value| value.as_ref().to_owned()).unwrap_or_default() on:input=move |event| leptos::prelude::Set::set(&support_url, leptos::prelude::event_target_value(&event)) /></label>
            <label><span>"Primary color"</span><input disabled=!can_update value=page.primary_color().map(|value| value.as_ref().to_owned()).unwrap_or_default() on:input=move |event| leptos::prelude::Set::set(&primary_color, leptos::prelude::event_target_value(&event)) /></label>
            <label><span>"Main logo"</span><input disabled=!can_update value=page.main_logo().map(|value| value.as_ref().to_owned()).unwrap_or_default() on:input=move |event| leptos::prelude::Set::set(&main_logo, leptos::prelude::event_target_value(&event)) /></label>
            <button type="submit" disabled=!can_update>"Save settings"</button>
        </form></article></section>
    }
}

#[leptos::component]
fn AdminNav(admin: server_admin_contract::AuthenticatedAdmin) -> impl leptos::prelude::IntoView {
    let pathname = web_sys::window()
        .and_then(|window| window.location().pathname().ok())
        .unwrap_or_default();
    let active_table = AdminCsrQuery::from_location()
        .ok()
        .and_then(|query| query.table);
    leptos::view! {
        <header class="topbar"><nav aria-label="Admin sections">
            {server_admin_contract::AdminDataTable::PG_ORDER.into_iter().filter(|table| {
                bool::from(admin.has_permission(server_admin_contract::AdminPermission::TablesRead))
                    && bool::from(admin.has_permission(table.permission()))
            }).map(|table| {
                let name = table.to_string();
                let href = table.frontend_path().to_string();
                leptos::view! { <a class=("active", active_table == Some(table)) href=href>{name}</a> }
            }).collect::<Vec<_>>()}
            {server_admin_contract::AdminPage::NAV_ORDER.into_iter().filter(|page| {
                bool::from(admin.can_access(*page))
            }).map(|page| {
                let spec = page.spec();
                let href = spec.path().as_ref().to_owned();
                let active = pathname == href;
                leptos::view! { <a class=("active", active) href=href>{spec.title().as_ref().to_ascii_lowercase().replace(' ', "_")}</a> }
            }).collect::<Vec<_>>()}
            <form on:submit=move |event| {
                event.prevent_default();
                if let Ok(path) = AdminCsrApiUrl::try_from(format!("{}{}", str_constants::API_V1, str_constants::ADMIN_API_SIGN_OUT_PATH)) {
                    reload_after(AdminMutationMethod::Post, path, server_admin_contract::AdminNoBody);
                }
            }><button type="submit">{str_constants::SIGN_OUT.to_ascii_lowercase().replace(' ', "_")}</button></form>
        </nav></header>
    }
}

#[leptos::component]
fn AdminApp() -> impl leptos::prelude::IntoView {
    let query_result = AdminCsrQuery::from_location();
    let page_result = AdminCsrPage::from_location();
    let initial_state = match (&page_result, &query_result) {
        (Ok(_page), Ok(_query)) => AdminLoadState::Loading,
        (Err(error), _) | (_, Err(error)) => AdminLoadState::Error(error.clone()),
    };
    let state = leptos::prelude::RwSignal::new(initial_state);
    if let (Ok(page), Ok(query)) = (page_result, query_result.clone()) {
        wasm_bindgen_futures::spawn_local(async move {
            let next_state = match fetch_page(page, &query).await {
                Ok(value) => value,
                Err(error) => AdminLoadState::Error(error),
            };
            leptos::prelude::Set::set(&state, next_state);
        });
    }
    leptos::view! {
        <div class="app-shell">
        {move || leptos::prelude::Get::get(&state).admin().cloned().map(|admin| leptos::view! { <AdminNav admin=admin /> })}
        <main class="main-content">{move || match leptos::prelude::Get::get(&state) {
            AdminLoadState::Empty(_admin) => leptos::prelude::IntoAny::into_any(leptos::view! { <p class="empty-state">"Choose a table."</p> }),
            AdminLoadState::Error(error) => leptos::prelude::IntoAny::into_any(leptos::view! { <p class="field-error" role="alert">{error.to_string()}</p> }),
            AdminLoadState::Loading => leptos::prelude::IntoAny::into_any(leptos::view! { <p class="loading-state" role="status">"Loading\u{2026}"</p> }),
            AdminLoadState::Permissions(_admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <AdminPermissionsView page=page query=query_result.clone().unwrap_or_default() /> }),
            AdminLoadState::Profile(admin) => leptos::prelude::IntoAny::into_any(leptos::view! { <AdminProfileView admin=admin /> }),
            AdminLoadState::Roles(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <AdminRolesView admin=admin page=page query=query_result.clone().unwrap_or_default() /> }),
            AdminLoadState::Sessions(_admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <AdminSessionsView page=page /> }),
            AdminLoadState::Settings(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <AdminSettingsView admin=admin page=page /> }),
            AdminLoadState::Table(_admin, view) => leptos::prelude::IntoAny::into_any(leptos::view! { <AdminDataGrid view=view query=query_result.clone().unwrap_or_default() /> }),
            AdminLoadState::Users(admin, page) => leptos::prelude::IntoAny::into_any(leptos::view! { <AdminUsersView admin=admin page=page query=query_result.clone().unwrap_or_default() /> }),
        }}</main></div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub(crate) fn start() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(element) = document.get_element_by_id(str_constants::ADMIN_CSR_ROOT_ID) else {
        return;
    };
    let Ok(root) = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(element) else {
        return;
    };
    root.set_inner_html(str_constants::EMPTY);
    leptos::mount::mount_to(root, AdminApp).forget();
}
