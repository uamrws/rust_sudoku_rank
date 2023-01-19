use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::types::{CommonTool, Sudoku};

impl Sudoku {
    pub fn new(data: [u16; 81]) -> Sudoku {
        let (rows, (cols, blocks)) = Self::build();
        let mut sudoku = Sudoku {
            data: data
                .iter()
                .map(|x| if *x == 0 { 0b11_1111_1111 } else { *x })
                .collect::<Vec<u16>>()
                .try_into()
                .unwrap(),
            rows,
            cols,
            blocks,
        };
        sudoku.init();
        sudoku
    }

    fn build() -> (
        Vec<Rc<Vec<usize>>>,
        (Vec<Rc<Vec<usize>>>, Vec<Rc<Vec<usize>>>),
    ) {
        (0..9)
            .into_iter()
            .map(|i| {
                let r = i * 9;
                let c = i;
                let b = i / 3 * 27 + i % 3 * 3;
                let (x, (y, z)) = (0..9)
                    .into_iter()
                    .map(|j| (r + j, (c + j * 9, b + j / 3 * 9 + j % 3)))
                    .unzip();
                (Rc::new(x), (Rc::new(y), Rc::new(z)))
            })
            .unzip()
    }

    // 初始化
    fn init(&mut self) {
        for i in 0..9 {
            self.set_candidature(self.rows[i].clone());
            self.set_candidature(self.cols[i].clone());
            self.set_candidature(self.blocks[i].clone());
        }
    }

    // 填入候选数
    fn set_candidature(&mut self, idxes: Rc<Vec<usize>>) {
        let filled = self.get_filled(&*idxes);
        (*idxes).iter().for_each(|&x| {
            let n = self[x];
            if Self::is_mineral(n) && n & filled != 0 {
                self[x] ^= n & filled;
            }
        })
    }

    // 获取该行/列/区块所有已经确定的数，以二进制表示
    fn get_filled(&self, idxes: &Vec<usize>) -> u16 {
        idxes.iter().fold(0, |acc, &x| {
            let n = self[x];
            if Self::is_mineral(n) {
                acc
            } else {
                acc | (1 << n >> 1)
            }
        })
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self {
            data: [0; 81],
            rows: Vec::with_capacity(9),
            cols: Vec::with_capacity(9),
            blocks: Vec::with_capacity(9),
        }
    }
}

impl Deref for Sudoku {
    type Target = [u16; 81];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Sudoku {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
