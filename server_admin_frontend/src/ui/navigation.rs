#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::field_scoped_visibility_modifiers,
    clippy::impl_trait_in_params,
    clippy::missing_const_for_fn,
    clippy::multiple_inherent_impl,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    unused_imports,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

#[allow(unused_import_braces, reason = "grouped Leptos prelude imports are required by workspace source policy")]
#[rustfmt::skip]
use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition across frontend modules"
)]
pub(crate) fn AdminNavigationLink(
    href: String,
    active: bool,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <singlestage::Link
            attr:data-name="NavigationMenuLink"
            class=if active { "active inline-flex items-center rounded-sm text-sm font-medium text-foreground transition-colors focus:outline-none" } else { "inline-flex items-center rounded-sm text-sm font-medium text-foreground/70 transition-colors hover:text-foreground focus:outline-none" }
            attr:aria-current=active.then_some("page")
            href=href
        >
            {children()}
        </singlestage::Link>
    }
}

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for navigation composition"
)]
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

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for navigation composition"
)]
pub(crate) fn AdminSidebarItem(
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <li>{children()}</li> }
}
