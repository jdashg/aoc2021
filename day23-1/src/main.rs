use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::cmp;
use std::str;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::{thread, time};

struct Graph {
   neighbor_costs_by_from: HashMap<Coord,Vec<(Coord,i64)>>,
}
impl Graph {
   fn new() -> Graph {
      Graph{neighbor_costs_by_from: HashMap::new()}
   }
   fn add_directed_edge(&mut self, from: &Coord, to: &Coord, cost: i64) {
      let e = self.neighbor_costs_by_from.entry(*from)
            .or_insert(Vec::new());
      e.push((*to,cost));
   }
   fn add_bidi_edge(&mut self, from_and_to: &Coord, to_and_from: &Coord, cost: i64) {
      self.add_directed_edge(from_and_to, to_and_from, cost);
      self.add_directed_edge(to_and_from, from_and_to, cost);
   }

}


const MARKED_MAP: &str = "\
#############
#12.3.4.5.67#
###A#B#C#D###
  #a#b#c#d#
  #########";

fn all_coords() -> Vec<Coord> {
   let coord_by_char: HashMap<char,Coord> = rough_parse(MARKED_MAP)
      .into_iter().map(|(coord,c)| {
         (c, coord)
      }).collect();
   coord_by_char.into_iter().map(|(c,coord)| coord).collect()
}

fn make_house_graph() -> Graph {
   let mut graph = Graph::new();
   let coord_by_char: HashMap<char,Coord> = rough_parse(MARKED_MAP)
      .into_iter().map(|(coord,c)| {
         (c, coord)
      }).collect();

   let mut add = |from: char, to: char, cost: i64| {
      let from = coord_by_char.get(&from).unwrap();
      let to = coord_by_char.get(&to).unwrap();
      graph.add_bidi_edge(from, to, cost);
   };

   add('1', '2', 1);
   add('2', '3', 2);
   add('3', '4', 2);
   add('4', '5', 2);
   add('5', '6', 2);
   add('6', '7', 1);

   add('a', 'A', 1);
   add('A', '2', 2);
   add('A', '3', 2);

   add('b', 'B', 1);
   add('B', '3', 2);
   add('B', '4', 2);

   add('c', 'C', 1);
   add('C', '4', 2);
   add('C', '5', 2);

   add('d', 'D', 1);
   add('D', '5', 2);
   add('D', '6', 2);

   graph
}

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
enum Amphipod {
   A = 1,
   B = 10,
   C = 100,
   D = 1000,
}

impl Ord for Amphipod {
   fn cmp(&self, rhs: &Self) -> cmp::Ordering {
      (*self as i64).cmp(&(*rhs as i64))
   }
}
impl PartialOrd for Amphipod {
   fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
      Some(self.cmp(rhs))
   }
}

type Coord = (i64,i64);


const BLANK_MAP: &str = "\
#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########
0123456789012";

#[derive(Clone,PartialEq,Eq,Hash,PartialOrd,Ord)]
struct PositionState {
   occupant_by_node: BTreeMap<Coord, Amphipod>,
}

fn manhat_dist(a: &Coord, b: &Coord) -> i64 {
   (a.0 - b.0).abs() +
   (a.1 - b.1).abs()
}
impl PositionState {
   fn new() -> PositionState{
      PositionState{occupant_by_node: BTreeMap::new()}
   }
   fn mov(&self, from: &Coord, to: &Coord) -> (PositionState, i64) {
      let mut next = self.clone();
      let who = next.occupant_by_node.remove(from).unwrap();
      let prev = next.occupant_by_node.insert(*to, who);
      assert_eq!(prev, None);

      let dist = if from.0 == to.0 {
         manhat_dist(from,to)
      } else {
         let from_hallway = (from.0, 1);
         let to_hallway = (to.0, 1);
         manhat_dist(from, &from_hallway) +
            manhat_dist(&from_hallway, &to_hallway) +
            manhat_dist(&to_hallway, to)
      };
      let mut cost = dist;
      cost *= who as i64;
      (next, cost)
   }
   fn est_cost_remaining(&self) -> i64 {
      self.occupant_by_node.iter().map(|(coord,who)| {
         let mut dist = match who {
            Amphipod::A => (coord.0 - 3),
            Amphipod::B => (coord.0 - 5),
            Amphipod::C => (coord.0 - 7),
            Amphipod::D => (coord.0 - 9),
         };
         dist = dist.abs();
         if dist >= 1 {
            dist += 1;
         }
         dist *= *who as i64;
         dist
      }).sum()
   }

