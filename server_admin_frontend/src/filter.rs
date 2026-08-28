#![allow(
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "the Leptos column-filter converts borrowed query values and is composed once by its column"
)]

pub(super) use crate::admin_data_grid_filter::admin_data_grid_filter;
use crate::leptos_admin_filter_operation_signal::LeptosAdminFilterOperationSignal;

// Root-owned module compatibility wrappers.
pub(crate) mod admin_data_grid_filter_option {
    pub use crate::admin_data_grid_filter_option::*;
}
pub(crate) mod admin_data_grid_input_type {
    pub use crate::admin_data_grid_input_type::*;
}
pub(crate) mod admin_data_grid_filter {
    pub use crate::admin_data_grid_filter::*;
}
pub(crate) mod leptos_admin_filter_operation_signal {
    pub use crate::leptos_admin_filter_operation_signal::*;
}
