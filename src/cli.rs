use anyhow::Context;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::data::GetArgs;

pub fn build_parser() -> Command {
    Command::new("defapp")
        .author("Sabarish, github.com/sabarish-vm")
        .about("File handler : Check which application handles certain file types by default")
        // ---------- get ----------
        .subcommand(
            Command::new("get")
                .about("get file handling data for the specified extensions")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("ext")
                        .short('e')
                        .long("ext")
                        .required(true)
                        .num_args(1..)
                        .help("List of extensions space seperated"),
                )
                .arg(
                    Arg::new("json_out")
                        .required(false)
                        .action(ArgAction::SetTrue)
                        .long("json")
                        .help("Using this flag to output the information as json"),
                ),
        )
        // ---------- set ----------
        .subcommand(
            Command::new("set")
                .about("set default application for a given extension")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("ext")
                        .short('e')
                        .long("ext")
                        .required(true)
                        .num_args(1..)
                        .help("Extension for which the default needs to be set"),
                )
                .arg(
                    Arg::new("application")
                        .short('a')
                        .long("app")
                        .required(true)
                        .num_args(1)
                        .help("The path to the application that needs to be set as default"),
                ),
        )
        // ---------- completions ----------
        .subcommand(
            Command::new("completions")
                .about("Generate shell autocompletion scripts")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("shell")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(clap::value_parser!(clap_complete::Shell)),
                ),
        )
}

pub fn handle_get(matches: &ArgMatches) -> Result<GetArgs, anyhow::Error> {
    let extensions = matches
        .get_many::<String>("ext")
        .context("Missing required --ext argument")?
        .cloned()
        .collect::<Vec<String>>();
    let json_needed = matches.get_flag("json_out");
    Ok(GetArgs {
        extensions,
        json_needed,
    })
}
