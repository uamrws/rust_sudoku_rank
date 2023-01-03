use std::pin::Pin;

use sudoku_core::Sudoku;

#[test]
fn deserialize() {
    let a = r#"[
        [1, 0, 3, 0, 0, 0, 9, 0, 0],
        [0, 0, 7, 0, 4, 0, 5, 2, 0],
        [8, 0, 4, 0, 5, 0, 0, 0, 0],
        [0, 6, 0, 0, 7, 0, 0, 5, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 3],
        [5, 0, 0, 2, 8, 0, 0, 0, 0],
        [0, 0, 0, 0, 2, 0, 3, 0, 6],
        [7, 0, 0, 1, 0, 0, 0, 8, 0],
        [0, 4, 0, 0, 3, 0, 0, 1, 0]
    ]"#;
    // println!("{:?}", serde_json::to_string(&a).unwrap());
    let b: Pin<Box<Sudoku>> = serde_json::from_str(a).unwrap();
    println!("{}", serde_json::to_string(&b).unwrap());
}
