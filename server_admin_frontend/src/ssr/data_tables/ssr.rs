#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branch requires attribute traits after macro expansion"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> crate::ssr::AdminSsrHtml;
}

impl<View> AdminSsrViewExt for View
where
    View: leptos::prelude::IntoAny,
{
    fn render_admin_ssr(self) -> crate::ssr::AdminSsrHtml {
        crate::ssr::AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(
            leptos::prelude::IntoAny::into_any(self),
        ))
        .unwrap_or_else(crate::ssr::AdminSsrHtml::from)
    }
}

#[must_use]
pub(in crate::ssr) fn render_data_tables(
    table: Option<&server_admin_contract::AdminDataTableView>,
    query: &server_admin_contract::AdminDataTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> crate::ssr::AdminSsrHtml {
    let content = leptos::view! {
        {table.map(|view| leptos::view! {
            <section class="table-page">
                {crate::ssr::data_table_grid(view, query)}
                {crate::ssr::table_pagination(server_admin_contract::AdminPage::Tables, query.page(), view.total(), Some(view.table()), bool::from(view.table().supports_filters()).then_some(query.filter()))}
            </section>
        })}
    }
    .render_admin_ssr();
    crate::ssr::render_admin_page_with_table_access(
        server_admin_contract::AdminPage::Tables,
        content,
        Some(admin),
        Some(branding),
        table.map(server_admin_contract::AdminDataTableView::table),
    )
}
