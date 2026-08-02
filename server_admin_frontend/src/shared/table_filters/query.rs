#![allow(
    clippy::shadow_reuse,
    reason = "converted query values intentionally replace borrowed inputs"
)]

#[derive(optml::Optml, Clone, Debug)]
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
