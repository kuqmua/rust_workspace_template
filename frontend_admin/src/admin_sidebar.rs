#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "Leptos component expansion models props as generated fields even though the source declares functions"
)]
#![allow(
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    unused_imports,
    reason = "Leptos component macro expansion generates framework-defined shapes"
)]

#[allow(unused_import_braces, reason = "grouped Leptos prelude imports are required by workspace source policy")]
#[rustfmt::skip]
use leptos::prelude::{AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for navigation composition"
)]
#[allow(
    clippy::single_call_fn,
    reason = "admin sidebar remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn AdminSidebar(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <div class="nav-menu">
            <label class="nav-menu-toggle">
                <input type="checkbox" />
                <span>{constants_str::ADMIN_BUTTON_NAVIGATION}</span>
            </label>
            <nav aria-label=constants_str::ADMIN_UI_ADMIN_SECTIONS data-name="NavigationMenu">
                <ul class="nav-menu-list">{children()}</ul>
            </nav>
        </div>
    }
}
