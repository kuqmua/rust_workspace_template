use std::env::var_os;

fn main() {
    drop(var_os("RUST_WORKSPACE_TEMPLATE_ENTRYPOINT_MARKER"));
}
