#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos component expansion generates framework-defined props fields and builder methods"
)]

use leptos::prelude::{CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos component props own the typed route when moving it into the generated view"
)]
#[allow(
    clippy::single_call_fn,
    reason = "the update action is a named Leptos component for shared table-row rendering"
)]
pub(crate) fn AdminUpdateAction(
    admin_route_path: server_admin_contract::admin_route_path::AdminRoutePath,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <crate::admin_table_action_trigger::AdminTableActionTrigger label=constants_str::PG_CRUD_UPDATE_RULE_ACTION href=admin_route_path>
            <svg viewBox="0 0 24 24" aria-hidden=constants_str::TRUE fill="currentColor">
                <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25m17.71-10.04a.996.996 0 0 0 0-1.41l-2.34-2.34a.996.996 0 0 0-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83"></path>
            </svg>
        </crate::admin_table_action_trigger::AdminTableActionTrigger>
    }
}
