#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AdminAuditDownloadState {
    Idle,
    Loading,
    Ready(crate::admin_audit_download_url::AdminAuditDownloadUrl),
    RequestFailed(crate::admin_table_load_error::AdminTableLoadError),
    EncodingFailed(crate::admin_audit_download_url::AdminAuditDownloadUrlTryFromStringError),
}
