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
    field_count: GeneratePgTableFieldCount,
    input: SynGeneratePgTableModelInput,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
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
    #[allow(clippy::single_call_fn)] // construction is isolated as the typed build-stage boundary
    pub(super) fn from_struct(input: SynGeneratePgTableModelInput) -> Self {
        let field_count = match &input.0.data {
            syn::Data::Struct(data) => data.fields.iter().count(),
            syn::Data::Enum(_) | syn::Data::Union(_) => 0usize,
        };
        Self {
            field_count: GeneratePgTableFieldCount::from(field_count),
            input,
        }
    }
    pub(super) fn into_input(self) -> SynGeneratePgTableModelInput {
        self.input
    }
    pub(super) fn validate(self) -> Result<Self, SynGeneratePgTableModelError> {
        if self.field_count.0 == 0usize {
            Err(syn::Error::new_spanned(
                &self.input.0.ident,
                str_constants::GENERATE_PG_TABLE_REQUIRES_FIELD,
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
            http_method: str_constants::PATCH,
            idempotency_capable: true,
            operation: str_constants::UO,
            operation_kind: str_constants::UPDATE_ONE,
            optimistic_concurrency_capable: true,
            permission_action: str_constants::PG_CRUD_UPDATE_PERMISSION_ACTION,
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
