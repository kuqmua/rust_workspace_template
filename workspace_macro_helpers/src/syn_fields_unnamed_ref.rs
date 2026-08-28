#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub struct SynFieldsUnnamedRef<'fields_lt>(pub(super) &'fields_lt syn::FieldsUnnamed);
impl<'fields_lt> From<&'fields_lt syn::FieldsUnnamed> for SynFieldsUnnamedRef<'fields_lt> {
    fn from(value: &'fields_lt syn::FieldsUnnamed) -> Self {
        Self(value)
    }
}
impl<'fields_lt> SynFieldsUnnamedRef<'fields_lt> {
    #[must_use]
    pub const fn get(self) -> &'fields_lt syn::FieldsUnnamed {
        self.0
    }
}
impl std::ops::Deref for SynFieldsUnnamedRef<'_> {
    type Target = syn::FieldsUnnamed;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
