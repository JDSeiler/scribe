use std::env;
use std::process::exit;

fn main() {
    let args = read_args();
    println!("{:#?}", args);
}

fn read_args() -> Option<Args> {
    let mut raw_args: Vec<String> = env::args().collect();
    // Discard the first argument, it's the program name and we don't care
    raw_args.remove(0);

    if raw_args.len() == 1 {
        match raw_args.get(0) {
            Some(option) => {
                if option.eq("--help") {
                    print_help_and_exit()
                } else {
                    println!("Unknown option {:#?}", option);
                    print_help_and_exit();
                }
            },
            None => {
                print_help_and_exit();
            }
        }
    } else if raw_args.len() > 1 && raw_args.len() < 4 {
        println!("Missing required arguments!");
        print_help_and_exit();
    } else {
       match &raw_args[..] {
           [source, label, raw_start, raw_quantity] => {
               let start_from: u64 = raw_start.parse::<u64>().expect("Could not parse argument start_from to a u64");
               let quantity: u64 = raw_quantity.parse::<u64>().expect("Could not parse argument quantity to a u64");
               return Some(Args {
                   input_source: source.to_owned(),
                   input_labels: label.to_owned(),
                   start_from,
                   quantity
               });
           },
           _ => {
               eprintln!("Incorrect number of arguments, expected 4, got {:#?}", raw_args.len());
           }
       }
    }
    None
}

fn print_help_and_exit() {
    println!("Scribe, a tool for converting MNIST data into BMP images.");
    println!("Usage:");
    println!("scribe [input_source] [input_labels] [start_from] [quantity]");
    println!("    input_source: Path to the data file to read from");
    println!("    input_labels: Path to the corresponding label file");
    println!("    start_from: Natural number describing which image to start from");
    println!("    quantity: Number of images to read, starting from `start_from`");
    println!("Use the --help option to print this message");
    exit(0);
}

#[derive(Debug)]
struct Args {
    input_source: String,
    input_labels: String,
    start_from: u64,
    quantity: u64
}
