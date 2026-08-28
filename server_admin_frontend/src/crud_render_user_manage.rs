#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the server-rendered CRUD forms require Leptos attribute traits after macro expansion"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes};

#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(super) fn crud_render_user_manage(
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
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::domain_types::AdminPage::Users,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Users"</p><h1>"Manage users"</h1><p>"Update account details or permanently delete an account."</p></div><crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::Users.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Back to users"</crate::domain_types::with_owner::button::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        admin,
        branding,
    )
}
