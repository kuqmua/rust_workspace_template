#![allow(
    clippy::unused_trait_names,
    reason = "the Leptos filter control requires attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    StyleAttribute,
};

#[allow(
    clippy::single_call_fn,
    reason = "the shared filter renderer has separate native and WebAssembly consumers selected by target configuration"
)]
pub(crate) fn admin_column_filter(
    admin_data_table_frontend_path: &server_admin_contract::admin_data_table_frontend_path::AdminDataTableFrontendPath,
    admin_text: &server_admin_contract::admin_text::AdminText,
    input_kind: frontend_contract::input_kind::InputKind,
    admin_data_filters: &[server_admin_contract::admin_data_filter::AdminDataFilter],
    active_field: Option<&server_admin_contract::admin_filter_field::AdminFilterField>,
    active_operation: Option<
        &server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey,
    >,
    active_value: Option<&server_admin_contract::admin_filter_value::AdminFilterValue>,
    active_end: Option<&server_admin_contract::admin_filter_value::AdminFilterValue>,
    admin_page_limit: server_admin_contract::admin_page_limit::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let action = admin_data_table_frontend_path.to_string();
    let limit = u16::from(admin_page_limit).to_string();
    let active_field = active_field.map(ToString::to_string);
    let active_operation = active_operation.map(ToString::to_string);
    let clear_href = admin_data_table_frontend_path.to_string();
    let filter_field = admin_text.to_string();
    let filter_id = format!("table-filter-{filter_field}");
    let close_filter_id = filter_id.clone();
    let anchor_name = format!("--{filter_id}");
    let trigger_style = format!("anchor-name:{anchor_name}");
    let popover_style = format!(
        "position-anchor:{anchor_name};inset:auto;position-area:block-end;position-try-fallbacks:flip-block"
    );
    let filter_label = format!("{}_{filter_field}", constants_str::ADMIN_BUTTON_FILTER);
    let filter_title = filter_field.clone();
    let input_type = crate::admin_data_grid_input_type::AdminDataGridInputType::from(input_kind);
    let is_active_field = active_field.as_deref() == Some(filter_field.as_str());
    let filters = admin_data_filters
        .iter()
        .copied()
        .filter(|filter| {
            input_kind != frontend_contract::input_kind::InputKind::Checkbox
                || filter.operation() != frontend_contract::filter_operation::FilterOperation::In
        })
        .collect::<Vec<_>>();
    let selected_operation = crate::leptos_admin_filter_operation_signal::LeptosAdminFilterOperationSignal::from(
        leptos::prelude::RwSignal::new(
            is_active_field
                .then(|| active_operation.clone())
                .flatten()
                .filter(|active_operation| {
                    filters.iter().any(|filter| {
                        server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(filter.operation()).to_string()
                            == *active_operation
                    })
                })
                .unwrap_or_else(|| {
                    filters
                        .first()
                        .map(|filter| {
                            server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(filter.operation()).to_string()
                        })
                        .unwrap_or_default()
                }),
        ),
    );
    let active_value = active_value.cloned();
    let active_end = active_end.cloned();
    let trigger_filter_id = filter_id.clone();
    let trigger_filter_label = filter_label.clone();
    leptos::prelude::IntoAny::into_any(crate::with_owner::with_owner(move || {
        leptos::view! {
            <singlestage::Popover attr:data-name="Popover" class="table-column-filter">
                <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button popover_target=trigger_filter_id aria_label=trigger_filter_label style=trigger_style>
                    <svg class="table-column-filter-icon" viewBox="0 0 24 24" aria-hidden=constants_str::TRUE>
                        <path fill="currentColor" d="M3 5h18l-7 8v5l-4 2v-7L3 5z"></path>
                    </svg>
                </crate::admin_button::AdminButton>
                <div data-name="PopoverContent" id=filter_id class="table-filter-operations relative z-50 my-[1ch] min-h-[150px] w-[250px] overflow-visible rounded-md border bg-card p-4 shadow-md" style=popover_style popover="auto" role="dialog" aria-label=filter_label>
                    <div class="table-filter-header"><h2>{filter_title}</h2></div>
                    <form class="table-filter-form" method="get" action=action>
                        <input type="hidden" name="filter_field" value=filter_field />
                        <input type="hidden" name="limit" value=limit />
                        <input type="hidden" name="offset" value="0" />
                        <singlestage::RadioGroup attr:data-name="RadioButtonGroup" class="table-filter-options" name="filter_operation" value=leptos::prelude::RwSignal::from(selected_operation)>
                            {filters.into_iter().map(|filter| {
                                let operation_key = server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(filter.operation()).to_string();
                                let is_active = is_active_field && active_operation.as_deref() == Some(operation_key.as_str());
                                let active_value = is_active.then_some(active_value.as_ref()).flatten();
                                let active_end = is_active.then_some(active_end.as_ref()).flatten();
                                let needs_end = bool::from(filter.requires_end());
                                let value = active_value.map(ToString::to_string).unwrap_or_default();
                                let end = active_end.map(ToString::to_string).unwrap_or_default();
                                let operation = filter.operation();
                                let radio_key = server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(operation).to_string();
                                let value_operation_key = radio_key.clone();
                                let end_operation_key = radio_key.clone();
                                let checked = leptos::prelude::Get::get(&leptos::prelude::RwSignal::from(selected_operation)) == radio_key;
                                leptos::view! {
                                    <div class="table-filter-option">
                                        <singlestage::Label attr:data-name="Label" class="table-filter-operation-label flex items-center gap-2 text-sm leading-none font-medium select-none">
                                            <singlestage::Radio class="radio__button peer size-4 shrink-0 rounded-full border border-input shadow-xs outline-none transition-shadow focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50" value=radio_key checked=checked />
                                            <span>{operation_key}</span>
                                        </singlestage::Label>
                                        {bool::from(filter.requires_value()).then(|| {
                                            let value_label = if needs_end { constants_str::ADMIN_UI_START } else { constants_str::ADMIN_UI_VALUE };
                                            leptos::prelude::IntoAny::into_any(leptos::view! {
                                                <singlestage::Label attr:data-name="Label" class="table-filter-input-label flex items-center gap-2 text-sm leading-none font-medium select-none">
                                                    <singlestage::Input attr:data-name="Input" attr:aria-label=value_label attr:class="flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50" name="filter_value" input_type=String::from(input_type.as_ref()) value=value placeholder=value_label required=true disabled=leptos::prelude::Signal::derive(move || leptos::prelude::Get::get(&leptos::prelude::RwSignal::from(selected_operation)) != value_operation_key) />
                                                </singlestage::Label>
                                            })
                                        })}
                                        {bool::from(filter.requires_end()).then(|| leptos::prelude::IntoAny::into_any(leptos::view! {
                                            <singlestage::Label attr:data-name="Label" class="table-filter-input-label flex items-center gap-2 text-sm leading-none font-medium select-none">
                                                <singlestage::Input attr:data-name="Input" attr:aria-label=constants_str::ADMIN_UI_END attr:class="flex h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs outline-none transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50" name="filter_end" input_type=String::from(input_type.as_ref()) value=end placeholder=constants_str::ADMIN_UI_END required=true disabled=leptos::prelude::Signal::derive(move || leptos::prelude::Get::get(&leptos::prelude::RwSignal::from(selected_operation)) != end_operation_key) />
                                            </singlestage::Label>
                                        }))}
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </singlestage::RadioGroup>
                        <div class="table-filter-actions [&>*]:w-full">
                            <crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_APPLY}</crate::admin_button::AdminButton>
                            <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button popover_target=close_filter_id popover_target_action="hide">{constants_str::ADMIN_BUTTON_CLOSE}</crate::admin_button::AdminButton>
                        </div>
                    </form>
                    {is_active_field.then(|| leptos::view! { <a class="table-filter-clear" href=clear_href>{constants_str::ADMIN_UI_CLEAR}</a> })}
                </div>
            </singlestage::Popover>
        }
    }))
}
