extern crate regex;

use regex::Regex;

/// reorg library reads orgmode files!

/// 1st create a struct to handle a heading
/// follow org file specs in here https://orgmode.org/worg/dev/org-syntax.html

// #[derive(Debug)]
// pub struct Document {
//     pub entries: Vec<>
// }

#[derive(Debug)]
/// Document is a org representation of a text file. It is a collection of
/// entries but it can be preceded by some content. The prologue is not yet
/// implemented.
pub struct Document {
    // pub content: Vec<String>,
    pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    pub heading: Heading,
    pub content: Vec<String>,

    pub children: Vec<Entry>,
}

#[derive(Debug)]
pub struct Heading {
    pub stars: usize,
    pub keyword: String,
    pub title: String,
    // tags: vec!<String>,
    // pub priority: String,
}

/// `read_heading_simple` reads a org entry only considering the amount of stars,
/// keyword and title.
pub fn build_heading(_from: String) -> Heading {
    let re = Regex::new(r"(?m)(?P<stars>\*+\s)?(?P<keyword>(TODO|DONE)\s)?(?P<title>.+)$").unwrap();

    let capture = match re.captures(&_from) {
        None => panic!("Error capturing heading from string"),
        Some(e) => e,
    };

    let stars = (&capture["stars"]).trim().len();
    let title = String::from(&capture["title"]);
    let keyword = match capture.name("keyword") {
        None => "".to_string(),
        Some(k) => k.as_str().trim().to_string(),
    };

    Heading {
        stars,
        title,
        keyword,
    }
}

pub fn build_entry(_from: String) -> Entry {
    let lines: Vec<&str> = _from.split("\n").collect();

    let heading = build_heading(lines[0].to_string());
    let mut content = Vec::new();

    let mut i = lines.iter();
    i.next(); // skip title line

    for line in i {
        content.push(line.to_string());
    };

    Entry {
        heading,
        content,

        children: Vec::new(),
    }
}

pub fn build_document(filecontents: String) -> Document {
    // this is what i have to do

    // 1. tokenize this file into level 1 headings (1 star)
    // 2. to do so, I will split by "^* "
    // 3. this will return a list of blocks to add, with their ** or other levels
    //    in content.
    // 4. to go into the document, I tokenize, the content of this entry, on "^** "
    //    and add those items, for each item, i tokenize for 1 star-level more... etc.

    // this implementation is not recursive, only for 1-star levels (top level)

    let re = regex::Regex::new(r"(^\*\s.*)").unwrap();
    let sections = re.captures_iter(&filecontents);
    let mut entries: Vec<Entry> = Vec::new();

    for section in sections {
        println!("section {}\n", section[0].to_string());
        entries.push(build_entry(section[0].to_string()));
    }

    Document{
        entries,
    }
}
