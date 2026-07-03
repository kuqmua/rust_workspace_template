#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgJsonFilterDimension {
    Four,
    One,
    Three,
    Two,
    Zero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgFilterName {
    AdjacentWithRange,
    AllElsEq,
    AllElsGreaterThan,
    AllElsRgx,
    Before,
    Btwn,
    ContainsAllElsOfArr,
    ContainsElGreaterThan,
    ContainsElRgx,
    CrntDate,
    CrntTime,
    CrntTimestamp,
    Eq,
    EqToEncodedStringRepresentation,
    ExcludedUpperBound,
    FindRangesThatFullyContainTheGivenRange,
    FindRangesWithinGivenRange,
    GreaterThan,
    GreaterThanCrntDate,
    GreaterThanCrntTime,
    GreaterThanCrntTimestamp,
    GreaterThanExcludedUpperBound,
    GreaterThanIncludedLowerBound,
    In,
    IncludedLowerBound,
    LenEq,
    LenGreaterThan,
    OverlapWithRange,
    OverlapsWithArr,
    RangeLen,
    Rgx,
    StrictlyToLeftOfRange,
    StrictlyToRightOfRange,
}

#[derive(Debug, Clone)]
pub enum PgTypeFlt {
    AdjacentWithRange { ident: proc_macro2::TokenStream },
    Before { ident: proc_macro2::TokenStream },
    Btwn { ident: proc_macro2::TokenStream },
    CrntDate,
    CrntTime,
    CrntTimestamp,
    DimOneAdjacentWithRange { ident: proc_macro2::TokenStream },
    DimOneBefore { ident: proc_macro2::TokenStream },
    DimOneBtwn { ident: proc_macro2::TokenStream },
    DimOneCrntDate,
    DimOneCrntTime,
    DimOneCrntTimestamp,
    DimOneEq { ident: proc_macro2::TokenStream },
    DimOneEqToEncodedStringRepresentation,
    DimOneExcludedUpperBound { ident: proc_macro2::TokenStream },
    DimOneFindRangesThatFullyContainTheGivenRange { ident: proc_macro2::TokenStream },
    DimOneFindRangesWithinGivenRange { ident: proc_macro2::TokenStream },
    DimOneGreaterThan { ident: proc_macro2::TokenStream },
    DimOneGreaterThanCrntDate,
    DimOneGreaterThanCrntTime,
    DimOneGreaterThanCrntTimestamp,
    DimOneGreaterThanExcludedUpperBound { ident: proc_macro2::TokenStream },
    DimOneGreaterThanIncludedLowerBound { ident: proc_macro2::TokenStream },
    DimOneIn { ident: proc_macro2::TokenStream },
    DimOneIncludedLowerBound { ident: proc_macro2::TokenStream },
    DimOneLenEq,
    DimOneLenGreaterThan,
    DimOneOverlapWithRange { ident: proc_macro2::TokenStream },
    DimOneRangeLen,
    DimOneRgx,
    DimOneStrictlyToLeftOfRange { ident: proc_macro2::TokenStream },
    DimOneStrictlyToRightOfRange { ident: proc_macro2::TokenStream },
    Eq { ident: proc_macro2::TokenStream },
    EqToEncodedStringRepresentation,
    ExcludedUpperBound { ident: proc_macro2::TokenStream },
    FindRangesThatFullyContainTheGivenRange { ident: proc_macro2::TokenStream },
    FindRangesWithinGivenRange { ident: proc_macro2::TokenStream },
    GreaterThan { ident: proc_macro2::TokenStream },
    GreaterThanCrntDate,
    GreaterThanCrntTime,
    GreaterThanCrntTimestamp,
    GreaterThanExcludedUpperBound { ident: proc_macro2::TokenStream },
    GreaterThanIncludedLowerBound { ident: proc_macro2::TokenStream },
    In { ident: proc_macro2::TokenStream },
    IncludedLowerBound { ident: proc_macro2::TokenStream },
    OverlapWithRange { ident: proc_macro2::TokenStream },
    RangeLen,
    Rgx,
    StrictlyToLeftOfRange { ident: proc_macro2::TokenStream },
    StrictlyToRightOfRange { ident: proc_macro2::TokenStream },
}

