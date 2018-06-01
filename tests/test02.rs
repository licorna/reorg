extern crate reorg;

#[test]
fn test_heading_1_star() {
    let test_str = "* valid title";
    match reorg::read_heading(test_str) {
        Some(h) => {
            assert_eq!(h.stars, 1);
            assert_eq!(h.title, "valid title".to_string());
            assert_eq!(h.keyword, "".to_string());
        },
        None => ()
    }
}

#[test]
fn test_heading_2_star() {
    let test_str = "** valid title 2";

    match reorg::read_heading(test_str) {
        Some(h) => {
            assert_eq!(h.stars, 2);
            assert_eq!(h.title, "valid title 2".to_string());
            assert_eq!(h.keyword, "".to_string());
        },
        None => {
            assert_eq!(true, false);
        }
    }
}

#[test]
fn test_entry_simple() {
    let test_entry = "**** valid title
some content
and some more";

    match reorg::read_section(test_entry) {
        Some(e) => {
            assert_eq!(e.heading.title, "valid title");
            assert_eq!(e.heading.stars, 4);
        },
        None => assert_eq!(true, false)
    }
}

#[test]
fn test_entry_simple0() {
    let test_entry = "*** some title
this is some content
and a bit more";
    match reorg::read_section(test_entry) {
        Some(e) => {
            assert_eq!(e.heading.stars, 3);
            assert_eq!(e.heading.title, "some title");
            assert_eq!(e.heading.keyword, "");
            assert_eq!(e.content, "this is some content\nand a bit more");
        },
        None => assert_eq!(true, false)
    }
}

#[test]
fn test_document_simple0() {
    let test_doc = "*** some entry
and some content
and some more
** and another section
with its own content";
    match reorg::read_document(test_doc) {
        Some(d) => {
            assert_eq!(d.sections[0].heading.title, "some entry");
            assert_eq!(d.sections[0].heading.stars, 3);
            assert_eq!(d.sections[1].heading.title, "and another section");
            assert_eq!(d.sections[1].heading.stars, 2);
        },
        None => {
            assert_eq!(true, false);
        }
    }
}
