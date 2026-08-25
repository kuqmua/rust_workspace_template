#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the shared Leptos grid composes its column and row renderers once"
)]

mod admin_data_grid_column;
mod admin_data_grid_row;

pub(crate) fn admin_data_table_grid(
    view: &server_admin_contract::domain_types::AdminDataTableView,
    active_field: Option<&server_admin_contract::domain_types::AdminFilterField>,
    active_operation: Option<&server_admin_contract::domain_types::AdminFilterOperationKey>,
    active_value: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    active_end: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    limit: server_admin_contract::domain_types::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let columns = view
        .columns()
        .iter()
        .map(|column| {
            admin_data_grid_column::admin_data_grid_column(
                view,
                column,
                active_field,
                active_operation,
                active_value,
                active_end,
                limit,
            )
        })
        .collect::<Vec<_>>();
    let rows = view
        .items()
        .iter()
        .map(|item| admin_data_grid_row::admin_data_grid_row(view, item))
        .collect::<Vec<_>>();
    leptos::view! {
        <crate::domain_types::with_owner::table::TableWrapper><crate::domain_types::with_owner::table::Table>
            <crate::domain_types::with_owner::table::TableHeader><crate::domain_types::with_owner::table::TableRow>{columns}</crate::domain_types::with_owner::table::TableRow></crate::domain_types::with_owner::table::TableHeader>
            <crate::domain_types::with_owner::table::TableBody>{rows}</crate::domain_types::with_owner::table::TableBody>
        </crate::domain_types::with_owner::table::Table></crate::domain_types::with_owner::table::TableWrapper>
    }
}
