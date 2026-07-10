#[derive(Debug, newtype::Newtype)]
#[newtype(as_ref_inner, from_inner)]
pub struct AttrIdentName<'name_lt>(&'name_lt str);
pub trait AttrIdentStr {
    fn attr_ident_str(&self) -> AttrIdentName<'_>;
}
