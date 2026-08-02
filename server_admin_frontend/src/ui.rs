pub(crate) mod alert;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod alert_dialog;
pub(crate) mod badge;
pub(crate) mod button;
pub(crate) mod card;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod checkbox;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) mod empty;
pub(crate) mod field;
pub(crate) mod input;
pub(crate) mod navigation;
pub(crate) mod spinner;
pub(crate) mod textarea;

#[cfg(test)]
mod tests;
