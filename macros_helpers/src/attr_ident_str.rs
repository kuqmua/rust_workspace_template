#[derive(Debug)]
pub struct AttrIdentName<'name_lt>(pub &'name_lt str);
pub trait AttrIdentStr {
    fn attr_ident_str(&self) -> AttrIdentName<'_>;
}
