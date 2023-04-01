use clap::{Parser as CLAPParser, Subcommand};
use pulldown_cmark::{Event, Parser, Tag};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

type RunResult<T> = Result<T, Box<dyn Error>>;

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
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> RunResult<()> {
    dbg!(args.sheets);
    dbg!(args.list);
    Ok(())
}

fn old_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <markdown-file>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);

    let markdown = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let parser = Parser::new(&markdown);
    #[allow(unused)]
    let mut in_heading = false;
    let mut in_section = false;
    let mut current_level = 0;

    for event in parser {
        match event {
            #[allow(unused)]
            Event::Start(Tag::Heading(level, ..)) => {
                in_heading = true;
                in_section = true;
                current_level = level as i32;
            }
            #[allow(unused)]
            Event::End(Tag::Heading(..)) => {
                println!("\n");
                in_heading = false;
            }
            #[allow(unreachable_patterns)]
            Event::Start(Tag::Heading(new_level, ..))
                if in_section && new_level as i32 <= current_level =>
            {
                in_section = false;
            }
            Event::Text(text) => {
                if in_section {
                    print!("{}", text);
                }
            }
            _ => {}
        }
    }
}
