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
            Ok(file) => process_new_sheet(file),
        }
    }

    Ok(())
}

// test this
fn get_file_from_name<'a>(name: String) -> Cow<'a, str> {
    let path = format!("./tests/inputs/{}.md", name);
    Cow::Owned(path)
}

fn open_file(filename: &str) -> RunResult<Box<dyn BufRead>> {
    match filename {
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
#[derive(Debug)]
struct MDSection {
    depth: usize,
    content: Vec<String>,
}

impl MDSection {
    fn new() -> MDSection {
        MDSection {
            depth: 0,
            content: Vec::new(),
        }
    }

    fn new_section(depth: usize) -> MDSection {
        MDSection {
            depth: depth,
            content: Vec::new(),
        }
    }

    fn add_line(&mut self, line: String) {
        self.content.push(line);
    }
}

fn process_new_sheet(sheet: Box<dyn BufRead>) {
    let mut parsed_sheet = Vec::<MDSection>::new();
    parsed_sheet.push(MDSection::new());
    for (line_num, line) in sheet.lines().enumerate() {
        let line = line.unwrap();
        let new_section = section_check(&line);
        match new_section {
            Some(mut section) => {
                section.add_line(String::from(&line));
                parsed_sheet.push(section);
            }
            _ => {
                if let Some(last) = parsed_sheet.last_mut() {
                    last.add_line(String::from(&line));
                }
            }
        }
    }

    println!("{:?}", parsed_sheet)
}

fn section_check(line: &str) -> Option<MDSection> {
    let mut depth = 0;

    for ch in line.chars() {
        match ch {
            '#' => depth += 1,
            _ => break,
        }
    }

    if depth == 0 {
        return None;
    }

    Some(MDSection::new_section(depth))
}
