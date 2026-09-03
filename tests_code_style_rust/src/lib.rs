#![allow(
    unused_crate_dependencies,
    reason = "proc_macro_frontend_contract is a dependency of the trybuild fixture crates"
)]

#[cfg(test)]
pub mod code_style;
#[cfg(test)]
pub mod domain_analysis;
#[cfg(test)]
pub mod runtime_analysis;
#[cfg(test)]
pub mod source_analysis;
#[cfg(test)]
pub mod test_code_style_advanced_policy;
#[cfg(test)]
pub mod test_code_style_cargo_policy;
#[cfg(test)]
pub mod test_code_style_ci_policy;
#[cfg(test)]
pub mod test_code_style_contract_source_policy;
#[cfg(test)]
pub mod test_code_style_deployment_policy;
#[cfg(test)]
pub mod test_code_style_domain_type_policy;
#[cfg(test)]
pub mod test_code_style_lint_sync;
#[cfg(test)]
pub mod test_code_style_module_policy;
#[cfg(test)]
pub mod test_code_style_reuse_policy;
#[cfg(test)]
pub mod test_code_style_route_contract_policy;
#[cfg(test)]
pub mod test_code_style_runtime_policy;
#[cfg(test)]
pub mod test_code_style_secret_policy;
#[cfg(test)]
pub mod test_code_style_snapshot;
#[cfg(test)]
pub mod test_code_style_source_policy;
#[cfg(test)]
pub mod types;