#[derive(Debug, Clone)]
pub enum PgJsonFlt {
    AllElsEq { ident: proc_macro2::TokenStream },
    AllElsGreaterThan { ident: proc_macro2::TokenStream },
    AllElsRgx,
    Btwn { ident: proc_macro2::TokenStream },
    ContainsAllElsOfArr { ident: proc_macro2::TokenStream },
    ContainsElGreaterThan { ident: proc_macro2::TokenStream },
    ContainsElRgx,
    DimFourAllElsEq { ident: proc_macro2::TokenStream },
    DimFourAllElsGreaterThan { ident: proc_macro2::TokenStream },
    DimFourAllElsRgx,
    DimFourBtwn { ident: proc_macro2::TokenStream },
    DimFourContainsAllElsOfArr { ident: proc_macro2::TokenStream },
    DimFourContainsElGreaterThan { ident: proc_macro2::TokenStream },
    DimFourContainsElRgx,
    DimFourEq { ident: proc_macro2::TokenStream },
    DimFourGreaterThan { ident: proc_macro2::TokenStream },
    DimFourIn { ident: proc_macro2::TokenStream },
    DimFourLenEq,
    DimFourLenGreaterThan,
    DimFourOverlapsWithArr { ident: proc_macro2::TokenStream },
    DimFourRgx,
    DimOneAllElsEq { ident: proc_macro2::TokenStream },
    DimOneAllElsGreaterThan { ident: proc_macro2::TokenStream },
    DimOneAllElsRgx,
    DimOneBtwn { ident: proc_macro2::TokenStream },
    DimOneContainsAllElsOfArr { ident: proc_macro2::TokenStream },
    DimOneContainsElGreaterThan { ident: proc_macro2::TokenStream },
    DimOneContainsElRgx,
    DimOneEq { ident: proc_macro2::TokenStream },
    DimOneGreaterThan { ident: proc_macro2::TokenStream },
    DimOneIn { ident: proc_macro2::TokenStream },
    DimOneLenEq,
    DimOneLenGreaterThan,
    DimOneOverlapsWithArr { ident: proc_macro2::TokenStream },
    DimOneRgx,
    DimThreeAllElsEq { ident: proc_macro2::TokenStream },
    DimThreeAllElsGreaterThan { ident: proc_macro2::TokenStream },
    DimThreeAllElsRgx,
    DimThreeBtwn { ident: proc_macro2::TokenStream },
    DimThreeContainsAllElsOfArr { ident: proc_macro2::TokenStream },
    DimThreeContainsElGreaterThan { ident: proc_macro2::TokenStream },
    DimThreeContainsElRgx,
    DimThreeEq { ident: proc_macro2::TokenStream },
    DimThreeGreaterThan { ident: proc_macro2::TokenStream },
    DimThreeIn { ident: proc_macro2::TokenStream },
    DimThreeLenEq,
    DimThreeLenGreaterThan,
    DimThreeOverlapsWithArr { ident: proc_macro2::TokenStream },
    DimThreeRgx,
    DimTwoAllElsEq { ident: proc_macro2::TokenStream },
    DimTwoAllElsGreaterThan { ident: proc_macro2::TokenStream },
    DimTwoAllElsRgx,
    DimTwoBtwn { ident: proc_macro2::TokenStream },
    DimTwoContainsAllElsOfArr { ident: proc_macro2::TokenStream },
    DimTwoContainsElGreaterThan { ident: proc_macro2::TokenStream },
    DimTwoContainsElRgx,
    DimTwoEq { ident: proc_macro2::TokenStream },
    DimTwoGreaterThan { ident: proc_macro2::TokenStream },
    DimTwoIn { ident: proc_macro2::TokenStream },
    DimTwoLenEq,
    DimTwoLenGreaterThan,
    DimTwoOverlapsWithArr { ident: proc_macro2::TokenStream },
    DimTwoRgx,
    Eq { ident: proc_macro2::TokenStream },
    GreaterThan { ident: proc_macro2::TokenStream },
    In { ident: proc_macro2::TokenStream },
    LenEq,
    LenGreaterThan,
    OverlapsWithArr { ident: proc_macro2::TokenStream },
    Rgx,
}

