#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum AdminPaginationQuery<'query> {
    #[cfg(target_arch = "wasm32")]
    Csr {
        admin_csr_query: &'query crate::admin_csr_query::AdminCsrQuery,
        admin_data_table: server_admin_contract::admin_data_table::AdminDataTable,
        admin_frontend_path: Option<server_admin_contract::admin_frontend_path::AdminFrontendPath>,
    },
    #[cfg(not(target_arch = "wasm32"))]
    Ssr {
        admin_table_query: &'query server_admin_contract::admin_table_query::AdminTableQuery,
        admin_data_table_filter_query: Option<
            &'query server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery,
        >,
    },
}

impl AdminPaginationQuery<'_> {
    pub(crate) fn hidden_inputs(
        &self,
        admin_page_limit: Option<server_admin_contract::admin_page_limit::AdminPageLimit>,
    ) -> impl leptos::prelude::IntoView + use<> {
        let limit = admin_page_limit.map(|value| leptos::view! { <input type="hidden" name="limit" value=u16::from(value).to_string() /> });
        match self {
            #[cfg(target_arch = "wasm32")]
            Self::Csr {
                admin_csr_query,
                admin_data_table,
                admin_frontend_path,
            } => {
                let query = (*admin_data_table
                    == server_admin_contract::admin_data_table::AdminDataTable::Roles
                    || *admin_frontend_path
                        == Some(
                            server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions,
                        ))
                .then(|| {
                    crate::admin_table_query_hidden_inputs::admin_table_query_hidden_inputs(
                        admin_csr_query.search(),
                        admin_csr_query.sort(),
                        &crate::admin_table_query_direction::AdminTableQueryDirection::Csr(
                            admin_csr_query.direction().cloned(),
                        ),
                    )
                });
                let supports_filters = bool::from(admin_data_table.supports_filters());
                let filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
                    supports_filters
                        .then_some(admin_csr_query.filter_field())
                        .flatten(),
                    supports_filters
                        .then_some(admin_csr_query.filter_operation())
                        .flatten(),
                    supports_filters
                        .then_some(admin_csr_query.filter_value())
                        .flatten(),
                    supports_filters
                        .then_some(admin_csr_query.filter_end())
                        .flatten(),
                );
                leptos::view! { {query}{limit}{filter} }
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::Ssr {
                admin_table_query,
                admin_data_table_filter_query,
            } => {
                let query = crate::admin_table_query_hidden_inputs::admin_table_query_hidden_inputs(
                    admin_table_query.search(),
                    admin_table_query.sort(),
                    &crate::admin_table_query_direction::AdminTableQueryDirection::Ssr(
                        admin_table_query.direction(),
                    ),
                );
                let operation = admin_data_table_filter_query.and_then(server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::operation).map(server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from);
                let filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
                    admin_data_table_filter_query.and_then(server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::field),
                    operation.as_ref(),
                    admin_data_table_filter_query.and_then(server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::value),
                    admin_data_table_filter_query.and_then(server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::end),
                );
                leptos::view! { {query}{filter}{limit} }
            }
        }
    }

    pub(crate) const fn limit(&self) -> server_admin_contract::admin_page_limit::AdminPageLimit {
        match self {
            #[cfg(target_arch = "wasm32")]
            Self::Csr {
                admin_csr_query, ..
            } => admin_csr_query.limit(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Ssr {
                admin_table_query, ..
            } => admin_table_query.limit(),
        }
    }

    pub(crate) const fn offset(&self) -> server_admin_contract::admin_page_offset::AdminPageOffset {
        match self {
            #[cfg(target_arch = "wasm32")]
            Self::Csr {
                admin_csr_query, ..
            } => admin_csr_query.offset(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Ssr {
                admin_table_query, ..
            } => admin_table_query.offset(),
        }
    }
}
