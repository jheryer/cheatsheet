use std::io::BufRead;
use termimad::crossterm::style::Color::*;
use termimad::*;

type MDSheet = Vec<MDSection>;
struct MDSection {
    anchor: String,
    content: Vec<String>,
}

impl MDSection {
    fn new() -> MDSection {
        MDSection {
            anchor: String::from(""),
            content: Vec::new(),
        }
    }

    fn new_section(anchor: String) -> MDSection {
        MDSection {
            anchor: anchor,
            content: Vec::new(),
        }
    }

    fn add_line(&mut self, line: String) {
        self.content.push(line);
    }
}

pub fn process_new_sheet(sheet: Box<dyn BufRead>, filter: &[String], list: bool) {
    let sections_to_filter = filter.to_vec();
    let target_sheet = parse_sheet(sheet);

    if list {
        display_sheet_anchors(&target_sheet)
    } else {
        display_sheet_details(&target_sheet, &sections_to_filter)
    }
}

fn display_sheet_anchors(sheet: &MDSheet) {
    for section in sheet {
        println!("{}", section.anchor);
    }
}

fn parse_sheet(sheet: Box<dyn BufRead>) -> MDSheet {
    let mut cheat_sheet = MDSheet::new();
    cheat_sheet.push(MDSection::new());
    for line in sheet.lines() {
        let line = line.unwrap();
        let new_section = create_new_section(&line);
        match new_section {
            Some(mut section) => {
                section.add_line(String::from(&line));
                cheat_sheet.push(section);
            }
            _ => {
                if let Some(last) = cheat_sheet.last_mut() {
                    last.add_line(String::from(&line));
                }
            }
        }
    }

    cheat_sheet
}

fn display_sheet_details(sheet: &MDSheet, filter: &Vec<String>) {
    let skin = make_skin();
    if filter.len() > 0 {
        for section in filter {
            let display_rows = filter_sections(&sheet, &section);
            let display_string = display_rows.join("\n");
            show(&skin, &display_string);
        }
    } else {
        let display_rows = all_sections(&sheet);
        let display_string = display_rows.join("\n");
        show(&skin, &display_string);
    }
}
fn filter_sections(sheet: &MDSheet, filter: &str) -> Vec<String> {
    sheet
        .iter()
        .filter(|s| s.anchor.eq(&filter))
        .flat_map(|s| &s.content)
        .cloned()
        .collect()
}

fn all_sections(sheet: &MDSheet) -> Vec<String> {
    sheet.iter().flat_map(|s| &s.content).cloned().collect()
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(Green);
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Blue);
    skin
}

fn show(skin: &MadSkin, src: &str) {
    skin.print_text(src);
}

fn create_new_section(line: &str) -> Option<MDSection> {
    match line.chars().nth(0) {
        Some('#') => {
            let anchor = heading_to_anchor(line);
            Some(MDSection::new_section(anchor))
        }
        _ => None,
    }
}

#[test]
fn test_basic_section_check() {
    let section_heading = create_new_section("this#isnot a section Heading");
    assert!(section_heading.is_none());

    let section_heading = create_new_section("#one section");
    let subject = section_heading.unwrap();
    assert_eq!(subject.anchor, "one-section");

    let section_heading = create_new_section("##two ");
    let subject = section_heading.unwrap();
    assert_eq!(subject.anchor, "two");

    let section_heading = create_new_section("####a really long section heading ");
    let subject = section_heading.unwrap();
    assert_eq!(subject.anchor, "a-really-long-section-heading");
}

fn heading_to_anchor(heading: &str) -> String {
    let without_hashes = heading.trim_start_matches('#').trim();
    let words = without_hashes.split_whitespace();
    words
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
        .join("-")
}

#[test]

fn test_heading_to_anchor() {
    let heading = "# simple";
    let subject = heading_to_anchor(heading);
    assert_eq!(subject, "simple");

    let heading = "#simple";
    let subject = heading_to_anchor(heading);
    assert_eq!(subject, "simple");

    let heading = "# heading one";
    let subject = heading_to_anchor(heading);
    assert_eq!(subject, "heading-one");

    let heading = "## heading two";
    let subject = heading_to_anchor(heading);
    assert_eq!(subject, "heading-two");

    let heading = "###     this is a really long heading three";
    let subject = heading_to_anchor(heading);
    assert_eq!(subject, "this-is-a-really-long-heading-three");
}
