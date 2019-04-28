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
//! let org_doc = String::from("* This is first item
//! with some content
//! and a second line
//! ** And we have another title
//! also with some content");
//! let doc = reorg::Document::from(org_doc).unwrap();
//!
//! assert_eq!(doc.sections.borrow()[0].heading.stars, 1);
//! assert_eq!(doc.sections.borrow()[0].heading.title, "This is first item");
//! assert_eq!(doc.sections.borrow()[0].children.borrow()[0].heading.stars, 2);
//! assert_eq!(doc.sections.borrow()[0].children.borrow()[0].heading.title, "And we have another title");
//!
//! assert_eq!(doc.sections.borrow()[0].content, "with some content\nand a second line\n");
//! assert_eq!(doc.sections.borrow()[0].children.borrow()[0].content, "also with some content");
//! ```

extern crate regex;
use regex::RegexBuilder;
use std::fs::File;
use std::io::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

type Sections<'a> = RefCell<Vec<Rc<Section<'a>>>>;

/// `Document` is an org representation of a text file. It is a collection of
/// entries but it can be preceded by some content. The prologue is not yet
/// implemented.
#[derive(Debug)]
pub struct Document<'a> {
    pub sections: Sections<'a>,
}

// impl<'a> Sections<'a> {
//     pub fn iter(&self) -> Iter<'_> {
//         Iter {
//             next: self.borrow().
//         }
//     }
// }

/// A `Section` is a `Heading` and some optional `Content`. A `Document` is
/// composed of many `Section`s.
#[derive(Debug,Clone)]
pub struct Section<'a> {
    pub heading: Heading,

    // content is a string containing the inner content of a given section
    pub content: String,

    pub children: RefCell<Vec<Rc<Section<'a>>>>,
}

/// `Heading` is the title of each section, it includes a number of stars,
/// which set the "priority" for this given `Section`.
#[derive(Debug,Clone)]
pub struct Heading {
    pub stars: usize,
    pub keyword: String,
    pub title: String,
}

impl<'b> Document<'b> {
    pub fn from_file(filename: &str) -> Option<Document> {
        let mut f = File::open(&filename).expect("file not found");

        let mut doc_text = String::new();

        match f.read_to_string(&mut doc_text) {
            Err(_) => panic!("Error reading file contents"),
            Ok(_) => (),
        }

        Document::from(doc_text)
    }

    /// Reads an org document in a string.
    pub fn from<'a>(document: String) -> Option<Document<'a>> {
        let re = RegexBuilder::new(r"^\*+\s").multi_line(true).build().unwrap();
        let mut section_offsets:Vec<usize> = Vec::new();

        let root:RefCell<Vec<Rc<Section>>> = RefCell::new(Vec::new());
        let insertion_stack:RefCell<Vec<Rc<Section>>> = RefCell::new(Vec::new());
        let mut istack = insertion_stack.borrow_mut();

        let mut iter = re.find_iter(&document);
        iter.next();
        for i in iter {
            section_offsets.push(i.start());
        }
        section_offsets.push(document.len());

        let mut last = 0;
        for offs in section_offsets {
            if let Some(section) = Document::read_section(String::from(&document[last..offs])) {

                while let Some(top) = istack.pop() {
                    if section.heading.stars > top.heading.stars {
                        top.children.borrow_mut().push(Rc::clone(&section));
                        istack.push(Rc::clone(&top));
                        istack.push(Rc::clone(&section));
                        break;
                    }
                }
                if istack.len() == 0 {
                    istack.push(Rc::clone(&section));
                    root.borrow_mut().push(section);
                }
            }
            last = offs;
        }

        Some(Document{
            sections: root,
        })
    }

    fn read_content(section: &str) -> String {
        match section.find('\n') {
            Some(u) => section[u+1..].to_string(),
            None => String::from("")
        }
    }

