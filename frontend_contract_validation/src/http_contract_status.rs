#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, newtype::TryFrom,
)]
#[try_from(
    error = frontend_contract::HttpStatusTryFromU16Error,
    validator = |value: &u16| {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(frontend_contract::HttpStatusTryFromU16Error)
        }
    }
)]
pub struct HttpContractStatus(u16);