pub trait PgFlt {
    #[must_use]
    fn filter_name(&self) -> PgFilterName;

    #[must_use]
    fn maybe_generic(&self) -> Option<proc_macro2::TokenStream>;
}

impl PgJsonFlt {
    #[must_use]
    pub const fn dim_eq(dimension: PgJsonFilterDimension, ident: proc_macro2::TokenStream) -> Self {
        match dimension {
            PgJsonFilterDimension::Zero => Self::Eq { ident },
            PgJsonFilterDimension::One => Self::DimOneEq { ident },
            PgJsonFilterDimension::Two => Self::DimTwoEq { ident },
            PgJsonFilterDimension::Three => Self::DimThreeEq { ident },
            PgJsonFilterDimension::Four => Self::DimFourEq { ident },
        }
    }

    #[must_use]
    pub const fn dim_len_eq(dimension: PgJsonFilterDimension) -> Self {
        match dimension {
            PgJsonFilterDimension::Zero => Self::LenEq,
            PgJsonFilterDimension::One => Self::DimOneLenEq,
            PgJsonFilterDimension::Two => Self::DimTwoLenEq,
            PgJsonFilterDimension::Three => Self::DimThreeLenEq,
            PgJsonFilterDimension::Four => Self::DimFourLenEq,
        }
    }

    #[must_use]
    pub const fn dim_rgx(dimension: PgJsonFilterDimension) -> Self {
        match dimension {
            PgJsonFilterDimension::Zero => Self::Rgx,
            PgJsonFilterDimension::One => Self::DimOneRgx,
            PgJsonFilterDimension::Two => Self::DimTwoRgx,
            PgJsonFilterDimension::Three => Self::DimThreeRgx,
            PgJsonFilterDimension::Four => Self::DimFourRgx,
        }
    }
}

impl PgFlt for PgTypeFlt {
    fn filter_name(&self) -> PgFilterName {
        match self.clone() {
            Self::Eq { .. } | Self::DimOneEq { .. } => PgFilterName::Eq,
            Self::GreaterThan { .. } | Self::DimOneGreaterThan { .. } => PgFilterName::GreaterThan,
            Self::Btwn { .. } | Self::DimOneBtwn { .. } => PgFilterName::Btwn,
            Self::In { .. } | Self::DimOneIn { .. } => PgFilterName::In,
            Self::Rgx | Self::DimOneRgx => PgFilterName::Rgx,
            Self::Before { .. } | Self::DimOneBefore { .. } => PgFilterName::Before,
            Self::CrntDate | Self::DimOneCrntDate => PgFilterName::CrntDate,
            Self::GreaterThanCrntDate | Self::DimOneGreaterThanCrntDate => {
                PgFilterName::GreaterThanCrntDate
            }
            Self::CrntTimestamp | Self::DimOneCrntTimestamp => PgFilterName::CrntTimestamp,
            Self::GreaterThanCrntTimestamp | Self::DimOneGreaterThanCrntTimestamp => {
                PgFilterName::GreaterThanCrntTimestamp
            }
            Self::CrntTime | Self::DimOneCrntTime => PgFilterName::CrntTime,
            Self::GreaterThanCrntTime | Self::DimOneGreaterThanCrntTime => {
                PgFilterName::GreaterThanCrntTime
            }
            Self::DimOneLenEq => PgFilterName::LenEq,
            Self::DimOneLenGreaterThan => PgFilterName::LenGreaterThan,
            Self::EqToEncodedStringRepresentation | Self::DimOneEqToEncodedStringRepresentation => {
                PgFilterName::EqToEncodedStringRepresentation
            }
            Self::FindRangesWithinGivenRange { .. }
            | Self::DimOneFindRangesWithinGivenRange { .. } => {
                PgFilterName::FindRangesWithinGivenRange
            }
            Self::FindRangesThatFullyContainTheGivenRange { .. }
            | Self::DimOneFindRangesThatFullyContainTheGivenRange { .. } => {
                PgFilterName::FindRangesThatFullyContainTheGivenRange
            }
            Self::StrictlyToLeftOfRange { .. } | Self::DimOneStrictlyToLeftOfRange { .. } => {
                PgFilterName::StrictlyToLeftOfRange
            }
            Self::StrictlyToRightOfRange { .. } | Self::DimOneStrictlyToRightOfRange { .. } => {
                PgFilterName::StrictlyToRightOfRange
            }
            Self::IncludedLowerBound { .. } | Self::DimOneIncludedLowerBound { .. } => {
                PgFilterName::IncludedLowerBound
            }
            Self::ExcludedUpperBound { .. } | Self::DimOneExcludedUpperBound { .. } => {
                PgFilterName::ExcludedUpperBound
            }
            Self::GreaterThanIncludedLowerBound { .. }
            | Self::DimOneGreaterThanIncludedLowerBound { .. } => {
                PgFilterName::GreaterThanIncludedLowerBound
            }
            Self::GreaterThanExcludedUpperBound { .. }
            | Self::DimOneGreaterThanExcludedUpperBound { .. } => {
                PgFilterName::GreaterThanExcludedUpperBound
            }
            Self::OverlapWithRange { .. } | Self::DimOneOverlapWithRange { .. } => {
                PgFilterName::OverlapWithRange
            }
            Self::AdjacentWithRange { .. } | Self::DimOneAdjacentWithRange { .. } => {
                PgFilterName::AdjacentWithRange
            }
            Self::RangeLen | Self::DimOneRangeLen => PgFilterName::RangeLen,
        }
    }

