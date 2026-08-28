#![allow(
    clippy::unused_trait_names,
    reason = "the shared Leptos grid composes its column and row renderers once"
)]

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
        <crate::domain_types::with_owner::tables::table_wrapper::TableWrapper><crate::domain_types::with_owner::tables::table::Table>
            <crate::domain_types::with_owner::tables::table_header::TableHeader><crate::domain_types::with_owner::tables::table_row::TableRow>{columns}</crate::domain_types::with_owner::tables::table_row::TableRow></crate::domain_types::with_owner::tables::table_header::TableHeader>
            <crate::domain_types::with_owner::tables::table_body::TableBody>{rows}</crate::domain_types::with_owner::tables::table_body::TableBody>
        </crate::domain_types::with_owner::tables::table::Table></crate::domain_types::with_owner::tables::table_wrapper::TableWrapper>
    }
}

// Root-owned module compatibility wrappers.
pub(crate) mod admin_data_grid_column {
    pub use crate::admin_data_grid_column::*;
}
pub(crate) mod admin_data_grid_row {
    pub use crate::admin_data_grid_row::*;
}
