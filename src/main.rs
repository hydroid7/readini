use ini::Ini;
use std::{env};
use std::process::exit;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("No arguments supplied. \nUsage: \n    {} /path/to/ini <section> <key> [optional default]", args[0]);
        exit(0);
    }

    let ini_path = &args[1];
    let section = &args[2];
    let key = &args[3];


    let ini_config = Ini::load_from_file(ini_path).unwrap();


    let value = if args.len() == 5 {
        let default = &args[4];
        ini_config.get_from_or(Option::Some(section), key, default)
    } else {
        ini_config.get_from(Option::Some(section), key).unwrap()
    };
    print!("{}", value);
}
