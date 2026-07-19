#![allow(
    clippy::unused_trait_names,
    reason = "Leptos view macro expansion requires these attribute traits in lexical scope and repository policy forbids underscore import aliases"
)]
use leptos::prelude::{
    AriaAttributes, ClassAttribute, ElementChild, GlobalAttributes, InnerHtmlAttribute,
};

const SSR_TEXT_MAX_BYTES: usize = 16_777_216usize;

trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> AdminSsrHtml;
}
impl<View> AdminSsrViewExt for View
where
    View: leptos::prelude::IntoAny,
{
    fn render_admin_ssr(self) -> AdminSsrHtml {
        AdminSsrHtml(leptos::prelude::RenderHtml::to_html(
            leptos::prelude::IntoAny::into_any(self),
        ))
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

fn render_document(title: &AdminSsrText, body: impl leptos::prelude::IntoAny) -> AdminSsrHtml {
    let rendered_body = body.render_admin_ssr();
    AdminSsrHtml(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{title}</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css\"></head><body>{}</body></html>",
        rendered_body.0
    ))
}

#[must_use]
pub fn render_sign_in(error: Option<AdminSsrErrorMessage>) -> AdminSsrHtml {
    render_document(
        &AdminSsrText(str_constants::ADMINISTRATOR_SIGN_IN.to_owned()),
        leptos::view! {
            <main class="auth-layout">
                <section class="auth-card">
                    <div class="auth-brand">
                        <span class="brand-mark" aria-hidden="true">"A"</span>
                        <div><strong>"Workspace Admin"</strong><small>"Secure operations console"</small></div>
                    </div>
                    <h1>"Sign in"</h1>
                    {error.map(|message| leptos::view! { <p class="field-error" role="alert">{message.0}</p> })}
                    <form method="post" action=server_admin_contract::AdminHtmlAction::SignIn.get()>
                        <label><span>"Login"</span><input name="login" autocomplete="username" required /></label>
                        <label><span>"Password"</span><input name="password" type="password" autocomplete="current-password" required /></label>
                        <button type="submit">"Sign in"</button>
                    </form>
                    <details><summary>"Sign in with TOTP"</summary>
                        <form method="post" action=server_admin_contract::AdminHtmlAction::SignInTotp.get()>
                            <label><span>"Login"</span><input name="login" autocomplete="username" required /></label>
                            <label><span>"Password"</span><input name="password" type="password" autocomplete="current-password" required /></label>
                            <label><span>"TOTP code"</span><input name="code" autocomplete="one-time-code" required /></label>
                            <button type="submit">"Sign in"</button>
                        </form>
                    </details>
                    <details><summary>"Sign in with a recovery code"</summary>
                        <form method="post" action=server_admin_contract::AdminHtmlAction::SignInRecovery.get()>
                            <label><span>"Login"</span><input name="login" autocomplete="username" required /></label>
                            <label><span>"Password"</span><input name="password" type="password" autocomplete="current-password" required /></label>
                            <label><span>"Recovery code"</span><input name="code" autocomplete="one-time-code" required /></label>
                            <button type="submit">"Sign in"</button>
                        </form>
                    </details>
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
    let spec = page.spec();
    let title = spec.title();
    render_document(
        &AdminSsrText(title.as_ref().to_owned()),
        leptos::view! {
            <div class="app-shell">
                <header class="topbar">
                    <a class="brand" href=server_admin_contract::AdminFrontendPath::Dashboard.get()>
                        <span class="brand-mark" aria-hidden="true">"A"</span>
                        <div><strong>"Workspace Admin"</strong><small>"Secure operations console"</small></div>
                    </a>
                    <nav aria-label="Admin sections">
                        {server_admin_contract::AdminPage::specs().iter().copied().map(|item| {
                            let item_page = item.page();
                            let href = String::from(item.path());
                            let label = String::from(item.title());
                            leptos::view! {
                                <a class=(item_page == page).then_some("active") href=href>
                                    <span class="nav-dot" aria-hidden="true"></span>{label}
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                    </nav>
                    <form method="post" action=server_admin_contract::AdminHtmlAction::SignOut.get()><button type="submit">"Sign out"</button></form>
                </header>
                <main class="main-content"><h1>{title.as_ref()}</h1><div inner_html=content.0></div></main>
            </div>
        },
    )
}

#[must_use]
pub fn render_dashboard(view: &server_admin_contract::AdminDashboardView) -> AdminSsrHtml {
    let cleanup = view.last_cleanup().map_or_else(
        || String::from(str_constants::NO_COMPLETED_CLEANUP_RECORDED),
        |value| {
            format!(
                "{} rows at {}",
                value.deleted_rows(),
                value.last_success_at()
            )
        },
    );
    let content_view = leptos::view! {
        <section class="dashboard-grid">
            <article class="summary-card"><span>"Active sessions"</span><strong>{view.active_sessions().to_string()}</strong></article>
            <article class="summary-card"><span>"Failed sign-ins (24h)"</span><strong>{view.failed_sign_ins_24h().to_string()}</strong></article>
            <article class="summary-card"><span>"Database healthy"</span><strong>{view.database_healthy().to_string()}</strong></article>
            <article class="summary-card"><span>"Uptime seconds"</span><strong>{view.uptime_seconds().to_string()}</strong></article>
            <article class="summary-card"><span>"Version"</span><strong>{view.version().to_string()}</strong></article>
            <article class="summary-card"><span>"Last cleanup"</span><strong>{cleanup}</strong></article>
        </section>
        <section class="recent-changes"><h2>"Recent changes"</h2><ul>
            {view.recent_changes().iter().map(|item| leptos::view! {
                <li><strong>{item.action().to_string()}</strong><span>{format!("{} - {}", item.resource(), item.created_at())}</span></li>
            }).collect::<Vec<_>>()}
        </ul></section>
    }.render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Dashboard, content_view)
}

fn table_filters(page: server_admin_contract::AdminPage) -> impl leptos::prelude::IntoView {
    let action = String::from(page.path());
    leptos::view! {
        <form class="table-tools" method="get" action=action>
            <label><span>"Search"</span><input name="search" /></label>
            <label><span>"Sort"</span><input name="sort" /></label>
            <label><span>"Direction"</span><select name="direction"><option value="asc">"Ascending"</option><option value="desc">"Descending"</option></select></label>
            <label><span>"Rows"</span><input name="limit" type="number" min="1" max="100" value="20" /></label>
            <input name="offset" type="hidden" value="0" />
            <button type="submit">"Apply"</button>
        </form>
    }
}

#[must_use]
pub fn render_users(page: &server_admin_contract::AdminUsersPage) -> AdminSsrHtml {
    let content = leptos::view! {
        {table_filters(server_admin_contract::AdminPage::Users)}
        <details class="mutation-form"><summary>"Create user"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserCreate.get()>
            <label><span>"Login"</span><input name="login" required /></label><label><span>"Display name"</span><input name="display_name" required /></label>
            <label><span>"Password"</span><input name="password" type="password" required /></label><button type="submit">"Create user"</button>
        </form></details>
        <p>{format!("{} users", page.total())}</p>
        <table><thead><tr><th>"ID"</th><th>"Login"</th><th>"Display name"</th><th>"Banned"</th><th>"Roles"</th><th>"Actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| { let expected_role_ids = item.role_ids().iter().map(ToString::to_string).collect::<Vec<_>>().join(","); leptos::view! {
            <tr><td>{item.id().to_string()}</td><td>{item.login().to_string()}</td><td>{item.display_name().to_string()}</td><td>{item.is_banned().to_string()}</td>
            <td><form method="post" action=server_admin_contract::AdminHtmlAction::UserRoles.get()><input type="hidden" name="user_id" value=item.id().to_string() />
                <input type="hidden" name="expected_role_ids" value=expected_role_ids />
                {page.roles().iter().map(|role| { let checked = item.role_ids().contains(&role.id()); let name = format!("role_{}", role.id()); leptos::view! { <label><input type="checkbox" name=name value=role.id().to_string() checked=checked />{role.name().to_string()}</label> } }).collect::<Vec<_>>()}
                <button type="submit">"Save roles"</button></form></td>
            <td><details><summary>"Edit"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserUpdate.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="login" value=item.login().to_string() required /><input name="display_name" value=item.display_name().to_string() required /><button type="submit">"Save"</button></form></details>
                <details><summary>"Password"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserPassword.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="password" type="password" required /><button type="submit">"Change password"</button></form></details>
                <form method="post" action=server_admin_contract::AdminHtmlAction::UserBan.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input type="hidden" name="is_banned" value=(!bool::from(item.is_banned())).to_string() /><button type="submit">{if bool::from(item.is_banned()) { "Unban" } else { "Ban" }}</button></form>
                <form method="post" action=server_admin_contract::AdminHtmlAction::UserDelete.get()><input type="hidden" name="user_id" value=item.id().to_string() /><button class="danger-button" type="submit">"Delete"</button></form></td></tr>
        }}).collect::<Vec<_>>()}</tbody></table>
    }.render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Users, content)
}

#[must_use]
pub fn render_roles(page: &server_admin_contract::AdminRolesPage) -> AdminSsrHtml {
    let content = leptos::view! {
        {table_filters(server_admin_contract::AdminPage::Roles)}
        <details class="mutation-form"><summary>"Create role"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::RoleCreate.get()><label><span>"Name"</span><input name="name" required /></label><button type="submit">"Create role"</button></form></details>
        <p>{format!("{} roles", page.total())}</p>
        <table><thead><tr><th>"ID"</th><th>"Name"</th><th>"System"</th><th>"Permissions"</th><th>"Actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| { let expected_permission_ids = item.permission_ids().iter().map(ToString::to_string).collect::<Vec<_>>().join(","); leptos::view! {
            <tr><td>{item.id().to_string()}</td><td>{item.name().to_string()}</td><td>{item.is_system().to_string()}</td><td><form method="post" action=server_admin_contract::AdminHtmlAction::RolePermissions.get()><input type="hidden" name="role_id" value=item.id().to_string() />
                <input type="hidden" name="expected_permission_ids" value=expected_permission_ids />
                {page.permissions().iter().map(|permission| { let checked = item.permission_ids().contains(&permission.id()); let name = format!("permission_{}", permission.id()); leptos::view! { <label><input type="checkbox" name=name value=permission.id().to_string() checked=checked />{permission.name().to_string()}</label> } }).collect::<Vec<_>>()}
                <button type="submit">"Save permissions"</button></form></td><td>
                <form method="post" action=server_admin_contract::AdminHtmlAction::RoleUpdate.get()><input type="hidden" name="role_id" value=item.id().to_string() /><input name="name" value=item.name().to_string() required /><button type="submit">"Save"</button></form>
                <form method="post" action=server_admin_contract::AdminHtmlAction::RoleDelete.get()><input type="hidden" name="role_id" value=item.id().to_string() /><button class="danger-button" type="submit" disabled=bool::from(item.is_system())>"Delete"</button></form></td></tr>
        }}).collect::<Vec<_>>()}</tbody></table>
    }.render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Roles, content)
}

