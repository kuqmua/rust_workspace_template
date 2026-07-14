use leptos::prelude::*;
pub(super) fn error(value: super::state::Text) -> impl IntoView {
    leptos::view! { <div class="alert error page-alert" role="alert"><strong>"Something went wrong"</strong><span>{value.to_string()}</span></div> }
}
pub(super) fn loading() -> impl IntoView {
    leptos::view! { <div class="loading-state"><span class="spinner"></span><strong>"Loading workspace"</strong><p>"Fetching the latest data..."</p></div> }
}
pub(super) fn users_view(
    values: Vec<server_admin_contract::AdminUserSummary>,
    client: super::AdminApiClient,
    page: RwSignal<super::Page>,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create =
        super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::CreateUser);
    let client_for_create = client.clone();
    let content = view! { <div class="crud-content">
    <button disabled=!can_create on:click=move |_| { if let (Some(login), Some(display_name), Some(password)) = (super::prompt("Login", ""), super::prompt("Display name", ""), super::prompt("Password", "")) && let (Ok(login), Ok(display_name), Ok(password)) = (server_admin_contract::AdminLogin::try_from(login.0), server_admin_contract::AdminDisplayName::try_from(display_name.0), server_admin_contract::AdminPassword::try_from(password.0)) { let body = server_admin_contract::AdminCreateUserReq::new(display_name, login, password); let action_client = client_for_create.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::CreateUser, body), action_client, page); } }>"Create user"</button>
    <table><thead><tr><th>"ID"</th><th>"Login"</th><th>"Display name"</th><th>"Banned"</th><th>"Actions"</th></tr></thead><tbody>
    {values.into_iter().map(|value| { let edit_client = client.clone(); let ban_client = client.clone(); let password_client = client.clone(); let roles_client = client.clone(); let delete_client = client.clone(); let id = value.id(); let edit_login = value.login().clone(); let edit_display_name = value.display_name().clone(); let delete_login = value.login().clone(); let is_banned = bool::from(value.is_banned()); view! { <tr><td>{id.to_string()}</td><td>{value.login().to_string()}</td><td>{value.display_name().to_string()}</td><td>{is_banned.to_string()}</td><td>
    <button disabled=!super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::UpdateUser(id)) on:click=move |_| { if let (Some(login), Some(display_name)) = (super::prompt("Login", edit_login.as_ref()), super::prompt("Display name", edit_display_name.as_ref())) && let (Ok(login), Ok(display_name)) = (server_admin_contract::AdminLogin::try_from(login.0), server_admin_contract::AdminDisplayName::try_from(display_name.0)) { let body = server_admin_contract::AdminUpdateUserReq::new(Some(display_name), Some(login)); let action_client = edit_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateUser(id), body), action_client, page); } }>"Edit"</button>
    <button disabled=!super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::SetUserBan(id)) on:click=move |_| { let body = server_admin_contract::AdminSetUserBanReq::new(server_admin_contract::AdminBool::from(!is_banned)); let action_client = ban_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserBan(id), body), action_client, page); }>{if is_banned { "Unban" } else { "Ban" }}</button>
    <button disabled=!super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::SetUserPassword(id)) on:click=move |_| { if let Some(password) = super::prompt("New password", "") && let Ok(password) = server_admin_contract::AdminPassword::try_from(password.0) { let body = server_admin_contract::AdminSetUserPasswordReq::new(password); let action_client = password_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserPassword(id), body), action_client, page); } }>"Password"</button>
    <button disabled=!super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::SetUserRoles(id)) on:click=move |_| { if let Some(value) = super::prompt("Role IDs separated by commas", "") { let body = server_admin_contract::AdminSetUserRolesReq::from_ids(super::forms::role_ids(&value.0)); let action_client = roles_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetUserRoles(id), body), action_client, page); } }>"Roles"</button>
    <button disabled=!super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::DeleteUser(id)) on:click=move |_| { let confirmed = super::browser_window().and_then(|window| window.confirm_with_message(&format!("Delete {delete_login}?")).ok()).unwrap_or(false); if confirmed { let action_client = delete_client.clone(); super::run_action(action_client.clone().send(server_admin_contract::AdminRoute::DeleteUser(id)), action_client, page); } }>"Delete"</button>
    </td></tr> } }).collect_view()}
    </tbody></table></div> };
    crud_page(server_admin_contract::AdminPage::Users, content)
}
pub(super) fn roles_view(
    values: Vec<server_admin_contract::AdminRoleSummary>,
    client: super::AdminApiClient,
    page: RwSignal<super::Page>,
    auth: &Option<server_admin_contract::AuthenticatedAdmin>,
) -> impl IntoView {
    let can_create =
        super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::CreateRole);
    let client_for_create = client.clone();
    let content = view! { <section class="crud-content"><button disabled=!can_create on:click=move |_| { if let Some(name) = super::prompt("Name", "") && let Ok(name) = server_admin_contract::AdminRoleName::try_from(name.0) { let body = server_admin_contract::AdminCreateRoleReq::new(name); let action_client = client_for_create.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::CreateRole, body), action_client, page); } }>"Create role"</button>
    <table><thead><tr><th>"ID"</th><th>"Name"</th><th>"System"</th><th>"Actions"</th></tr></thead><tbody>{values.into_iter().map(|value| { let edit_client = client.clone(); let permissions_client = client.clone(); let delete_client = client.clone(); let id = value.id(); let edit_name = value.name().clone(); let delete_name = value.name().clone(); view! { <tr><td>{id.to_string()}</td><td>{value.name().to_string()}</td><td>{value.is_system().to_string()}</td><td><button disabled=!super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::UpdateRole(id)) on:click=move |_| { if let Some(name) = super::prompt("Name", edit_name.as_ref()) && let Ok(name) = server_admin_contract::AdminRoleName::try_from(name.0) { let body = server_admin_contract::AdminUpdateRoleReq::new(name); let action_client = edit_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::UpdateRole(id), body), action_client, page); } }>"Edit"</button><button disabled=!super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::SetRolePermissions(id)) on:click=move |_| { if let Some(value) = super::prompt("Permission IDs separated by commas", "") { let body = server_admin_contract::AdminSetRolePermissionsReq::from_ids(super::forms::permission_ids(&value.0)); let action_client = permissions_client.clone(); super::run_action(action_client.clone().send_json(server_admin_contract::AdminRoute::SetRolePermissions(id), body), action_client, page); } }>"Permissions"</button><button disabled=!super::pages::has_route_permission(auth, server_admin_contract::AdminRoute::DeleteRole(id)) on:click=move |_| { let confirmed = super::browser_window().and_then(|window| window.confirm_with_message(&format!("Delete {delete_name}?")).ok()).unwrap_or(false); if confirmed { let action_client = delete_client.clone(); super::run_action(action_client.clone().send(server_admin_contract::AdminRoute::DeleteRole(id)), action_client, page); } }>"Delete"</button></td></tr> } }).collect_view()}</tbody></table></section> };
    crud_page(server_admin_contract::AdminPage::Roles, content)
}
fn crud_page(page: server_admin_contract::AdminPage, content: impl IntoView) -> impl IntoView {
    view! { <section><div class="page-heading"><div><p class="eyebrow">"Administration"</p><h1>{page.title().as_ref().to_owned()}</h1></div></div>{content}</section> }
}
