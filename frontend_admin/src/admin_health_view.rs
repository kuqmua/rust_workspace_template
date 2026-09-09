#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
#[allow(
    clippy::single_call_fn,
    reason = "Leptos requires a named component to generate the props factory used by the page view"
)]
pub(crate) fn AdminHealthView() -> impl leptos::prelude::IntoView {
    leptos::view! {
        <section class="profile-grid profile-fields" data-renderer="csr">
            {[server_admin_contract::admin_route::AdminRoute::Health,
              server_admin_contract::admin_route::AdminRoute::HealthCheck,
              server_admin_contract::admin_route::AdminRoute::HealthLive,
              server_admin_contract::admin_route::AdminRoute::HealthReady].into_iter().map(|route| {
                leptos::view! { <crate::admin_health_probe::AdminHealthProbe admin_route=route /> }
              }).collect::<Vec<_>>()}
        </section>
    }
}
