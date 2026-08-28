#[cfg(test)]
mod advanced_policy;
#[cfg(test)]
mod cargo_policy;
#[cfg(test)]
mod ci_policy;
#[cfg(test)]
mod code_style;
#[cfg(test)]
mod contract_source_policy;
#[cfg(test)]
mod deployment_policy;
#[cfg(test)]
mod domain_analysis;
#[cfg(test)]
mod domain_type_policy;
#[cfg(test)]
mod lint_sync;
#[cfg(test)]
mod module_policy;
#[cfg(test)]
mod reuse_policy;
#[cfg(test)]
mod route_contract_policy;
#[cfg(test)]
mod runtime_analysis;
#[cfg(test)]
mod runtime_policy;
#[cfg(test)]
mod secret_policy;
#[cfg(test)]
mod snapshot;
#[cfg(test)]
mod source_analysis;
#[cfg(test)]
mod source_policy;
#[cfg(test)]
mod types;

#[cfg(test)]
pub(crate) use code_style::*;
