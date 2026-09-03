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
use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for navigation composition"
)]
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn AdminSidebar(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <div class="nav-menu">
            <label class="nav-menu-toggle">
                <input type="checkbox" />
                <span>"Navigation"</span>
            </label>
            <nav data-name="NavigationMenu" class="relative z-10 max-w-max flex-1 items-center justify-center">
                <ul class="nav-menu-list">{children()}</ul>
            </nav>
        </div>
    }
}
