#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the sort selector is composed once by the shared table-filter form and Leptos macro expansion resolves grouped attribute traits"
)]

use leptos::prelude::{ElementChild, GlobalAttributes};

pub(super) fn admin_table_sort(
    sort: &server_admin_contract::AdminTableSortKey,
    sort_fields: &[server_admin_contract::AdminTableSortField],
) -> impl leptos::prelude::IntoView + use<> {
    let sort = sort.as_ref().to_owned();
    leptos::view! {
        <label>
            <span>"Sort"</span>
            <select name="sort">
                <option value="" selected=sort.is_empty()>"Default"</option>
                {sort_fields.iter().copied().map(|field| {
                    let key = field.key().as_ref().to_owned();
                    let selected = sort == key;
                    leptos::view! {
                        <option value=key selected=selected>{field.label().as_ref().to_owned()}</option>
                    }
                }).collect::<Vec<_>>()}
            </select>
        </label>
    }
}
