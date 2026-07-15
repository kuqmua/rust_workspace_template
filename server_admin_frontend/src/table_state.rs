#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::field_scoped_visibility_modifiers,
    clippy::integer_division,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate
)] // UI paging favors direct arithmetic; sibling view code needs crate-scoped wrapper representations

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SortDir {
    Asc,
    Desc,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Newtype)]
#[newtype(from_inner)]
pub(crate) struct AdminFrontendTableIndex(pub(crate) usize);

#[derive(Clone, Debug, Default, Eq, PartialEq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(max = 8192usize)]
#[newtype(as_ref_owned)]
pub(crate) struct AdminFrontendTableText(pub(crate) String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Newtype)]
#[newtype(from_inner)]
pub(crate) struct AdminFrontendTableTextRef<'value_lt>(pub(crate) &'value_lt str);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TableState {
    page: AdminFrontendTableIndex,
    page_size: AdminFrontendTableIndex,
    search: AdminFrontendTableText,
    sort: AdminFrontendTableText,
    sort_dir: SortDir,
}

impl TableState {
    pub(crate) fn new(sort: AdminFrontendTableText) -> Self {
        Self {
            page: AdminFrontendTableIndex::from(0usize),
            page_size: AdminFrontendTableIndex::from(20usize),
            search: AdminFrontendTableText::default(),
            sort,
            sort_dir: SortDir::Asc,
        }
    }

    pub(crate) fn apply_search(&mut self, value: AdminFrontendTableText) {
        self.search = value;
        self.page = AdminFrontendTableIndex::from(0usize);
    }

    pub(crate) fn apply_page_size(&mut self, value: AdminFrontendTableIndex) {
        self.page_size = AdminFrontendTableIndex::from(value.0.clamp(10usize, 100usize));
        self.page = AdminFrontendTableIndex::from(0usize);
    }

    pub(crate) fn apply_sort(&mut self, value: AdminFrontendTableText) {
        if self.sort.0 == value.0 {
            self.sort_dir = match self.sort_dir {
                SortDir::Asc => SortDir::Desc,
                SortDir::Desc => SortDir::Asc,
            };
        } else {
            self.sort = value;
            self.sort_dir = SortDir::Asc;
        }
        self.page = AdminFrontendTableIndex::from(0usize);
    }

    pub(crate) fn next(&mut self, total: AdminFrontendTableIndex) {
        if self.end(total).0 < total.0 {
            self.page = AdminFrontendTableIndex::from(self.page.0.saturating_add(1usize));
        }
    }

    pub(crate) fn previous(&mut self) {
        self.page = AdminFrontendTableIndex::from(self.page.0.saturating_sub(1usize));
    }

    pub(crate) fn start(&self, total: AdminFrontendTableIndex) -> AdminFrontendTableIndex {
        AdminFrontendTableIndex::from(self.page.0.saturating_mul(self.page_size.0).min(total.0))
    }

    pub(crate) fn end(&self, total: AdminFrontendTableIndex) -> AdminFrontendTableIndex {
        AdminFrontendTableIndex::from(
            self.start(total)
                .0
                .saturating_add(self.page_size.0)
                .min(total.0),
        )
    }

    pub(crate) fn page_number(&self) -> AdminFrontendTableIndex {
        AdminFrontendTableIndex::from(self.page.0.saturating_add(1usize))
    }

    pub(crate) fn search(&self) -> AdminFrontendTableTextRef<'_> {
        AdminFrontendTableTextRef::from(self.search.0.as_str())
    }

    pub(crate) fn sort(&self) -> AdminFrontendTableTextRef<'_> {
        AdminFrontendTableTextRef::from(self.sort.0.as_str())
    }

    pub(crate) fn sort_dir(&self) -> SortDir {
        self.sort_dir
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn search_and_sort_reset_paging_and_sort_toggles_direction() {
        let mut state = super::TableState::new(
            super::AdminFrontendTableText::try_from(str_constants::expr::S_1468.to_owned())
                .expect("3f98f927"),
        );
        state.next(super::AdminFrontendTableIndex::from(50usize));
        assert_eq!(state.page_number().0, 2usize);
        state.apply_search(
            super::AdminFrontendTableText::try_from(str_constants::expr::S_1685.to_owned())
                .expect("5e68820e"),
        );
        assert_eq!(state.page_number().0, 1usize);
        assert_eq!(state.search().0, "root");
        state.apply_sort(
            super::AdminFrontendTableText::try_from(str_constants::expr::S_1468.to_owned())
                .expect("7c2035b3"),
        );
        assert_eq!(state.sort_dir(), super::SortDir::Desc);
        state.apply_sort(
            super::AdminFrontendTableText::try_from(str_constants::expr::S_1194.to_owned())
                .expect("8215b5f6"),
        );
        assert_eq!(state.sort().0, "display_name");
        assert_eq!(state.sort_dir(), super::SortDir::Asc);
    }

    #[test]
    fn paging_is_bounded_and_page_size_is_validated() {
        let mut state = super::TableState::new(
            super::AdminFrontendTableText::try_from(str_constants::expr::S_1536.to_owned())
                .expect("fe54b186"),
        );
        state.apply_page_size(super::AdminFrontendTableIndex::from(1usize));
        assert_eq!(
            state.end(super::AdminFrontendTableIndex::from(100usize)).0,
            10usize
        );
        state.next(super::AdminFrontendTableIndex::from(21usize));
        state.next(super::AdminFrontendTableIndex::from(21usize));
        state.next(super::AdminFrontendTableIndex::from(21usize));
        assert_eq!(state.page_number().0, 3usize);
        assert_eq!(
            state.start(super::AdminFrontendTableIndex::from(21usize)).0,
            20usize
        );
        assert_eq!(
            state.end(super::AdminFrontendTableIndex::from(21usize)).0,
            21usize
        );
        state.previous();
        assert_eq!(state.page_number().0, 2usize);
    }
}
