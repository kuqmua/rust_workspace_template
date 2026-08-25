#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "each Leptos filter operation is composed once by its column filter"
)]

mod range_end;
mod value;

use leptos::prelude::{AddAnyAttr, ClassAttribute, ElementChild};

pub(super) fn admin_data_grid_filter_option(
    filter: server_admin_contract::domain_types::AdminDataFilter,
    active_value: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    active_end: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    input_type: super::input_kind::AdminDataGridInputType,
    selected_operation: super::LeptosAdminFilterOperationSignal,
) -> impl leptos::prelude::IntoView + use<> {
    let operation = filter.operation();
    let operation_key =
        server_admin_contract::domain_types::AdminFilterOperationKey::from(operation).to_string();
    let checked = leptos::prelude::Get::get(&selected_operation.0) == operation_key;
    leptos::view! {
        <div class="table-filter-option">
            <singlestage::Label attr:data-name="Label" class="table-filter-operation-label flex items-center gap-2 text-sm leading-none font-medium select-none">
                <singlestage::Radio
                    class="radio__button peer size-4 shrink-0 rounded-full border border-input shadow-xs outline-none transition-shadow focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50"
                    value=operation_key
                    checked=checked
                />
                <span>{format!("{operation:?}")}</span>
            </singlestage::Label>
            {value::admin_filter_value(filter, active_value, input_type, selected_operation)}
            {range_end::admin_filter_range_end(filter, active_end, input_type, selected_operation)}
        </div>
    }
}
