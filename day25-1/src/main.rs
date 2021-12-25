use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

type Coord = (i64,i64);

#[derive(Clone,Copy,PartialEq)]
enum Herd {
   East = 0,
   South = 1,
}

#[derive(Clone)]
struct State {
   herd_by_coord: HashMap<Coord,Herd>,
   size: Coord,
}
impl State {
   fn step(&self) -> (State, i64) {
      let (next_e, moves_e) = self.step_for(Herd::East);
      let (next_s, moves_s) = next_e.step_for(Herd::South);
      (next_s, moves_e + moves_s)
   }
   fn step_for(&self, move_herd: Herd) -> (State, i64) {
      let mut next = self.clone();
      let offset = match move_herd {
         Herd::East => (1,0),
         Herd::South => (0,1),
      };
      next.herd_by_coord.clear();
      let mut moves = 0;
      for (pos, herd) in self.herd_by_coord.iter() {
         if *herd == move_herd {
            if let Some(next_pos) = self.try_mov(pos, &offset) {
               next.herd_by_coord.insert(next_pos, *herd);
               moves += 1;
               continue;
            } // Else don't move
         }
         next.herd_by_coord.insert(*pos, *herd);
      }
      (next, moves)
   }
   fn try_mov(&self, at: &Coord, offset: &Coord) -> Option<Coord> {
      let mut at = (at.0+offset.0,
                    at.1+offset.1);
      at.0 = (at.0 + self.size.0) % self.size.0;
      at.1 = (at.1 + self.size.1) % self.size.1;
      if !self.herd_by_coord.contains_key(&at) {
         Some(at)
      } else {
         None
      }
   }
}

fn parse(input: &str) -> State {
   let mut state = State {
      herd_by_coord: HashMap::new(),
      size: (0,0),
   };

   input.trim().lines().enumerate().for_each(|(y,line)| {
      state.size.1 += 1;
      state.size.0 = line.len() as i64;
      line.chars().enumerate().for_each(|(x,c)| {
         let pos = (x as i64,y as i64);
         let herd = match c {
            '>' => Herd::East,
            'v' => Herd::South,
            '.' => { return; },
            _ => panic!("{}", c),
         };
         state.herd_by_coord.insert(pos, herd);
      });
   });
   state
}

// "What is the first step on which no sea cucumbers move?"

fn solve(input: &str) -> usize {
   let mut state = parse(input);
   for i in 1.. {
      let (next, moves) = state.step();
      if moves == 0 {
         return i;
      }
      state = next;
   }
   panic!("unreachable");
}

// -

//#[test]
fn test_example() {
   let input = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";
   assert_eq!(solve(&input), 58);
}

fn main() {
   test_example();
   println!("Examples ran clean!");

   let path = Path::new("day25-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
