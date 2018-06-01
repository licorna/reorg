extern crate regex;

//use regex::{Regex, RegexBuilder};

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

    // content is a string containing the inner content of a given section
    pub content: String,

    pub children: Vec<Entry>,
}

#[derive(Debug)]
pub struct Heading {
    pub stars: usize,
    pub keyword: String,
    pub title: String,

    // not implemented
    // tags: vec!<String>,
    // pub priority: String,
}

pub fn read_heading0(_: &str) -> Option<Heading> {
    Some(Heading {
        stars: 1,
        title: "valid title".to_string(),
        keyword: "".to_string(),
    })
}

// returns number of stars from beginning of 1st line.
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

// reads a title: everything after stars and 1st whitespace. input is considered
// sane for now.
fn read_title(heading: &str) -> &str {
    let start = heading.find("* ").unwrap();

    return &heading[start+2..]
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

// /// `read_heading_simple` reads a org entry only considering the amount of stars,
// /// keyword and title.
// pub fn build_heading(head: String) -> Option<Heading> {
//     // FIX: super ugly hack
//     if head.len() == 2 {
//         return None
//     }
//     let re = Regex::new(r"(?P<stars>\*+\s)?(?P<keyword>(TODO|DONE)\s)?(?P<title>.+)").unwrap();

//     let capture = re.captures(&head)?;

//     let stars = (&capture["stars"]).trim().len();
//     let title = String::from(&capture["title"]);
//     let keyword = match capture.name("keyword") {
//         None => "".to_string(),
//         Some(k) => k.as_str().trim().to_string(),
//     };

//     Some(Heading {
//         stars,
//         title,
//         keyword,
//     })
// }

// pub fn build_entry(_from: String) -> Option<Entry> {
//     let re = Regex::new(r"^\*+\s(.+)$").unwrap();
//     let lines: Vec<&str> = _from.split("\n").collect();

//     let capture = re.captures(&_from)?;
//     let title = &capture["title"];
//     println!("title: {}", title);
//     let heading = build_heading(String::from(title))?;
//     let mut content = Vec::new();

//     let mut i = lines.iter();
//     i.next(); // skip title line

//     for line in i {
//         content.push(line.to_string());
//     };

//     Some(Entry {
//         heading,
//         content,

//         children: Vec::new(),
//     })
// }

// pub fn build_document(contents: String, level: usize) -> Option<Document> {
//     // this is what i have to do

//     // 1. tokenize this file into level 1 headings (1 star)
//     // 2. to do so, I will split by "^* "
//     // 3. this will return a list of blocks to add, with their ** or other levels
//     //    in content.
//     // 4. to go into the document, I tokenize, the content of this entry, on "^** "
//     //    and add those items, for each item, i tokenize for 1 star-level more... etc.

//     // this implementation is not recursive, only for 1-star levels (top level)

//     let mut full_section = r"\*".repeat(level).to_owned();
//     full_section.push_str(r"\s");
//     let mut regex = "^".to_owned();
//     regex.push_str(&full_section);
//     let re = RegexBuilder::new(&regex).multi_line(true).build().unwrap();
//     let sections = re.split(&contents);
//     let mut entries: Vec<Entry> = Vec::new();

//     for section in sections {
//         let mut fixed = "*".repeat(level).to_string();
//         fixed.push_str(" ");
//         fixed.push_str(section);

//         println!("section: {}\n", fixed);
//         match build_entry(fixed) {
//             Some(e) => entries.push(e),
//             None => continue, // but this is the prologue of the section. will come back later
//         }
//     }

//     Some(Document{
//         entries,
//     })
// }
