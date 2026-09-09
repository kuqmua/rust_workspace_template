#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

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
pub(crate) fn AdminHealthProbe(
    admin_route: server_admin_contract::admin_route::AdminRoute,
) -> impl leptos::prelude::IntoView {
    #[wasm_bindgen::prelude::wasm_bindgen(module = "/static/health_probe.js")]
    extern "C" {
        #[wasm_bindgen::prelude::wasm_bindgen(catch, js_name = fetchHealthText)]
        async fn fetch_health_text(url: &str) -> Result<String, wasm_bindgen::JsValue>;
    }
    let state = leptos::prelude::RwSignal::new(None);
    wasm_bindgen_futures::spawn_local(async move {
        let result = fetch_health_text(admin_route.path().as_ref())
            .await
            .map_err(crate::admin_health_wasm_bindgen_error::AdminHealthWasmBindgenError::from);
        leptos::prelude::Set::set(&state, Some(result));
    });
    leptos::view! {
        <div class="health-label">{admin_route.path().to_string()}</div>
        <div class="health-result">{move || match leptos::prelude::Get::get(&state) {
            None => constants_str::ADMIN_UI_LOADING.to_owned(),
            Some(Ok(text)) => text,
            Some(Err(error)) => error.to_string(),
        }}</div>
    }
}
