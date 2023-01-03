// use std::{marker::PhantomPinned, pin::Pin};

// use serde::{ser::SerializeTuple, Deserialize, Serialize};

// use crate::{core_traits::rules::SudokuRule, Sudoku};

// impl SudokuRule for Sudoku {
//     fn naked_single(&mut self) {
//         for r in self.neg_rows {
//             for c in r {}
//         }
//     }

//     fn naked_pair(&mut self) {
//         todo!()
//     }

//     fn naked_triplet(&mut self) {
//         todo!()
//     }

//     fn naked_quad(&mut self) {
//         todo!()
//     }

//     fn hidden_single(&mut self) {
//         todo!()
//     }

//     fn hidden_pair(&mut self) {
//         todo!()
//     }

//     fn hidden_triplet(&mut self) {
//         todo!()
//     }

//     fn hidden_quad(&mut self) {
//         todo!()
//     }

//     fn intersection_removel(&mut self) {
//         todo!()
//     }

//     fn x_wing(&mut self) {
//         todo!()
//     }

//     fn xy_wing(&mut self) {
//         todo!()
//     }

//     fn xyz_wing(&mut self) {
//         todo!()
//     }

//     fn swordfish(&mut self) {
//         todo!()
//     }

//     fn trial_and_error(&mut self) {
//         todo!()
//     }
// }

// impl Serialize for Pin<Box<Sudoku>> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut s = serializer.serialize_tuple(9)?;
//         for i in &self.inner {
//             s.serialize_element(i)?;
//         }
//         s.end()
//     }
// }

// impl<'de> Deserialize<'de> for Pin<Box<Sudoku>> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         Ok(Sudoku::new(<[[u16; 9]; 9] as Deserialize>::deserialize(
//             deserializer,
//         )?))
//     }
// }

// // impl<'de> Deserialize<'de> for Sudoku {
// //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// //     where
// //         D: serde::Deserializer<'de>,
// //     {
// //         deserializer.deserialize_tuple(9, SudokuVisitor)
// //     }
// // }
// // struct SudokuVisitor;

// // impl<'de> Visitor<'de> for SudokuVisitor {
// //     type Value = Sudoku;

// //     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
// //         formatter.write_str("tuple must be like [[u16; 9]; 9]")
// //     }
// //     fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
// //     where
// //         A: de::SeqAccess<'de>,
// //     {
// //         let mut i = 0;
// //         let mut inner: [[u16; 9]; 9] = [[0; 9]; 9];
// //         while let Ok(Some(v)) = seq.next_element() {
// //             inner[i] = v;
// //             i += 1;
// //         }
// //         Ok(Self::Value::new(inner))
// //     }
// // }
