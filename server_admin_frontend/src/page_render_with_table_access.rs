#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the administrator page shell requires its local set of document attribute traits"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

pub(super) fn page_render_with_table_access(
    admin_page: server_admin_contract::admin_page::AdminPage,
    admin_ssr_html: crate::admin_ssr_html::AdminSsrHtml,
    admin: Option<&server_admin_contract::authenticated_admin::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::admin_branding_view::AdminBrandingView>,
    active_table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let spec = admin_page.spec();
    let title = spec.title();
    let document_title = branding
        .and_then(server_admin_contract::admin_branding_view::AdminBrandingView::tab_title)
        .map_or_else(
            || title.as_ref().to_owned(),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::admin_branding_view::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    let navigation = {
        let tables = server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER
            .into_iter()
            .filter(|table| {
                admin.is_none_or(|value| {
                    bool::from(value.has_permission(
                        server_admin_contract::admin_permission::AdminPermission::TablesRead,
                    )) && bool::from(value.has_permission(table.permission()))
                })
            })
            .map(|table| {
                let name = table.to_string();
                let href = table.frontend_path().to_string();
                leptos::view! {
                    <crate::admin_sidebar_item::AdminSidebarItem><crate::admin_navigation_link::AdminNavigationLink bool=active_table == Some(table) string=href>{name}</crate::admin_navigation_link::AdminNavigationLink></crate::admin_sidebar_item::AdminSidebarItem>
                }
            })
            .collect::<Vec<_>>();
        let pages = server_admin_contract::admin_page::AdminPage::navigation()
            .filter(|item_page| {
                admin.is_none_or(|value| bool::from(value.can_access(*item_page)))
            })
            .map(|item_page| {
                let item = item_page.spec();
                let href = String::from(item.path());
                let label = item.route_name().as_ref().to_owned();
                leptos::view! {
                    <crate::admin_sidebar_item::AdminSidebarItem><crate::admin_navigation_link::AdminNavigationLink bool=item_page == admin_page string=href>{label}</crate::admin_navigation_link::AdminNavigationLink></crate::admin_sidebar_item::AdminSidebarItem>
                }
            })
            .collect::<Vec<_>>();
        leptos::view! {
            <crate::admin_sidebar::AdminSidebar>
                {tables}
                {pages}
                <crate::admin_sidebar_item::AdminSidebarItem><form method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.get()><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>{server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</crate::admin_button::AdminButton></form></crate::admin_sidebar_item::AdminSidebarItem>
            </crate::admin_sidebar::AdminSidebar>
        }
    };
    crate::render_document::render_document(
        &crate::admin_ssr_text::AdminSsrText::try_from(document_title)
            .unwrap_or_else(crate::admin_ssr_text::AdminSsrText::from),
        leptos::view! {
            <div class="app-shell" style=primary_color>
                <header class="topbar">
                    {navigation}
                </header>
                <main class="main-content"><div class="page-frame"><crate::admin_alert::AdminAlert admin_alert_variant=crate::admin_alert_variant::AdminAlertVariant::Success option="saved">"Changes saved."</crate::admin_alert::AdminAlert><div inner_html=String::from(admin_ssr_html)></div></div></main>
            </div>
        },
    )
}
