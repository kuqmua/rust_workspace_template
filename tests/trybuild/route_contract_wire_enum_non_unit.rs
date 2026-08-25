#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(newtype::WireEnum)]
#[wire_enum(ref_type = str, error_message = "invalid value")]
enum NonUnitWireValue {
    #[wire("value")]
    Value(frontend_contract::domain_types::ContractStr),
}

fn main() {}
