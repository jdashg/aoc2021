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
  #########
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
         let destination_x = match who {
            Amphipod::A => 3,
            Amphipod::B => 5,
            Amphipod::C => 7,
            Amphipod::D => 9,
         };
         let mut dist = (coord.0 - destination_x).abs();
         dist = dist.abs();
         if dist >= 1 {
            dist += 1;
         }
         let mut cost = dist * *who as i64;
         if coord.1 > 1 && dist != 0 {
            // Wrong room
            let room_is_for = match coord.0 {
               3 => Amphipod::A,
               5 => Amphipod::B,
               7 => Amphipod::C,
               9 => Amphipod::D,
               _ => panic!("impossible coord: {:?}", coord),
            };
            cost += (room_is_for as i64);
         }
         cost
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

fn parse(input: &str) -> (PositionState, i64) {
   let rough = rough_parse(input);
   let mut state = PositionState::new();
   let mut last_y = 0;
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
      last_y = cmp::max(last_y, coord.1);
   }
   (state, last_y)
}

/*
const GOAL_STATE: &str = "\
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
";*/
fn make_goal_state(last_y: i64) -> PositionState {
   assert!(last_y == 3 || last_y == 5);
   let mut state = PositionState::new();
   for to_y in 2..=last_y {
      state.occupant_by_node.insert((3 as i64, to_y as i64), Amphipod::A);
      state.occupant_by_node.insert((5 as i64, to_y as i64), Amphipod::B);
      state.occupant_by_node.insert((7 as i64, to_y as i64), Amphipod::C);
      state.occupant_by_node.insert((9 as i64, to_y as i64), Amphipod::D);
   }
   state
}

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

