#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the shared Leptos grid composes its column and row renderers once"
)]

mod column;
mod row;

pub(crate) fn admin_data_table_grid(
    view: &server_admin_contract::AdminDataTableView,
    active_field: Option<&server_admin_contract::AdminFilterField>,
    active_operation: Option<&server_admin_contract::AdminFilterOperationKey>,
    active_value: Option<&server_admin_contract::AdminFilterValue>,
    active_end: Option<&server_admin_contract::AdminFilterValue>,
    limit: server_admin_contract::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let columns = view
        .columns()
        .iter()
        .map(|column| {
            column::admin_data_grid_column(
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
        .map(|item| row::admin_data_grid_row(view, item))
        .collect::<Vec<_>>();
    leptos::view! {
        <crate::ui::table::TableWrapper><crate::ui::table::Table>
            <crate::ui::table::TableHeader><crate::ui::table::TableRow>{columns}</crate::ui::table::TableRow></crate::ui::table::TableHeader>
            <crate::ui::table::TableBody>{rows}</crate::ui::table::TableBody>
        </crate::ui::table::Table></crate::ui::table::TableWrapper>
    }
}
