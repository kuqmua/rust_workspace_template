#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos component expansion generates framework-defined props fields and builder methods"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
};

#[leptos::component]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own the typed route so generated component factories can move it into view closures"
)]
#[allow(
    clippy::single_call_fn,
    reason = "the read action is a shared Leptos component invoked through view macro expansion in table and session actions"
)]
pub(crate) fn AdminReadAction(
    read_path: server_admin_contract::admin_route_path::AdminRoutePath,
    children: leptos::prelude::Children,
) -> impl leptos::prelude::IntoView {
    let dialog_id = format!("read-{read_path}");
    leptos::view! {
        <crate::admin_table_action_trigger::AdminTableActionTrigger label=constants_str::PG_CRUD_READ_RULE_ACTION dialog_id=dialog_id.clone()>
            <svg viewBox="0 0 24 24" aria-hidden=constants_str::TRUE fill="currentColor">
                <path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5M12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5m0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3"></path>
            </svg>
        </crate::admin_table_action_trigger::AdminTableActionTrigger>
        <dialog id=dialog_id class="table-cell-dialog" aria-label=constants_str::PG_CRUD_READ_RULE_ACTION>
            <h2>{constants_str::PG_CRUD_READ_RULE_ACTION}</h2>
            <div class="table-cell-content admin-read-content"><div class="profile-grid profile-fields">{children()}</div></div>
            <button type=crate::admin_button_kind::AdminButtonKind::Button.value() commandfor=dialog_id.clone() command=constants_str::ADMIN_BUTTON_CLOSE>{constants_str::ADMIN_BUTTON_CLOSE}</button>
        </dialog>
    }
}
