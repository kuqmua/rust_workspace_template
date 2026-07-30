#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the filter value control is composed once by its filter operation"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

pub(super) fn admin_filter_value(
    filter: server_admin_contract::AdminDataFilter,
    active_value: Option<&server_admin_contract::AdminFilterValue>,
    input_type: super::super::input_kind::AdminDataGridInputType,
    selected_operation: super::super::LeptosAdminFilterOperationSignal,
) -> impl leptos::prelude::IntoView + use<> {
    let needs_end = bool::from(filter.requires_end());
    let value = active_value.map(ToString::to_string).unwrap_or_default();
    let operation =
        server_admin_contract::AdminFilterOperationKey::from(filter.operation()).to_string();
    bool::from(filter.requires_value()).then(|| {
        let value_label = if needs_end { "Start" } else { "Value" };
        let value_placeholder = needs_end.then_some(value_label);
        leptos::prelude::IntoAny::into_any(leptos::view! {
            <label class="table-filter-input-label">
                <span>{value_label}</span>
                <input
                    name="filter_value"
                    type=input_type.as_ref()
                    value=value
                    placeholder=value_placeholder
                    required
                    disabled=move || leptos::prelude::Get::get(&selected_operation.0) != operation
                />
            </label>
        })
    })
}
