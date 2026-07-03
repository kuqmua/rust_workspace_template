#![forbid(unsafe_code)]

fn main() {
    let _location_marker: loc_lib::LocationDeriveAvailable =
        core::hint::black_box(loc_lib::LocationDeriveAvailable);
}
