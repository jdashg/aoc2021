use std::fs::File;
use std::io::Read;
use std::path::Path;

struct HeightMap {
   rows: Vec<Vec<isize>>,
}

impl HeightMap {
   fn low_points(&self) -> Vec<(usize,usize,isize)> {
      let mut ret = Vec::new();
      for y in 1..(self.rows.len()-1) {
         let row_prev = &self.rows[y-1];
         let row = &self.rows[y];
         let row_next = &self.rows[y+1];
         for x in 1..(row.len()-1) {
            let v = &row[x];
            let a = &row[x-1];
            let b = &row[x+1];
            if (!(v < a && v < b)) { continue; }
            let c = &row_prev[x];
            let d = &row_next[x];
            if (!(v < c && v < d)) { continue; }
            ret.push((x,y,*v));
         }
      }
      ret
   }
}

fn parse_inputs(input: &str) -> HeightMap {
   let lines = input.trim().split("\n");
   let raw_width = lines.clone().next().unwrap().len();

   let mut hm = HeightMap { rows: Vec::new() };
   let mut row: Vec<isize> = Vec::new();
   const BORDER_HEIGHT: isize = 100;
   row.resize(1+raw_width+1, BORDER_HEIGHT);
   hm.rows.push(row.clone());

   hm.rows.extend(
      lines
         .map(|line| {
            row.resize(1, BORDER_HEIGHT);
            row.extend(
               line.chars().map(|c| c.to_digit(10).unwrap() as isize)
            );
            row.push(BORDER_HEIGHT);
            row.clone()
         })
      );
   row.fill(BORDER_HEIGHT);
   hm.rows.push(row);
   hm
}

// "Find all of the low points on your heightmap. What is the sum of the
//  risk levels of all low points on your heightmap?"
fn solve(input: &str) -> isize {
   let hm = parse_inputs(input);
   hm.low_points().into_iter().map(|(_,_,v)| 1+v).sum()
}


#[test]
fn test_example() {
   let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
   assert_eq!(solve(&input), 15);
}

fn main() {
   let path = Path::new("day9-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
