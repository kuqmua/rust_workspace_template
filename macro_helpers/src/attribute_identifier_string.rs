#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefInner, newtype::FromInner,
)]
pub struct AttrIdentifierName<'name_lt>(&'name_lt str);
pub trait AttrIdentifierStr {
    fn attribute_identifier_string(&self) -> AttrIdentifierName<'_>;
}
