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
