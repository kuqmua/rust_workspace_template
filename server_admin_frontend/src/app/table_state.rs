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

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub(crate) struct AdminFrontendTableIndex(pub(crate) usize);

#[derive(Clone, Debug, Default, Eq, PartialEq, newtype::BoundedString, newtype::AsRefOwned)]
#[bounded_string(max = 8192usize)]
pub(crate) struct AdminFrontendTableText(pub(crate) String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub(crate) struct AdminFrontendTableTextRef<'value_lt>(pub(crate) &'value_lt str);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TableState {
    page: AdminFrontendTableIndex,
    page_size: AdminFrontendTableIndex,
    search: AdminFrontendTableText,
    sort: server_admin_contract::AdminTableSortField,
    sort_dir: SortDir,
}

impl TableState {
    pub(crate) fn new(sort: server_admin_contract::AdminTableSortField) -> Self {
        Self {
            page: AdminFrontendTableIndex::from(0usize),
            page_size: AdminFrontendTableIndex::from(20usize),
            search: AdminFrontendTableText::default(),
            sort,
            sort_dir: SortDir::Asc,
        }
    }

    #[allow(clippy::single_call_fn)] // called by the browser build and directly exercised by native contract tests
    pub(crate) fn from_query(
        default_sort: server_admin_contract::AdminTableSortField,
        options: &[server_admin_contract::AdminTableSortField],
        query: &str,
    ) -> Self {
        let mut state = Self::new(default_sort);
        let mut offset = 0usize;
        query.trim_start_matches('?').split('&').for_each(|part| {
            let Some((key, encoded_value)) = part.split_once('=') else {
                return;
            };
            let decoded = percent_decode(encoded_value).unwrap_or_default();
            match key {
                str_constants::LIMIT => {
                    if let Ok(page_size) = decoded.parse::<usize>() {
                        state.page_size =
                            AdminFrontendTableIndex::from(page_size.clamp(10usize, 100usize));
                    }
                }
                str_constants::OFFSET_ALT => offset = decoded.parse::<usize>().unwrap_or_default(),
                str_constants::SEARCH_ALT => {
                    if let Ok(search) = AdminFrontendTableText::try_from(decoded) {
                        state.search = search;
                    }
                }
                str_constants::SORT_ALT => {
                    if let Ok(sort) = server_admin_contract::AdminTableSortField::try_from_key(
                        options,
                        server_admin_contract::AdminTableSortKeyRef::from(decoded.as_str()),
                    ) {
                        state.sort = sort;
                    }
                }
                str_constants::DIRECTION if decoded == str_constants::DESC_ALT => {
                    state.sort_dir = SortDir::Desc;
                }
                _ => {}
            }
        });
        state.page = AdminFrontendTableIndex::from(
            offset.checked_div(state.page_size.0).unwrap_or_default(),
        );
        state
    }

    pub(crate) fn query(&self) -> String {
        let direction = match self.sort_dir {
            SortDir::Asc => str_constants::ASC_ALT,
            SortDir::Desc => str_constants::DESC_ALT,
        };
        format!(
            "limit={}&offset={}&search={}&sort={}&direction={direction}",
            self.page_size.0,
            self.page.0.saturating_mul(self.page_size.0),
            percent_encode(self.search.0.as_str()),
            self.sort.key(),
        )
    }

    pub(crate) fn apply_search(&mut self, value: AdminFrontendTableText) {
        self.search = value;
        self.page = AdminFrontendTableIndex::from(0usize);
    }

    pub(crate) fn apply_page_size(&mut self, value: AdminFrontendTableIndex) {
        self.page_size = AdminFrontendTableIndex::from(value.0.clamp(10usize, 100usize));
        self.page = AdminFrontendTableIndex::from(0usize);
    }

