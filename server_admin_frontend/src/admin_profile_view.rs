use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(crate) fn AdminProfileView(
    admin: server_admin_contract::domain_types::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <section class="profile-grid" data-renderer="csr">
            {admin_profile_account::admin_profile_account(&admin)}
            {admin_change_password::admin_change_password()}
        </section>
    }
}

// Root-owned module compatibility wrappers.
pub(crate) mod admin_change_password {
    pub use super::super::admin_change_password::*;
}
pub(crate) mod admin_profile_account {
    pub use super::super::admin_profile_account::*;
}
