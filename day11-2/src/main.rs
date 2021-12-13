use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Clone)]
struct Point {
   x: usize,
   y: usize,
}
struct Grid<T> {
   rows: Vec<Vec<T>>,
}
impl<T: Clone> Grid<T> {
   fn new() -> Grid<T> {
      Grid::<T>{rows: Vec::new()}
   }
   fn len(&self) -> Point {
      Point{x: self.rows[0].len(),
            y: self.rows.len()}
   }
   //fn at(&self, p: &Point) -> &T {
   //   &self.rows[p.y][p.x]
   //}
   fn at_mut(&mut self, p: &Point) -> &mut T {
      &mut self.rows[p.y][p.x]
   }

   fn insert_row(&mut self, index: usize, elems: Vec<T>) {
      if !self.rows.is_empty() {
         assert_eq!(elems.len(), self.rows[0].len());
      }
      self.rows.insert(index, elems);
   }
   fn insert_col(&mut self, index: usize, elems: Vec<T>) {
      assert_eq!(elems.len(), self.rows.len());
      for (row, elem) in self.rows.iter_mut().zip(elems) {
         row.insert(index, elem);
      }
   }
   fn push_row(&mut self, elems: Vec<T>) {
      self.insert_row(self.len().y, elems);
   }
   fn push_col(&mut self, elems: Vec<T>) {
      self.insert_col(self.len().x, elems);
   }

   fn insert_border(&mut self, elem: T) {
      let size = self.len();
      let mut border = Vec::with_capacity(2+std::cmp::max(size.x, size.y));
      // Col first
      border.resize(size.y, elem.clone());
      self.insert_col(0, border.clone());
      self.push_col(border.clone());

      border.resize(2+size.x, elem.clone());
      self.insert_row(0, border.clone());
      self.push_row(border);
   }
   fn set_border(&mut self, elem: T) {
      let size = self.len();
      self.rows[0].fill(elem.clone());
      for row in self.rows.iter_mut() {
         row[0] = elem.clone();
         row[size.x-1] = elem.clone();
      }
      self.rows[size.y-1].fill(elem);
   }
}

// -

fn step(grid: &mut Grid<usize>) -> usize {
   let size = grid.len();
   let mut flashes = 0;

   // "First, the energy level of each octopus increases by 1."

   fn inc_energy(p: &Point, val: &mut usize, to_flash: &mut Vec<Point>) {
      *val += 1;
      if *val == 10 {
         to_flash.push((*p).clone());
      }
   }

   let mut to_flash = Vec::new();
   for y in 1..(size.y-1) {
      for x in 1..(size.x-1) {
         let p = Point{x:x, y:y};
         inc_energy(&p, grid.at_mut(&p), &mut to_flash);
      }
   }

   while let Some(p) = to_flash.pop() {
      flashes += 1;
      [
         Point{x:p.x-1, y:p.y-1},
         Point{x:p.x  , y:p.y-1},
         Point{x:p.x+1, y:p.y-1},
         Point{x:p.x-1, y:p.y  },
       //Point{x:p.x  , y:p.y  },
         Point{x:p.x+1, y:p.y  },
         Point{x:p.x-1, y:p.y+1},
         Point{x:p.x  , y:p.y+1},
         Point{x:p.x+1, y:p.y+1},
      ].iter().for_each(|p| {
         inc_energy(&p, grid.at_mut(&p), &mut to_flash);
      });
   }

   // "Finally, any octopus that flashed during this step has its
   //  energy level set to 0, as it used all of its energy to flash."
   for y in 1..(size.y-1) {
      for x in 1..(size.x-1) {
         let p = Point{x:x, y:y};
         let val = grid.at_mut(&p);
         if *val > 9 {
            *val = 0;
         }
      }
   }
   grid.set_border(0);
   flashes
}


// -

fn parse_inputs(input: &str) -> Grid<usize> {
   let mut grid = Grid::new();
   grid.rows = input.trim().split("\n")
      .map(|line| {
         let row: Vec<usize> = line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
         row
      }).collect();
   grid
}

// "What is the first step during which all octopuses flash?"
fn solve(input: &str) -> usize {
   let mut grid = parse_inputs(input);
   let size = grid.len();
   let num_octos = size.x * size.y;

   grid.insert_border(0);

   for i in 1..1000 {
      let flashes = step(&mut grid);
      if flashes == num_octos { return i; }
   }
   panic!("Not found");
}

// -

#[test]
fn test_example() {
   let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
   assert_eq!(solve(&input), 195);
}

fn main() {
   let path = Path::new("day11-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
