#[derive(Debug)]
pub struct AttrIdentName<'name_lt>(&'name_lt str);
impl<'name_lt> From<&'name_lt str> for AttrIdentName<'name_lt> {
    fn from(value: &'name_lt str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for AttrIdentName<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
pub trait AttrIdentStr {
    fn attr_ident_str(&self) -> AttrIdentName<'_>;
}
