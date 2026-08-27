#![allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module parses and consumes these domain models

#[path = "domain_types/syn_expr.rs"]
mod syn_expr;
pub(crate) use syn_expr::*;
#[path = "domain_types/syn_type.rs"]
mod syn_type;
pub(crate) use syn_type::*;
#[path = "domain_types/syn_ident.rs"]
mod syn_ident;
pub(crate) use syn_ident::*;
#[path = "domain_types/std_bool.rs"]
mod std_bool;
pub(crate) use std_bool::*;
#[path = "domain_types/syn_attributes_ref.rs"]
mod syn_attributes_ref;
pub(crate) use syn_attributes_ref::*;
#[path = "domain_types/contract_struct_api_args.rs"]
mod contract_struct_api_args;
pub(crate) use contract_struct_api_args::*;
#[path = "domain_types/contract_struct_api_field_args.rs"]
mod contract_struct_api_field_args;
pub(crate) use contract_struct_api_field_args::*;
#[path = "domain_types/route_catalog_args.rs"]
mod route_catalog_args;
pub(crate) use route_catalog_args::*;
#[path = "domain_types/route_catalog_route_args.rs"]
mod route_catalog_route_args;
pub(crate) use route_catalog_route_args::*;
#[path = "domain_types/page_catalog_args.rs"]
mod page_catalog_args;
pub(crate) use page_catalog_args::*;
#[path = "domain_types/page_catalog_page_args.rs"]
mod page_catalog_page_args;
pub(crate) use page_catalog_page_args::*;
#[path = "domain_types/typed_route_args.rs"]
mod typed_route_args;
pub(crate) use typed_route_args::*;
#[path = "domain_types/syn_typed_route_errors.rs"]
mod syn_typed_route_errors;
pub(crate) use syn_typed_route_errors::*;
#[path = "domain_types/route_registry_binding.rs"]
mod route_registry_binding;
pub(crate) use route_registry_binding::*;
#[path = "domain_types/syn_route_registry_endpoint.rs"]
mod syn_route_registry_endpoint;
pub(crate) use syn_route_registry_endpoint::*;
#[path = "domain_types/syn_route_registry_route.rs"]
mod syn_route_registry_route;
pub(crate) use syn_route_registry_route::*;
#[path = "domain_types/syn_route_registry_bindings.rs"]
mod syn_route_registry_bindings;
pub(crate) use syn_route_registry_bindings::*;
#[path = "domain_types/syn_route_registry_schemas.rs"]
mod syn_route_registry_schemas;
pub(crate) use syn_route_registry_schemas::*;
#[path = "domain_types/syn_route_registry_state.rs"]
mod syn_route_registry_state;
pub(crate) use syn_route_registry_state::*;
#[path = "domain_types/syn_route_registry_family.rs"]
mod syn_route_registry_family;
pub(crate) use syn_route_registry_family::*;
#[path = "domain_types/endpoint_registry_binding.rs"]
mod endpoint_registry_binding;
pub(crate) use endpoint_registry_binding::*;
#[path = "domain_types/syn_endpoint_registry_contract.rs"]
mod syn_endpoint_registry_contract;
pub(crate) use syn_endpoint_registry_contract::*;
#[path = "domain_types/syn_endpoint_registry_endpoint.rs"]
mod syn_endpoint_registry_endpoint;
pub(crate) use syn_endpoint_registry_endpoint::*;
#[path = "domain_types/syn_endpoint_registry_bindings.rs"]
mod syn_endpoint_registry_bindings;
pub(crate) use syn_endpoint_registry_bindings::*;
#[path = "domain_types/syn_endpoint_registry_state.rs"]
mod syn_endpoint_registry_state;
pub(crate) use syn_endpoint_registry_state::*;
#[path = "domain_types/endpoint_registry_args.rs"]
mod endpoint_registry_args;
pub(crate) use endpoint_registry_args::*;
#[path = "domain_types/route_registry_args.rs"]
mod route_registry_args;
pub(crate) use route_registry_args::*;
