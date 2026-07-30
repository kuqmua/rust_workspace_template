use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent roles module"
)]
pub(super) fn AdminCreateRole(
    can_create: server_admin_contract::AdminBool,
) -> impl leptos::prelude::IntoView {
    let create_name = leptos::prelude::RwSignal::new(String::new());
    bool::from(can_create).then(|| leptos::view! {
        <form class="mutation-form" on:submit=move |event| {
            event.prevent_default();
            if let (Ok(name), Ok(path)) = (
                server_admin_contract::AdminRoleName::try_from(leptos::prelude::Get::get(&create_name)),
                super::super::http::admin_api_url(server_admin_contract::AdminRoute::CreateRole),
            ) {
                super::super::mutation::reload_after(
                    super::super::mutation::AdminMutationMethod::Post,
                    path,
                    server_admin_contract::AdminCreateRoleReq::new(name),
                );
            }
        }>
            <input placeholder="Role name" required on:input=move |event| leptos::prelude::Set::set(&create_name, leptos::prelude::event_target_value(&event)) />
            <button type="submit">"Create role"</button>
        </form>
    })
}
