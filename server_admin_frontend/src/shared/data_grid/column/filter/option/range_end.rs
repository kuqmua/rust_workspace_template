#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the filter range-end control is composed once by its range operation"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

pub(super) fn admin_filter_range_end(
    filter: server_admin_contract::AdminDataFilter,
    active_end: Option<&server_admin_contract::AdminFilterValue>,
    input_type: super::super::input_kind::AdminDataGridInputType,
    selected_operation: super::super::LeptosAdminFilterOperationSignal,
) -> impl leptos::prelude::IntoView + use<> {
    let end = active_end.map(ToString::to_string).unwrap_or_default();
    let operation =
        server_admin_contract::AdminFilterOperationKey::from(filter.operation()).to_string();
    bool::from(filter.requires_end()).then(|| {
        leptos::prelude::IntoAny::into_any(leptos::view! {
            <label data-name="Label" class="table-filter-input-label flex items-center gap-2 text-sm leading-none font-medium select-none">
                <span>"End"</span>
                <input
                    data-name="Input"
                    class="flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50"
                    name="filter_end"
                    type=input_type.as_ref()
                    value=end
                    placeholder="End"
                    required
                    disabled=move || leptos::prelude::Get::get(&selected_operation.0) != operation
                />
            </label>
        })
    })
}