#[must_use]
pub fn render_permissions(page: &server_admin_contract::AdminPermissionsPage) -> AdminSsrHtml {
    let content = leptos::view! {
        {table_filters(server_admin_contract::AdminPage::Permissions)}
        <p>{format!("{} permissions", page.total())}</p>
        <table><thead><tr><th>"ID"</th><th>"Permission"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td>{item.id().to_string()}</td><td>{item.name().to_string()}</td></tr>
        }).collect::<Vec<_>>()}</tbody></table>
    }
    .render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Permissions, content)
}

#[must_use]
pub fn render_sessions(items: &[server_admin_contract::AdminSessionView]) -> AdminSsrHtml {
    let content = leptos::view! {
        <table><thead><tr><th>"Session"</th><th>"Created"</th><th>"Expires"</th><th>"Current"</th></tr></thead>
        <tbody>{items.iter().map(|item| leptos::view! {
            <tr><td>{item.id().to_string()}</td><td>{item.created_at().to_string()}</td><td>{item.expires_at().to_string()}</td><td>{item.is_current().to_string()}</td><td><form method="post" action=server_admin_contract::AdminHtmlAction::SessionRevoke.get()><input type="hidden" name="session_id" value=item.id().to_string() /><button type="submit">"Revoke"</button></form></td></tr>
        }).collect::<Vec<_>>()}</tbody></table>
    }.render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Sessions, content)
}

