#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[cfg(test)]
mod tests {
    #[test]
    fn type_contract_preserves_input_metadata() {
        let contract = super::TypeContract::new(
            super::InputKind::Number,
            super::ValueFormat::Int64,
            super::Nullability::NonNullable,
        );
        assert_eq!(contract.input_kind(), super::InputKind::Number);
        assert_eq!(contract.format(), super::ValueFormat::Int64);
    }
}
#[path = "field_contract/contract_str.rs"]
mod contract_str;
pub use contract_str::*;
#[path = "field_contract/input_kind.rs"]
mod input_kind;
pub use input_kind::*;
#[path = "field_contract/value_format.rs"]
mod value_format;
pub use value_format::*;
#[path = "field_contract/nullability.rs"]
mod nullability;
pub use nullability::*;
#[path = "field_contract/capability_support.rs"]
mod capability_support;
pub use capability_support::*;
#[path = "field_contract/filter_operation.rs"]
mod filter_operation;
pub use filter_operation::*;
#[path = "field_contract/filter_value_shape.rs"]
mod filter_value_shape;
pub use filter_value_shape::*;
#[path = "field_contract/filter_contracts.rs"]
mod filter_contracts;
pub use filter_contracts::*;
#[path = "field_contract/has_filter_contracts.rs"]
mod has_filter_contracts;
pub use has_filter_contracts::*;
#[path = "field_contract/input_step.rs"]
mod input_step;
pub use input_step::*;
#[path = "field_contract/numeric_bound.rs"]
mod numeric_bound;
pub use numeric_bound::*;
#[path = "field_contract/contract_i64.rs"]
mod contract_i64;
pub use contract_i64::*;
#[path = "field_contract/value_example.rs"]
mod value_example;
pub use value_example::*;
#[path = "field_contract/type_contract.rs"]
mod type_contract;
pub use type_contract::*;
#[path = "field_contract/has_type_contract.rs"]
mod has_type_contract;
pub use has_type_contract::*;
#[path = "field_contract/form_value.rs"]
mod form_value;
pub use form_value::*;
#[path = "field_contract/form_value_ref.rs"]
mod form_value_ref;
pub use form_value_ref::*;
#[path = "field_contract/form_field_name_ref.rs"]
mod form_field_name_ref;
pub use form_field_name_ref::*;
#[path = "field_contract/form_value_error.rs"]
mod form_value_error;
pub use form_value_error::*;
#[path = "field_contract/filter_wire_json.rs"]
mod filter_wire_json;
pub use filter_wire_json::*;
#[path = "field_contract/form_value_contract.rs"]
mod form_value_contract;
pub use form_value_contract::*;
#[path = "field_contract/filter_form_value_contract.rs"]
mod filter_form_value_contract;
pub use filter_form_value_contract::*;
#[path = "field_contract/form_field_error.rs"]
mod form_field_error;
pub use form_field_error::*;
#[path = "field_contract/field_capability.rs"]
mod field_capability;
pub use field_capability::*;
#[path = "field_contract/primary_key_kind.rs"]
mod primary_key_kind;
pub use primary_key_kind::*;
#[path = "field_contract/field_order.rs"]
mod field_order;
pub use field_order::*;
#[path = "field_contract/field_visibility.rs"]
mod field_visibility;
pub use field_visibility::*;
#[path = "field_contract/field_placeholder.rs"]
mod field_placeholder;
pub use field_placeholder::*;
#[path = "field_contract/field_contract.rs"]
mod field_contract;
pub use field_contract::*;
#[path = "field_contract/field_contracts.rs"]
mod field_contracts;
pub use field_contracts::*;
#[path = "field_contract/empty_filter_contracts.rs"]
mod empty_filter_contracts;
use empty_filter_contracts::EMPTY_FILTER_CONTRACTS;
