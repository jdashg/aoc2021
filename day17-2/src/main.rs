use std::fs::File;
use std::io::Read;
use std::path::Path;

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

fn step(p: &mut Vec2, v: &mut Vec2) {
   p.x += v.x;
   p.y += v.y;
   // Drag:
   if v.x > 0 {
      v.x -= 1;
   } else if v.x < 0 {
      v.x += 1;
   }
   v.y -= 1; // Gravity;
}

fn solve(input: &str) -> usize {
   let area = parse_inputs(&input);

   let possible_yv0s = {
      let p0 = Vec2{x:0, y: 0};
      let mut ret = Vec::new();
      for yv0 in -200..1000 {
         if yv0 % 100 == 0 {
            println!("trying yv0 {}...", yv0);
         }
         let mut p = p0.clone();
         let mut v = Vec2{x:0, y: yv0};
         while area.y.cmp(p.y) == -1 {
            step(&mut p, &mut v);
         }
         while area.y.cmp(p.y) == 1 {
            step(&mut p, &mut v);
         }
         if area.y.cmp(p.y) == 0 {
            ret.push(yv0);
            println!("y hit at {:?}", yv0);
         }
      }
      ret
   };
   let possible_xv0s = {
      let p0 = Vec2{x:0, y: 0};
      let mut ret = Vec::new();
      for xv0 in 0..1000 {
         if xv0 % 100 == 0 {
            println!("trying xv0 {}...", xv0);
         }
         let mut p = p0.clone();
         let mut v = Vec2{x:xv0, y: 0};
         while v.x > 0 && area.x.cmp(p.x) == -1 {
            step(&mut p, &mut v);
         }
         if area.x.cmp(p.x) == 0 {
            ret.push(xv0);
            println!("x hit at {:?}", xv0);
         }
      }
      ret
   };

   let possible_v0s = {
      let p0 = Vec2{x:0, y: 0};

      let mut ret = Vec::new();
      for yv0 in possible_yv0s.iter() {
         for xv0 in possible_xv0s.iter() {
            let v0 = Vec2{x:*xv0, y: *yv0};
            let mut p = p0.clone();
            let mut v = v0.clone();
            while area.y.cmp(p.y) == -1 {
               step(&mut p, &mut v);
            }
            while area.y.cmp(p.y) == 1 {
               step(&mut p, &mut v);
            }
            while area.y.cmp(p.y) != -1 {
               if area.contains(&p) {
                  println!("v0 hit #{} at {:?}", ret.len(), v0);
                  ret.push(v0);
                  break;
               }
               step(&mut p, &mut v);
            }
         }
      }
      ret
   };
   possible_v0s.len()
}

// -

//#[test]
fn test_example() {
   assert_eq!(solve("target area: x=20..30, y=-10..-5"), 112);
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

   println!("solve(input) -> {}", solve(&input));
}