    fn maybe_generic(&self) -> Option<proc_macro2::TokenStream> {
        match self.clone() {
            Self::AdjacentWithRange { ident }
            | Self::Before { ident }
            | Self::Btwn { ident }
            | Self::DimOneAdjacentWithRange { ident }
            | Self::DimOneBefore { ident }
            | Self::DimOneBtwn { ident }
            | Self::DimOneEq { ident }
            | Self::DimOneExcludedUpperBound { ident }
            | Self::DimOneFindRangesThatFullyContainTheGivenRange { ident }
            | Self::DimOneFindRangesWithinGivenRange { ident }
            | Self::DimOneGreaterThan { ident }
            | Self::DimOneGreaterThanExcludedUpperBound { ident }
            | Self::DimOneGreaterThanIncludedLowerBound { ident }
            | Self::DimOneIn { ident }
            | Self::DimOneIncludedLowerBound { ident }
            | Self::DimOneOverlapWithRange { ident }
            | Self::DimOneStrictlyToLeftOfRange { ident }
            | Self::DimOneStrictlyToRightOfRange { ident }
            | Self::Eq { ident }
            | Self::ExcludedUpperBound { ident }
            | Self::FindRangesThatFullyContainTheGivenRange { ident }
            | Self::FindRangesWithinGivenRange { ident }
            | Self::GreaterThan { ident }
            | Self::GreaterThanExcludedUpperBound { ident }
            | Self::GreaterThanIncludedLowerBound { ident }
            | Self::In { ident }
            | Self::IncludedLowerBound { ident }
            | Self::OverlapWithRange { ident }
            | Self::StrictlyToLeftOfRange { ident }
            | Self::StrictlyToRightOfRange { ident } => Some(ident),
            Self::CrntDate
            | Self::CrntTime
            | Self::CrntTimestamp
            | Self::DimOneCrntDate
            | Self::DimOneCrntTime
            | Self::DimOneCrntTimestamp
            | Self::DimOneEqToEncodedStringRepresentation
            | Self::DimOneGreaterThanCrntDate
            | Self::DimOneGreaterThanCrntTime
            | Self::DimOneGreaterThanCrntTimestamp
            | Self::DimOneLenEq
            | Self::DimOneLenGreaterThan
            | Self::DimOneRangeLen
            | Self::DimOneRgx
            | Self::EqToEncodedStringRepresentation
            | Self::GreaterThanCrntDate
            | Self::GreaterThanCrntTime
            | Self::GreaterThanCrntTimestamp
            | Self::RangeLen
            | Self::Rgx => None,
        }
    }
}