#[must_use]
pub fn render_profile(
    admin: &server_admin_contract::AuthenticatedAdmin,
    mfa: &server_admin_contract::AdminMfaStatus,
) -> AdminSsrHtml {
    let roles = admin
        .roles()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(str_constants::COMMA_SPACE);
    let content = leptos::view! {
        <section class="security-card"><h2>"Identity"</h2><p><strong>{admin.display_name().to_string()}</strong></p><p>{admin.login().to_string()}</p><p>{roles}</p></section>
        <section class="security-card"><h2>"Multi-factor authentication"</h2><p>{format!("Enabled: {}", mfa.enabled())}</p><p>{format!("Recovery codes remaining: {}", mfa.recovery_codes_remaining())}</p></section>
        <section class="security-card"><h2>"MFA actions"</h2>
            <form method="post" action=server_admin_contract::AdminHtmlAction::MfaEnroll.get()><label><span>"Current password"</span><input name="current_password" type="password" required /></label><button type="submit">"Start enrollment"</button></form>
            <form method="post" action=server_admin_contract::AdminHtmlAction::MfaStepUpTotp.get()><label><span>"Current password"</span><input name="current_password" type="password" required /></label><label><span>"TOTP code"</span><input name="code" required /></label><button type="submit">"Verify TOTP"</button></form>
            <form method="post" action=server_admin_contract::AdminHtmlAction::MfaStepUpRecovery.get()><label><span>"Current password"</span><input name="current_password" type="password" required /></label><label><span>"Recovery code"</span><input name="code" required /></label><button type="submit">"Verify recovery code"</button></form>
            <form method="post" action=server_admin_contract::AdminHtmlAction::MfaDisableTotp.get()><label><span>"Current password"</span><input name="current_password" type="password" required /></label><label><span>"TOTP code"</span><input name="code" required /></label><button class="danger-button" type="submit">"Disable MFA with TOTP"</button></form>
            <form method="post" action=server_admin_contract::AdminHtmlAction::MfaDisableRecovery.get()><label><span>"Current password"</span><input name="current_password" type="password" required /></label><label><span>"Recovery code"</span><input name="code" required /></label><button class="danger-button" type="submit">"Disable MFA with recovery code"</button></form>
        </section>
        <section class="security-card"><h2>"Change password"</h2><form method="post" action=server_admin_contract::AdminHtmlAction::ProfilePassword.get()>
            <label><span>"Current password"</span><input name="current_password" type="password" required /></label>
            <label><span>"New password"</span><input name="new_password" type="password" required /></label>
            <label><input name="revoke_other_sessions" type="checkbox" value="true" />"Revoke other sessions"</label><button type="submit">"Change password"</button>
        </form></section>
    }.render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Profile, content)
}

