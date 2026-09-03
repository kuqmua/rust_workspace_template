#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(proc_macro_newtype::WireEnum)]
#[wire_enum(ref_type = str, error_message = "invalid value")]
enum NonUnitWireValue {
    #[wire("value")]
    Value(frontend_contract::contract_str::ContractStr),
}

fn main() {
    let _arguments = std::env::args_os();
}
