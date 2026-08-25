use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

mod account;
mod password;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::domain_types::app) fn AdminProfileView(
    admin: server_admin_contract::domain_types::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <section class="profile-grid" data-renderer="csr">
            {account::admin_profile_account(&admin)}
            {password::admin_change_password()}
        </section>
    }
}
