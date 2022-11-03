use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};
pub struct Sudoku {
    pub inner: [[u16; 9]; 9],

    // 9 个 数独行 的数组, 记录所有 确定 的数字。
    // 用0b000_000_000表示每行, 用位来表示已经确定的数字1-9
    pub pos_rows: [u16; 9],

    // 9 个 数独区块 的数组, 记录所有 确定 的数字。
    // 用0b000_000_000表示每个区块, 用位来表示已经确定的数字1-9
    pub pos_blocks: [u16; 9],

    // 9 个 数独列 的数组, 记录所有 确定 的数字。
    // 用0b000_000_000表示每列, 用位来表示已经确定的数字1-9
    pub pos_columns: [u16; 9],

    // 9 个 数独行 的数组, 记录所有 还未确定 的数字的下标。
    // 用0b000_000_000表示未确定的数字, 用位来表示可能的数字1-9
    pub neg_rows: [Vec<u8>; 9],

    // 9 个 数独区块 的数组, 记录所有 还未确定 的数字的下标。
    // 用0b000_000_000表示未确定的数字, 用位来表示可能的数字1-9
    pub neg_blocks: [Vec<u8>; 9],

    // 9 个 数独列 的数组, 记录所有 还未确定 的数字的下标。
    // 用0b000_000_000表示未确定的数字, 用位来表示可能的数字1-9
    pub neg_columns: [Vec<u8>; 9],
}

impl Sudoku {
    pub fn new(inner: [[u16; 9]; 9]) -> Self {
        let mut obj = Self {
            inner,
            pos_rows: Default::default(),
            pos_blocks: Default::default(),
            pos_columns: Default::default(),
            neg_rows: Default::default(),
            neg_blocks: Default::default(),
            neg_columns: Default::default(),
        };
        obj.inner.iter_mut().enumerate().for_each(|(irow, x)| {
            x.iter_mut().enumerate().for_each(|(icol, y)| {
                if *y > 0 {
                    let iblock = Self::locate_block(irow, icol);
                    let shift = *y - 1;
                    obj.pos_rows[irow] &= 1 << shift;
                    obj.pos_columns[icol] &= 1 << shift;
                    obj.pos_blocks[iblock] &= 1 << shift;
                }
            })
        });
        todo!()
    }
    pub fn locate_block(irow: usize, icol: usize) -> usize {
        irow / 3 * 3 + icol / 3
    }
}

impl Serialize for Sudoku {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_tuple(9)?;
        for i in &self.inner {
            s.serialize_element(i)?;
        }
        s.end()
    }
}

impl<'de> Deserialize<'de> for Sudoku {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::new(<[[u16; 9]; 9] as Deserialize>::deserialize(
            deserializer,
        )?))
    }
}

// impl<'de> Deserialize<'de> for Sudoku {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         deserializer.deserialize_tuple(9, SudokuVisitor)
//     }
// }
// struct SudokuVisitor;

// impl<'de> Visitor<'de> for SudokuVisitor {
//     type Value = Sudoku;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("tuple must be like [[u16; 9]; 9]")
//     }
//     fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//     where
//         A: de::SeqAccess<'de>,
//     {
//         let mut i = 0;
//         let mut inner: [[u16; 9]; 9] = [[0; 9]; 9];
//         while let Ok(Some(v)) = seq.next_element() {
//             inner[i] = v;
//             i += 1;
//         }
//         Ok(Self::Value::new(inner))
//     }
// }
