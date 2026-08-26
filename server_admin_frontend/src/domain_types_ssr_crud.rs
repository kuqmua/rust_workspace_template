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
    page: server_admin_contract::domain_types::AdminPage,
    content: impl leptos::prelude::IntoAny,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let active_table = match page {
        server_admin_contract::domain_types::AdminPage::Users => {
            Some(server_admin_contract::domain_types::AdminDataTable::Users)
        }
        server_admin_contract::domain_types::AdminPage::Roles => {
            Some(server_admin_contract::domain_types::AdminDataTable::Roles)
        }
        server_admin_contract::domain_types::AdminPage::Metrics
        | server_admin_contract::domain_types::AdminPage::OpenApi
        | server_admin_contract::domain_types::AdminPage::Permissions
        | server_admin_contract::domain_types::AdminPage::Profile
        | server_admin_contract::domain_types::AdminPage::Sessions
        | server_admin_contract::domain_types::AdminPage::Settings
        | server_admin_contract::domain_types::AdminPage::Tables
        | server_admin_contract::domain_types::AdminPage::Version => None,
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
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    render_shell(
        server_admin_contract::domain_types::AdminPage::Users,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Users"</p><h1>"Create user"</h1><p>"Add a user account with initial credentials."</p></div><crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::Users.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Back to users"</crate::domain_types::with_owner::button::AdminButtonLink></div>
            <crate::domain_types::with_owner::card::AdminCard><form class="crud-form" method="post" action=server_admin_contract::domain_types::AdminHtmlAction::UserCreate.get()>
                <crate::domain_types::with_owner::field::AdminField label="Login"><crate::domain_types::with_owner::input::AdminInput name="login" autocomplete="username" required=true /></crate::domain_types::with_owner::field::AdminField>
                <crate::domain_types::with_owner::field::AdminField label="Display name"><crate::domain_types::with_owner::input::AdminInput name="display_name" required=true /></crate::domain_types::with_owner::field::AdminField>
                <crate::domain_types::with_owner::field::AdminField label="Initial password"><crate::domain_types::with_owner::input::AdminInput name="password" kind=crate::domain_types::with_owner::input::AdminInputKind::Password autocomplete="new-password" required=true /></crate::domain_types::with_owner::field::AdminField>
                <div class="crud-actions"><crate::domain_types::with_owner::button::AdminButton>"Create user"</crate::domain_types::with_owner::button::AdminButton></div>
            </form></crate::domain_types::with_owner::card::AdminCard></section>
        },
        admin,
        branding,
    )
}

pub(super) fn render_user_manage(
    page: &server_admin_contract::domain_types::AdminUsersPage,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::UsersUpdate),
    );
    let can_delete = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::UsersDelete),
    );
    let cards = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let article_id = format!("user-{}", item.id());
        let title = format!("{} (#{})", item.login(), item.id());
        let login = item.login().to_string();
        let display_name = item.display_name().to_string();
        let status = if bool::from(item.is_banned()) {
            constants_str::VALUE_05EB2107
        } else {
            constants_str::VALUE_92340695
        };
        leptos::view! {
            <crate::domain_types::with_owner::card::AdminCard><article class="crud-record" id=article_id>
                <div class="crud-record-heading"><h2>{title}</h2><span>{status}</span></div>
                {can_update.then(|| leptos::view! { <form class="crud-form crud-form-compact" method="post" action=server_admin_contract::domain_types::AdminHtmlAction::UserUpdate.get()>
                    <input type="hidden" name="user_id" value=id.clone() />
                    <crate::domain_types::with_owner::field::AdminField label="Login"><crate::domain_types::with_owner::input::AdminInput name="login" initial_value=login required=true /></crate::domain_types::with_owner::field::AdminField>
                    <crate::domain_types::with_owner::field::AdminField label="Display name"><crate::domain_types::with_owner::input::AdminInput name="display_name" initial_value=display_name required=true /></crate::domain_types::with_owner::field::AdminField>
                    <div class="crud-actions"><crate::domain_types::with_owner::button::AdminButton>"Save changes"</crate::domain_types::with_owner::button::AdminButton></div>
                </form> })}
                {can_delete.then(|| leptos::view! { <form class="crud-delete" method="post" action=server_admin_contract::domain_types::AdminHtmlAction::UserDelete.get()>
                    <input type="hidden" name="user_id" value=id />
                    <div><p>"Permanently remove this user and their administrator access."</p><label class="crud-confirm"><crate::domain_types::with_owner::admin_checkbox::AdminCheckbox name="confirmation" value="true" required=true />"I understand this cannot be undone"</label></div><crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Danger>"Delete user"</crate::domain_types::with_owner::button::AdminButton>
                </form> })}
            </article></crate::domain_types::with_owner::card::AdminCard>
        }
    }).collect::<Vec<_>>();
    render_shell(
        server_admin_contract::domain_types::AdminPage::Users,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Users"</p><h1>"Manage users"</h1><p>"Update account details or permanently delete an account."</p></div><crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::Users.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Back to users"</crate::domain_types::with_owner::button::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        admin,
        branding,
    )
}