   fn to_string(&self) -> String {
      let mut map = BLANK_MAP.as_bytes().to_vec();
      map.split_mut(|b| *b == b'\n').enumerate().for_each(|(y,line)| {
         line.iter_mut().enumerate().for_each(|(x,c)| {
            let coord = (x as i64, y as i64);
            if let Some(who) = self.occupant_by_node.get(&coord) {
               *c = match who {
                  Amphipod::A => b'A',
                  Amphipod::B => b'B',
                  Amphipod::C => b'C',
                  Amphipod::D => b'D',
               };
            }
         });
      });
      String::from(str::from_utf8(&map).unwrap())
   }
}

fn rough_parse(input: &str) -> Vec<(Coord,char)> {
   let mut ret = Vec::new();
   input.trim().lines().enumerate().for_each(|(y,line)| {
      line.chars().enumerate().for_each(|(x,c)| {
         match c {
            '.' | '#' | ' ' => (),
            _ => {ret.push(((x as i64,y as i64),c)); },
         }
      });
   });
   ret
}

fn parse(input: &str) -> PositionState {
   let rough = rough_parse(input);
   let mut state = PositionState::new();
   for (coord,c) in rough.iter() {
      let cost = match c {
         'A' => Amphipod::A,
         'B' => Amphipod::B,
         'C' => Amphipod::C,
         'D' => Amphipod::D,
         _ => {
            panic!("{}{}@{:?}", input, c, coord)
         },
      };
      let prev = state.occupant_by_node.insert(*coord, cost);
      assert!(prev == None);
   }
   state
}

const GOAL_STATE: &str = "\
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
";

struct PriorityMap<V,P> {
   priority_by_val: HashMap<V,P>,
   ordered_set: BTreeSet<(P,V)>,
}
impl<V,P> PriorityMap<V,P>
   where V: Clone+Eq+Ord+std::hash::Hash,
         P: Clone+Eq+Ord
{
   fn new() -> PriorityMap<V,P> {
      PriorityMap{
         priority_by_val: HashMap::new(),
         ordered_set: BTreeSet::new(),
      }
   }
   fn len(&self) -> usize {
      self.ordered_set.len()
   }
   fn insert(&mut self, val: V, prio: P) {
      let mut tup: (P,V) = (prio, val);
      if let Some(prev_prio) = self.priority_by_val.get(&tup.1) {
         if prev_prio >= &tup.0 {
            // keep the old one
            return;
         }
         let prio = tup.0;
         tup.0 = prev_prio.clone();
         self.ordered_set.remove(&tup);
         tup.0 = prio;
      }
      self.priority_by_val.insert(tup.1.clone(), tup.0.clone());
      self.ordered_set.insert(tup);
   }
   fn pop_min(&mut self) -> Option<(V,P)> {
      // I'm so mad. `pop_first` is nightly only, really??
      // Is it supposed to be actually impossible to make a fucking pqueue?
      // No, no-replace heaps don't count.
      if let Some(tup) = self.ordered_set.iter().cloned().next() {
         self.ordered_set.remove(&tup);
         let (prio,val) = tup;
         self.priority_by_val.remove(&val).unwrap();
         Some((val, prio))
      } else {
         None
      }
   }
}

