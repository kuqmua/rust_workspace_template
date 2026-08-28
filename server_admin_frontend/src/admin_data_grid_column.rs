#![allow(
    clippy::unused_trait_names,
    reason = "the Leptos column heading is composed once by the shared grid"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(super) fn admin_data_grid_column(
    view: &server_admin_contract::domain_types::AdminDataTableView,
    column: &server_admin_contract::domain_types::AdminDataColumn,
    active_field: Option<&server_admin_contract::domain_types::AdminFilterField>,
    active_operation: Option<&server_admin_contract::domain_types::AdminFilterOperationKey>,
    active_value: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    active_end: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    limit: server_admin_contract::domain_types::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let field = column.name().to_string();
    let label = column.label().to_string();
    let filter_count = column.filters().len().to_string();
    let filter = filter::admin_data_grid_filter(
        view,
        column,
        active_field,
        active_operation,
        active_value,
        active_end,
        limit,
    );
    leptos::view! {
        <crate::domain_types::with_owner::tables::table_head::TableHead data_field=field data_filter_count=filter_count>
            <div class="table-column-heading">
                <span>{label}</span>
                {filter}
            </div>
        </crate::domain_types::with_owner::tables::table_head::TableHead>
    }
}

// Root-owned module compatibility wrappers.
pub(crate) mod filter {
    pub use crate::filter::*;
}
