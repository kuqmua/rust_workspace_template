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

#[must_use]
pub fn render_user_manage(
    page: &server_admin_contract::admin_users_page::AdminUsersPage,
    admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    branding: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::admin_permission::AdminPermission::UsersUpdate),
    );
    let can_delete = bool::from(
        admin.has_permission(server_admin_contract::admin_permission::AdminPermission::UsersDelete),
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
            <crate::admin_card::AdminCard><article class="crud-record" id=article_id>
                <div class="crud-record-heading"><h2>{title}</h2><span>{status}</span></div>
                {can_update.then(|| leptos::view! { <form class="crud-form crud-form-compact" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::UserUpdate.get()>
                    <input type="hidden" name="user_id" value=id.clone() />
                    <crate::admin_field::AdminField label="Login"><crate::admin_input::AdminInput name="login" initial_value=login required=true /></crate::admin_field::AdminField>
                    <crate::admin_field::AdminField label="Display name"><crate::admin_input::AdminInput name="display_name" initial_value=display_name required=true /></crate::admin_field::AdminField>
                    <div class="crud-actions"><crate::admin_button::AdminButton>"Save changes"</crate::admin_button::AdminButton></div>
                </form> })}
                {can_delete.then(|| leptos::view! { <form class="crud-delete" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::UserDelete.get()>
                    <input type="hidden" name="user_id" value=id />
                    <div><p>"Permanently remove this user and their administrator access."</p><label class="crud-confirm"><crate::admin_checkbox::AdminCheckbox name="confirmation" value="true" required=true />"I understand this cannot be undone"</label></div><crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Danger>"Delete user"</crate::admin_button::AdminButton>
                </form> })}
            </article></crate::admin_card::AdminCard>
        }
    }).collect::<Vec<_>>();
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::admin_page::AdminPage::Users,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Users"</p><h1>"Manage users"</h1><p>"Update account details or permanently delete an account."</p></div><crate::admin_button_link::AdminButtonLink href=server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get() variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Back to users"</crate::admin_button_link::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        admin,
        branding,
    )
}
