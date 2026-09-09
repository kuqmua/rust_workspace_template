#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, thiserror::Error)]
pub(crate) enum AdminTableLoadError {
    #[error("{}", constants_str::ADMIN_UI_THE_TABLE_REQUEST_FAILED)]
    ReadBrowser(#[from] crate::wasm_bindgen_admin_read_error::WasmBindgenAdminReadError),
    #[error("{}", constants_str::ADMIN_UI_THE_TABLE_RESPONSE_WAS_INVALID)]
    ReadJson(#[from] crate::std_rc_serde_json_error::StdRcSerdeJsonError),
    #[error("{}", constants_str::ADMIN_UI_THE_TABLE_QUERY_IS_INVALID)]
    ReadUtf8(#[from] crate::std_str_utf8_error::StdStrUtf8Error),
    #[error("{}", constants_str::ADMIN_UI_THE_TABLE_QUERY_IS_INVALID)]
    ReadSort(#[from] server_admin_contract::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError),
    #[error("{}", constants_str::ADMIN_UI_THE_TABLE_QUERY_IS_INVALID)]
    ReadBody(#[from] frontend_contract::frontend_contract_body_error::FrontendContractBodyError),
    #[error("{}", constants_str::ADMIN_UI_THE_TABLE_QUERY_IS_INVALID)]
    ReadPath(frontend_contract::transport_path::TransportPathTryFromStringError),
    #[error("{}", constants_str::ADMIN_UI_AUTHENTICATION_REQUIRED)]
    MissingCsrf,
    #[error("{message}", message = constants_str::ADMIN_UI_THE_TABLE_REQUEST_FAILED)]
    Fetch,
    #[error("{message}_{0}_{connector}_{1}", message = constants_str::ADMIN_UI_THE_SERVER_RETURNED_STATUS, connector = constants_str::ADMIN_UI_FOR)]
    Http(
        crate::admin_http_status::AdminHttpStatus,
        crate::admin_csr_api_url::AdminCsrApiUrl,
    ),
    #[error("{message}", message = constants_str::ADMIN_UI_THE_TABLE_QUERY_IS_INVALID)]
    Query,
    #[error("{message}", message = constants_str::ADMIN_UI_THE_TABLE_RESPONSE_WAS_INVALID)]
    Response,
}

impl AdminTableLoadError {
    pub(crate) fn requires_session_refresh(&self) -> server_admin_contract::admin_bool::AdminBool {
        server_admin_contract::admin_bool::AdminBool::from(match self {
            Self::MissingCsrf => true,
            Self::Http(status, _) => {
                *status == crate::admin_http_status::AdminHttpStatus::from(401u16)
            }
            Self::Fetch
            | Self::Query
            | Self::Response
            | Self::ReadBrowser(_)
            | Self::ReadJson(_)
            | Self::ReadUtf8(_)
            | Self::ReadSort(_)
            | Self::ReadBody(_)
            | Self::ReadPath(_) => false,
        })
    }
}
