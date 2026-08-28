use super::{FilterContracts, FilterOperation};

pub trait HasFilterContracts {
    const FILTER_CONTRACTS: &'static [FilterOperation];
    #[must_use]
    fn filter_contracts() -> FilterContracts {
        FilterContracts::from(Self::FILTER_CONTRACTS)
    }
}