impl PgFlt for PgJsonFlt {
    fn filter_name(&self) -> PgFilterName {
        match self.clone() {
            Self::Eq { .. }
            | Self::DimOneEq { .. }
            | Self::DimTwoEq { .. }
            | Self::DimThreeEq { .. }
            | Self::DimFourEq { .. } => PgFilterName::Eq,
            Self::AllElsEq { .. }
            | Self::DimOneAllElsEq { .. }
            | Self::DimTwoAllElsEq { .. }
            | Self::DimThreeAllElsEq { .. }
            | Self::DimFourAllElsEq { .. } => PgFilterName::AllElsEq,
            Self::LenEq
            | Self::DimOneLenEq
            | Self::DimTwoLenEq
            | Self::DimThreeLenEq
            | Self::DimFourLenEq => PgFilterName::LenEq,
            Self::LenGreaterThan
            | Self::DimOneLenGreaterThan
            | Self::DimTwoLenGreaterThan
            | Self::DimThreeLenGreaterThan
            | Self::DimFourLenGreaterThan => PgFilterName::LenGreaterThan,
            Self::GreaterThan { .. }
            | Self::DimOneGreaterThan { .. }
            | Self::DimTwoGreaterThan { .. }
            | Self::DimThreeGreaterThan { .. }
            | Self::DimFourGreaterThan { .. } => PgFilterName::GreaterThan,
            Self::ContainsElGreaterThan { .. }
            | Self::DimOneContainsElGreaterThan { .. }
            | Self::DimTwoContainsElGreaterThan { .. }
            | Self::DimThreeContainsElGreaterThan { .. }
            | Self::DimFourContainsElGreaterThan { .. } => PgFilterName::ContainsElGreaterThan,
            Self::AllElsGreaterThan { .. }
            | Self::DimOneAllElsGreaterThan { .. }
            | Self::DimTwoAllElsGreaterThan { .. }
            | Self::DimThreeAllElsGreaterThan { .. }
            | Self::DimFourAllElsGreaterThan { .. } => PgFilterName::AllElsGreaterThan,
            Self::Btwn { .. }
            | Self::DimOneBtwn { .. }
            | Self::DimTwoBtwn { .. }
            | Self::DimThreeBtwn { .. }
            | Self::DimFourBtwn { .. } => PgFilterName::Btwn,
            Self::In { .. }
            | Self::DimOneIn { .. }
            | Self::DimTwoIn { .. }
            | Self::DimThreeIn { .. }
            | Self::DimFourIn { .. } => PgFilterName::In,
            Self::Rgx
            | Self::DimOneRgx
            | Self::DimTwoRgx
            | Self::DimThreeRgx
            | Self::DimFourRgx => PgFilterName::Rgx,
            Self::ContainsElRgx
            | Self::DimOneContainsElRgx
            | Self::DimTwoContainsElRgx
            | Self::DimThreeContainsElRgx
            | Self::DimFourContainsElRgx => PgFilterName::ContainsElRgx,
            Self::AllElsRgx
            | Self::DimOneAllElsRgx
            | Self::DimTwoAllElsRgx
            | Self::DimThreeAllElsRgx
            | Self::DimFourAllElsRgx => PgFilterName::AllElsRgx,
            Self::ContainsAllElsOfArr { .. }
            | Self::DimOneContainsAllElsOfArr { .. }
            | Self::DimTwoContainsAllElsOfArr { .. }
            | Self::DimThreeContainsAllElsOfArr { .. }
            | Self::DimFourContainsAllElsOfArr { .. } => PgFilterName::ContainsAllElsOfArr,
            Self::OverlapsWithArr { .. }
            | Self::DimOneOverlapsWithArr { .. }
            | Self::DimTwoOverlapsWithArr { .. }
            | Self::DimThreeOverlapsWithArr { .. }
            | Self::DimFourOverlapsWithArr { .. } => PgFilterName::OverlapsWithArr,
        }
    }

