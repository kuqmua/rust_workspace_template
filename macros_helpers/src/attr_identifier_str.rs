#[derive(Debug, newtype::Newtype)]
#[newtype(as_ref_inner, from_inner)]
pub struct AttrIdentifierName<'name_lt>(&'name_lt str);
pub trait AttrIdentifierStr {
    fn attr_identifier_str(&self) -> AttrIdentifierName<'_>;
}
