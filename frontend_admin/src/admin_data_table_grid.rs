#![allow(
    clippy::unused_trait_names,
    reason = "the Leptos grid cells and column headings require attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    StyleAttribute,
};

#[allow(
    clippy::single_call_fn,
    reason = "admin data table grid remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn admin_data_table_grid(
    admin_data_table_view: &server_admin_contract::admin_data_table_view::AdminDataTableView,
    active_field: Option<&server_admin_contract::admin_filter_field::AdminFilterField>,
    active_operation: Option<
        &server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey,
    >,
    active_value: Option<&server_admin_contract::admin_filter_value::AdminFilterValue>,
    active_end: Option<&server_admin_contract::admin_filter_value::AdminFilterValue>,
    admin_page_limit: server_admin_contract::admin_page_limit::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let columns = admin_data_table_view
        .columns()
        .iter()
        .map(|column| {
            let field = column.name().to_string();
            let label = column.label().to_string();
            let filter_count = column.filters().len().to_string();
let filter = {
                let table_path = admin_data_table_view.table().frontend_path();
                let action = table_path.to_string();
                let supports_filter =
                    bool::from(admin_data_table_view.table().supports_filters()) && !column.filters().is_empty();
                let limit = u16::from(admin_page_limit).to_string();
                let active_field = active_field.map(ToString::to_string);
                let active_operation = active_operation.map(ToString::to_string);
                let clear_href = table_path.to_string();
                let flt_field = column.name().to_string();
                let filter_id = format!("table-filter-{flt_field}");
                let close_filter_id = filter_id.clone();
                let anchor_name = format!("--{filter_id}");
                let trigger_style = format!("anchor-name:{anchor_name}");
                let popover_style = format!(
                    "position-anchor:{anchor_name};inset:auto;position-area:block-end;position-try-fallbacks:flip-block"
                );
                let flt_label = column.label().to_string();
                let input_type =
                    super::admin_data_grid_input_type::AdminDataGridInputType::from(column.input_kind());
                let is_active_field = active_field.as_deref() == Some(flt_field.as_str());
                let filter_label = format!("Filter {flt_label}");
                let filter_title = format!("Filter by {flt_label}");
                let selected_operation =
                    crate::leptos_admin_filter_operation_signal::LeptosAdminFilterOperationSignal::from(leptos::prelude::RwSignal::new(
                        is_active_field
                            .then(|| active_operation.clone())
                            .flatten()
                            .unwrap_or_else(|| {
                                column
                                    .filters()
                                    .first()
                                    .map(|filter| {
                                        server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(
                                            filter.operation(),
                                        )
                                        .to_string()
                                    })
                                    .unwrap_or_default()
                            }),
                    ));
                let filters = column.filters().to_vec();
                let active_value = active_value.cloned();
                let active_end = active_end.cloned();
                let trigger_filter_id = filter_id.clone();
                let trigger_filter_label = filter_label.clone();
                {
                    supports_filter.then(|| leptos::prelude::IntoAny::into_any(crate::with_owner::with_owner(move || leptos::view! {
                                <singlestage::Popover attr:data-name="Popover" class="table-column-filter">
                                    <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button popover_target=trigger_filter_id aria_label=trigger_filter_label style=trigger_style>{constants_str::ADMIN_BUTTON_FILTER}</crate::admin_button::AdminButton>
                                    <div data-name="PopoverContent" id=filter_id class="table-filter-operations relative z-50 my-[1ch] min-h-[150px] w-[250px] overflow-visible rounded-md border bg-card p-4 shadow-md" style=popover_style popover="auto" role="dialog" aria-label=filter_label>
                                        <div class="table-filter-header"><h2>{filter_title}</h2></div>
                                        <form class="table-filter-form" method="get" action=action.clone()>
                                            <input type="hidden" name="filter_field" value=flt_field.clone() />
                                            <input type="hidden" name="limit" value=limit.clone() />
                                            <input type="hidden" name="offset" value="0" />
                                            <singlestage::RadioGroup attr:data-name="RadioButtonGroup" class="table-filter-options flex flex-col gap-3" name="filter_operation" value=leptos::prelude::RwSignal::from(selected_operation)>
                                                {filters.into_iter().map(|filter| {
                                                    let operation_key = server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(filter.operation()).to_string();
                                                    let is_active = is_active_field && active_operation.as_deref() == Some(operation_key.as_str());
                {
                                                        let active_value = is_active.then_some(active_value.as_ref()).flatten();
                                                        let active_end = is_active.then_some(active_end.as_ref()).flatten();
                                                        let operation = filter.operation();
                                                        let radio_key =
                                                            server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(operation).to_string();
                                                        let checked = leptos::prelude::Get::get(&leptos::prelude::RwSignal::from(selected_operation)) == radio_key;
                                                        leptos::view! {
                                                            <div class="table-filter-option">
                                                                <singlestage::Label attr:data-name="Label" class="table-filter-operation-label flex items-center gap-2 text-sm leading-none font-medium select-none">
                                                                    <singlestage::Radio
                                                                        class="radio__button peer size-4 shrink-0 rounded-full border border-input shadow-xs outline-none transition-shadow focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50"
                                                                        value=radio_key
                                                                        checked=checked
                                                                    />
                                                                    <span>{format!("{operation:?}")}</span>
                                                                </singlestage::Label>
                                                                {{
                                                                    let needs_end = bool::from(filter.requires_end());
                                                                    let value = active_value.map(ToString::to_string).unwrap_or_default();
                                                                    let op_key = server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(filter.operation()).to_string();
                                                                    bool::from(filter.requires_value()).then(|| {
                                                                        let value_label = if needs_end { constants_str::VALUE_E4BB9F1E } else { constants_str::CODE_STYLE_VALUE };
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
                                                                                    disabled=leptos::prelude::Signal::derive(move || leptos::prelude::Get::get(&leptos::prelude::RwSignal::from(selected_operation)) != op_key)
                                                                                />
                                                                            </singlestage::Label>
                                                                        })
                                                                    })
                                                                }}
                                                                {{
                                                                    let end = active_end.map(ToString::to_string).unwrap_or_default();
                                                                    let op_key = server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from(filter.operation()).to_string();
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
                                                                                    disabled=leptos::prelude::Signal::derive(move || leptos::prelude::Get::get(&leptos::prelude::RwSignal::from(selected_operation)) != op_key)
                                                                                />
                                                                            </singlestage::Label>
                                                                        })
                                                                    })
                                                                }}
                                                            </div>
                                                        }
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </singlestage::RadioGroup>
                                            <div class="table-filter-actions [&>*]:w-full">
                                                <crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_APPLY}</crate::admin_button::AdminButton>
                                                <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary admin_button_kind=crate::admin_button_kind::AdminButtonKind::Button popover_target=close_filter_id popover_target_action="hide">{constants_str::ADMIN_BUTTON_CLOSE}</crate::admin_button::AdminButton>
                                            </div>
                                        </form>
                                        {is_active_field.then(|| leptos::view! { <a class="table-filter-clear" href=clear_href.clone()>"Clear"</a> })}
                                    </div>
                                </singlestage::Popover>
                            })))
                }
            };
            leptos::view! {
                <crate::table_head::TableHead data_field=field data_filter_count=filter_count>
                    <div class="table-column-heading">
                        <span>{label}</span>
                        {filter}
                    </div>
                </crate::table_head::TableHead>
            }
        })
        .collect::<Vec<_>>();
    let rows = admin_data_table_view
        .items()
        .iter()
        .map(|item| {
            let cells = item
                .values()
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let column = admin_data_table_view.columns().get(index);
                    let label =
                        column.map_or_else(String::new, |column| column.label().to_string());
                    let field =
                        column.map_or_else(String::new, |column| column.name().to_string());
                    let numeric = column.is_some_and(|column| {
                        matches!(column.input_kind(), frontend_contract::input_kind::InputKind::Number)
                    });
                    let value_text = value.to_string();
                    leptos::view! { <crate::table_cell::TableCell data_label=label data_field=field class=if numeric { "numeric-cell" } else { "" }>{value_text}</crate::table_cell::TableCell> }
                })
                .collect::<Vec<_>>();
            leptos::view! {
                <crate::table_row::TableRow>{cells}</crate::table_row::TableRow>
            }
        })
        .collect::<Vec<_>>();
    leptos::view! {
        <crate::table_wrapper::TableWrapper><crate::table::Table>
            <crate::table_header::TableHeader><crate::table_row::TableRow>{columns}</crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody>
        </crate::table::Table></crate::table_wrapper::TableWrapper>
    }
}
