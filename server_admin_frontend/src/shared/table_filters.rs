#![allow(
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos view expansion requires named attribute traits and converted query values intentionally replace borrowed inputs"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[derive(Clone, Copy, Debug)]
pub(crate) enum AdminTableFilterDirection {
    Asc,
    Desc,
    #[cfg(target_arch = "wasm32")]
    Other,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum AdminTableFilterPresentation {
    #[cfg(target_arch = "wasm32")]
    Csr,
    #[cfg(not(target_arch = "wasm32"))]
    Ssr,
}

impl AdminTableFilterDirection {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn from_csr(value: Option<&server_admin_contract::AdminText>) -> Self {
        match value.map(|direction| direction.as_ref().as_str()) {
            None | Some(str_constants::ASC_ALT) => Self::Asc,
            Some(str_constants::DESC_ALT) => Self::Desc,
            Some(_) => Self::Other,
        }
    }
}

impl From<server_admin_contract::AdminSortDirection> for AdminTableFilterDirection {
    fn from(value: server_admin_contract::AdminSortDirection) -> Self {
        match value {
            server_admin_contract::AdminSortDirection::Asc => Self::Asc,
            server_admin_contract::AdminSortDirection::Desc => Self::Desc,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AdminTableQueryDirection {
    #[cfg(target_arch = "wasm32")]
    Csr(Option<server_admin_contract::AdminText>),
    #[cfg(not(target_arch = "wasm32"))]
    Ssr(server_admin_contract::AdminSortDirection),
}

pub(crate) fn admin_table_query_hidden_inputs(
    search: &server_admin_contract::AdminTableSearch,
    sort: &server_admin_contract::AdminTableSortKey,
    direction: &AdminTableQueryDirection,
    limit: server_admin_contract::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let search = search.as_ref().to_owned();
    let sort = sort.as_ref().to_owned();
    let direction = match direction {
        #[cfg(target_arch = "wasm32")]
        AdminTableQueryDirection::Csr(value) => {
            value.as_ref().map(ToString::to_string).unwrap_or_default()
        }
        #[cfg(not(target_arch = "wasm32"))]
        AdminTableQueryDirection::Ssr(value) => value.as_ref().to_owned(),
    };
    let limit = u16::from(limit).to_string();
    leptos::view! {
        <input type="hidden" name="search" value=search /><input type="hidden" name="sort" value=sort />
        <input type="hidden" name="direction" value=direction /><input type="hidden" name="limit" value=limit />
    }
}

pub(crate) fn admin_table_filters(
    action: server_admin_contract::AdminFrontendPath,
    search: &server_admin_contract::AdminTableSearch,
    sort: &server_admin_contract::AdminTableSortKey,
    direction: AdminTableFilterDirection,
    limit: server_admin_contract::AdminPageLimit,
    sort_fields: &[server_admin_contract::AdminTableSortField],
    presentation: AdminTableFilterPresentation,
) -> impl leptos::prelude::IntoView + use<> {
    let search = search.as_ref().to_owned();
    let sort = sort.as_ref().to_owned();
    let ascending = matches!(direction, AdminTableFilterDirection::Asc);
    let descending = matches!(direction, AdminTableFilterDirection::Desc);
    let editable_limit = match presentation {
        #[cfg(target_arch = "wasm32")]
        AdminTableFilterPresentation::Csr => true,
        #[cfg(not(target_arch = "wasm32"))]
        AdminTableFilterPresentation::Ssr => false,
    };
    let limit = u16::from(limit).to_string();
    leptos::view! {
        <form class="table-tools" method="get" action=action.get()>
            <label><span>"Search"</span><input name="search" value=search /></label>
            <label><span>"Sort"</span><select name="sort">
                <option value="" selected=sort.is_empty()>"Default"</option>
                {sort_fields.iter().copied().map(|field| {
                    let key = field.key().as_ref().to_owned();
                    let selected = sort == key;
                    leptos::view! { <option value=key selected=selected>{field.label().as_ref().to_owned()}</option> }
                }).collect::<Vec<_>>()}
            </select></label>
            <label><span>"Direction"</span><select name="direction"><option value="asc" selected=ascending>"Ascending"</option><option value="desc" selected=descending>"Descending"</option></select></label>
            {editable_limit.then(|| leptos::view! {
                <input name="limit" type="number" min=server_admin_contract::AdminPageLimit::MIN max=server_admin_contract::AdminPageLimit::MAX value=limit.clone() />
            })}
            {(!editable_limit).then(|| leptos::view! {
                <input name="limit" type="hidden" value=limit />
            })}
            <input name="offset" type="hidden" value="0" /><button type="submit">"Apply"</button>
        </form>
    }
}

pub(crate) fn admin_filter_hidden_inputs(
    field: Option<&server_admin_contract::AdminFilterField>,
    operation: Option<&server_admin_contract::AdminFilterOperationKey>,
    value: Option<&server_admin_contract::AdminFilterValue>,
    end: Option<&server_admin_contract::AdminFilterValue>,
) -> impl leptos::prelude::IntoView + use<> {
    let field = field.map(ToString::to_string);
    let operation = operation.map(ToString::to_string);
    let value = value.map(ToString::to_string);
    let end = end.map(ToString::to_string);
    leptos::view! {
        {field.map(|field_text| leptos::view! { <input type="hidden" name="filter_field" value=field_text /> })}
        {operation.map(|operation_text| leptos::view! { <input type="hidden" name="filter_operation" value=operation_text /> })}
        {value.map(|filter_text| leptos::view! { <input type="hidden" name="filter_value" value=filter_text /> })}
        {end.map(|end_text| leptos::view! { <input type="hidden" name="filter_end" value=end_text /> })}
    }
}
