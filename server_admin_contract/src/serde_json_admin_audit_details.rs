#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerdeJsonAdminAuditDetails(serde_json::Value);

impl TryFrom<serde_json::Value> for SerdeJsonAdminAuditDetails {
    type Error = crate::admin_audit_details_too_large::AdminAuditDetailsTooLarge;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let actual_bytes = value.to_string().len();
        if actual_bytes > crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES {
            return Err(
                crate::admin_audit_details_too_large::AdminAuditDetailsTooLarge::from(
                    crate::admin_audit_details_bytes::AdminAuditDetailsBytes::from(actual_bytes),
                ),
            );
        }
        Ok(Self(value))
    }
}
