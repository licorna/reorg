extern crate regex;

use regex::RegexBuilder;

/// reorg library reads orgmode files!

/// 1st create a struct to handle a heading
/// follow org file specs in here https://orgmode.org/worg/dev/org-syntax.html

/// Document is a org representation of a text file. It is a collection of
/// entries but it can be preceded by some content. The prologue is not yet
/// implemented.
#[derive(Debug)]
pub struct Document {
    pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    pub heading: Heading,

    // content is a string containing the inner content of a given section
    pub content: String,

    pub children: Vec<Entry>,
}

#[derive(Debug)]
pub struct Heading {
    pub stars: usize,
    pub keyword: String,
    pub title: String,
}

pub fn read_document(document: &str) -> Option<Document> {
    let re = RegexBuilder::new(r"^\*+\s").multi_line(true).build().unwrap();
    let mut entries:Vec<Entry> = Vec::new();
    let mut entry_offsets:Vec<usize> = Vec::new();

    let mut iter = re.find_iter(document);
    iter.next();
    for i in iter {
        entry_offsets.push(i.start());
    }
    entry_offsets.push(document.len());

    let mut last = 0;
    for offs in entry_offsets {
        match read_entry(&document[last..offs]) {
            Some(e) => entries.push(e),
            None => (),
        };
        last = offs;
    }

    Some(Document{
        entries,
    })
}

fn read_content(section: &str) -> &str {
    match section.find('\n') {
        Some(u) => &section[u+1..],
        None => &""
    }
}

/// Returns full entry, this is, a title and a section.
pub fn read_entry(section: &str) -> Option<Entry> {
    let heading = read_heading(section);
    let content = read_content(section);

    Some(Entry{
        heading: heading.unwrap(),
        content: content.to_string(),
        children: vec![],
    })
}

/// Returns number of stars from beginning of 1st line.
fn read_stars(heading: &str) -> usize {
    let mut stars:usize = 0;
    for c in heading.to_string().chars() {
        if c == '*' {
            stars += 1;
        } else {
            break;
        }
    }

    stars
}

/// Reads a title: everything after stars and 1st whitespace. input is considered
/// sane for now.
fn read_title(heading: &str) -> &str {
    let start = heading.find("* ").unwrap();
    match heading.find('\n') {
        Some(u) => &heading[start+2..u],
        None => &heading[start+2..]
    }
}

pub fn read_heading(heading: &str) -> Option<Heading> {
    let stars = read_stars(heading);
    let title = read_title(heading);

    Some(Heading{
        stars,
        title: title.to_string(),
        keyword: "".to_string(),
    })
}
