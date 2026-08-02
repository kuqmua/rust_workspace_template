#[derive(optml::Optml, Debug, newtype::AsRefInner, newtype::FromInner)]
pub struct AttrIdentifierName<'name_lt>(&'name_lt str);
pub trait AttrIdentifierStr {
    fn attr_identifier_str(&self) -> AttrIdentifierName<'_>;
}
