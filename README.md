# reorg

`reorg` is a convenient library to read org-mode files from Rust.

It has many limitations as of now, as it can only read very simple
files. A file is a collection of Sections, each section contains a
heading: a line with one or more '*' and a title. After the heading
section a contents section is expected, which is a multi-line string.

## Examples

```rust
let org_doc = String::from("* This is first item
with some content
and a second line
** And we have another title
also with some content");
let doc = reorg::read_document(org_doc).unwrap();

assert_eq!(doc.sections.borrow()[0].heading.stars, 1);
assert_eq!(doc.sections.borrow()[0].heading.title, "This is first item");
assert_eq!(doc.sections.borrow()[0].children.borrow()[0].heading.stars, 2);
assert_eq!(doc.sections.borrow()[0].children.borrow()[0].heading.title, "And we have another title");

assert_eq!(doc.sections.borrow()[0].content, "with some content\nand a second line\n");
assert_eq!(doc.sections.borrow()[0].children.borrow()[0].content, "also with some content");
```
