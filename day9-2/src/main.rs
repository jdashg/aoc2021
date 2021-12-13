use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashSet;

struct HeightMap {
   rows: Vec<Vec<isize>>,
}

#[derive(PartialEq,Eq,Hash,Clone)]
struct Point {
   x: usize,
   y: usize,
}

impl HeightMap {
   fn at(&self, p: &Point) -> isize {
      self.rows[p.y][p.x].clone()
   }

   fn low_points(&self) -> Vec<Point> {
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
            ret.push(Point{x:x,y:y});
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
   const BORDER_HEIGHT: isize = 9;
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

// "What do you get if you multiply together the sizes of the three
//  largest basins?"
fn solve(input: &str) -> isize {
   let hm = parse_inputs(input);
   let mut basin_sizes: Vec<isize> = hm.low_points().into_iter()
      .map(|initial_p| {
         let mut basin_points = HashSet::new();
         let mut edge_points = Vec::new();
         basin_points.insert(initial_p.clone());
         edge_points.push(initial_p.clone());

         while let Some(p) = edge_points.pop() {
            //println!("popped {},{}", p.x, p.y);
            [
               Point{x:p.x-1, y:p.y},
               Point{x:p.x+1, y:p.y},
               Point{x:p.x, y:p.y-1},
               Point{x:p.x, y:p.y+1},
            ].iter().for_each(|p| {
               //println!("   trying {},{}", p.x, p.y);
               if basin_points.contains(p) || hm.at(p) >= 9 { return; }
               basin_points.insert(p.clone());
               edge_points.push(p.clone());
            });
         }
         //edge_points
         basin_points.len() as isize
      }).collect();
   basin_sizes.sort_unstable_by_key(|v| -v);
   basin_sizes[0..3].iter().fold(1, |a,b| a*b)
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
   assert_eq!(solve(&input), 1134);
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
