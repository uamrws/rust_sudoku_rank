use std::ops::{Index, IndexMut};

trait Cell {
    fn bits(&self) -> u16;
}
#[derive(Debug, Clone, Copy)]
pub struct Row([*mut u16; 9]);
#[derive(Debug, Clone, Copy)]
pub struct Col([*mut u16; 9]);
#[derive(Debug, Clone, Copy)]
pub struct Block([*mut u16; 9]);

impl TryFrom<Vec<*mut u16>> for Row {
    type Error = Vec<*mut u16>;

    fn try_from(value: Vec<*mut u16>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}
impl TryFrom<Vec<*mut u16>> for Col {
    type Error = Vec<*mut u16>;

    fn try_from(value: Vec<*mut u16>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}
impl TryFrom<Vec<*mut u16>> for Block {
    type Error = Vec<*mut u16>;

    fn try_from(value: Vec<*mut u16>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

pub struct Sudoku {
    rows: [Row; 9],
    cols: [Col; 9],
    blocks: [Block; 9],
}

impl Sudoku {
    fn rows(&mut self, r: usize) -> [&mut u16; 9] {
        todo!()
    }
    fn cols(&mut self, r: usize) -> [&mut u16; 9] {
        todo!()
    }
    fn blocks(&mut self, r: usize) -> [&mut u16; 9] {
        todo!()
    }
}

impl Sudoku {
    pub fn new(data: &mut [[u16; 9]; 9]) -> Sudoku {
        let rows: [Row; 9] = data
            .iter_mut()
            .map(|x| {
                x.iter_mut()
                    .map(|x| x as *mut u16)
                    .collect::<Vec<*mut u16>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<Row>>()
            .try_into()
            .unwrap();
        let cols: [Col; 9] = (0..9)
            .map(|c| {
                (0..9)
                    .map(|r| rows[r].0[c])
                    .collect::<Vec<*mut u16>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<Col>>()
            .try_into()
            .unwrap();
        let blocks: [Block; 9] = (0..9)
            .map(|b| {
                (0..9)
                    .map(|i| {
                        let (r, c) = Self::bi_to_rc(b, i);
                        rows[r].0[c]
                    })
                    .collect::<Vec<*mut u16>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<Block>>()
            .try_into()
            .unwrap();
        Sudoku { rows, cols, blocks }
    }
    // 行列索引转换为块索引
    pub fn rc_to_bi(r: usize, c: usize) -> (usize, usize) {
        (r / 3 * 3 + c / 3, r % 3 * 3 + c % 3 * 3)
    }
    // 块索引转换为行列索引
    pub fn bi_to_rc(b: usize, i: usize) -> (usize, usize) {
        (b / 3 * 3 + i / 3, b % 3 * 3 + i % 3)
    }
}

// pub struct Block<T> {
//     pub rows: [[T; 3]; 3],
//     pub cols: [[T; 3]; 3],
// }

// pub struct Sudoku {
//     pub inner: Block<Block<u16>>,

//     // 9 个 数独行 的数组, 记录所有 确定 的数字。
//     // 用0b000_000_000表示每行, 用位来表示已经确定的数字1-9
//     pub pos_rows: [u16; 9],

//     // 9 个 数独区块 的数组, 记录所有 确定 的数字。
//     // 用0b000_000_000表示每个区块, 用位来表示已经确定的数字1-9
//     pub pos_blocks: [u16; 9],

//     // 9 个 数独列 的数组, 记录所有 确定 的数字。
//     // 用0b000_000_000表示每列, 用位来表示已经确定的数字1-9
//     pub pos_columns: [u16; 9],

//     // 9 个 数独行 的数组, 记录所有 还未确定 的数字的下标。
//     // 用0b000_000_000表示未确定的数字, 用位来表示可能的数字1-9
//     pub neg_rows: [[*mut u16; 9]; 9],

//     // 9 个 数独区块 的数组, 记录所有 还未确定 的数字的下标。
//     // 用0b000_000_000表示未确定的数字, 用位来表示可能的数字1-9
//     pub neg_blocks: [[*mut u16; 9]; 9],

//     // 9 个 数独列 的数组, 记录所有 还未确定 的数字的下标。
//     // 用0b000_000_000表示未确定的数字, 用位来表示可能的数字1-9
//     pub neg_columns: [[[*mut u16; 3]; 3]; 9],
//     pub _maker: PhantomPinned,
// }
