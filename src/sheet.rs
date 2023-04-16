#[path = "./output.rs"]
mod output;
use std::io::BufRead;
use termimad::crossterm::style::Color::*;
use termimad::*;

use self::output::Output;
type MDSheet = Vec<MDSection>;
#[derive(Debug)]
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
    let mut skin = MadSkin::default();
    skin.set_headers_fg(Green);
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Blue);

    let mut output = output::Console { skin: skin };
    display_sheet(sheet, filter, list, &mut output)
}

fn display_sheet<T: Output>(
    sheet: Box<dyn BufRead>,
    filter: &[String],
    list: bool,
    output: &mut T,
) {
    let sections_to_filter = filter.to_vec();
    let target_sheet = parse_sheet(sheet);

    if list {
        display_sheet_anchors(&target_sheet, output)
    } else {
        display_sheet_details(&target_sheet, &sections_to_filter, output)
    }
}

#[test]
fn test_process_new_sheet_when_list_false() {
    let mut output = output::MockConsole {
        write_was_called: false,
    };

    let input_bytes = "# header\n test line \n## header2 \n test line2 \n test line 3".as_bytes();
    let sheet = Box::new(input_bytes);
    display_sheet(sheet, &["header".to_string()], false, &mut output);
    assert_eq!(output.write_was_called, true);

    let sheet = Box::new(input_bytes);
    output.write_was_called = false;
    display_sheet(sheet, &["header".to_string()], true, &mut output);
    assert_eq!(output.write_was_called, true);
}

#[test]
fn test_process_new_sheet_when_list_true() {
    let mut output = output::MockConsole {
        write_was_called: false,
    };

    let input_bytes = "# header\n test line \n## header2 \n test line2 \n test line 3".as_bytes();
    let sheet = Box::new(input_bytes);
    display_sheet(sheet, &["header".to_string()], true, &mut output);
    assert_eq!(output.write_was_called, true);
}

fn display_sheet_anchors<T: Output>(sheet: &MDSheet, output: &mut T) {
    for section in sheet {
        output.write(section.anchor.as_str());
    }
}

#[test]
fn test_empty_sheet() {
    let input_bytes = "".as_bytes();
    let subject = parse_sheet(Box::new(input_bytes));
    assert_eq!(subject.len(), 1);
    assert_eq!(subject.get(0).unwrap().anchor, "");
    assert!(subject.get(0).unwrap().content.is_empty());
}

#[test]
fn test_parse_simple_sheet() {
    let input_bytes = "# header\n test line \n## header2 \n test line2 \n test line 3".as_bytes();
    let subject = parse_sheet(Box::new(input_bytes));
    assert_eq!(subject.len(), 3);
    assert_eq!(subject.get(2).unwrap().anchor, "header2");
    assert_eq!(subject.get(1).unwrap().content.len(), 2);
    assert_eq!(subject.get(2).unwrap().content.len(), 3);
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
fn display_sheet_details<T: Output>(sheet: &MDSheet, filter: &Vec<String>, output: &mut T) {
    let skin = make_skin();
    if filter.len() > 0 {
        for section in filter {
            let display_rows = get_sections_with_filter(&sheet, &section);
            let display_string = display_rows.join("\n");
            // show(&skin, &display_string);
            output.write(&display_string)
        }
    } else {
        let display_rows = get_all_sections(&sheet);
        let display_string = display_rows.join("\n");
        // show(&skin, &display_string);
        output.write(&display_string)
    }
}

#[test]
fn test_filter_section() {
    let mut subject = MDSheet::default();
    let mut section: MDSection = create_new_section("# one").unwrap();
    let mut section2: MDSection = create_new_section("# two").unwrap();
    section.add_line(String::from("section 1 line 1"));
    section.add_line(String::from("section 1 line 2"));
    section2.add_line(String::from("section 2 line 1"));
    section2.add_line(String::from("section 2 line 2"));
    subject.push(section);
    subject.push(section2);

    let results = get_sections_with_filter(&subject, "one");
    assert_eq!(results.get(0).unwrap(), "section 1 line 1");
    assert_eq!(results.get(1).unwrap(), "section 1 line 2");
    let results = get_sections_with_filter(&subject, "two");
    assert_eq!(results.get(0).unwrap(), "section 2 line 1");
    assert_eq!(results.get(1).unwrap(), "section 2 line 2");
    let results = get_sections_with_filter(&subject, "none");
    assert_eq!(results.len(), 0);
}
#[test]
fn test_all_sections() {
    let mut subject = MDSheet::default();
    let mut section: MDSection = create_new_section("# one").unwrap();
    let mut section2: MDSection = create_new_section("# two").unwrap();
    section.add_line(String::from("section 1 line 1"));
    section.add_line(String::from("section 1 line 2"));
    section2.add_line(String::from("section 2 line 1"));
    section2.add_line(String::from("section 2 line 2"));
    subject.push(section);
    subject.push(section2);

    let results = get_all_sections(&subject);

    assert!(results.len() == 4);
    assert_eq!(results.get(0).unwrap(), "section 1 line 1");
    assert_eq!(results.get(1).unwrap(), "section 1 line 2");
    assert_eq!(results.get(2).unwrap(), "section 2 line 1");
    assert_eq!(results.get(3).unwrap(), "section 2 line 2");
}

fn get_sections_with_filter(sheet: &MDSheet, filter: &str) -> Vec<String> {
    sheet
        .iter()
        .filter(|s| s.anchor.eq(&filter))
        .flat_map(|s| &s.content)
        .cloned()
        .collect()
}

fn get_all_sections(sheet: &MDSheet) -> Vec<String> {
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
