use clap::{App, Arg, ArgMatches};
use std::num::ParseIntError;

fn get_cmd_line_input() -> ArgMatches<'static> {
    let matches = App::new("bbshark")
        .version("0.1.2")
        .author("https://github.com/dunlopWill")
        .about("Rust baby shark")
        .arg(
            Arg::with_name("doos")
                .value_name("DOOS")
                .help("Number of 'doo's (between 1 and 127)")
                .required(false)
                .default_value("6")
                .short("d")
                .long("doos")
                .max_values(1)
        )
        .get_matches();
    return matches
    }

fn check_is_u8(number_result: Result<u8, ParseIntError>, doos: &String) -> u8 {
    let number = match number_result {
        Ok(num) => num,
        Err(_error) => {
            eprintln!("Must enter an unsigned integer between 1 and 127 (u8). Got '{}'", doos);
            std::process::exit(1)  // Exit with a non-zero status to indicate error
        },
    };
    return number
}

fn check_number_is_not_zero(number:u8, doos: &String) -> u8 {
    if number == 0 {
        eprintln!("Must enter an unsigned integer between 1 and 127 (u8). Got '{}'", doos);
        std::process::exit(1)  // Exit with a non-zero status to indicate error
    };
    return number
} 

fn get_number_of_doos() -> u8 {
    let matches = get_cmd_line_input();
    let binding = matches.values_of_lossy("doos").unwrap();
    let doos = binding.first().unwrap();
    let number_result = doos.parse::<u8>();
    let number = check_is_u8(number_result, doos);
    return check_number_is_not_zero(number, doos)
}

fn print_doos_times(number: u8) {
    print!("Baby shark... ");
    let mut i = 0;
    while i < number {
        print!("doo");
        i += 1;
        if i < number {
            print!(", ");
        }
    }
    print!(".\n");
    print!("Baby shark!");
}

fn main() {
    let number = get_number_of_doos();
    print_doos_times(number);
    std::process::exit(0)  // Exit with a zero status to indicate success
}
