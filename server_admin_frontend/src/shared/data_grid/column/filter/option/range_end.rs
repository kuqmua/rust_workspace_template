#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the filter range-end control is composed once by its range operation"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

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
            <label class="table-filter-input-label">
                <span>"End"</span>
                <input
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
