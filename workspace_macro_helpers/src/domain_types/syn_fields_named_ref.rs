#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub struct SynFieldsNamedRef<'fields_lt>(pub(super) &'fields_lt syn::FieldsNamed);
impl<'fields_lt> From<&'fields_lt syn::FieldsNamed> for SynFieldsNamedRef<'fields_lt> {
    fn from(value: &'fields_lt syn::FieldsNamed) -> Self {
        Self(value)
    }
}
impl<'fields_lt> SynFieldsNamedRef<'fields_lt> {
    #[must_use]
    pub const fn get(self) -> &'fields_lt syn::FieldsNamed {
        self.0
    }
}
impl std::ops::Deref for SynFieldsNamedRef<'_> {
    type Target = syn::FieldsNamed;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
