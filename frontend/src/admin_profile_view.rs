use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(crate) fn AdminProfileView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <section class="profile-grid" data-renderer="csr">
            {crate::admin_profile_account::admin_profile_account(&authenticated_admin)}
            {crate::admin_change_password::admin_change_password()}
        </section>
    }
}
