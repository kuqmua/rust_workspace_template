#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the server-rendered CRUD forms require Leptos attribute traits after macro expansion"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes};

fn render_shell(
    page: server_admin_contract::AdminPage,
    content: impl leptos::prelude::IntoAny,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let active_table = match page {
        server_admin_contract::AdminPage::Users => {
            Some(server_admin_contract::AdminDataTable::Users)
        }
        server_admin_contract::AdminPage::Roles => {
            Some(server_admin_contract::AdminDataTable::Roles)
        }
        server_admin_contract::AdminPage::Metrics
        | server_admin_contract::AdminPage::OpenApi
        | server_admin_contract::AdminPage::Permissions
        | server_admin_contract::AdminPage::Profile
        | server_admin_contract::AdminPage::Sessions
        | server_admin_contract::AdminPage::Settings
        | server_admin_contract::AdminPage::Tables
        | server_admin_contract::AdminPage::Version => None,
    };
    super::render_admin_page_with_table_access(
        page,
        super::render_view(content),
        Some(admin),
        Some(branding),
        active_table,
    )
}

pub(super) fn render_user_create(
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    render_shell(
        server_admin_contract::AdminPage::Users,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Users"</p><h1>"Create user"</h1><p>"Add a user account with initial credentials."</p></div><crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::Users.get() variant=crate::ui::button::AdminButtonVariant::Secondary>"Back to users"</crate::ui::button::AdminButtonLink></div>
            <crate::ui::card::AdminCard><form class="crud-form" method="post" action=server_admin_contract::AdminHtmlAction::UserCreate.get()>
                <crate::ui::field::AdminField label="Login"><crate::ui::input::AdminInput name="login" autocomplete="username" required=true /></crate::ui::field::AdminField>
                <crate::ui::field::AdminField label="Display name"><crate::ui::input::AdminInput name="display_name" required=true /></crate::ui::field::AdminField>
                <crate::ui::field::AdminField label="Initial password"><crate::ui::input::AdminInput name="password" kind=crate::ui::input::AdminInputKind::Password autocomplete="new-password" required=true /></crate::ui::field::AdminField>
                <div class="crud-actions"><crate::ui::button::AdminButton>"Create user"</crate::ui::button::AdminButton></div>
            </form></crate::ui::card::AdminCard></section>
        },
        admin,
        branding,
    )
}

pub(super) fn render_user_manage(
    page: &server_admin_contract::AdminUsersPage,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersUpdate));
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersDelete));
    let cards = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let article_id = format!("user-{}", item.id());
        let title = format!("{} (#{})", item.login(), item.id());
        let login = item.login().to_string();
        let display_name = item.display_name().to_string();
        let status = if bool::from(item.is_banned()) {
            "Banned"
        } else {
            "Active"
        };
        leptos::view! {
            <crate::ui::card::AdminCard><article class="crud-record" id=article_id>
                <div class="crud-record-heading"><h2>{title}</h2><span>{status}</span></div>
                {can_update.then(|| leptos::view! { <form class="crud-form crud-form-compact" method="post" action=server_admin_contract::AdminHtmlAction::UserUpdate.get()>
                    <input type="hidden" name="user_id" value=id.clone() />
                    <crate::ui::field::AdminField label="Login"><crate::ui::input::AdminInput name="login" initial_value=login required=true /></crate::ui::field::AdminField>
                    <crate::ui::field::AdminField label="Display name"><crate::ui::input::AdminInput name="display_name" initial_value=display_name required=true /></crate::ui::field::AdminField>
                    <div class="crud-actions"><crate::ui::button::AdminButton>"Save changes"</crate::ui::button::AdminButton></div>
                </form> })}
                {can_delete.then(|| leptos::view! { <form class="crud-delete" method="post" action=server_admin_contract::AdminHtmlAction::UserDelete.get()>
                    <input type="hidden" name="user_id" value=id />
                    <div><p>"Permanently remove this user and their administrator access."</p><label class="crud-confirm"><crate::ui::checkbox::AdminCheckbox name="confirmation" value="true" required=true />"I understand this cannot be undone"</label></div><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Danger>"Delete user"</crate::ui::button::AdminButton>
                </form> })}
            </article></crate::ui::card::AdminCard>
        }
    }).collect::<Vec<_>>();
    render_shell(
        server_admin_contract::AdminPage::Users,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Users"</p><h1>"Manage users"</h1><p>"Update account details or permanently delete an account."</p></div><crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::Users.get() variant=crate::ui::button::AdminButtonVariant::Secondary>"Back to users"</crate::ui::button::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        admin,
        branding,
    )
}

pub(super) fn render_role_create(
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    render_shell(
        server_admin_contract::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Roles"</p><h1>"Create role"</h1><p>"Create a role before assigning its permissions."</p></div><crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::Roles.get() variant=crate::ui::button::AdminButtonVariant::Secondary>"Back to roles"</crate::ui::button::AdminButtonLink></div>
            <crate::ui::card::AdminCard><form class="crud-form" method="post" action=server_admin_contract::AdminHtmlAction::RoleCreate.get()>
                <crate::ui::field::AdminField label="Role name"><crate::ui::input::AdminInput name="name" required=true /></crate::ui::field::AdminField>
                <div class="crud-actions"><crate::ui::button::AdminButton>"Create role"</crate::ui::button::AdminButton></div>
            </form></crate::ui::card::AdminCard></section>
        },
        admin,
        branding,
    )
}

pub(super) fn render_role_manage(
    page: &server_admin_contract::AdminRolesPage,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate));
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesDelete));
    let cards = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let article_id = format!("role-{}", item.id());
        let title = format!("{} (#{})", item.name(), item.id());
        let name = item.name().to_string();
        let is_system = bool::from(item.is_system());
        let status = if is_system {
            "System role"
        } else {
            "Custom role"
        };
        leptos::view! {
            <crate::ui::card::AdminCard><article class="crud-record" id=article_id>
                <div class="crud-record-heading"><h2>{title}</h2><span>{status}</span></div>
                {can_update.then(|| leptos::view! { <form class="crud-form crud-form-compact" method="post" action=server_admin_contract::AdminHtmlAction::RoleUpdate.get()>
                    <input type="hidden" name="role_id" value=id.clone() />
                    <crate::ui::field::AdminField label="Role name"><crate::ui::input::AdminInput name="name" initial_value=name required=true disabled=is_system /></crate::ui::field::AdminField>
                    <div class="crud-actions"><crate::ui::button::AdminButton disabled=is_system>"Save changes"</crate::ui::button::AdminButton></div>
                </form> })}
                {(can_delete && !is_system).then(|| leptos::view! { <form class="crud-delete" method="post" action=server_admin_contract::AdminHtmlAction::RoleDelete.get()>
                    <input type="hidden" name="role_id" value=id />
                    <div><p>"Permanently remove this role and its assignments."</p><label class="crud-confirm"><crate::ui::checkbox::AdminCheckbox name="confirmation" value="true" required=true />"I understand this cannot be undone"</label></div><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Danger>"Delete role"</crate::ui::button::AdminButton>
                </form> })}
            </article></crate::ui::card::AdminCard>
        }
    }).collect::<Vec<_>>();
    render_shell(
        server_admin_contract::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Roles"</p><h1>"Manage roles"</h1><p>"Rename custom roles or remove roles that are no longer needed."</p></div><crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::Roles.get() variant=crate::ui::button::AdminButtonVariant::Secondary>"Back to roles"</crate::ui::button::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        admin,
        branding,
    )
}
