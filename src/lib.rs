//use clap::{App, Arg};
use std::error::Error;

//type InputResult<T> = Result<T, Box<dyn Error>>;

pub struct Args {
    inputs: Vec<String>,
    list: bool,
}

// pub fn arg_parse() -> InputResult<Args> {
//     let inputs_arg = Arg::new("inputs")
//         .value_name("TEXT")
//         .help("Cheat Sheet Inputs")
//         .required(true)
//         .min_values(1);

//     let list_arg = Arg::new("list")
//         .short("l")
//         .long("list")
//         .value_name("LIST")
//         .help("list sheets")
//         .takes_value(false);

//     let matches = App::new("cheatsheet")
//         .version("0.0.1")
//         .author("jheryer")
//         .about("Cheat Sheet Cli")
//         .arg(inputs_arg)
//         .arg(list_arg)
//         .get_matches();

//     let inputs = matches.values_of_lossy("inputs").unwrap();
//     let list = matches.is_present("list").unwrap();
//     Ok(Args {
//         inputs: inputs,
//         list: list,
//     })
// }
