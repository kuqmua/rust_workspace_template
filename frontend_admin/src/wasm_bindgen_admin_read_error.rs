#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    Clone,
    Debug,
    thiserror::Error,
)]
#[error("{}", constants_str::REQUEST_FAILED)]
pub(crate) struct WasmBindgenAdminReadError(wasm_bindgen::JsValue);
