use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

mod admin_change_password;
mod admin_profile_account;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::domain_types::start) fn AdminProfileView(
    admin: server_admin_contract::domain_types::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <section class="profile-grid" data-renderer="csr">
            {admin_profile_account::admin_profile_account(&admin)}
            {admin_change_password::admin_change_password()}
        </section>
    }
}
