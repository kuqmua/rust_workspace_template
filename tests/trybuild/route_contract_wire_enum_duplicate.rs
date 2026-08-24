#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(newtype::WireEnum)]
#[wire_enum(ref_type = str, error_message = "invalid value")]
enum DuplicateWireValue {
    #[wire("same")]
    First,
    #[wire("same")]
    Second,
}

fn main() {}