fn gather_valid_moves(from: &Coord, state: &PositionState, last_y: i64) -> Vec<(PositionState,i64)> {
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

   // Ok, here are our possible states:
   // If we're in the destination room already:
   //    If there is a Stranger below us,
   //       only useful move is into the hallway (somewhere)
   //    else
   //       assert that we're tightly packed with friends
   // else if we're in the hallway
   //    we must move to destination room (as deep as possible)
   //    but only if there are no strangers
   // else we must be in the wrong room
   //    we must move to hallway, or to destination room (as deep as possible)
   //    (we might keep this simple and just move into hallway,
   //     since we'll then move to destination on the next step anyway)

   let in_hallway = from.1 == 1;
   let in_destination = from.0 == destination_x;
   if SPEW { println!("in_hallway: {}", in_hallway); }
   if SPEW { println!("in_destination: {}", in_destination); }

   let is_stranger_in_destination = |at_or_below_y| {
      for to_y in at_or_below_y..=last_y {
         let to: Coord = (destination_x, to_y);
         if let Some(body) = state.occupant_by_node.get(&to) {
            if body != mover {
               return true;
            }
         }
      }
      false
   };

   let mut move_to_hallway = false;
   let mut move_to_destination = false;
   if in_destination {
      if is_stranger_in_destination(from.1) {
         move_to_hallway = true; // Stranger below us!
      } else {
         for to_y in (from.1)..=last_y {
            let to: Coord = (destination_x, to_y);
            let at = state.occupant_by_node.get(&to);
            assert_eq!(at, Some(mover));
         }
         if SPEW { println!("room complete"); }
         return vec![];
      }
   } else if in_hallway {
      if is_stranger_in_destination(2) {
         // Can't move there!
         if SPEW { println!("is_stranger_in_destination"); }
         return vec![];
      } else {
         move_to_destination = true;
      }
   } else {
      // Can we leave?
      let mut front_to_us = (2..=(from.1-1));
      for to_y in front_to_us.rev() {
         let to: Coord = (from.0, to_y);
         if state.occupant_by_node.contains_key(&to) {
            if SPEW { println!("trapped!"); }
            return vec![];
         }
      }

      move_to_hallway = true;
   }
   assert_ne!(move_to_hallway, move_to_destination); // Should only have one.

   // -

   if SPEW { println!("move_to_hallway: {}", move_to_hallway); }
   if SPEW { println!("move_to_destination: {}", move_to_destination); }

   if move_to_hallway {
      try_coords = HALLWAY_X_LIST.iter().map(|to_x| (*to_x, 1 as i64)).collect();
   }
   if move_to_destination {
      let mut avail: Option<Coord> = None;
      for to_y in 2..=last_y {
         let to: Coord = (destination_x, to_y);
         if state.occupant_by_node.contains_key(&to) {
            break;
         } else {
            avail = Some(to);
         }
      }
      if let Some(to) = avail {
         try_coords.push(to);
      } else {
         if SPEW { println!("no dest slots avail"); }
      }
   }

   // -
   // Constrain by blocking out X vals that are blocked in the hallway.

   const HALLWAY_X_LIST: [i64; 7] = [1,2,4,6,8,10,11];

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

   let (initial_pstate, last_y) = parse(input);
   let goal_state = make_goal_state(last_y);

   for _ in 0..10 {
      println!("");
   }
   println!("input:\n{}", input);
   println!("initial_state:\n{}", initial_pstate.to_string());
   println!("goal_state:\n{}", goal_state.to_string());

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
      if SPEW || i % 1000 <= 0 {
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
         let valid_moves = gather_valid_moves(from, &state, last_y);
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
   assert_eq!(parse(&input).0.est_cost_remaining(), 0);
   // -

   let input = "\
#############
#.A.........#
###.#B#C#D###
  #A#B#C#D#
  #########
";
   assert_eq!(parse(&input).0.est_cost_remaining(), 2);
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
   assert_eq!(parse(&input).0.est_cost_remaining(), 20);

   let input = "\
#############
#.....C.....#
###A#B#.#D###
  #A#B#C#D#
  #########
";
   assert_eq!(solve(&input), 200);
   assert_eq!(parse(&input).0.est_cost_remaining(), 200);

   let input = "\
#############
#.......D...#
###A#B#C#.###
  #A#B#C#D#
  #########
";
   assert_eq!(solve(&input), 2000);
   assert_eq!(parse(&input).0.est_cost_remaining(), 2000);

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
   //assert_eq!(solve(&input), 12521);
   //panic!("good!");

   let input = "\
#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########

#############
#..........D#
###B#C#B#.###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########

#############
#A.........D#
###B#C#B#.###
  #D#C#B#.#
  #D#B#A#C#
  #A#D#C#A#
  #########

#############
#A........BD#
###B#C#.#.###
  #D#C#B#.#
  #D#B#A#C#
  #A#D#C#A#
  #########

#############
#A......B.BD#
###B#C#.#.###
  #D#C#.#.#
  #D#B#A#C#
  #A#D#C#A#
  #########

#############
#AA.....B.BD#
###B#C#.#.###
  #D#C#.#.#
  #D#B#.#C#
  #A#D#C#A#
  #########

#############
#AA.....B.BD#
###B#.#.#.###
  #D#C#.#.#
  #D#B#C#C#
  #A#D#C#A#
  #########

#############
#AA.....B.BD#
###B#.#.#.###
  #D#.#C#.#
  #D#B#C#C#
  #A#D#C#A#
  #########

#############
#AA...B.B.BD#
###B#.#.#.###
  #D#.#C#.#
  #D#.#C#C#
  #A#D#C#A#
  #########

#############
#AA.D.B.B.BD#
###B#.#.#.###
  #D#.#C#.#
  #D#.#C#C#
  #A#.#C#A#
  #########

#############
#AA.D...B.BD#
###B#.#.#.###
  #D#.#C#.#
  #D#.#C#C#
  #A#B#C#A#
  #########

#############
#AA.D.....BD#
###B#.#.#.###
  #D#.#C#.#
  #D#B#C#C#
  #A#B#C#A#
  #########

#############
#AA.D......D#
###B#.#.#.###
  #D#B#C#.#
  #D#B#C#C#
  #A#B#C#A#
  #########

#############
#AA.D......D#
###B#.#C#.###
  #D#B#C#.#
  #D#B#C#.#
  #A#B#C#A#
  #########

#############
#AA.D.....AD#
###B#.#C#.###
  #D#B#C#.#
  #D#B#C#.#
  #A#B#C#.#
  #########

#############
#AA.......AD#
###B#.#C#.###
  #D#B#C#.#
  #D#B#C#.#
  #A#B#C#D#
  #########

#############
#AA.......AD#
###.#B#C#.###
  #D#B#C#.#
  #D#B#C#.#
  #A#B#C#D#
  #########

#############
#AA.......AD#
###.#B#C#.###
  #.#B#C#.#
  #D#B#C#D#
  #A#B#C#D#
  #########

#############
#AA.D.....AD#
###.#B#C#.###
  #.#B#C#.#
  #.#B#C#D#
  #A#B#C#D#
  #########

#############
#A..D.....AD#
###.#B#C#.###
  #.#B#C#.#
  #A#B#C#D#
  #A#B#C#D#
  #########

#############
#...D.....AD#
###.#B#C#.###
  #A#B#C#.#
  #A#B#C#D#
  #A#B#C#D#
  #########

#############
#.........AD#
###.#B#C#.###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########

#############
#..........D#
###A#B#C#.###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########

#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########
";
/*
   let parts: Vec<&str> = input.trim().split("\n\n").collect();
   parts.iter().rev().enumerate().for_each(|(i,part)| {
      println!("{} from completion:\n{}", i, part);
      solve(part);
   });
   panic!("great!");
*/



   let input = "\
#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########
";
   assert_eq!(solve(&input), 44169);
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

   input = "\
#############
#...........#
###C#A#D#D###
  #D#C#B#A#
  #D#B#A#C#
  #B#A#B#C#
  #########
".to_string();

   println!("solve_p2(input) -> {}", solve(&input));
}
