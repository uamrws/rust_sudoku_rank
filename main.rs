// use std::{ops::BitXor, pin::Pin};

// use sudoku_core::Sudoku;

fn main() {
    let a = 15u16;
    let mut b: u16 = 1 << a;

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
    let mut b = serde_json::from_str::<[[u16; 9]; 9]>(&a).unwrap();

    let c: [[*mut u16; 9]; 9] = b
        .iter_mut()
        .map(|x| {
            x.iter_mut()
                .map(|x| x as *mut u16)
                .collect::<Vec<*mut u16>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[*mut u16; 9]>>()
        .try_into()
        .unwrap();
    let d: [[*mut u16; 9]; 9] = (0..9)
        .map(|col| {
            (0..9)
                .map(|r| c[r][col])
                .collect::<Vec<*mut u16>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[*mut u16; 9]>>()
        .try_into()
        .unwrap();
    let a: String = String::from("as");
    
    println!("{}", 1 / 3 * 3);
    println!("{:?}", d);

    // let b: Pin<Box<Sudoku>> = serde_json::from_str(a).unwrap();
    // println!("{}", serde_json::to_string(&b).unwrap());
}
