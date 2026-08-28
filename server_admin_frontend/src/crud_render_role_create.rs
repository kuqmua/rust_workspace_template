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

pub(super) fn crud_render_role_create(
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    super::crud_render_shell::crud_render_shell(
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