#[must_use]
pub fn render_mfa_enrollment(view: &server_admin_contract::AdminMfaEnrollRes) -> AdminSsrHtml {
    let content = leptos::view! {
        <section class="security-card"><h2>"Add the TOTP secret to your authenticator"</h2>
            <p><code>{AsRef::<str>::as_ref(view.secret()).to_owned()}</code></p><p><code>{AsRef::<str>::as_ref(view.uri()).to_owned()}</code></p>
            <form method="post" action=server_admin_contract::AdminHtmlAction::MfaConfirm.get()><label><span>"TOTP code"</span><input name="code" required /></label><button type="submit">"Confirm enrollment"</button></form>
        </section>
    }.render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Profile, content)
}

#[must_use]
pub fn render_mfa_recovery_codes(view: &server_admin_contract::AdminMfaConfirmRes) -> AdminSsrHtml {
    let content = leptos::view! {
        <section class="security-card"><h2>"Save these one-time recovery codes"</h2><ul>{view.recovery_codes().iter().map(|code| leptos::view! { <li><code>{AsRef::<str>::as_ref(code).to_owned()}</code></li> }).collect::<Vec<_>>()}</ul><a href=server_admin_contract::AdminFrontendPath::Profile.get()>"Return to profile"</a></section>
    }.render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Profile, content)
}

