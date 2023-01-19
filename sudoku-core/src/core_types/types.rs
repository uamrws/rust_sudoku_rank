use std::{collections::HashMap, rc::Rc};

use phf::phf_map;

static KNOWN_MAP: phf::Map<u16, u16> = phf_map! {
    0b1_0_0000_0001_u16 => 1,
    0b1_0_0000_0010_u16 => 2,
    0b1_0_0000_0100_u16 => 3,
    0b1_0_0000_1000_u16 => 4,
    0b1_0_0001_0000_u16 => 5,
    0b1_0_0010_0000_u16 => 6,
    0b1_0_0100_0000_u16 => 7,
    0b1_0_1000_0000_u16 => 8,
    0b1_1_0000_0000_u16 => 9,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sudoku {
    pub data: [u16; 81],
    pub rows: Vec<Rc<Vec<usize>>>,
    pub cols: Vec<Rc<Vec<usize>>>,
    pub blocks: Vec<Rc<Vec<usize>>>,
}
enum ScoreRule {
    NakedSingle = 1,
    HiddenSingle = 5,

    NakedPair = 10,
    HiddenPair = 15,

    NakedTriplet = 20,
    HiddenTriplet = 25,

    NakedQuad = 30,
    HiddenQuad = 35,

    IntersectionRemovel = 40,

    XWing = 500,
    Swordfish = 1000,
    XYWing = 3000,
    XYZWing = 5000,
    Trial = 8000,
}
pub struct SudokuScore {
    score_board: HashMap<ScoreRule, u16>,
}

// 通用工具
pub trait CommonTool {
    const MINERAL_FLAG: u16 = 0b10_0000_0000;

    fn is_mineral(stone: u16) -> bool {
        stone & Self::MINERAL_FLAG != 0
    }

    fn refine(mineral: u16) -> u16 {
        KNOWN_MAP[&mineral]
    }

    fn transform(loc: usize) -> (usize, usize, usize) {
        (loc / 9, loc % 9, loc / 27 * 3 + loc % 9 / 3)
    }
}
impl<'a> CommonTool for SudokuTracer<'a> {}
impl CommonTool for NakedBotMiner {}
impl CommonTool for Sudoku {}

// 显示机器矿工
pub struct NakedBotMiner {
    // 矿镐(需要消除数字)
    mining_pick: u16,
    // 矿区路径: 对应的行/列/区块
    route: Vec<Rc<Vec<usize>>>,
    skip: Vec<usize>,
    // 挖矿成功的位置记录
    loc_record: Vec<usize>,
}

// 智能探矿车
// sudoku ai tracer
pub struct SudokuTracer<'a> {
    pub sudoku: &'a mut Sudoku,
    pub loc: usize,
}
pub struct NakedScanner;
impl NakedScanner {
    fn scan(sudoku: &mut Sudoku) {}
}

impl NakedBotMiner {
    fn new(mining_pick: u16, route: Vec<Rc<Vec<usize>>>, skip: Vec<usize>) -> NakedBotMiner {
        NakedBotMiner {
            // 矿镐(需要消除数字)
            mining_pick,
            route,
            skip,
            loc_record: Vec::with_capacity(9),
        }
    }
    fn work(&mut self, sudoku: &mut Sudoku) {
        // 遍历所有的矿区(行/列/区块)
        self.route.iter().for_each(|x| {
            // 筛选掉需要跳过的位置
            (*x).iter()
                .filter(|&i| !self.skip.contains(i))
                .for_each(|&i| {
                    let n = sudoku[i];
                    // 挖矿成功的记录下位置
                    if Self::is_mineral(n) && n & self.mining_pick != 0 {
                        sudoku[i] ^= n & self.mining_pick;
                        self.loc_record.push(i)
                    }
                })
        })
    }
}

// 矿脉: 当前的行/列/区块
// 矿点: 未解答出来的单元格

// 步骤一 进入第一个矿点A位置
// 步骤二 使用扫描枪(各种规则)扫描矿点的整个矿区
// 显示规则扫描枪:
//      步骤1 扫描位置A的所有矿区(行/列/区块)
//      步骤2 按规则比对，派挖矿机器人挖掘符合条件的矿石，并记录挖矿成功的位置
//      步骤3 挖掘完毕后，比对记录的位置B和位置A的大小，如果B<A,重复步骤1
// 其他规则:
// 步骤三 进入下一个矿点，重复步骤二
impl<'a> SudokuTracer<'a> {
    pub fn new(sudoku: &mut Sudoku) -> SudokuTracer {
        SudokuTracer { sudoku, loc: 0 }
    }

    pub fn start(&mut self) {
        while self.loc < self.sudoku.len() {
            if Self::is_mineral(self.sudoku[self.loc]) {
                self.work();
            }
            self.walk();
        }
    }

    pub fn work(&mut self) {
        self.naked_scan(self.loc)
    }

    pub fn walk(&mut self) {
        self.loc += 1
    }

    // 显示规则扫描枪:
    //      步骤1 扫描位置A的所有矿区(行/列/区块)
    //      步骤2 按规则比对，派挖矿机器人挖掘符合条件的矿石，并记录挖矿成功的位置
    //      步骤3 挖掘完毕后，比对记录的位置B和位置A的大小，如果B<A,重复步骤1
    pub fn naked_scan(&mut self, loc: usize) {
        let mineral = self.sudoku[loc];

        let mut loc_record = vec![];
        let (r, c, b) = Self::transform(loc);
        let row = self.sudoku.rows[r].clone();
        let col = self.sudoku.cols[c].clone();
        let block = self.sudoku.blocks[b].clone();
        match mineral.count_ones() {
            2 => {
                self.sudoku[loc] = Self::refine(mineral);
                let mut naked_bot = NakedBotMiner::new(
                    mineral ^ Self::MINERAL_FLAG,
                    vec![row, col, block],
                    vec![loc],
                );
                naked_bot.work(&mut self.sudoku);
                loc_record.extend(naked_bot.loc_record);
            }
            x if x <= 5 => {
                let mut scan = |v: Rc<Vec<usize>>| {
                    let mut skip = vec![loc];
                    for i in &*v {
                        if *i != loc && mineral == self.sudoku[*i] {
                            skip.push(*i);
                        }
                        if skip.len() == x as usize {
                            let mut naked_bot =
                                NakedBotMiner::new(mineral ^ Self::MINERAL_FLAG, vec![v], skip);
                            naked_bot.work(&mut self.sudoku);
                            loc_record.extend(naked_bot.loc_record);
                            break;
                        }
                    }
                };
                scan(row);
                scan(col);
                scan(block);
            }
            _ => return,
        }
        loc_record.iter().filter(|&i| i != &loc).for_each(|&i| {
            if Self::is_mineral(self.sudoku[i]) {
                self.naked_scan(i)
            }
        });
    }
}
