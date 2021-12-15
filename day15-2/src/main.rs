use std::fs::File;
use std::io::Read;
use std::path::Path;
//use std::collections::HashSet;
use std::iter::FromIterator;
use priority_queue::PriorityQueue;

// -

#[derive(PartialEq,Eq,Hash,Clone)]
struct Point {
   x: isize,
   y: isize,
}
impl Point {
   fn add(mut self, rhs: (isize,isize)) -> Self {
      self.x += rhs.0;
      self.y += rhs.1;
      self
   }
   fn nearest4(&self) -> Vec<Point> {
      vec![
         self.clone().add((-1,  0)),
         self.clone().add(( 1,  0)),
         self.clone().add(( 0, -1)),
         self.clone().add(( 0,  1)),
      ]
   }
}

#[derive(Clone)]
struct Grid<T> {
   rows: Vec<Vec<T>>,
}
impl<T: Clone> Grid<T> {
   fn new() -> Grid<T> {
      Grid::<T>{rows: Vec::new()}
   }
   fn len(&self) -> Point {
      if self.rows.len() == 0 {
         return Point{x:0,y:0};
      }
      Point{x: self.rows[0].len() as isize,
            y: self.rows.len() as isize}
   }
   fn at(&self, p: &Point) -> &T {
      &self.rows[p.y as usize][p.x as usize]
   }
   fn at_mut(&mut self, p: &Point) -> &mut T {
      &mut self.rows[p.y as usize][p.x as usize]
   }

   fn insert_row(&mut self, index: usize, elems: Vec<T>) {
      if !self.rows.is_empty() {
         assert_eq!(elems.len(), self.rows[0].len());
      }
      self.rows.insert(index, elems);
   }
   fn _insert_col(&mut self, index: usize, elems: Vec<T>) {
      assert_eq!(elems.len(), self.rows.len());
      for (row, elem) in self.rows.iter_mut().zip(elems) {
         row.insert(index, elem);
      }
   }
   fn push_row(&mut self, elems: Vec<T>) {
      self.insert_row(self.len().y as usize, elems);
   }
   fn _push_col(&mut self, elems: Vec<T>) {
      self._insert_col(self.len().x as usize, elems);
   }

   fn fill(&mut self, elem: T) {
      self.rows.iter_mut().for_each(|row| {
         row.iter_mut().for_each(|v| *v = elem.clone());
      });
   }

   fn is_valid(&self, p: &Point) -> bool {
      let size = self.len();
      0 <= p.x && p.x < size.x &&
      0 <= p.y && p.y < size.y
   }

   fn nearest4(&self, p: &Point) -> Vec<Point> {
      p.nearest4().into_iter().filter(|p| self.is_valid(p)).collect()
   }
}

impl<T: Clone> FromIterator<Vec<T>> for Grid<T> {
   fn from_iter<I: IntoIterator<Item=Vec<T>>>(iter: I) -> Self {
      let mut ret = Grid::new();
      iter.into_iter().for_each(|row| ret.push_row(row));
      ret
   }
}

// -

fn parse_inputs(input: &str) -> Grid<isize> {
   input.trim().lines().map(|line| {
      line.chars().map(|c| c.to_digit(10).unwrap() as isize).collect()
   }).collect()
}

//fn true_cost(grid: &Grid<isize>, p: &Point) -> isize {
//   let size = grid.len();
//   let mod_p = Point{x: p.x % size.x, y: p.y % size.y};
//   let div_p = Point{x: p.x / size.x, y: p.y / size.y};
//   if
//   let subgrid_increase = p.x / 5 + p.y / 5;
//   if 0 <= p.x &&

// "What is the lowest total risk of any path from the top left to the
//  bottom right?"
fn fix_cost(cost: isize) -> isize {
   assert!(cost - 9 < 10);
   if cost >= 10 {
      return cost - 9;
   }
   cost
}

fn inc_row_cost(row: &Vec<isize>, inc: isize) -> Vec<isize> {
   row.iter().map(|x| fix_cost(x+inc)).collect()
}

