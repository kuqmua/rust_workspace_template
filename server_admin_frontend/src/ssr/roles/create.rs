#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR role-creation view is composed once by the roles screen"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

pub(super) fn admin_create_role(
    can_create: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView {
    bool::from(can_create).then(|| {
        leptos::view! {
            <details class="mutation-form">
                <summary>"Create role"</summary>
                <form method="post" action=server_admin_contract::AdminHtmlAction::RoleCreate.get()>
                    <label><span>"Name"</span><input name="name" required /></label>
                    <button type="submit">"Create role"</button>
                </form>
            </details>
        }
    })
}
