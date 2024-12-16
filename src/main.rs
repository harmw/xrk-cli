use clap::{Command, arg, command, value_parser};
use std::path::Path;
use std::path::PathBuf;
use xdrk::Run;

mod commands;

fn main() {
    let matches = command!() // requires `cargo` feature
        .subcommand_required(true)
        .about("AiM XRK Data Reader")
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
        .subcommand(
            Command::new("laps").about("Print lap timings"), // .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .subcommand(Command::new("channels").about("Get data channel info"))
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
        match Run::load(file_path) {
            Ok(run) => {
                commands::channels::display_channels_list(&run);
            }
            Err(err) => {
                eprintln!("Failed to load: {}", err);
            }
        }
    }
}
