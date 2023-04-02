use pulldown_cmark::{Event, Parser, Tag};
use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type RunResult<T> = Result<T, Box<dyn Error>>;

pub fn run(sheets: Vec<String>, list: bool) -> RunResult<()> {
    if sheets.len() <= 0 {
        return Err(From::from("Zero Sheets to find."));
    }

    if let Some(file_sheet) = sheets.get(0) {
        let path = get_file_from_name(file_sheet.to_string()).into_owned();
        match open_file(&path) {
            Err(err) => eprintln!("{}: {}", path, err),
            Ok(file) => process_sheet(file),
        }
    }

    Ok(())
}

fn get_file_from_name<'a>(name: String) -> Cow<'a, str> {
    let path = format!("./tests/inputs/{}.md", name);
    Cow::Owned(path)
}

fn open_file(filename: &str) -> RunResult<Box<dyn BufRead>> {
    match filename {
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn process_sheet(reader: Box<dyn BufRead>) {
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