    /// Reads a full `Section`. The `section` parameter is expected to only
    /// contain one `Section`.
    pub fn read_section<'a>(section: String) -> Option<Rc<Section<'a>>> {
        let heading = Document::read_heading(&section)?;
        let content = Document::read_content(&section);

        Some(Rc::new(Section{
            heading: heading,
            content: content.to_string(),
            children: RefCell::new(Vec::new()),
        }))
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

    /// Reads a title: everything after stars and 1st whitespace.
    fn read_title(section: &str) -> &str {
        let start = section.find("* ").unwrap();
        match section.find('\n') {
            Some(u) => &section[start+2..u],
            None => &section[start+2..]
        }
    }

    /// Reads a heading, this is, a number of stars from the beginning, and
    /// a title.
    pub fn read_heading(section: &str) -> Option<Heading> {
        let stars = Document::read_stars(section);
        let title = Document::read_title(section);

        Some(Heading{
            stars,
            title: title.to_string(),
            keyword: "".to_string(),
        })
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn one_section() {
        let text = String::from("* One section");
        let doc = Document::from(text).unwrap();

        assert_eq!(doc.sections.borrow().len(), 1);
    }

    #[test]
    fn two_section() {
        let text = String::from("* One section
* Two sections");
        let doc = Document::from(text).unwrap();

        assert_eq!(doc.sections.borrow().len(), 2);
    }

    #[test]
    fn correct_number_of_sections() {
        // all sections have level 1 asterisk
        let simple_doc = String::from("* This is a simple document
with some content
here and there
* With a second section with
some data
* And a third and final one
with some data");

        let doc = Document::from(simple_doc).unwrap();

        assert_eq!(doc.sections.borrow().len(), 3);
    }


    #[test]
    fn all_sections_are_read() {
        let simple_doc = String::from("* This is a simple document
with some content
here and there
* With a second section with
some data
** And a third and final one
with some data");

        let doc = Document::from(simple_doc).unwrap();

        assert_eq!(doc.sections.borrow().len(), 2);
        assert_eq!(doc.sections.borrow()[1].children.borrow().len(), 1);
    }

    #[test]
    fn title_is_obtained_correctly () {
        let simple_doc = String::from("* This is a simple document
with some content
here and there
* With a second section with
some data
** And a third and final one
with some data");

        let doc = Document::from(simple_doc).unwrap();

        assert_eq!(doc.sections.borrow()[0].heading.title, "This is a simple document");
        assert_eq!(doc.sections.borrow()[1].heading.title, "With a second section with");
        assert_eq!(doc.sections.borrow()[1].children.borrow()[0].heading.title, "And a third and final one");
    }

    #[test]
    fn section_level_is_obtained_correctly () {
        let simple_doc = String::from("* This is a simple document
with some content
here and there
* With a second section with
some data
** And a third and final one
with some data");

        let doc = Document::from(simple_doc).unwrap();

        assert_eq!(doc.sections.borrow()[0].heading.stars, 1);
        assert_eq!(doc.sections.borrow()[1].heading.stars, 1);
        assert_eq!(doc.sections.borrow()[1].children.borrow()[0].heading.stars, 2);
    }

    #[test]
    fn subsection_is_obtained_correctly () {
        let simple_doc = String::from("* This is a simple document
with some content
here and there
* With a second section with
some data
** And a third and final one
with some data");

        let doc = Document::from(simple_doc).unwrap();

        assert_eq!(doc.sections.borrow()[0].heading.stars, 1);
        assert_eq!(doc.sections.borrow()[1].heading.stars, 1);
        assert_eq!(doc.sections.borrow()[1].children.borrow()[0].heading.title, "And a third and final one");
        assert_eq!(doc.sections.borrow()[1].children.borrow()[0].heading.stars, 2);
    }

    #[test]
    fn sections_can_be_iterated() {
        let simple_doc = String::from("* This is a simple document
with some content
here and there
* With a second section with
some data
** And a third and final one
with some data");
        let doc = Document::from(simple_doc).unwrap();

        assert_eq!((2, Some(2)), doc.sections.borrow().iter().size_hint());
    }

    #[test]
    fn read_org_from_file() {
        match Document::from_file("todo.org") {
            Some(doc) => {
                assert_eq!(doc.sections.borrow().len(), 6)
            }
            _ => {
                panic!("Could not read file")
            }
        }
    }
}
