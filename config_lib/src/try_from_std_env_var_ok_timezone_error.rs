#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum TryFromStdEnvVarOkTimezoneError {
    #[error("{chrono_fixed_offset:?}")]
    ChronoFixedOffset {
        chrono_fixed_offset: crate::chrono_fixed_offset_error::ChronoFixedOffsetError,
    },
    #[error("{i32_parsing:?}")]
    I32Parsing {
        i32_parsing: crate::i32_parse_int_error::I32ParseIntError,
    },
}
