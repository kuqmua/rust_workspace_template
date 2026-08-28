#![allow(
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the Leptos column-filter converts borrowed query values and is composed once by its column"
)]

#[path = "admin_data_grid_filter_option.rs"]
mod admin_data_grid_filter_option;
#[path = "admin_data_grid_input_type.rs"]
mod admin_data_grid_input_type;

#[path = "admin_data_grid_filter.rs"]
mod admin_data_grid_filter;
#[path = "leptos_admin_filter_operation_signal.rs"]
mod leptos_admin_filter_operation_signal;

pub(super) use admin_data_grid_filter::admin_data_grid_filter;
use leptos_admin_filter_operation_signal::LeptosAdminFilterOperationSignal;
