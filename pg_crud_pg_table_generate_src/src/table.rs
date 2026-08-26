#![allow(clippy::field_scoped_visibility_modifiers)] // sibling emitters read the private descriptor directly while it remains hidden outside the generator
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct GeneratePgTableFieldCount(usize);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct GeneratePgTableModel {
    pub(super) field_count: GeneratePgTableFieldCount,
    pub(super) input: SynGeneratePgTableModelInput,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(super) struct SynGeneratePgTableModelInput(syn::DeriveInput);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(super) struct SynGeneratePgTableModelError(syn::Error);
impl GeneratePgTableModel {
    #[must_use]
    pub const fn field_count(&self) -> GeneratePgTableFieldCount {
        self.field_count
    }
    pub(super) fn into_input(self) -> SynGeneratePgTableModelInput {
        self.input
    }
    pub(super) fn validate(self) -> Result<Self, SynGeneratePgTableModelError> {
        if self.field_count.0 == constants_usize::ZERO {
            Err(syn::Error::new_spanned(
                &self.input.0.ident,
                constants_str::GENERATE_PG_TABLE_REQUIRES_FIELD,
            )
            .into())
        } else {
            Ok(self)
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) struct OperationDsc<
    Capability,
    HttpMethod,
    Operation,
    OperationKind,
    PermissionAction,
    StatusCode,
> {
    pub(super) http_method: HttpMethod,
    pub(super) idempotency_capable: Capability,
    pub(super) operation: Operation,
    pub(super) operation_kind: OperationKind,
    pub(super) optimistic_concurrency_capable: Capability,
    pub(super) permission_action: PermissionAction,
    pub(super) success_status_code: StatusCode,
}
#[cfg(test)]
mod tests {
    #[test]
    fn operation_descriptor_keeps_transport_permission_and_capabilities_together() {
        let spec = super::OperationDsc {
            http_method: constants_str::PATCH,
            idempotency_capable: true,
            operation: constants_str::UO,
            operation_kind: constants_str::UPDATE_ONE,
            optimistic_concurrency_capable: true,
            permission_action: constants_str::PG_CRUD_UPDATE_PERMISSION_ACTION,
            success_status_code: 200u16,
        };
        assert_eq!(spec.http_method, "PATCH");
        assert!(spec.idempotency_capable);
        assert_eq!(spec.operation, "uo");
        assert_eq!(spec.operation_kind, "update_one");
        assert!(spec.optimistic_concurrency_capable);
        assert_eq!(spec.permission_action, "update");
        assert_eq!(spec.success_status_code, 200u16);
    }
}
