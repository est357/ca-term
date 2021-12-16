extern crate exitcode;
use std::process::exit;

extern crate clap;
use clap::{App, AppSettings, Arg};

/// Processes CLI arguments using clap
pub fn process_args() -> (u32, u32, u8, u64, u8, char) {
    let app = App::new("ca-term")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.0.1")
        .about("Description: Cellular automata for terminal")
        .author("Author: est357")
        .args(&[
            Arg::with_name("width")
                .help("Width of the screen.Number value < 256.")
                .short("w")
                .long("width")
                .default_value("200"),
            Arg::with_name("generations")
                .help("How many lines it should generate. Number value.")
                .short("g")
                .long("generations")
                .default_value("100"),
            Arg::with_name("rule")
                .help("The CA rule number according to https://mathworld.wolfram.com/ElementaryCellularAutomaton.html. Number value.")
                .short("r")
                .long("rule")
                .default_value("30"),
            Arg::with_name("interval_between_generations")
                .help("Time interval in us to wait bewtween generations. Number value.")
                .short("i")
                .long("interval")
                .default_value("200000"),
            Arg::with_name("initial_bit")
                .help("Initial bit 1 position. Between 0 and width value. Number value.")
                .short("b")
                .long("init_bit")
                .default_value("100"),
            Arg::with_name("display_character")
                .help("The character with which it will be drawn. Just 1 character.")
                .short("c")
                .long("character")
                .default_value("\u{25FC}"),
        ]);
    let mut app_help = app.clone();
    let matches = app.get_matches();

    let width: u32 = matches
        .value_of("width")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("ERROR: Value is not numeric or is more than max size !");
            app_help
                .print_help()
                .expect("Something is very wrong. This shouldn't have happened!");
            print!("\n");
            exit(exitcode::USAGE)
        });
    let generations: u32 = matches
        .value_of("generations")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("ERROR: Value is not numeric or is more than max size !");
            app_help
                .print_help()
                .expect("Something is very wrong. This shouldn't have happened!");
            print!("\n");
            exit(exitcode::USAGE)
        });
    let rule: u8 = matches
        .value_of("rule")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("ERROR: Value is not numeric or is more than max size !");
            app_help
                .print_help()
                .expect("Something is very wrong. This shouldn't have happened!");
            print!("\n");
            exit(exitcode::USAGE)
        });
    let interval_between_generations: u64 = matches
        .value_of("interval_between_generations")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("ERROR: Value is not numeric or is more than max size !");
            app_help
                .print_help()
                .expect("Something is very wrong. This shouldn't have happened!");
            print!("\n");
            exit(exitcode::USAGE)
        });
    let initial_bit: u8 = matches
        .value_of("initial_bit")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("ERROR: Value is not numeric or is more than max size !");
            app_help
                .print_help()
                .expect("Something is very wrong. This shouldn't have happened!");
            print!("\n");
            exit(exitcode::USAGE)
        });
    let arg_d_str = matches.value_of("display_character").unwrap();
    if arg_d_str.chars().count() > 1 {
        eprintln!("ERROR: Only one character is allowed for -d. Some unicode symbols are more than 1 character !");
        app_help
            .print_help()
            .expect("Something is very wrong. This shouldn't have happened!");
        print!("\n");
        exit(exitcode::USAGE)
    }
    let display_character: char = arg_d_str.chars().collect::<Vec<char>>()[0];
    (
        width,
        generations,
        rule,
        interval_between_generations,
        initial_bit,
        display_character,
    )
}
