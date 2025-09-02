use clap::{App, Arg, ArgMatches};
use std::num::ParseIntError;

fn get_cmd_line_input() -> ArgMatches<'static> {
    let matches = App::new("bbshark")
        .version("1.0.0")
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
        .arg(
            Arg::with_name("relative")
            .value_name("RELATIVE")
            .help("The relative (e.g. 'Baby').")
            .required(false)
            .default_value("Baby")
            .short("r")
            .long("relative")
                .max_values(1)
        )
        .arg(
            Arg::with_name("animal")
            .value_name("ANIMAL")
            .help("The animal (e.g. 'shark').")
            .required(false)
            .default_value("shark")
            .short("a")
            .long("animal")
            .max_values(1)
        )
        .arg(
            Arg::with_name("word")
            .value_name("WORD")
            .help("The word to repeat (e.g. 'doo').")
            .required(false)
            .default_value("doo")
            .short("w")
            .long("word")
            .max_values(1)
        )
        .arg(
            Arg::with_name("sep")
            .value_name("SEP")
            .help("The separator to use between words (e.g. ',').")
            .required(false)
            .default_value(",")
            .short("s")
            .long("sep")
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

fn get_number_of_doos(matches: ArgMatches<'static>) -> u8 {
    let binding = matches.values_of_lossy("doos").unwrap();
    let doos = binding.first().unwrap();
    let number_result = doos.parse::<u8>();
    let number = check_is_u8(number_result, doos);
    return check_number_is_not_zero(number, doos)
}

fn print_word(word: &str, number: u8, sep: &str) {
    let mut i = 0;
    while i < number {
        print!("{}", word);
        i += 1;
        if i < number {
            print!("{} ", sep);
        }
    }
}

fn get_arg_value(matches: ArgMatches<'static>, arg_name: &str) -> String {
    let binding = matches.values_of_lossy(arg_name).unwrap();
    let result = binding.first().unwrap();
    let word_result = result.parse::<String>();
    let word = match word_result {
        Ok(wording) => wording,
        Err(_error) => {
            std::process::exit(2)  // Exit with a non-zero status to indicate error
        }
    };
    return word
}


fn main() {
    let matches = get_cmd_line_input();
    let number = get_number_of_doos(matches.clone());
    let animal = get_arg_value(matches.clone(), "animal");
    let relative = get_arg_value(matches.clone(), "relative");
    let sep = get_arg_value(matches.clone(), "sep");
    let word = get_arg_value(matches.clone(), "word");
    print!("{} {}... ", relative, animal);
    print_word(&word, number, &sep);
    print!(".\n");
    print!("{} {}!", relative, animal);
    std::process::exit(0)  // Exit with a zero status to indicate success
}
