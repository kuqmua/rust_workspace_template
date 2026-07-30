#![allow(
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the Leptos column-filter converts borrowed query values and is composed once by its column"
)]

mod input_kind;
mod option;

use leptos::prelude::{AriaAttributes, ClassAttribute, ElementChild, GlobalAttributes};

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct LeptosAdminFilterOperationSignal(leptos::prelude::RwSignal<String>);

pub(super) fn admin_data_grid_filter(
    view: &server_admin_contract::AdminDataTableView,
    column: &server_admin_contract::AdminDataColumn,
    active_field: Option<&server_admin_contract::AdminFilterField>,
    active_operation: Option<&server_admin_contract::AdminFilterOperationKey>,
    active_value: Option<&server_admin_contract::AdminFilterValue>,
    active_end: Option<&server_admin_contract::AdminFilterValue>,
    limit: server_admin_contract::AdminPageLimit,
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
    let label = column.label().to_string();
    let input_type = input_kind::AdminDataGridInputType::from(column.input_kind());
    let is_active_field = active_field.as_deref() == Some(field.as_str());
    let filter_label = format!("Filter {label}");
    let filter_title = format!("Filter by {label}");
    let selected_operation =
        LeptosAdminFilterOperationSignal::from(leptos::prelude::RwSignal::new(
            is_active_field
                .then(|| active_operation.clone())
                .flatten()
                .unwrap_or_else(|| {
                    column
                        .filters()
                        .first()
                        .map(|filter| {
                            server_admin_contract::AdminFilterOperationKey::from(filter.operation())
                                .to_string()
                        })
                        .unwrap_or_default()
                }),
        ));
    {
        supports_filter.then(|| leptos::prelude::IntoAny::into_any(leptos::view! {
                    <details class="table-column-filter">
                        <summary class=("active", is_active_field) aria-label=filter_label.clone()><span class="table-filter-open-label">"Filter"</span><span class="table-filter-close-label">"Close"</span></summary>
                        <div class="table-filter-operations" role="dialog" aria-modal="true" aria-label=filter_label>
                            <div class="table-filter-header"><h2>{filter_title}</h2></div>
                            <form class="table-filter-form" method="get" action=action.clone()>
                                <input type="hidden" name="filter_field" value=field.clone() />
                                <input type="hidden" name="limit" value=limit.clone() />
                                <input type="hidden" name="offset" value="0" />
                                <div class="table-filter-options">
                                    {column.filters().iter().map(|filter| {
                                        let operation_key = server_admin_contract::AdminFilterOperationKey::from(filter.operation()).to_string();
                                        let is_active = is_active_field && active_operation.as_deref() == Some(operation_key.as_str());
                                        option::admin_data_grid_filter_option(
                                            *filter,
                                            is_active.then_some(active_value).flatten(),
                                            is_active.then_some(active_end).flatten(),
                                            input_type,
                                            selected_operation,
                                        )
                                    }).collect::<Vec<_>>()}
                                </div>
                                <button type="submit">"Apply"</button>
                            </form>
                            {is_active_field.then(|| leptos::view! { <a class="table-filter-clear" href=clear_href.clone()>"Clear"</a> })}
                        </div>
                    </details>
                }))
    }
}
