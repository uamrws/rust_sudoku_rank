// use std::{ops::BitXor, pin::Pin};

use std::mem::transmute;

use sudoku_core::core_types::types::{Sudoku, SudokuTracer};

fn main() {
    println!("{:b}", 513);
    println!("{:}", 513_u16.count_ones());
    let a = r#"[
        1, 0, 3, 0, 0, 0, 9, 0, 0,
        0, 0, 7, 0, 4, 0, 5, 2, 0,
        8, 0, 4, 0, 5, 0, 0, 0, 0,
        0, 6, 0, 0, 7, 0, 0, 5, 0,
        0, 0, 0, 0, 1, 0, 0, 0, 3,
        5, 0, 0, 2, 8, 0, 0, 0, 0,
        0, 0, 0, 0, 2, 0, 3, 0, 6,
        7, 0, 0, 1, 0, 0, 0, 8, 0,
        0, 4, 0, 0, 3, 0, 0, 1, 0
    ]"#;
    let a = r#"[
        0, 5, 0, 9, 1, 0, 0, 0, 0,
        0, 1, 0, 0, 3, 0, 5, 8, 0,
        7, 4, 0, 0, 0, 0, 1, 2, 0,
        4, 3, 0, 0, 0, 9, 0, 0, 7,
        2, 0, 0, 0, 5, 8, 0, 0, 0,
        9, 8, 1, 3, 0, 4, 2, 0, 5,
        0, 0, 3, 0, 6, 5, 0, 7, 2,
        0, 6, 7, 0, 0, 3, 0, 5, 1,
        5, 0, 4, 0, 0, 0, 6, 0, 0
    ]"#;
    let mut b: [u16; 81] = serde_json::from_str::<Vec<u16>>(&a)
        .unwrap()
        .try_into()
        .unwrap();
    let mut a = Sudoku::new(b);
    let mut st = SudokuTracer::new(&mut a);

    st.start();
    println!("{:?}", a.data);
}