fn scale_grid(part: Grid<isize>, scale: isize) -> Grid<isize> {
   let size_before = part.len();
   let mut rows = part.rows;
   let mut new_rows_chunks: Vec<Vec<Vec<isize>>> = Vec::new();
   for _ in 1..scale {
      new_rows_chunks.push(Vec::new());
   }
   for row in rows.iter_mut() {
      let mut big_row = Vec::with_capacity(row.len()*scale as usize);
      for i in 0..scale {
         let part = inc_row_cost(&row, i);
         big_row.extend_from_slice(part.as_slice());
      }
      for i in 1..scale {
         new_rows_chunks[(i-1) as usize].push(inc_row_cost(&big_row, i));
      }
      *row = big_row;
   }
   new_rows_chunks.into_iter()
      .for_each(|new_rows| rows.extend_from_slice(new_rows.as_slice()));

   let ret = Grid{rows:rows};
   assert_eq!(ret.len().x, scale*size_before.x);
   assert_eq!(ret.len().y, scale*size_before.y);
   ret
}

fn initial_fill(path_cost_grid: &mut Grid<isize>, enter_cost_grid: &Grid<isize>) {
   let size = path_cost_grid.len();
   for i in 1..size.y {
      let p = Point{x:0, y:i-1};
      let n = Point{x:0, y:i};
      let path_p_cost = *path_cost_grid.at(&p);
      let enter_n_cost = enter_cost_grid.at(&n);
      let new_path_n_cost = path_p_cost + *enter_n_cost;
      *path_cost_grid.at_mut(&n) = new_path_n_cost;
   }
   for y in 0..size.y {
      for x in 1..size.x {
         let p = Point{x:x-1, y:y};
         let n = Point{x:x  , y:y};
         let path_p_cost = *path_cost_grid.at(&p);
         let enter_n_cost = enter_cost_grid.at(&n);
         let new_path_n_cost = path_p_cost + *enter_n_cost;
         *path_cost_grid.at_mut(&n) = new_path_n_cost;
      }
   }
   for y in 0..size.y {
      for x in 0..size.x {
         if x == 0 && y == 0 { continue; }
         let n = Point{x:x  , y:y};
         *path_cost_grid.at_mut(&n) += 1;
      }
   }
}

fn priority(path_cost_grid: &Grid<isize>, p: &Point) -> isize {
   let size = path_cost_grid.len();
   let cost = *path_cost_grid.at(&p);
   let estimated_remaining_cost = (size.x - p.x) + (size.y - p.y);
   -(cost + estimated_remaining_cost)
}

fn solve(input: &str, scale: isize) -> isize {
   let enter_cost_grid_part = parse_inputs(input);
   let enter_cost_grid = scale_grid(enter_cost_grid_part, scale);

   let mut path_cost_grid = enter_cost_grid.clone();
   path_cost_grid.fill(isize::MAX);
   let mut cost_refreshed_set = PriorityQueue::new();

   let start = Point{x:0,y:0};
   *path_cost_grid.at_mut(&start) = 0;
   initial_fill(&mut path_cost_grid, &enter_cost_grid);
   cost_refreshed_set.push_increase(start.clone(), priority(&path_cost_grid, &start));

   let mut i = 0;
//   while let Some(p) = cost_refreshed_set.iter().next().cloned() {
   while let Some((p,_)) = cost_refreshed_set.pop() {
      i += 1;
      if i % 100000 == 0 {
         println!("{}", i);
      }
      //cost_refreshed_set.take(&p);

      let path_p_cost = *path_cost_grid.at(&p);
      let neighbors = path_cost_grid.nearest4(&p);
      neighbors.into_iter().for_each(|n| {
         let cur_path_n_cost = path_cost_grid.at_mut(&n);
         let enter_n_cost = enter_cost_grid.at(&n);
         let new_path_n_cost = path_p_cost + *enter_n_cost;
         if *cur_path_n_cost > new_path_n_cost {
            *cur_path_n_cost = new_path_n_cost;
            //cost_refreshed_set.insert(n);
            cost_refreshed_set.push_increase(n.clone(), priority(&path_cost_grid, &n));
         }
      });
   }
   let end = path_cost_grid.len().add((-1,-1));
   *path_cost_grid.at(&end)
}

// -

//#[test]
fn test_example() {
   let input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
   assert_eq!(solve(&input,1), 40);
   println!("part1 ok");
   assert_eq!(solve(&input,5), 315);
   println!("part2 ok");
}

fn main() {
   test_example();

   let path = Path::new("day15-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input,1) -> {}", solve(&input,1));
   println!("solve(input,5) -> {}", solve(&input,5));
}
