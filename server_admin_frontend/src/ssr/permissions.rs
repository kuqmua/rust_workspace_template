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
pub(super) fn render_permissions(
    page: &server_admin_contract::AdminPermissionsPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let content = leptos::view! {
        <section class="table-page">
        {crate::shared::table_filters::form::admin_table_filters(server_admin_contract::AdminFrontendPath::Permissions, query.search(), query.sort(), crate::shared::table_filters::form::AdminTableFilterDirection::from(query.direction()), query.limit(), &server_admin_contract::AdminTableSortField::PERMISSION, crate::shared::table_filters::form::AdminTableFilterPresentation::Ssr)}
        <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"permission"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| leptos::view! {
            <tr><td data-label="id">{item.id().to_string()}</td><td data-label="permission">{item.name().to_string()}</td></tr>
        }).collect::<Vec<_>>()}</tbody></table></div>
        {super::table_pagination(server_admin_contract::AdminPage::Permissions, query, page.total(), None, None)}
        </section>
    }
    .render_admin_ssr();
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Permissions,
        content,
        Some(admin),
        Some(branding),
    )
}
