use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
   x: isize,
   y: isize,
}

impl Point {
   fn new(s: &str) -> Point {
      let (x,y) = s.split_once(',').unwrap();
      Point{x:x.parse::<isize>().unwrap(),
            y:y.parse::<isize>().unwrap()}
   }
}

struct Line {
   a: Point,
   b: Point,
}

impl Line {
   fn new(s: &str) -> Line {
      let (a, b) = s.split_once(" -> ")
                    .unwrap();
      Line{a:Point::new(a),
            b:Point::new(b)}
   }
}

fn sign(v: isize) -> isize {
   if v == 0 { 0 }
   else if v < 0 { -1 }
   else { 1 }
}

fn solve(input: &str) -> usize {
   let inputs = input.split("\n")
                     .filter(|x| !x.is_empty());
   let lines = inputs.map(|s| Line::new(s));

   let mut map = HashMap::new();
   let mut dangers = 0;
   for line in lines {
      let mut p = line.a.clone();
      let mut dx = sign(line.b.x - line.a.x);
      let mut dy = sign(line.b.y - line.a.y);
      loop {
         {
            let e = map.entry(p.clone()).or_insert(0);
            *e += 1;
            if *e == 2 {
               dangers += 1;
            }
         }

         if p == line.b { break; }
         if p.x != line.b.x { p.x += dx; }
         if p.y != line.b.y { p.y += dy; }
      }
   }
   dangers
}

#[test]
fn example() {
   let example_input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
   assert_eq!(solve(example_input), 12);
}

fn main() {
   let path = Path::new("day5-1/src/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
