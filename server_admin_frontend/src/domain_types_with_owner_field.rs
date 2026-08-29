pub(crate) use super::admin_field::AdminField;
pub(crate) use super::admin_field_label::AdminFieldLabel;
// Root-owned module compatibility wrappers.
pub(crate) mod admin_field {
    pub use super::super::admin_field::*;
}
pub(crate) mod admin_field_label {
    pub use super::super::admin_field_label::*;
}
