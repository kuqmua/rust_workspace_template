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

pub(super) fn render_user_create(
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    super::crud_render_shell::render_shell(
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
