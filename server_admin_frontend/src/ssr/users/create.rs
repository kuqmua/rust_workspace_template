#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the SSR user-creation view is composed once by the users screen"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

pub(super) fn admin_create_user(
    can_create: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView {
    bool::from(can_create).then(|| leptos::view! {
        <details class="mutation-form">
            <summary>"Create user"</summary>
            <form method="post" action=server_admin_contract::AdminHtmlAction::UserCreate.get()>
                <label><span>"Login"</span><input name="login" required /></label>
                <label><span>"Display name"</span><input name="display_name" required /></label>
                <label><span>"Password"</span><input name="password" type="password" required /></label>
                <button type="submit">"Create user"</button>
            </form>
        </details>
    })
}