    pub(crate) fn apply_sort(&mut self, value: server_admin_contract::AdminTableSortField) {
        if self.sort == value {
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
        if total.0 <= self.page_size.0 {
            return AdminFrontendTableIndex::from(0usize);
        }
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

    pub(crate) fn sort(&self) -> server_admin_contract::AdminTableSortField {
        self.sort
    }

    pub(crate) fn sort_dir(&self) -> SortDir {
        self.sort_dir
    }
}

#[allow(clippy::single_call_fn)] // profile links provide a second browser-only caller
pub(crate) fn percent_encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    value.bytes().for_each(|byte| {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(hex_char(byte >> 4u8));
            encoded.push(hex_char(byte & 0x0fu8));
        }
    });
    encoded
}

#[allow(clippy::single_call_fn)] // kept separate so URL decoding remains independently testable
pub(crate) fn percent_decode(value: &str) -> Option<String> {
    let mut bytes = value.bytes();
    let mut decoded = Vec::with_capacity(value.len());
    while let Some(byte) = bytes.next() {
        match byte {
            b'%' => {
                let high = hex(bytes.next()?)?;
                let low = hex(bytes.next()?)?;
                decoded.push(high.saturating_mul(16u8).saturating_add(low));
            }
            b'+' => {
                decoded.push(b' ');
            }
            literal => {
                decoded.push(literal);
            }
        }
    }
    String::from_utf8(decoded).ok()
}

fn hex(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => value.checked_sub(b'0'),
        b'a'..=b'f' => {
            let digit = value.checked_sub(b'a')?;
            digit.checked_add(10u8)
        }
        b'A'..=b'F' => {
            let digit = value.checked_sub(b'A')?;
            digit.checked_add(10u8)
        }
        _ => None,
    }
}

const fn hex_char(value: u8) -> char {
    match value {
        0u8 => '0',
        1u8 => '1',
        2u8 => '2',
        3u8 => '3',
        4u8 => '4',
        5u8 => '5',
        6u8 => '6',
        7u8 => '7',
        8u8 => '8',
        9u8 => '9',
        10u8 => 'A',
        11u8 => 'B',
        12u8 => 'C',
        13u8 => 'D',
        14u8 => 'E',
        _ => 'F',
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn search_and_sort_reset_paging_and_sort_toggles_direction() {
        let mut state =
            super::TableState::new(server_admin_contract::AdminTableSortField::UserLogin);
        state.next(super::AdminFrontendTableIndex::from(50usize));
        assert_eq!(state.page_number().0, 2usize);
        state.apply_search(
            super::AdminFrontendTableText::try_from(str_constants::ROOT.to_owned())
                .expect("5e68820e"),
        );
        assert_eq!(state.page_number().0, 1usize);
        assert_eq!(state.search().0, "root");
        state.apply_sort(server_admin_contract::AdminTableSortField::UserLogin);
        assert_eq!(state.sort_dir(), super::SortDir::Desc);
        state.apply_sort(server_admin_contract::AdminTableSortField::UserDisplayName);
        assert_eq!(
            state.sort(),
            server_admin_contract::AdminTableSortField::UserDisplayName
        );
        assert_eq!(state.sort_dir(), super::SortDir::Asc);
    }

    #[test]
    fn paging_is_bounded_and_page_size_is_validated() {
        let mut state =
            super::TableState::new(server_admin_contract::AdminTableSortField::RoleName);
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

    #[test]
    fn query_round_trip_preserves_server_table_state() {
        let state = super::TableState::from_query(
            server_admin_contract::AdminTableSortField::UserLogin,
            &server_admin_contract::AdminTableSortField::USER,
            str_constants::AUDIT_TABLE_QUERY_FIXTURE,
        );
        assert_eq!(state.page_number().0, 3usize);
        assert_eq!(state.search().0, "Alpha Operator");
        assert_eq!(
            state.sort(),
            server_admin_contract::AdminTableSortField::UserDisplayName
        );
        assert_eq!(state.sort_dir(), super::SortDir::Desc);
        assert_eq!(
            state.query(),
            str_constants::AUDIT_TABLE_QUERY_FIXTURE.trim_start_matches('?')
        );
    }
}