#[must_use]
pub fn render_settings(view: &server_admin_contract::AdminSettingsView) -> AdminSsrHtml {
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
    let content = leptos::view! {
        <form class="settings-form" method="post" action=server_admin_contract::AdminHtmlAction::SettingsUpdate.get()>
            <label><span>"Site name"</span><input name="site_name" value=site_name required /></label>
            <label><span>"Default route"</span><input name="default_admin_route" value=default_admin_route required /></label>
            <label><span>"Tab title"</span><input name="tab_title" value=tab_title /></label>
            <label><span>"Main logo URL"</span><input name="main_logo" value=main_logo /></label>
            <label><span>"Primary color"</span><input name="primary_color" value=primary_color /></label>
            <label><span>"Organization"</span><input name="organization_name" value=organization_name /></label>
            <label><span>"Organization contacts"</span><textarea name="organization_contacts">{organization_contacts}</textarea></label>
            <label><span>"Support URL"</span><input name="support_url" value=support_url /></label>
            <button type="submit">"Save settings"</button>
        </form>
    }.render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Settings, content)
}

#[must_use]
pub fn render_audit(page: &server_admin_contract::AdminAuditPage) -> AdminSsrHtml {
    let content = leptos::view! {
        <form class="audit-filters" method="get" action=server_admin_contract::AdminFrontendPath::Audit.get()>
            <label><span>"Action"</span><input name="action" /></label><label><span>"Resource"</span><input name="resource" /></label><label><span>"Resource ID"</span><input name="resource_id" /></label>
            <label><span>"User login"</span><input name="user_login" /></label><label><span>"Limit"</span><input name="limit" type="number" min="1" max="100" value="50" /></label><button type="submit">"Apply"</button>
        </form>
        <table><thead><tr><th>"Time"</th><th>"User"</th><th>"Action"</th><th>"Resource"</th><th>"Result"</th></tr></thead><tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td>{item.created_at().to_string()}</td><td>{item.user_login().map(ToString::to_string).unwrap_or_default()}</td><td>{item.action().to_string()}</td><td>{item.resource().to_string()}</td><td>{item.succeeded().to_string()}</td></tr>
        }).collect::<Vec<_>>()}</tbody></table>
    }.render_admin_ssr();
    render_admin_page(server_admin_contract::AdminPage::Audit, content)
}

#[must_use]
pub fn render_text_page(
    page: server_admin_contract::AdminPage,
    title: AdminSsrText,
    text: AdminSsrText,
) -> AdminSsrHtml {
    let content = leptos::view! {
        <section><div class="page-heading"><h1>{title.0}</h1></div>
        <div class="code-card"><pre>{text.0}</pre></div></section>
    }
    .render_admin_ssr();
    render_admin_page(page, content)
}

#[cfg(test)]
mod tests {
    #[test]
    fn server_rendered_pages_contain_forms_and_no_scripts() {
        let sign_in = super::render_sign_in(None);
        assert!(sign_in.as_ref().contains("<form method=\"post\""));
        assert!(!sign_in.as_ref().contains("<script"));
        assert!(!sign_in.as_ref().contains(".wasm"));

        let page = super::render_admin_page(
            server_admin_contract::AdminPage::Dashboard,
            super::AdminSsrHtml::try_from(String::from("<p>ready</p>")).expect("c78bd3a1"),
        );
        assert!(page.as_ref().contains("<p>ready</p>"));
        assert!(!page.as_ref().contains("<script"));
    }
}
