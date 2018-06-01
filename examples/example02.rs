extern crate reorg;

fn main() {
    let doc_text = "* This is a sample file for reorg to parse
The contents of this file are meant to be a TODO
for this project. This file should be parseable at any time.

* Needs to read this file
This file needs to be readable always.

* Sections can have content
Like this one

* But also might not have content
* This section has children
** But these childrens are not bound to parents
** So they are independent from parents
";
    let doc = reorg::read_document(doc_text).unwrap();

    println!("{:?}", doc)
}