    fn maybe_generic(&self) -> Option<proc_macro2::TokenStream> {
        match self.clone() {
            Self::AllElsEq { ident }
            | Self::AllElsGreaterThan { ident }
            | Self::Btwn { ident }
            | Self::ContainsAllElsOfArr { ident }
            | Self::ContainsElGreaterThan { ident }
            | Self::DimFourAllElsEq { ident }
            | Self::DimFourAllElsGreaterThan { ident }
            | Self::DimFourBtwn { ident }
            | Self::DimFourContainsAllElsOfArr { ident }
            | Self::DimFourContainsElGreaterThan { ident }
            | Self::DimFourEq { ident }
            | Self::DimFourGreaterThan { ident }
            | Self::DimFourIn { ident }
            | Self::DimFourOverlapsWithArr { ident }
            | Self::DimOneAllElsEq { ident }
            | Self::DimOneAllElsGreaterThan { ident }
            | Self::DimOneBtwn { ident }
            | Self::DimOneContainsAllElsOfArr { ident }
            | Self::DimOneContainsElGreaterThan { ident }
            | Self::DimOneEq { ident }
            | Self::DimOneGreaterThan { ident }
            | Self::DimOneIn { ident }
            | Self::DimOneOverlapsWithArr { ident }
            | Self::DimThreeAllElsEq { ident }
            | Self::DimThreeAllElsGreaterThan { ident }
            | Self::DimThreeBtwn { ident }
            | Self::DimThreeContainsAllElsOfArr { ident }
            | Self::DimThreeContainsElGreaterThan { ident }
            | Self::DimThreeEq { ident }
            | Self::DimThreeGreaterThan { ident }
            | Self::DimThreeIn { ident }
            | Self::DimThreeOverlapsWithArr { ident }
            | Self::DimTwoAllElsEq { ident }
            | Self::DimTwoAllElsGreaterThan { ident }
            | Self::DimTwoBtwn { ident }
            | Self::DimTwoContainsAllElsOfArr { ident }
            | Self::DimTwoContainsElGreaterThan { ident }
            | Self::DimTwoEq { ident }
            | Self::DimTwoGreaterThan { ident }
            | Self::DimTwoIn { ident }
            | Self::DimTwoOverlapsWithArr { ident }
            | Self::Eq { ident }
            | Self::GreaterThan { ident }
            | Self::In { ident }
            | Self::OverlapsWithArr { ident } => Some(ident),
            Self::AllElsRgx
            | Self::ContainsElRgx
            | Self::DimFourAllElsRgx
            | Self::DimFourContainsElRgx
            | Self::DimFourLenEq
            | Self::DimFourLenGreaterThan
            | Self::DimFourRgx
            | Self::DimOneAllElsRgx
            | Self::DimOneContainsElRgx
            | Self::DimOneLenEq
            | Self::DimOneLenGreaterThan
            | Self::DimOneRgx
            | Self::DimThreeAllElsRgx
            | Self::DimThreeContainsElRgx
            | Self::DimThreeLenEq
            | Self::DimThreeLenGreaterThan
            | Self::DimThreeRgx
            | Self::DimTwoAllElsRgx
            | Self::DimTwoContainsElRgx
            | Self::DimTwoLenEq
            | Self::DimTwoLenGreaterThan
            | Self::DimTwoRgx
            | Self::LenEq
            | Self::LenGreaterThan
            | Self::Rgx => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn json_dimension_constructor_maps_zero_and_two() -> Result<(), String> {
        let ident = quote::quote! { TestIdent };
        let zero_filter =
            crate::flts::PgJsonFlt::dim_eq(crate::flts::PgJsonFilterDimension::Zero, ident.clone());
        let two_filter =
            crate::flts::PgJsonFlt::dim_eq(crate::flts::PgJsonFilterDimension::Two, ident);
        if matches!(zero_filter, crate::flts::PgJsonFlt::Eq { .. })
            && matches!(two_filter, crate::flts::PgJsonFlt::DimTwoEq { .. })
        {
            return Ok(());
        }
        Err("unexpected json dimension filter variant".to_owned())
    }

    #[test]
    fn filter_trait_returns_generic_payload_for_token_filters() -> Result<(), String> {
        let filter = crate::flts::PgTypeFlt::Eq {
            ident: quote::quote! { SomeIdent },
        };
        let Some(generic) = crate::flts::PgFlt::maybe_generic(&filter) else {
            return Err("expected generic token payload".to_owned());
        };
        if generic.to_string() == "SomeIdent"
            && crate::flts::PgFlt::filter_name(&filter) == crate::flts::PgFilterName::Eq
        {
            return Ok(());
        }
        Err(generic.to_string())
    }
}
