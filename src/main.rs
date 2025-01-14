use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use xdrk::Run;

mod commands;

fn main() {
    let matches = command!() // requires `cargo` feature
        .subcommand_required(true)
        .about("XRK Data Reader")
        .arg(
            arg!(
                -f --file <FILE> "Data file to load"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <OUTPUT> "Output, can be <text> or <json>"
            )
            .required(false)
            .default_value("text")
            .value_parser(value_parser!(String)),
        )
        .arg(arg!(
            -v --verbose ... "Enable verbose logging"
        ))
        .subcommand(Command::new("info").about("Get session info"))
        .subcommand(Command::new("lap").about("Preview single lap data for all channels"))
        .subcommand(
            Command::new("laps").about("Print lap timings"), // .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("channels").about("Preview data from all channels").arg(
                Arg::new("preview")
                    .long("preview")
                    .help("Give a small preview of datapoints available in channel")
                    .action(ArgAction::SetTrue),
            ),
        )
        .subcommand(
            Command::new("export").about("Export channel data").arg(
                Arg::new("channels")
                    .short('c')
                    .long("channels")
                    // .takes_value(true)
                    .value_name("CHANNELS")
                    .help("Comma-separated list of channels to export (e.g., \"Logger Temperature\",P_BRK_FRONT)"),
            ),
        )
        .get_matches();

    let data_file = matches
        .get_one::<std::path::PathBuf>("file")
        .expect("required");

    // println!("Using output mode: {output_mode}");

    // match matches
    //     .get_one::<u8>("verbose")
    //     .expect("Counts are defaulted")
    // {
    //     0 => println!(),
    //     1 => println!("Verbose logging enabled"),
    //     _ => println!("Extra verbose logging enabled"),
    // }

    let file_path = Path::new(&data_file);
    if !file_path.exists() {
        eprintln!("Error: The file '{}' does not exist.", file_path.display());
        std::process::exit(1);
    }

    if let Some(matches) = matches.subcommand_matches("info") {
        match Run::load(file_path) {
            Ok(run) => {
                commands::info::display_run_info(&run);
            }
            Err(err) => {
                eprintln!("Failed to load: {}", err);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("laps") {
        match Run::load(file_path) {
            Ok(run) => {
                commands::laps::display_laps_info(&run);
            }
            Err(err) => {
                eprintln!("Failed to load: {}", err);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("channels") {
        let preview_enabled = matches.get_flag("preview");

        match Run::load(file_path) {
            Ok(run) => {
                commands::channels::display_channels_list(&run, preview_enabled);
            }
            Err(err) => {
                eprintln!("Failed to load: {}", err);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("lap") {
        match Run::load(file_path) {
            Ok(run) => {
                commands::lap::display_run_info(&run);
            }
            Err(err) => {
                eprintln!("Failed to load: {}", err);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("export") {
        eprintln!("Loading data from file");

        let desired_channels: Option<HashSet<&str>> =
            matches.get_one::<String>("channels").map(|channels_str| {
                channels_str
                    .split(',')
                    .map(|s| s.trim())
                    .collect::<HashSet<&str>>()
            });

        match Run::load(file_path) {
            Ok(run) => {
                commands::export::export(&run, desired_channels);
            }
            Err(err) => {
                eprintln!("Failed to load: {}", err);
            }
        }
    }
}