fn gather_valid_moves(from: &Coord, state: &PositionState) -> Vec<(PositionState,i64)> {
   let mut try_coords: Vec<Coord> = Vec::new();

   let mover = state.occupant_by_node.get(from).unwrap();
   let destination_x = match mover {
      Amphipod::A => 3,
      Amphipod::B => 5,
      Amphipod::C => 7,
      Amphipod::D => 9,
   };

   // -

   if SPEW {
      println!("gather_valid_moves({:?})\n{}", from, state.to_string());
   }

   const HALLWAY_X_LIST: [i64; 7] = [1,2,4,6,8,10,11];
   let in_hallway = from.1 == 1;
   let in_destination = from.0 == destination_x;

   if SPEW { println!("in_hallway: {}", in_hallway); }
   if SPEW { println!("in_destination: {}", in_destination); }
   if !in_hallway {
      let front_of_room = (from.0, 2 as i64);
      let back_of_room = (from.0, 3 as i64);
      if in_destination {
         if *from == back_of_room {
            if SPEW { println!("no reason to leave"); }
            return vec![]; // No reason to leave
         }
         // So we must be in front_of_room!
         // Anyone in back?
         if let Some(body) = // Shrek'd.
               state.occupant_by_node.get(&back_of_room) {
            if body == mover {
               return vec![]; // We're all friends here
            }
         }
         // Maybe try to go deeper?
         try_coords.push(back_of_room);

         // we have access to hallway
      } else {
         // Can we get to the hallway?
         if *from == back_of_room {
            if state.occupant_by_node.contains_key(&front_of_room) {
               return vec![]; // Trapped!
            }
         }

         // we have access to hallway
      }
      try_coords.extend(HALLWAY_X_LIST.iter().map(|to_x| (*to_x, 1 as i64)));
   }
   // We are in the hallway, or have access to it.

   if !in_destination {
      let front_of_room = (destination_x, 2 as i64);
      let back_of_room = (destination_x, 3 as i64);

      if !state.occupant_by_node.contains_key(&front_of_room) {
         // Cool, unless there's a Stranger in here.
         if let Some(other) = state.occupant_by_node.get(&back_of_room) {
            if *other == *mover {
               try_coords.push(front_of_room);
            } else {
               // Stranger, do not enter!
            }
         } else {
            // Both Empty.
            try_coords.push(front_of_room);
            try_coords.push(back_of_room);
         }
      }
   }

   if SPEW {
      println!("From {:?}:\n{}", from, state.to_string());
   }
   let mut left_end = 0;
   let mut right_end = 12;
   for hallway_person in state.occupant_by_node.iter()
            .map(|(coord,_)| coord)
            .filter(|coord| *coord != from && coord.1 == 1) {
      if hallway_person.0 < from.0 {// left
         left_end = cmp::max(left_end, hallway_person.0);
      } else {// right
         right_end = cmp::min(right_end, hallway_person.0);
      }
   }
   if SPEW {
      println!("  ends: {} {}", left_end, right_end);
      println!("  try_coords: {:?}", try_coords);
   }

   let next_coords: Vec<Coord> = try_coords.into_iter().filter(|to| {
      if to.0 <= left_end || right_end <= to.0 {
         return false;
      }
      if state.occupant_by_node.contains_key(&to) {
         return false;
      }
      true
   }).collect();
   if SPEW {
      println!("  next_coords: {:?}", next_coords);
   }

   let ret: Vec<(PositionState,i64)> = next_coords.into_iter().map(|to| {
      state.mov(from, &to)
   }).collect();

   // -

   ret
}

const SPEW: bool = false;

