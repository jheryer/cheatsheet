use clap::Parser as CLAPParser;

/*

TODO AC
https://dockerlabs.collabnix.com/docker/cheatsheet/
help: cheatsheet -h --help
list sheets: cheatsheet -l
show commands all: cheatsheet docker
show commands sub section: cheatsheet docker run_container
list sections for sheet: cheatsheet docker -l

install cheetsheat
default directory ~/.cheatsheet/files.md

 */

#[derive(CLAPParser)]
#[command(author,version,about,long_about=None)]
pub struct Args {
    #[arg(required = true)]
    sheets: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    list: bool,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = cheatsheet::run(args.sheets, args.list) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
