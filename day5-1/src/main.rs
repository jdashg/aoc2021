use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Point {
   x: usize,
   y: usize,
}

impl Point {
   fn new(s: &str) -> Point {
      let (x,y) = s.split_once(',').unwrap();
      Point{x:x.parse::<usize>().unwrap(),
            y:y.parse::<usize>().unwrap()}
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


fn solve(input: &str) -> usize {
   let inputs = input.split("\n")
                     .filter(|x| !x.is_empty());
   let lines = inputs.map(|s| Line::new(s));

   let mut map = HashMap::new();
   let mut dangers = 0;
   for line in lines {
      let points : Vec<Point> =
         if line.a.x == line.b.x {
            let (mut low, mut high) = (line.a.y, line.b.y);
            if high < low {
               std::mem::swap(&mut low, &mut high);
            }
            (low ..= high).map(|y| {
               Point{x:line.a.x, y:y}
            }).collect()
         } else if line.a.y == line.b.y {
            let (mut low, mut high) = (line.a.x, line.b.x);
            if high < low {
               std::mem::swap(&mut low, &mut high);
            }
            (low ..= high).map(|x| {
               Point{x:x, y:line.a.y}
            }).collect()
         } else {
            continue;
         };
      //println!("");
      for p in points {
         //println!("{},{}", p.x, p.y);
         let e = map.entry(p).or_insert(0);
         *e += 1;
         if *e == 2 {
            dangers += 1;
         }
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
   assert_eq!(solve(example_input), 5);
}

fn main() {
   println!("solve(example) correct");

   // -

   let path = Path::new("day5-1/src/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
