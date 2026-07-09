const CARGO_FMT_CHECK_ARGS: [&str; 2] = ["fmt", "--check"];
const CARGO_CLIPPY_ARGS: [&str; 6] = [
    "clippy",
    "--all-targets",
    "--all-features",
    "--",
    "-D",
    "warnings",
];
const CARGO_TEST_STYLE_ARGS: [&str; 4] = ["test", "-p", "tests", "--lib"];
const CARGO_TEST_GEN_PG_TBL_ARGS: [&str; 5] =
    ["test", "-p", "gen_pg_tbl_test", "--features", "test-utils"];
const CARGO_TEST_GEN_PG_TYPES_ARGS: [&str; 5] = [
    "test",
    "-p",
    "gen_pg_types_test",
    "--features",
    "test-utils",
];
const CARGO_TEST_GEN_WH_FLTS_ARGS: [&str; 5] =
    ["test", "-p", "gen_wh_flts_test", "--features", "test-utils"];
const STATIC_COMMANDS: [(&str, &[&str]); 3] = [
    ("cargo", &CARGO_FMT_CHECK_ARGS),
    ("cargo", &CARGO_CLIPPY_ARGS),
    ("cargo", &CARGO_TEST_STYLE_ARGS),
];
const MACRO_GENERATION_COMMANDS: [(&str, &[&str]); 3] = [
    ("cargo", &CARGO_TEST_GEN_PG_TBL_ARGS),
    ("cargo", &CARGO_TEST_GEN_PG_TYPES_ARGS),
    ("cargo", &CARGO_TEST_GEN_WH_FLTS_ARGS),
];
fn main() {
    let mode = std::env::args().nth(1);
    let run_commands: fn(&[(&str, &[&str])]) -> Result<(), ()> = |commands| {
        commands.iter().try_fold(
            (),
            |(), (program, args)| match std::process::Command::new(program).args(*args).status() {
                Ok(status) if status.success() => Ok(()),
                Ok(status) => {
                    eprintln!("command failed: {program} {args:?}: {status}");
                    Err(())
                }
                Err(error) => {
                    eprintln!("failed to spawn command: {program} {args:?}: {error}");
                    Err(())
                }
            },
        )
    };
    let result = match mode.as_deref() {
        None | Some("static") => run_commands(&STATIC_COMMANDS),
        Some("macro-generation") => run_commands(&MACRO_GENERATION_COMMANDS),
        Some("all") => {
            run_commands(&STATIC_COMMANDS).and_then(|()| run_commands(&MACRO_GENERATION_COMMANDS))
        }
        Some(other) => {
            eprintln!("unknown mode `{other}`; expected `static`, `macro-generation`, or `all`");
            Err(())
        }
    };
    if result.is_err() {
        std::process::exit(1);
    }
}
