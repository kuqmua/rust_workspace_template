#![allow(
    clippy::shadow_reuse,
    reason = "converted query values intentionally replace borrowed inputs"
)]

pub(crate) fn admin_table_query_hidden_inputs(
    admin_table_search: &server_admin_contract::admin_table_search::AdminTableSearch,
    admin_table_sort_key: &server_admin_contract::admin_table_sort_key::AdminTableSortKey,
    admin_table_query_direction: &crate::admin_table_query_direction::AdminTableQueryDirection,
    admin_page_limit: server_admin_contract::admin_page_limit::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let search = admin_table_search.as_ref().to_owned();
    let sort = admin_table_sort_key.as_ref().to_owned();
    let direction = match admin_table_query_direction {
        #[cfg(target_arch = "wasm32")]
        crate::admin_table_query_direction::AdminTableQueryDirection::Csr(value) => {
            value.as_ref().map(ToString::to_string).unwrap_or_default()
        }
        #[cfg(not(target_arch = "wasm32"))]
        crate::admin_table_query_direction::AdminTableQueryDirection::Ssr(value) => {
            value.as_ref().to_owned()
        }
    };
    let limit = u16::from(admin_page_limit).to_string();
    leptos::view! {
        <input type="hidden" name="search" value=search /><input type="hidden" name="sort" value=sort />
        <input type="hidden" name="direction" value=direction /><input type="hidden" name="limit" value=limit />
    }
}
