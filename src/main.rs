//! sach

/// help constants
const HELP: &'static str = r#"
USAGE
    sach <command>

COMMANNDS
    edit    Edit config            [aliases: e]
    path    Prints the config path [aliases: p]
    help    Prints this message    [aliases: h]

NOTE
    Just use `sach` without any command will attach to your substrate service directly.
"#;

/// prints help
fn help() {
    println!("{}", HELP);
}

/// entry
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let cmd = &args[1];
    println!("{:#?}", args);
}
