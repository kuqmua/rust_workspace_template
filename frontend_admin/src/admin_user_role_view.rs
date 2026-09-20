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
pub(crate) fn AdminUserRoleView(
    admin_data_table_view: server_admin_contract::admin_data_table_view::AdminDataTableView,
) -> impl leptos::prelude::IntoView {
    let content = admin_data_table_view.items().first().map_or_else(
        || {
            leptos::prelude::IntoAny::into_any(leptos::view! {
                <crate::admin_alert::AdminAlert>{constants_str::RESOURCE_NOT_FOUND}</crate::admin_alert::AdminAlert>
            })
        },
        |admin_data_row| {
            let fields = admin_data_table_view.columns().iter().zip(admin_data_row.values()).map(|(admin_data_column, admin_text)| {
                let label = admin_data_column.name().to_string();
                let value = admin_text.to_string();
                leptos::view! {
                    <div class="health-label">{label}</div>
                    <div class="health-result">{value}</div>
                }
            }).collect::<Vec<_>>();
            leptos::prelude::IntoAny::into_any(fields)
        },
    );
    leptos::view! {
        <section class="profile-grid profile-fields" data-renderer="csr" data-page="user-role-read">
            {content}
        </section>
    }
}
