#![allow(
    clippy::arbitrary_source_item_ordering,
    dead_code,
    clippy::field_scoped_visibility_modifiers,
    clippy::impl_trait_in_params,
    clippy::missing_const_for_fn,
    clippy::multiple_inherent_impl,
    clippy::needless_pass_by_value,
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    unreachable_pub,
    reason = "Leptos component macro expansion generates builders, fields, and bindings with framework-defined shapes"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{AddAnyAttr};

#[leptos::component]
pub(crate) fn TableWrapper(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::ScrollArea attr:data-name="TableWrapper" class="table-scroll max-h-96 overflow-auto rounded-md border">{children()}</singlestage::ScrollArea> }
}

#[leptos::component]
pub(crate) fn Table(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::Table attr:data-name="Table" attr:class="w-full max-w-7xl text-sm caption-bottom">{children()}</singlestage::Table> }
}

#[leptos::component]
pub(crate) fn TableCaption(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::TableCaption attr:data-name="TableCaption" attr:class="mt-4 text-sm text-muted-foreground">{children()}</singlestage::TableCaption> }
}

#[leptos::component]
pub(crate) fn TableHeader(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::TableHeader attr:data-name="TableHeader" attr:class="[&_tr]:border-b sticky top-0 z-10 bg-card">{children()}</singlestage::TableHeader> }
}

#[leptos::component]
pub(crate) fn TableBody(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::TableBody attr:data-name="TableBody" attr:class="[&_tr:last-child]:border-0">{children()}</singlestage::TableBody> }
}

#[leptos::component]
pub(crate) fn TableFooter(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::TableFooter attr:data-name="TableFooter" attr:class="font-medium border border-t bg-muted/50 [&>tr]:last:border-b-0">{children()}</singlestage::TableFooter> }
}

#[leptos::component]
pub(crate) fn TableRow(children: leptos::prelude::Children) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::TableRow attr:data-name="TableRow" attr:class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50">{children()}</singlestage::TableRow> }
}

#[leptos::component]
pub(crate) fn TableHead(
    #[prop(optional)] data_field: Option<String>,
    #[prop(optional)] data_filter_count: Option<String>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    leptos::view! { <singlestage::TableHead attr:data-name="TableHead" attr:data-field=data_field attr:data-filter-count=data_filter_count attr:class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">{children()}</singlestage::TableHead> }
}

#[leptos::component]
pub(crate) fn TableCell(
    #[prop(optional, into)] data_label: Option<std::borrow::Cow<'static, str>>,
    #[prop(optional, into)] data_field: Option<std::borrow::Cow<'static, str>>,
    #[prop(optional)] class: Option<&'static str>,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    let class = class.map_or_else(
        || {
            std::borrow::Cow::Borrowed(
                constants_str::VALUE_19AB4EBD,
            )
        },
        |class| {
            std::borrow::Cow::Owned(format!(
                "p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3 {class}"
            ))
        },
    );
    leptos::view! { <singlestage::TableCell attr:data-name="TableCell" attr:data-label=data_label attr:data-field=data_field attr:class=class>{children()}</singlestage::TableCell> }
}
