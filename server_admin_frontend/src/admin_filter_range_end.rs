#![allow(
    clippy::unused_trait_names,
    reason = "the filter range-end control is composed once by its range operation"
)]

use leptos::prelude::{AddAnyAttr, ElementChild};

#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(super) fn admin_filter_range_end(
    filter: server_admin_contract::domain_types::AdminDataFilter,
    active_end: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    input_type: crate::admin_data_grid_input_type::AdminDataGridInputType,
    selected_operation: crate::LeptosAdminFilterOperationSignal,
) -> impl leptos::prelude::IntoView + use<> {
    let end = active_end.map(ToString::to_string).unwrap_or_default();
    let operation =
        server_admin_contract::domain_types::AdminFilterOperationKey::from(filter.operation())
            .to_string();
    bool::from(filter.requires_end()).then(|| {
        leptos::prelude::IntoAny::into_any(leptos::view! {
            <singlestage::Label attr:data-name="Label" class="table-filter-input-label flex items-center gap-2 text-sm leading-none font-medium select-none">
                <span>"End"</span>
                <singlestage::Input
                    attr:data-name="Input"
                    attr:class="flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50"
                    name="filter_end"
                    input_type=String::from(input_type.as_ref())
                    value=end
                    placeholder="End"
                    required=true
                    disabled=leptos::prelude::Signal::derive(move || leptos::prelude::Get::get(&selected_operation.0) != operation)
                />
            </singlestage::Label>
        })
    })
}
