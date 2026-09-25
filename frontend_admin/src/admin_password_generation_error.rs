#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminPasswordGenerationError {
    #[error("{message}", message = constants_str::ADMIN_UI_PASSWORD_GENERATION_FAILED)]
    BrowserUnavailable,
    #[error("{message}", message = constants_str::ADMIN_UI_PASSWORD_GENERATION_FAILED)]
    Randomness(
        crate::wasm_bindgen_password_generation_exception::WasmBindgenPasswordGenerationException,
    ),
    #[error("{message}", message = constants_str::ADMIN_UI_PASSWORD_GENERATION_FAILED)]
    Policy(server_admin_contract::admin_new_password::AdminNewPasswordTryFromStringError),
}
