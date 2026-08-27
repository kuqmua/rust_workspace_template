use super::{ChronoFixedOffsetError, I32ParseIntError};

#[derive(Debug, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout)]
pub enum TryFromStdEnvVarOkTimezoneError {
    #[error("{chrono_fixed_offset:?}")]
    ChronoFixedOffset {
        chrono_fixed_offset: ChronoFixedOffsetError,
    },
    #[error("{i32_parsing:?}")]
    I32Parsing { i32_parsing: I32ParseIntError },
}
