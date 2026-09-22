#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the application shell"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it into the view"
)]
pub(crate) fn AdminRuleView(
    admin_rules_page: server_admin_contract::admin_rules_page::AdminRulesPage,
) -> impl leptos::prelude::IntoView {
    let content = admin_rules_page.items().first().map_or_else(
        || {
            leptos::prelude::IntoAny::into_any(leptos::view! {
                <crate::admin_alert::AdminAlert>{constants_str::RESOURCE_NOT_FOUND}</crate::admin_alert::AdminAlert>
            })
        },
        |admin_rule_summary| {
            let id = admin_rule_summary.id().to_string();
            let name = admin_rule_summary.name().to_string();
            let created_at = admin_rule_summary.created_at().to_string();
            leptos::prelude::IntoAny::into_any(leptos::view! {
                <div class="health-label">{constants_str::SQL_NAMES_ID}</div>
                <div class="health-result">{id}</div>
                <div class="health-label">{constants_str::NAME}</div>
                <div class="health-result">{name}</div>
                <div class="health-label">{constants_str::CREATED_AT}</div>
                <div class="health-result">{created_at}</div>
            })
        },
    );
    leptos::view! {
        <section class="profile-grid profile-fields" data-renderer="csr" data-page="rule-read">
            {content}
        </section>
    }
}
