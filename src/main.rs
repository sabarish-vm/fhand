use objc2_app_kit::NSWorkspace;

mod cli;
mod data;
mod helper;
mod output;

use crate::{
    cli::{build_parser, handle_get},
    data::Extdata,
    output::output_get,
};

#[cfg(target_os = "macos")]
mod macos;

fn main() {
    let workspace = NSWorkspace::sharedWorkspace();
    let mut parser = build_parser();
    let matches = parser.get_matches_mut();

    match matches.subcommand() {
        Some(("get", get_matches)) => {
            let ret = handle_get(get_matches);
            output_get(ret, &workspace);
        }
        Some(("set", _)) => {
            todo!("set_matches");
        }
        _ => {
            let _ = parser.print_help();
        }
    }
}
