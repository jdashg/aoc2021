use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashSet;
use std::cmp;

type SparseGrid = HashSet<(i32,i32)>;

struct BigInt {
   chunks: Vec<u64>,
}
impl BigInt {
   fn new() -> BigInt {
      BigInt{chunks: Vec::new()}
   }
   fn set_bit(&mut self, i: &usize, bval: bool) {
      let chunk_id = i / 64;
      let chunk_i = i % 64;
      while chunk_id >= self.chunks.len() {
         self.chunks.push(0);
      }
      let val = (bval as u64) << chunk_i;
      if val != 0 {
         self.chunks[chunk_id] |= val;
      } else {
         self.chunks[chunk_id] &= !val;
      }
   }
   fn get_bit(&self, i: &usize) -> bool {
      let chunk_id = i / 64;
      let chunk_i = i % 64;
      assert!(chunk_id < self.chunks.len());
      let val = self.chunks[chunk_id];
      ((val >> chunk_i) & 1) != 0
   }
   fn invert(&self) -> BigInt {
      BigInt {
         chunks: self.chunks.iter().map(|c| !c).collect(),
      }
   }
}

fn parse(input: &str) -> (BigInt, SparseGrid) {
   let (lookup, grid_s) = input.split_once("\n\n").unwrap();

   let bi = {
      let mut bi = BigInt::new();
      let mut i = 0;
      lookup.trim().lines().for_each(|line| {
         line.chars().for_each(|c| {
            let bval = (c == '#');
            bi.set_bit(&i, bval);
            i += 1;
         });
      });
      assert_eq!(i, 512);
      bi
   };

   let grid = {
      let mut grid = SparseGrid::new();
      grid_s.trim().lines().enumerate().for_each(|(y,line)| {
         line.chars().enumerate().for_each(|(x,c)| {
            let bval = (c == '#');
            if bval {
               grid.insert((x as i32, y as i32));
            }
         });
      });
      grid
   };

   (bi, grid)
}

fn contains_with_offset(grid: &SparseGrid, p: &(i32,i32), o: &(i32,i32)) -> bool {
   grid.contains(&(p.0+o.0,
                   p.1+o.1))
}

fn insert_with_offset(grid: &mut SparseGrid, p: &(i32,i32), o: &(i32,i32)) {
   grid.insert((p.0+o.0,
                p.1+o.1));
}

const KERNEL_OFFSETS: [(i32, i32); 9] = [
   (-1,-1),
   ( 0,-1),
   ( 1,-1),

   (-1, 0),
   ( 0, 0),
   ( 1, 0),

   (-1, 1),
   ( 0, 1),
   ( 1, 1),
];

fn solve_p1(input: &str, reps: usize) -> usize {
   let (lookup, mut grid) = parse(input);

   //println!("\n\n\nInitial:");
   //print_bi(&lookup);
   //println!("");
   //println(&grid);

   //let inv_lookup = lookup.invert();
   let mut bright_val = true;
   let bright_flips = lookup.get_bit(&0);

   for i in 0..reps {
      let mut next_grid = SparseGrid::new();
      for p in grid.iter() {
         for o in KERNEL_OFFSETS.iter() {
            insert_with_offset(&mut next_grid, p, o);
         }
      }
      next_grid = next_grid.into_iter().filter(|p| {
         let mut kernel: usize = 0;
         for o in KERNEL_OFFSETS.iter() {
            kernel <<= 1;
            kernel |= contains_with_offset(&grid, &p, o) as usize;
         }
         if bright_val == false {
            kernel = !kernel & 0b1_1111_1111;
         }
         let mut bval = lookup.get_bit(&kernel);
         //println!("[{}] {:?} -> {} -> {}", i+1, p, kernel, bval);
         if bright_flips && bright_val == true {
            bval = !bval;
         }
         bval
      }).collect();
      grid = next_grid;
      if bright_flips {
         bright_val = !bright_val;
      }
      //println!("\nrep {}", i+1);
      //println!("bright_val {}", bright_val);
      //println(&grid);
   }
   grid.len()
}

// -


fn print_bi(bi: &BigInt) {
   let row: String = (0..(64*bi.chunks.len()))
      .map(|i| if bi.get_bit(&i) { "#" } else { "." }).collect();
   println!("{}", row);
}

fn println(grid: &SparseGrid) {
   let mut min: (i32,i32) = (0,0);
   let mut max: (i32,i32) = (0,0);
   for p in grid {
      min.0 = cmp::min(min.0, p.0);
      min.1 = cmp::min(min.1, p.1);
      max.0 = cmp::max(max.0, p.0);
      max.1 = cmp::max(max.1, p.1);
   }
   println!("{:?}..={:?} ({} on)", min, max, grid.len());

   let row: String = (min.0..=max.0).map(|_| '.').collect();
   let mut rows: Vec<String> = (min.1..=max.1).map(|_| row.clone()).collect();

   for p in grid {
      let x = (p.0 - min.0) as usize;
      let y = p.1 - min.1;
      let row = &mut rows[y as usize];
      row.replace_range(x..x+1, "#");
   }
   for row in rows.iter() {
      println!("{}", row);
   }
}

// -

//#[test]
fn test_example() {
   let input = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";
   assert_eq!(solve_p1(&input,0), 10);
   assert_eq!(solve_p1(&input,2), 35);
   assert_eq!(solve_p1(&input,50), 3351);

   let input2 = "\
#.#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";
   //assert_eq!(solve_p1(&input2,0), 10);
   //assert_eq!(solve_p1(&input2,2), 35);
}

fn main() {
   test_example();
   println!("Examples ran clean!");

   let path = Path::new("day20-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve_p1(input,2) -> {}", solve_p1(&input,2));
   println!("solve_p1(input,50) -> {}", solve_p1(&input,50));
}
