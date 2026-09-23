#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

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
    admin_data_table_view: server_admin_contract::admin_data_table_view::AdminDataTableView,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <crate::admin_record_view::AdminRecordView
            admin_data_table_view=admin_data_table_view
            admin_record_read_page=crate::admin_record_read_page::AdminRecordReadPage::Rule
        />
    }
}
