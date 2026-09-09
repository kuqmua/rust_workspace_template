#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
#[allow(
    clippy::single_call_fn,
    reason = "Leptos requires a named component to generate the props factory used by the page view"
)]
pub(crate) fn AdminBrandingDetails(
    admin_branding_view: server_admin_contract::admin_branding_view::AdminBrandingView,
) -> impl leptos::prelude::IntoView {
    let site_name = admin_branding_view.site_name().as_ref().to_owned();
    let tab_title = admin_branding_view
        .tab_title()
        .map(|value| value.as_ref().to_owned())
        .unwrap_or_default();
    let main_logo = admin_branding_view
        .main_logo()
        .map(|value| value.as_ref().to_owned())
        .unwrap_or_default();
    let primary_color = admin_branding_view
        .primary_color()
        .map(|value| value.as_ref().to_owned())
        .unwrap_or_default();
    let support_url = admin_branding_view
        .support_url()
        .map(|value| value.as_ref().to_owned())
        .unwrap_or_default();
    let default_admin_route = admin_branding_view
        .default_admin_route()
        .as_ref()
        .to_owned();
    leptos::view! {
        <section class="profile-grid profile-fields" data-renderer="csr">
            <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_SITE_NAME><span>{site_name}</span></crate::admin_field::AdminField>
            <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_TAB_TITLE><span>{tab_title}</span></crate::admin_field::AdminField>
            <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_MAIN_LOGO_URL><span>{main_logo}</span></crate::admin_field::AdminField>
            <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_PRIMARY_COLOR><span>{primary_color}</span></crate::admin_field::AdminField>
            <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_SUPPORT_URL><span>{support_url}</span></crate::admin_field::AdminField>
            <crate::admin_field::AdminField admin_field_label=constants_str::VALUE_ACD40F02><span>{default_admin_route}</span></crate::admin_field::AdminField>
        </section>
    }
}