// "What is the least energy required to organize the amphipods?"
fn solve(input: &str) -> i64 {
   let graph = make_house_graph();
   let initial_pstate = parse(input);
   let goal_state = parse(GOAL_STATE);

   for _ in 0..10 {
      println!("");
   }
   println!("initial_state:\n{}", initial_pstate.to_string());

   let mut cost_by_state: HashMap<PositionState, i64> = HashMap::new();
   let mut fringe: PriorityMap<PositionState, i64> = PriorityMap::new();
   cost_by_state.insert(initial_pstate.clone(), 0);
   fringe.insert(initial_pstate.clone(), 0);
   let mut ii = 1..;
   while let Some((state,_)) = fringe.pop_min() {
      if state == goal_state {
         break;
      }

      let i = ii.next().unwrap();
      if SPEW || i % 1000 <= 1 {
         println!("\n[{}] Popping one from among {}:", i, fringe.len());
         println!("{}", state.to_string());
         //panic!("stop there");
      }
      if i >= 10 {
         //panic!("stop there");
      }

      //println!("");
      //println!("");
      //println!("Popping {} from among {}...", est, fringe.len());
      //println!("{}", state.to_string());
      let cur_cost = *cost_by_state.get(&state).unwrap();
      for (from,_) in state.occupant_by_node.iter() {
         let valid_moves = gather_valid_moves(from, &state);
         for (new_state,additional_cost) in valid_moves {
            let new_cost = cur_cost + additional_cost;

            if SPEW {
               println!("   new_state:\n{}", new_state.to_string());
            }

            let e = cost_by_state.entry(new_state.clone())
                  .or_insert(new_cost+1); // Always "find" a new min!

            if SPEW {
               println!("  new_cost {}, current best: {}", new_cost, *e);
            }
            if new_cost < *e {
               *e = new_cost;
               let cost_remaining = new_state.est_cost_remaining();
               //println!("      estimated cost: {}", cost_remaining);
               let est_total_cost = new_cost + cost_remaining;
               fringe.insert(new_state, est_total_cost);
            }
         }
      }
   }

   *cost_by_state.get(&goal_state).unwrap()
}

// -

//#[test]
fn test_example() {
   let input = "\
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
";
   assert_eq!(solve(&input), 0);
   assert_eq!(parse(&input).est_cost_remaining(), 0);
   // -

   let input = "\
#############
#.A.........#
###.#B#C#D###
  #A#B#C#D#
  #########
";
   assert_eq!(parse(&input).est_cost_remaining(), 2);
   println!("est_cost_remaining ok");
   assert_eq!(solve(&input), 2);

   let input = "\
#############
#...B.......#
###A#.#C#D###
  #A#B#C#D#
  #########
";
   assert_eq!(solve(&input), 20);
   assert_eq!(parse(&input).est_cost_remaining(), 20);

   let input = "\
#############
#.....C.....#
###A#B#.#D###
  #A#B#C#D#
  #########
";
   assert_eq!(solve(&input), 200);
   assert_eq!(parse(&input).est_cost_remaining(), 200);

   let input = "\
#############
#.......D...#
###A#B#C#.###
  #A#B#C#D#
  #########
";
   assert_eq!(solve(&input), 2000);
   assert_eq!(parse(&input).est_cost_remaining(), 2000);

   // -

   let input = "\
#############
#...........#
###D#B#C#A###
  #A#B#C#D#
  #########
";
   //assert_eq!(solve(&input), 8010);

   let input = "\
#############
#.D.........#
###.#B#C#D###
  #A#B#C#A#
  #########
";
   assert_eq!(solve(&input), 13009);
   //panic!("good!");
   // -
   let input = "\
#############
#...........#
###D#B#C#D###
  #A#B#C#A#
  #########
";
   assert_eq!(solve(&input), 13011);
   //panic!("good!");
   // -

   let input = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
   assert_eq!(solve(&input), 12521);
   //panic!("good!!");
}

fn main() {
   test_example();
   println!("Examples ran clean!");
   thread::sleep(time::Duration::from_millis(1000));

   let path = Path::new("day23-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve_p2(input) -> {}", solve(&input));
}
