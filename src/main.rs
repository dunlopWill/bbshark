use clap::{App, Arg};

fn main() {
    let matches = App::new("baby-shark")
        .version("0.1.0")
        .author("dunlopWill.github.io")
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
    let binding = matches.values_of_lossy("doos").unwrap();
    let doos = binding.first().unwrap();
    let number_result = doos.parse::<u8>();
    let number = match number_result {
        Ok(num) => num,
        Err(_error) => {
            eprintln!("Must enter an unsigned integer between 1 and 127 (u8). Got '{}'", doos);
            std::process::exit(1)  // Exit with a non-zero status to indicate error
        },
    };
    if number == 0 {
        eprintln!("Must enter an unsigned integer between 1 and 127 (u8). Got '{}'", doos);
        std::process::exit(1)  // Exit with a non-zero status to indicate error
    }
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
    std::process::exit(0)  // Exit with a zero status to indicate success
}
