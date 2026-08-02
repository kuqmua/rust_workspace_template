#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the Leptos column heading is composed once by the shared grid"
)]

mod filter;

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

pub(super) fn admin_data_grid_column(
    view: &server_admin_contract::AdminDataTableView,
    column: &server_admin_contract::AdminDataColumn,
    active_field: Option<&server_admin_contract::AdminFilterField>,
    active_operation: Option<&server_admin_contract::AdminFilterOperationKey>,
    active_value: Option<&server_admin_contract::AdminFilterValue>,
    active_end: Option<&server_admin_contract::AdminFilterValue>,
    limit: server_admin_contract::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let field = column.name().to_string();
    let label = column.label().to_string();
    let filter_count = column.filters().len().to_string();
    leptos::view! {
        <th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]" data-field=field data-filter-count=filter_count>
            <div class="table-column-heading">
                <span>{label}</span>
                {filter::admin_data_grid_filter(
                    view,
                    column,
                    active_field,
                    active_operation,
                    active_value,
                    active_end,
                    limit,
                )}
            </div>
        </th>
    }
}
