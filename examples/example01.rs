extern crate reorg;

fn main() {
    let simple_doc = "* This is a simple document
with some content
here and there
* With a second section with
some data
** And a third and final one
with some data";

    let doc = reorg::read_document(simple_doc);

    println!("{:?}", doc.unwrap());
}
