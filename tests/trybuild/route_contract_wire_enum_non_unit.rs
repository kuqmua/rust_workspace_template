#[derive(newtype::WireEnum)]
#[wire_enum(ref_type = str, error_message = "invalid value")]
enum NonUnitWireValue {
    #[wire("value")]
    Value(frontend_contract::ContractStr),
}

fn main() {}
