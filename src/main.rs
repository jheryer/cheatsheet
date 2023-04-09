use clap::Parser as CLAPParser;

#[derive(CLAPParser)]
#[command(author,version,about,long_about=None)]
pub struct Args {
    sheets: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    list: bool,
}

fn main() {
    let sheet_path = "tests/inputs/";
    let args = Args::parse();

    if args.sheets.len() == 0 && args.list == true {
        if let Err(e) = cheatsheet::list(sheet_path) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    } else {
        if let Err(e) = cheatsheet::run(args.sheets, args.list) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