pub(super) fn render_role_create(
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    render_shell(
        server_admin_contract::domain_types::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Roles"</p><h1>"Create role"</h1><p>"Create a role before assigning its permissions."</p></div><crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::Roles.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Back to roles"</crate::domain_types::with_owner::button::AdminButtonLink></div>
            <crate::domain_types::with_owner::card::AdminCard><form class="crud-form" method="post" action=server_admin_contract::domain_types::AdminHtmlAction::RoleCreate.get()>
                <crate::domain_types::with_owner::field::AdminField label="Role name"><crate::domain_types::with_owner::input::AdminInput name="name" required=true /></crate::domain_types::with_owner::field::AdminField>
                <div class="crud-actions"><crate::domain_types::with_owner::button::AdminButton>"Create role"</crate::domain_types::with_owner::button::AdminButton></div>
            </form></crate::domain_types::with_owner::card::AdminCard></section>
        },
        admin,
        branding,
    )
}

pub(super) fn render_role_manage(
    page: &server_admin_contract::domain_types::AdminRolesPage,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesUpdate),
    );
    let can_delete = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesDelete),
    );
    let cards = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let article_id = format!("role-{}", item.id());
        let title = format!("{} (#{})", item.name(), item.id());
        let name = item.name().to_string();
        let is_system = bool::from(item.is_system());
        let status = if is_system {
            constants_str::VALUE_91C86A3E
        } else {
            constants_str::VALUE_5B58E07E
        };
        leptos::view! {
            <crate::domain_types::with_owner::card::AdminCard><article class="crud-record" id=article_id>
                <div class="crud-record-heading"><h2>{title}</h2><span>{status}</span></div>
                {can_update.then(|| leptos::view! { <form class="crud-form crud-form-compact" method="post" action=server_admin_contract::domain_types::AdminHtmlAction::RoleUpdate.get()>
                    <input type="hidden" name="role_id" value=id.clone() />
                    <crate::domain_types::with_owner::field::AdminField label="Role name"><crate::domain_types::with_owner::input::AdminInput name="name" initial_value=name required=true disabled=is_system /></crate::domain_types::with_owner::field::AdminField>
                    <div class="crud-actions"><crate::domain_types::with_owner::button::AdminButton disabled=is_system>"Save changes"</crate::domain_types::with_owner::button::AdminButton></div>
                </form> })}
                {(can_delete && !is_system).then(|| leptos::view! { <form class="crud-delete" method="post" action=server_admin_contract::domain_types::AdminHtmlAction::RoleDelete.get()>
                    <input type="hidden" name="role_id" value=id />
                    <div><p>"Permanently remove this role and its assignments."</p><label class="crud-confirm"><crate::domain_types::with_owner::admin_checkbox::AdminCheckbox name="confirmation" value="true" required=true />"I understand this cannot be undone"</label></div><crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Danger>"Delete role"</crate::domain_types::with_owner::button::AdminButton>
                </form> })}
            </article></crate::domain_types::with_owner::card::AdminCard>
        }
    }).collect::<Vec<_>>();
    render_shell(
        server_admin_contract::domain_types::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Roles"</p><h1>"Manage roles"</h1><p>"Rename custom roles or remove roles that are no longer needed."</p></div><crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::Roles.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Back to roles"</crate::domain_types::with_owner::button::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        admin,
        branding,
    )
}
