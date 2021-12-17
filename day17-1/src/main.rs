use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::cmp::max;

// -

#[derive(Debug,Clone)]
struct Vec2 {
   x: i64,
   y: i64,
}

struct Range {
   a: i64,
   b: i64,
}
impl Range {
   fn from(s: &str) -> Range {
      let (a,b) = s.split_once("..").unwrap();
      let ret = Range{a: a.parse().unwrap(),
       b: b.parse().unwrap()};
      assert!(ret.a <= ret.b);
      ret
   }
   fn contains(&self, c: i64) -> bool {
      self.a <= c && c <= self.b
   }
   fn cmp(&self, c: i64) -> isize {
      if c < self.a { return -1; }
      else if c > self.b { return 1; }
      else { return 0; }
   }
}

struct Area {
   x: Range,
   y: Range,
}
impl Area {
   fn contains(&self, p: &Vec2) -> bool {
      self.x.contains(p.x) && self.y.contains(p.y)
   }
}

fn parse_inputs(mut input: &str) -> Area {
   input = input.trim();
   let (_,input2) = input.split_once("x=").unwrap();
   let (sx,sy) = input2.split_once(", y=").unwrap();
   let y = Range::from(sy);
   Area{x: Range::from(sx),
        y: Range::from(sy)}
}

fn step(p: &mut Vec2, v: &mut Vec2, max_y: &mut i64) {
   p.x += v.x;
   p.y += v.y;
   // Drag:
   if v.x > 0 {
      v.x -= 1;
   } else if v.x < 0 {
      v.x += 1;
   }
   v.y -= 1; // Gravity;
   *max_y = max(*max_y, p.y.clone());
}

fn solve_p1(input: &str) -> usize {
   let area = parse_inputs(&input);

   let x0 = area.x.a; // Irrelevant to y.
   let p0 = Vec2{x:x0, y: 0};
   let mut best = (p0.clone(), 0);
   for yv0 in -100..10000 {
      if yv0 % 100 == 0 {
         println!("trying yv0 {}...", yv0);
      }
      let mut p = p0.clone();
      let mut v = Vec2{x:0, y: yv0};
      let mut max_y = p.y;
      while area.y.cmp(p.y) == -1 {
         step(&mut p, &mut v, &mut max_y);
      }
      while area.y.cmp(p.y) == 1 {
         step(&mut p, &mut v, &mut max_y);
      }
      if area.y.cmp(p.y) == 0 {
         if max_y > best.1 {
            best = (v, max_y);
            println!("new best: {:?}", best);
         }
      }
   }
   best.1 as usize
}

// -

//#[test]
fn test_example() {
   assert_eq!(solve_p1("target area: x=20..30, y=-10..-5"), 45);
   println!("Examples complete.");
}

fn main() {
   test_example();

   let path = Path::new("day17-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve_p1(&input));
}
