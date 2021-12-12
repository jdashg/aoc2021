use std::fs::File;
use std::io::Read;
use std::path::Path;

struct BingoNumber {
   val: usize,
   marked: bool,
}

struct BingoBoard {
   rows: Vec<Vec<BingoNumber>>,
}

impl BingoBoard {
   fn mark(&mut self, val: usize) -> Option<(usize,usize)> {
      for (y, row) in self.rows.iter_mut().enumerate() {
         for (x, cur) in row.iter_mut().enumerate() {
            if cur.val == val {
               cur.marked = true;
               return Some((x,y));
            }
         }
      }
      return None;
   }
   fn is_bingo(&self, x: usize, y: usize) -> bool {
      return self.rows[y].iter().map(|bn| bn.marked as usize).sum::<usize>() == 5 ||
             self.rows.iter().map(|row| row[x].marked as usize).sum::<usize>() == 5;
   }
   fn unmarked_sum(&self) -> usize {
      return self.rows.iter().flatten().map(|bn| (!bn.marked) as usize * bn.val).sum();
   }

   fn try_mark(&mut self, val: usize) -> Option<usize> {
      match self.mark(val) {
         Some((x,y)) => {
            if self.is_bingo(x,y) {
               return Some(self.unmarked_sum() * val);
            }
            None
         },
         _ => None,
      }
   }
}

fn result(input: &str) -> usize {
   let mut inputs = input.split("\n")
                     .filter(|x| !x.is_empty());
   let mark_list = inputs.next().unwrap()
                         .split(",")
                         .map(|s| s.parse::<usize>().unwrap());

   let mut bbs = Vec::<BingoBoard>::new();
   let mut rows = Vec::<Vec<BingoNumber>>::new();
   for line in inputs {
      let row: Vec<BingoNumber> = line.trim()
                                      .split_whitespace()
                                      .map(|s| BingoNumber{
                                          val: s.parse::<usize>().unwrap(),
                                          marked: false,
                                       })
                                      .collect();
      rows.push(row);

      if rows.len() == 5 {
         bbs.push(BingoBoard {rows: rows});
         rows = Vec::<Vec<BingoNumber>>::new();
      }
   }
   assert_eq!(rows.len(), 0);

   for new_mark in mark_list {
      for bb in bbs.iter_mut() {
         match bb.try_mark(new_mark) {
            Some(score) => return score,
            _ => {},
         }
      }
   }
   panic!("Bingo not found!");
}

fn main() {
   let example = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
   assert_eq!(result(example), 4512);
   println!("result(example) correct");

   // -

   let path = Path::new("day4-input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("result(input) -> {}", result(&input));
}
