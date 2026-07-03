#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnqVecTryNewEr<T> {
    NotUnq(T),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnqVec<T>(Vec<T>);

impl<T> From<UnqVec<T>> for Vec<T> {
    fn from(value: UnqVec<T>) -> Self {
        value.0
    }
}

impl<T: PartialEq> TryFrom<Vec<T>> for UnqVec<T> {
    type Error = UnqVecTryNewEr<T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let search_values = pg_crud_cmn::DuplicateSearchValues::from(value);
        match pg_crud_cmn::take_fst_dup(search_values) {
            pg_crud_cmn::DuplicateSearchResult::Unique(unique_values) => {
                Ok(Self(unique_values.into()))
            }
            pg_crud_cmn::DuplicateSearchResult::Duplicate(duplicate_value) => {
                Err(UnqVecTryNewEr::NotUnq(duplicate_value.into_inner()))
            }
        }
    }
}

impl<T: pg_crud_cmn::DfltSomeOneEl> pg_crud_cmn::DfltSomeOneEl for UnqVec<T> {
    fn dflt_some_one_el() -> Self {
        Self(vec![T::dflt_some_one_el()])
    }
}
