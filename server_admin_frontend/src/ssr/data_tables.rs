#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> super::AdminSsrHtml;
}
impl<View> AdminSsrViewExt for View
where
    View: leptos::prelude::IntoAny,
{
    fn render_admin_ssr(self) -> super::AdminSsrHtml {
        super::AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(
            leptos::prelude::IntoAny::into_any(self),
        ))
        .unwrap_or_else(super::AdminSsrHtml::from)
    }
}

#[must_use]
pub(super) fn render_data_tables(
    table: Option<&server_admin_contract::AdminDataTableView>,
    query: &server_admin_contract::AdminDataTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let content = leptos::view! {
        {table.map(|view| leptos::view! {
            <section class="table-page">
                {super::data_table_grid(view, query)}
                {super::table_pagination(server_admin_contract::AdminPage::Tables, query.page(), view.total(), Some(view.table()), bool::from(view.table().supports_filters()).then_some(query.filter()))}
            </section>
        })}
    }
    .render_admin_ssr();
    super::render_admin_page_with_table_access(
        server_admin_contract::AdminPage::Tables,
        content,
        Some(admin),
        Some(branding),
        table.map(server_admin_contract::AdminDataTableView::table),
    )
}

#[must_use]
pub(super) fn render_data_tables_csr(
    active_table: Option<server_admin_contract::AdminDataTable>,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    render_admin_csr(
        server_admin_contract::AdminPage::Tables,
        active_table,
        admin,
        branding,
    )
}

#[must_use]
pub(super) fn render_admin_csr(
    page: server_admin_contract::AdminPage,
    _active_table: Option<server_admin_contract::AdminDataTable>,
    _admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let title = branding.tab_title().map_or_else(
        || page.spec().title().as_ref().to_owned(),
        |value| value.as_ref().to_owned(),
    );
    let primary_color = branding
        .primary_color()
        .map(|value| format!("--accent:{}", value.as_ref()));
    super::render_document(
        &super::AdminSsrText::try_from(title).unwrap_or_else(super::AdminSsrText::from),
        leptos::view! {
            <div id=str_constants::ADMIN_CSR_ROOT_ID style=primary_color>
                <div class="loading-state" role="status" aria-live="polite">
                    <span class="loading-spinner" aria-hidden="true"></span>
                    <span class="sr-only">"Loading\u{2026}"</span>
                </div>
            </div>
            <script type="module" src="/admin/assets/csr_bootstrap.js?v=20260730-36"></script>
        },
    )
}
