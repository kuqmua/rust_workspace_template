#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the filter value control is composed once by its filter operation"
)]

use leptos::prelude::{AddAnyAttr, ElementChild};

pub(super) fn admin_filter_value(
    filter: server_admin_contract::domain_types::AdminDataFilter,
    active_value: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    input_type: super::super::input_kind::AdminDataGridInputType,
    selected_operation: super::super::LeptosAdminFilterOperationSignal,
) -> impl leptos::prelude::IntoView + use<> {
    let needs_end = bool::from(filter.requires_end());
    let value = active_value.map(ToString::to_string).unwrap_or_default();
    let operation =
        server_admin_contract::domain_types::AdminFilterOperationKey::from(filter.operation())
            .to_string();
    bool::from(filter.requires_value()).then(|| {
        let value_label = if needs_end { "Start" } else { "Value" };
        let value_placeholder = needs_end.then_some(value_label);
        leptos::prelude::IntoAny::into_any(leptos::view! {
            <singlestage::Label attr:data-name="Label" class="table-filter-input-label flex items-center gap-2 text-sm leading-none font-medium select-none">
                <span>{value_label}</span>
                <singlestage::Input
                    attr:data-name="Input"
                    attr:class="flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50"
                    name="filter_value"
                    input_type=String::from(input_type.as_ref())
                    value=value
                    placeholder=value_placeholder.map(String::from)
                    required=true
                    disabled=leptos::prelude::Signal::derive(move || leptos::prelude::Get::get(&selected_operation.0) != operation)
                />
            </singlestage::Label>
        })
    })
}
