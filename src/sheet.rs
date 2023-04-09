use std::io::BufRead;
use termimad::crossterm::style::Color::*;
use termimad::*;

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

pub fn process_new_sheet(sheet: Box<dyn BufRead>, filter: &[String]) {
    let sections_to_filter = filter.to_vec();
    let mut parsed_sheet = MDSheet::new();
    parsed_sheet.push(MDSection::new());
    for line in sheet.lines() {
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

    let skin = make_skin();
    if sections_to_filter.len() > 0 {
        for section in sections_to_filter {
            let display_rows = filter_sections(&parsed_sheet, &section);
            let display_string = display_rows.join("\n");
            show(&skin, &display_string);
        }
    } else {
        let display_rows = all_sections(&parsed_sheet);
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
    let anchor = heading_to_anchor(line);
    Some(MDSection::new_section(anchor))
}

#[test]
fn test_heading_to_anchor() {
    let heading_simple = "# simple";
    let heading_simple_two = "#simple";
    let heading_one = "# heading one";
    let heading_two = "## heading two";
    let heading_three = "###     this is a really long heading three";

    let subject_one = heading_to_anchor(heading_one);
    let subject_two = heading_to_anchor(heading_two);
    let subject_three = heading_to_anchor(heading_three);
    let subject_four = heading_to_anchor(heading_simple);
    let subject_five = heading_to_anchor(heading_simple_two);

    assert_eq!(subject_one, "heading-one");
    assert_eq!(subject_two, "heading-two");
    assert_eq!(subject_three, "this-is-a-really-long-heading-three");
    assert_eq!(subject_four, "simple");
    assert_eq!(subject_five, "simple");
}

fn heading_to_anchor(heading: &str) -> String {
    let without_hashes = heading.trim_start_matches('#').trim();
    let words = without_hashes.split_whitespace();
    words
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
        .join("-")
}
