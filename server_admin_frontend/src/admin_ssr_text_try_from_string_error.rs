#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum AdminSsrTextTryFromStringError {
    #[error("{message}", message = constants_str::ADMIN_SSR_TITLE_TOO_LONG)]
    TooLarge,
}
