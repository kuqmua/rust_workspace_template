pub trait HasFilterContracts {
    const FILTER_CONTRACTS: &'static [crate::filter_operation::FilterOperation];
    #[must_use]
    fn filter_contracts() -> crate::filter_contracts::FilterContracts {
        crate::filter_contracts::FilterContracts::from(Self::FILTER_CONTRACTS)
    }
}
