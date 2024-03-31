use clap::{Arg, ArgAction, Command};
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::{error, warn};
use owo_colors::colored::*;

use std::{
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    process,
};

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!(
            "{} {} {} {}",
            "Received Ctrl-C!".bold().red(),
            "🤬",
            "Exit program!".bold().red(),
            "☠",
        );
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    // get config dir
    let config_dir = check_create_config_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a config directory: {err}");
        process::exit(1);
    });

    // initialize the logger
    let _logger = Logger::try_with_str("info") // log warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(
            FileSpec::default()
                .directory(&config_dir)
                .suppress_timestamp(),
        ) // change directory for logs, no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    // handle arguments
    let matches = minisort().get_matches();
    let reverse_flag = matches.get_flag("reverse");
    let numerical_flag = matches.get_flag("numerical");
    let string_flag = matches.get_flag("string");

    if let Some(_) = matches.subcommand_matches("log") {
        if let Ok(logs) = show_log_file(&config_dir) {
            println!("{}", "Available logs:".bold().yellow());
            println!("{}", logs);
        } else {
            error!("Unable to read logs");
            process::exit(1);
        }
    } else {
        let mut file = PathBuf::new();
        if let Some(arg) = matches.get_one::<String>("arg") {
            // get filepath
            file.push(&arg);
        } else {
            // TODO remove later
            // let _ = peakfile().print_help();
            // process::exit(0);

            // read input from pipe
            let pipe_input = read_pipe();
            file.push(pipe_input);
        }

        let path = file.as_path();

        if !path.exists() {
            warn!("Path '{}' doesn`t exist", path.display());
            process::exit(0);
        }

        if !path.is_file() {
            warn!("Path '{}' is not a file", path.display());
            process::exit(0);
        }

        // read content from file
        let mut content = String::new();
        let file_content = fs::read_to_string(path).unwrap_or_else(|err| {
            match err.kind() {
                io::ErrorKind::InvalidData => {
                    warn!("Path \'{}\' contains invalid data: {}", path.display(), err)
                }
                io::ErrorKind::NotFound => {
                    warn!("Path \'{}\' not found: {}", path.display(), err);
                }
                io::ErrorKind::PermissionDenied => {
                    warn!(
                        "Missing permission to read path \'{}\': {}",
                        path.display(),
                        err
                    )
                }
                _ => {
                    error!(
                        "Failed to access path: \'{}\'\nUnexpected error occurred: {}",
                        path.display(),
                        err
                    )
                }
            }
            process::exit(0);
        });

        content.push_str(&file_content);

        let lines = split_content_by_lines(content);

        let mut sorted_content = Vec::new();
        if numerical_flag {
            let sorted_vec = sort_only_numbers(lines);
            sorted_content.push(sorted_vec);
        } else if string_flag {
            let sorted_vec = sort_all_as_string(lines);
            sorted_content.push(sorted_vec);
        } else {
            // TODO for testing -> change default later
            let sorted_vec = sort(lines);
            sorted_content.push(sorted_vec);
        }

        if reverse_flag {
            sorted_content
                .iter()
                .flatten()
                .rev()
                .for_each(|l| println!("{}", l));
        } else {
            sorted_content
                .iter()
                .flatten()
                .for_each(|l| println!("{}", l));
        }
    }
}

fn read_pipe() -> String {
    let mut input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");

    let _ = input.pop();

    input.trim().to_string()
}

fn split_content_by_lines(content: String) -> Vec<String> {
    content.lines().map(|l| l.to_string()).collect()
}

fn sort(mut content: Vec<String>) -> Vec<String> {
    // interpret everything as a literal string
    // sorts numbers (as strings) first, than words
    content.sort_by(|a, b| a.cmp(&b));
    content
}

fn sort_all_as_string(mut content: Vec<String>) -> Vec<String> {
    // interpret everything as a literal string
    // sort numbers first, than words
    content.sort_by(|a, b| a.cmp(&b));
    content
}

fn sort_only_numbers(mut content: Vec<String>) -> Vec<String> {
    // INFO only sorts integers
    // INFO i64::MAX == 9223372036854775807
    // sort only the integers in the file and print at the beginning of the file
    content.sort_by_cached_key(|k| k.parse::<i64>().unwrap_or(i64::MAX));
    content
}

// build cli
fn minisort() -> Command {
    Command::new("ms")
        .bin_name("ms")
        .before_help(format!(
            "{}\n{}",
            "MS".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        .about("Mini Sort")
        .before_long_help(format!(
            "{}\n{}",
            "MS".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        .long_about(format!(
            "{}\n\n{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n",
            "Mini Sort",
            "Sort file content or output from a previous command",
            "- alphabetical",
            "- numerical",
            "- reverse",
            "- random",
            "- by month",
            "- remove duplicates"
        ))
        // TODO update version
        .version("1.0.0")
        .author("Leann Phydon <leann.phydon@gmail.com>")
        .arg(
            Arg::new("arg")
                .help("The filepath to work with")
                .action(ArgAction::Set)
                .num_args(1)
                .value_name("PATH"),
        )
        .arg(
            Arg::new("numerical")
                .short('n')
                .long("numerical")
                .help("Sort file content numerical")
                .long_help(format!(
                    "{}\n{}\n{}",
                    "Sort file content numerical",
                    "WARNING: only sorts integers, no floating point numbers",
                    "WARNING: only sorts integers up to 9223372036854775806",
                ))
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("string")
                .short('s')
                .long("string")
                .help("Sort file content. Interpret everything as a literal string")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("reverse")
                .short('r')
                .long("reverse")
                .help("Reverse sort file content")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .long_flag("log")
                .about("Show content of the log file"),
        )
}

fn check_create_config_dir() -> io::Result<PathBuf> {
    let mut new_dir = PathBuf::new();
    match dirs::config_dir() {
        Some(config_dir) => {
            new_dir.push(config_dir);
            new_dir.push("ms");
            if !new_dir.as_path().exists() {
                fs::create_dir(&new_dir)?;
            }
        }
        None => {
            error!("Unable to find config directory");
        }
    }

    Ok(new_dir)
}

fn show_log_file(config_dir: &PathBuf) -> io::Result<String> {
    let log_path = Path::new(&config_dir).join("ms.log");
    return match log_path.try_exists()? {
        true => Ok(format!(
            "{} {}\n{}",
            "Log location:".italic().dimmed(),
            &log_path.display(),
            fs::read_to_string(&log_path)?
        )),
        false => Ok(format!(
            "{} {}",
            "No log file found:"
                .truecolor(250, 0, 104)
                .bold()
                .to_string(),
            log_path.display()
        )),
    };
}
