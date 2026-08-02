#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the shared Leptos grid composes its column and row renderers once"
)]

mod column;
mod row;

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

pub(crate) fn admin_data_table_grid(
    view: &server_admin_contract::AdminDataTableView,
    active_field: Option<&server_admin_contract::AdminFilterField>,
    active_operation: Option<&server_admin_contract::AdminFilterOperationKey>,
    active_value: Option<&server_admin_contract::AdminFilterValue>,
    active_end: Option<&server_admin_contract::AdminFilterValue>,
    limit: server_admin_contract::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    leptos::view! {
        <div data-name="TableWrapper" class="table-scroll max-h-96 overflow-auto rounded-md border"><table data-name="Table" class="w-full max-w-7xl text-sm caption-bottom">
            <thead data-name="TableHeader" class="[&_tr]:border-b sticky top-0 z-10 bg-card"><tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50">{view.columns().iter().map(|column| {
                column::admin_data_grid_column(
                    view,
                    column,
                    active_field,
                    active_operation,
                    active_value,
                    active_end,
                    limit,
                )
            }).collect::<Vec<_>>()}</tr></thead>
            <tbody data-name="TableBody" class="[&_tr:last-child]:border-0">{view.items().iter().map(|item| {
                row::admin_data_grid_row(view, item)
            }).collect::<Vec<_>>()}</tbody>
        </table></div>
    }
}
