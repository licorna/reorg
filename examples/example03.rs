extern crate reorg;

fn main() {
    let doc_filename = "todo.org";
    let doc = reorg::from_file(doc_filename).unwrap();

    println!("{:?}", doc)
}
