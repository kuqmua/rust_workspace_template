#![allow(
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos table-filter rendering requires named attribute traits and converted query values replace borrowed inputs"
)]

mod sort;

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
            {sort::admin_table_sort(sort, sort_fields)}
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
