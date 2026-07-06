use naming::prm::PgTypeWhSelfUcc;
use naming::{
    AdjacentWithRangeUcc, BeforeUcc, BtwnUcc, CrntDateUcc, CrntTimeUcc, CrntTimestampUcc,
    DisplayPlusToTokens, EqToEncodedStringRepresentationUcc, EqUcc, ExcludedUpperBoundUcc,
    FindRangesThatFullyContainTheGivenRangeUcc, FindRangesWithinGivenRangeUcc,
    GreaterThanCrntDateUcc, GreaterThanCrntTimeUcc, GreaterThanCrntTimestampUcc,
    GreaterThanExcludedUpperBoundUcc, GreaterThanIncludedLowerBoundUcc, GreaterThanUcc, InUcc,
    IncludedLowerBoundUcc, OverlapWithRangeUcc, RangeLenUcc, RgxUcc, StrictlyToLeftOfRangeUcc,
    StrictlyToRightOfRangeUcc,
};
use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use strum_macros::{Display, EnumIter};
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Display, EnumIter, Optml)]
pub enum PgTypeFlt {
    Eq { ident: Ts2 },
    GreaterThan { ident: Ts2 },
    Btwn { ident: Ts2 },
    In { ident: Ts2 },
    Rgx,
    Before { ident: Ts2 },
    CrntDate,
    GreaterThanCrntDate,
    CrntTimestamp,
    GreaterThanCrntTimestamp,
    CrntTime,
    GreaterThanCrntTime,
    EqToEncodedStringRepresentation,
    FindRangesWithinGivenRange { ident: Ts2 },
    FindRangesThatFullyContainTheGivenRange { ident: Ts2 },
    StrictlyToLeftOfRange { ident: Ts2 },
    StrictlyToRightOfRange { ident: Ts2 },
    IncludedLowerBound { ident: Ts2 },
    ExcludedUpperBound { ident: Ts2 },
    GreaterThanIncludedLowerBound { ident: Ts2 },
    GreaterThanExcludedUpperBound { ident: Ts2 },
    OverlapWithRange { ident: Ts2 },
    AdjacentWithRange { ident: Ts2 },
    RangeLen,
    //BitVecPositionEq,//currently deactivated
}
impl PgFlt for PgTypeFlt {
    fn mb_generic(&self) -> Option<Ts2> {
        match &self {
            Self::Eq { ident }
            | Self::GreaterThan { ident }
            | Self::Btwn { ident }
            | Self::In { ident }
            | Self::Before { ident }
            | Self::FindRangesWithinGivenRange { ident }
            | Self::FindRangesThatFullyContainTheGivenRange { ident }
            | Self::StrictlyToLeftOfRange { ident }
            | Self::StrictlyToRightOfRange { ident }
            | Self::IncludedLowerBound { ident }
            | Self::ExcludedUpperBound { ident }
            | Self::GreaterThanIncludedLowerBound { ident }
            | Self::GreaterThanExcludedUpperBound { ident }
            | Self::OverlapWithRange { ident }
            | Self::AdjacentWithRange { ident } => Some(ident.clone()),
            Self::Rgx
            | Self::CrntDate
            | Self::GreaterThanCrntDate
            | Self::CrntTimestamp
            | Self::GreaterThanCrntTimestamp
            | Self::CrntTime
            | Self::GreaterThanCrntTime
            | Self::EqToEncodedStringRepresentation
            | Self::RangeLen => None,
        }
    }
    fn prefix_wh_self_ucc(&self) -> Ts2 {
        let v = PgTypeWhSelfUcc::from_display(&self.ucc());
        quote! {#v}
    }
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens {
        match &self {
            Self::Eq { .. } => &EqUcc,
            Self::GreaterThan { .. } => &GreaterThanUcc,
            Self::Btwn { .. } => &BtwnUcc,
            Self::In { .. } => &InUcc,
            Self::Rgx => &RgxUcc,
            Self::Before { .. } => &BeforeUcc,
            Self::CrntDate => &CrntDateUcc,
            Self::GreaterThanCrntDate => &GreaterThanCrntDateUcc,
            Self::CrntTimestamp => &CrntTimestampUcc,
            Self::GreaterThanCrntTimestamp => &GreaterThanCrntTimestampUcc,
            Self::CrntTime => &CrntTimeUcc,
            Self::GreaterThanCrntTime => &GreaterThanCrntTimeUcc,
            Self::EqToEncodedStringRepresentation => &EqToEncodedStringRepresentationUcc,
            Self::FindRangesWithinGivenRange { .. } => &FindRangesWithinGivenRangeUcc,
            Self::FindRangesThatFullyContainTheGivenRange { .. } => {
                &FindRangesThatFullyContainTheGivenRangeUcc
            }
            Self::StrictlyToLeftOfRange { .. } => &StrictlyToLeftOfRangeUcc,
            Self::StrictlyToRightOfRange { .. } => &StrictlyToRightOfRangeUcc,
            Self::IncludedLowerBound { .. } => &IncludedLowerBoundUcc,
            Self::ExcludedUpperBound { .. } => &ExcludedUpperBoundUcc,
            Self::GreaterThanIncludedLowerBound { .. } => &GreaterThanIncludedLowerBoundUcc,
            Self::GreaterThanExcludedUpperBound { .. } => &GreaterThanExcludedUpperBoundUcc,
            Self::OverlapWithRange { .. } => &OverlapWithRangeUcc,
            Self::AdjacentWithRange { .. } => &AdjacentWithRangeUcc,
            Self::RangeLen => &RangeLenUcc,
        }
    }
}
pub trait PgFlt {
    fn mb_generic(&self) -> Option<Ts2>;
    fn prefix_wh_self_ucc(&self) -> Ts2;
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens;
}
