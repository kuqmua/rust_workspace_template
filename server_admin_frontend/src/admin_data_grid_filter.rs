use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    StyleAttribute,
};

#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn admin_data_grid_filter(
    view: &server_admin_contract::domain_types::AdminDataTableView,
    column: &server_admin_contract::domain_types::AdminDataColumn,
    active_field: Option<&server_admin_contract::domain_types::AdminFilterField>,
    active_operation: Option<&server_admin_contract::domain_types::AdminFilterOperationKey>,
    active_value: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    active_end: Option<&server_admin_contract::domain_types::AdminFilterValue>,
    limit: server_admin_contract::domain_types::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let table_path = view.table().frontend_path();
    let action = table_path.to_string();
    let supports_filter =
        bool::from(view.table().supports_filters()) && !column.filters().is_empty();
    let limit = u16::from(limit).to_string();
    let active_field = active_field.map(ToString::to_string);
    let active_operation = active_operation.map(ToString::to_string);
    let clear_href = table_path.to_string();
    let field = column.name().to_string();
    let filter_id = format!("table-filter-{field}");
    let close_filter_id = filter_id.clone();
    let anchor_name = format!("--{filter_id}");
    let trigger_style = format!("anchor-name:{anchor_name}");
    let popover_style = format!(
        "position-anchor:{anchor_name};inset:auto;position-area:block-end;position-try-fallbacks:flip-block"
    );
    let label = column.label().to_string();
    let input_type =
        super::admin_data_grid_input_type::AdminDataGridInputType::from(column.input_kind());
    let is_active_field = active_field.as_deref() == Some(field.as_str());
    let filter_label = format!("Filter {label}");
    let filter_title = format!("Filter by {label}");
    let selected_operation =
        super::LeptosAdminFilterOperationSignal::from(leptos::prelude::RwSignal::new(
            is_active_field
                .then(|| active_operation.clone())
                .flatten()
                .unwrap_or_else(|| {
                    column
                        .filters()
                        .first()
                        .map(|filter| {
                            server_admin_contract::domain_types::AdminFilterOperationKey::from(
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
        supports_filter.then(|| leptos::prelude::IntoAny::into_any(crate::domain_types::with_owner::with_owner(move || leptos::view! {
                    <singlestage::Popover attr:data-name="Popover" class="table-column-filter">
                        <crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary kind=crate::domain_types::with_owner::button::AdminButtonKind::Button popover_target=trigger_filter_id aria_label=trigger_filter_label style=trigger_style>"Filter"</crate::domain_types::with_owner::button::AdminButton>
                        <div data-name="PopoverContent" id=filter_id class="table-filter-operations relative z-50 my-[1ch] min-h-[150px] w-[250px] overflow-visible rounded-md border bg-card p-4 shadow-md" style=popover_style popover="auto" role="dialog" aria-label=filter_label>
                            <div class="table-filter-header"><h2>{filter_title}</h2></div>
                            <form class="table-filter-form" method="get" action=action.clone()>
                                <input type="hidden" name="filter_field" value=field.clone() />
                                <input type="hidden" name="limit" value=limit.clone() />
                                <input type="hidden" name="offset" value="0" />
                                <singlestage::RadioGroup attr:data-name="RadioButtonGroup" class="table-filter-options flex flex-col gap-3" name="filter_operation" value=selected_operation.0>
                                    {filters.into_iter().map(|filter| {
                                        let operation_key = server_admin_contract::domain_types::AdminFilterOperationKey::from(filter.operation()).to_string();
                                        let is_active = is_active_field && active_operation.as_deref() == Some(operation_key.as_str());
                                        super::admin_data_grid_filter_option::admin_data_grid_filter_option(
                                            filter,
                                            is_active.then_some(active_value.as_ref()).flatten(),
                                            is_active.then_some(active_end.as_ref()).flatten(),
                                            input_type,
                                            selected_operation,
                                        )
                                    }).collect::<Vec<_>>()}
                                </singlestage::RadioGroup>
                                <div class="table-filter-actions [&>*]:w-full">
                                    <crate::domain_types::with_owner::button::AdminButton>"Apply"</crate::domain_types::with_owner::button::AdminButton>
                                    <crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary kind=crate::domain_types::with_owner::button::AdminButtonKind::Button popover_target=close_filter_id popover_target_action="hide">"Close"</crate::domain_types::with_owner::button::AdminButton>
                                </div>
                            </form>
                            {is_active_field.then(|| leptos::view! { <a class="table-filter-clear" href=clear_href.clone()>"Clear"</a> })}
                        </div>
                    </singlestage::Popover>
                })))
    }
}
