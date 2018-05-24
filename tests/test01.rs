extern crate reorg;


#[test]
fn get_anything() {
    let heading = reorg::build_heading("* My something".to_string());

    assert_eq!(heading.stars, 1);
}

#[test]
fn get_correct_starts() {
    let h1 = reorg::build_heading("* 1 star".to_string());
    assert_eq!(h1.stars, 1);
    assert_eq!(h1.title, "1 star");

    let h2 = reorg::build_heading("** 2 stars".to_string());
    assert_eq!(h2.stars, 2);
    assert_eq!(h2.title, "2 stars");

    let h3 = reorg::build_heading("*** 3 stars".to_string());
    assert_eq!(h3.stars, 3);
    assert_eq!(h3.title, "3 stars");

}

#[test]
fn finds_todo_item() {
    let h1 = reorg::build_heading("* TODO something".to_string());
    assert_eq!(h1.keyword, "TODO");
    assert_eq!(h1.title, "something");
}

#[test]
fn finds_done_item() {
    let h2 = reorg::build_heading("* DONE something".to_string());
    assert_eq!(h2.keyword, "DONE");
    assert_eq!(h2.title, "something");
}

#[test]
fn finds_no_keyword() {
    let h3 = reorg::build_heading("* nope something".to_string());
    assert_eq!(h3.keyword, "");
    assert_eq!(h3.title, "nope something")
}

#[test]
fn builds_simple_entry() {
    let text = "* TODO This is my main title

And I have some content that is not headings.
This is my last line.
".to_string();

    let entry = reorg::build_entry(text);

    assert_eq!(entry.heading.title, "This is my main title");
    assert_eq!(entry.heading.keyword, "TODO");
    assert_eq!(entry.heading.stars, 1);

    assert_eq!(entry.content.len(), 4);
    let mut iter = entry.content.iter();
    assert_eq!(iter.next(), Some(&"".to_string()));
    assert_eq!(iter.next(), Some(&"And I have some content that is not headings.".to_string()));
    assert_eq!(iter.next(), Some(&"This is my last line.".to_string()));
    assert_eq!(iter.next(), Some(&"".to_string()));
    assert_eq!(iter.next(), None);
}

#[test]
fn builds_document_one_element() {
    let doc = "* This is a document
With some content here
and there.
".to_string();
    let doc = reorg::build_document(doc);

    assert_eq!(doc.entries[0].heading.title, "This is a document");
    assert_eq!(doc.entries[0].heading.stars, 1);

    //assert_eq!(doc.entries[0].content.len(), 3);
}
