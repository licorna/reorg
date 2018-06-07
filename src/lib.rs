//! `reorg` is a convenient library to read org-mode files from Rust.
//!
//! It has many limitations as of now, as it can only read very simple
//! files. A file is a collection of Sections, each section contains a
//! heading: a line with one or more '*' and a title. After the heading
//! section a contents section is expected, which is a multi-line string.
//!
//! # Examples
//!
//! ```rust
//! # use reorg;
//! let org_doc = "* This is first item
//! with some content
//! and a second line
//! ** And we have another title
//! also with some content";
//! let doc = reorg::read_document(org_doc).unwrap();
//!
//! assert_eq!(doc.sections[0].heading.stars, 1);
//! assert_eq!(doc.sections[0].heading.title, "This is first item");
//! assert_eq!(doc.sections[1].heading.stars, 2);
//! assert_eq!(doc.sections[1].heading.title, "And we have another title");
//!
//! assert_eq!(doc.sections[0].content, "with some content\nand a second line\n");
//! assert_eq!(doc.sections[1].content, "also with some content");
//! ```

extern crate regex;
use regex::RegexBuilder;
use std::fs::File;
use std::io::prelude::*;


/// `Document` is an org representation of a text file. It is a collection of
/// entries but it can be preceded by some content. The prologue is not yet
/// implemented.
#[derive(Debug)]
pub struct Document {
    pub sections: Vec<Section>,
}

/// A `Section` is a `Heading` and some optional `Content`. A `Document` is
/// composed of many `Section`s.
#[derive(Debug)]
pub struct Section {
    pub heading: Heading,

    // content is a string containing the inner content of a given section
    pub content: String,

    pub children: Vec<Section>,
}

/// `Heading` is the title of each section, it includes a number of stars,
/// which set the "priority" for this given `Section`.
#[derive(Debug)]
pub struct Heading {
    pub stars: usize,
    pub keyword: String,
    pub title: String,
}

/// Reads an org document from a file.
pub fn from_file(filename: &str) -> Option<Document> {
    let mut f = File::open(&filename).expect("file not found");

    let mut doc_text = String::new();

    match f.read_to_string(&mut doc_text) {
        Err(_) => panic!("Error reading file contents"),
        Ok(_) => (),
    }

    read_document(&doc_text)
}

/// Reads an org document in a string.
pub fn read_document(document: &str) -> Option<Document> {
    let re = RegexBuilder::new(r"^\*+\s").multi_line(true).build().unwrap();
    let mut sections:Vec<Section> = Vec::new();
    let mut section_offsets:Vec<usize> = Vec::new();

    let mut iter = re.find_iter(document);
    iter.next();
    for i in iter {
        section_offsets.push(i.start());
    }
    section_offsets.push(document.len());

    let mut last = 0;
    for offs in section_offsets {
        match read_section(&document[last..offs]) {
            Some(e) => sections.push(e),
            None => (),
        };
        last = offs;
    }

    Some(Document{
        sections,
    })
}

fn read_content(section: &str) -> &str {
    match section.find('\n') {
        Some(u) => &section[u+1..],
        None => &""
    }
}

/// Reads a full `Section`. The `section` parameter is expected to only
/// contain one `Section`.
pub fn read_section(section: &str) -> Option<Section> {
    let heading = read_heading(section)?;
    let content = read_content(section);

    Some(Section{
        heading: heading,
        content: content.to_string(),
        children: vec![],
    })
}

/// Returns number of stars from beginning of 1st line of section text.
fn read_stars(section: &str) -> usize {
    let mut stars:usize = 0;
    for c in section.to_string().chars() {
        if c == '*' {
            stars += 1;
        } else {
            break;
        }
    }

    stars
}

/// This is a simple function to test the Github API.
fn simple_addition(_: &str) -> bool {
    true
}

/// Reads a title: everything after stars and 1st whitespace.
fn read_title(section: &str) -> &str {
    let start = section.find("* ").unwrap();
    match section.find('\n') {
        Some(u) => &section[start+2..u],
        None => &section[start+2..]
    }
}

/// This is another simple function to test
fn simple_something(_: &str) -> bool {
    false
}

/// Reads a heading, this is, a number of stars from the beginning, and
/// a title.
pub fn read_heading(section: &str) -> Option<Heading> {
    let stars = read_stars(section);
    let title = read_title(section);

    Some(Heading{
        stars,
        title: title.to_string(),
        keyword: "".to_string(),
    })
}
