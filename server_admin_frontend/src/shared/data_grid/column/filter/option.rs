#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "each Leptos filter operation is composed once by its column filter"
)]

mod range_end;
mod value;

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

pub(super) fn admin_data_grid_filter_option(
    filter: server_admin_contract::AdminDataFilter,
    active_value: Option<&server_admin_contract::AdminFilterValue>,
    active_end: Option<&server_admin_contract::AdminFilterValue>,
    input_type: super::input_kind::AdminDataGridInputType,
    selected_operation: super::LeptosAdminFilterOperationSignal,
) -> impl leptos::prelude::IntoView + use<> {
    let operation = filter.operation();
    let operation_key = server_admin_contract::AdminFilterOperationKey::from(operation).to_string();
    let checked_operation = operation_key.clone();
    let changed_operation = operation_key.clone();
    leptos::view! {
        <div class="table-filter-option">
            <label data-name="Label" class="table-filter-operation-label flex items-center gap-2 text-sm leading-none font-medium select-none">
                <input
                    data-name="RadioButton"
                    class="radio__button peer size-4 shrink-0 rounded-full border border-input shadow-xs outline-none transition-shadow focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50"
                    type="radio"
                    name="filter_operation"
                    value=operation_key
                    checked=move || leptos::prelude::Get::get(&selected_operation.0) == checked_operation
                    on:change=move |_event| leptos::prelude::Set::set(&selected_operation.0, changed_operation.clone())
                />
                <span>{format!("{operation:?}")}</span>
            </label>
            {value::admin_filter_value(filter, active_value, input_type, selected_operation)}
            {range_end::admin_filter_range_end(filter, active_end, input_type, selected_operation)}
        </div>
    }
}
