use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::cmp;
use std::str;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

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
  #########";

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
         let from_hallway = (from.x, 1);
         let to_hallway = (to.x, 1);
         manhat_dist(from, from_hallway) +
            manhat_dist(from_hallway, to_hallway) +
            manhat_dist(to_hallway, to)
      }
      let mut cost = dist;
      cost *= *who as i64;
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

fn gather_valid_moves(from: &Coord, state: &PositionState,
         graph: &Graph) -> Vec<(Coord,i64)> {
   let mut ret: Vec<(PositionState,i64)> = Vec::new();

   let mover = state.occupant_by_node.get(from).unwrap();
   let destination_x = match mover {
      Amphipod::A => 3,
      Amphipod::B => 5,
      Amphipod::C => 7,
      Amphipod::D => 9,
   };

   // -
   // Always try to get to destination room
   {
      let mut destination_contains_others = false;
      for to_y in 2..=3 {
         let to = (destination_x,to_y);
         if let Some(body) = // Shrek'd.
               state.occupant_by_node.get(&to) {
            if body != mover {
               destination_contains_others = true;
               break;
            }
         }
      }
      if !destination_contains_others {
         for to_y in 2..=3 {
            let to = (destination_x,to_y);
            if !state.occupant_by_node.contains_key(&to) {
               ret.push(state.mov(from, to))
            }
         }
      }
   }
   // Can we move in the same room even if it's the wrong one?
   if from.1 > 1 && // not in the hallway
         from.0 != destination_x { // but the wrong room
      // Try to leave
      let to = (from.0,2);
      if !state.occupant_by_node.contains_key(&to) {
         ret.push(state.mov(from, to))
      }


      // We can move through people in the hallway.
      // "#12.3.4.5.67#"
      //  012345678901
      let hallway_x_list = [1,2,4,6,8,10,11];
      for to_x in hallway_x_list.iter() {
         let to = (hallway_x,1);
         if !state.occupant_by_node.contains_key(&to) {
            ret.push(state.mov(from, to))
         }
      }
   }
}

/*
   for (to, move_dist) in graph.neighbor_costs_by_from.get(from)
            .unwrap().iter() {
      if from.1 == to.1 {
         continue; // No milling in the hallway
      }
      if state.occupant_by_node.contains_key(to) {
         continue; // Someone already there
      }
      if from.1 == 1 { // from hallway into room
         let destination_room = match mover {
            Amphipod::A => 3,
            Amphipod::B => 5,
            Amphipod::C => 7,
            Amphipod::D => 9,
         };
         if to.0 != destination_room {
            continue; // That's not my room.
         }
         let mut contains_others = false;
         for check_y in 2..=3 {
            if let Some(body) = // Shrek'd.
                  state.occupant_by_node.get(to) {
               if body != mover {
                  contains_others = true;
                  break;
               }
            }
         }
         if contains_others {
            continue;  // Gotta wait until The Others clear out.
         }
      }
      let our_cost = move_dist * (*mover as i64);
      ret.push((*to,our_cost));
   }
   ret
}
*/
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
   let mut i = 10;
   while let Some((state,est)) = fringe.pop_min() {
      if state == goal_state {
         break;
      }

      if i == 0 {
         //panic!("stop there");
      }
      i -= 1;

      println!("");
      println!("");
      println!("Popping {} from among {}...", est, fringe.len());
      println!("{}", state.to_string());
      let cur_cost = *cost_by_state.get(&state).unwrap();
      for (from,_) in state.occupant_by_node.iter() {
         let valid_moves = gather_valid_moves(from, &state, &graph);
         for (to,move_cost) in valid_moves {
            let new_state = state.mov(from, &to);
            let new_cost = cur_cost + move_cost;

            let e = cost_by_state.entry(new_state.clone())
                  .or_insert(new_cost+1); // Always "find" a new min!
            if new_cost < *e {
               *e = new_cost;
               let cost_remaining = new_state.est_cost_remaining();
               println!("      estimated cost: {}", cost_remaining);
               let est_total_cost = new_cost + cost_remaining;
               fringe.insert(new_state, est_total_cost);
            }
         }
         /*
         let neighbor_costs = graph.neighbor_costs_by_from.get(from).unwrap();
         for (to,dist) in neighbor_costs.iter() {
            if let Some((next_state, move_cost))
                        = state.move_n(from, to, *dist) {
               let next_cost = cur_cost + move_cost;
               //println!("   for {}+{} cost:\n{}", cur_cost, move_cost,
               //  next_state.to_string());
               let e = cost_by_state.entry(next_state.clone())
                     .or_insert(next_cost+1); // Always "find" a new min!
               if next_cost < *e {
                  *e = next_cost;
                  let cost_remaining = next_state.est_cost_remaining();
                  //println!("      estimated cost: {}", cost_remaining);
                  let est_total_cost = next_cost + cost_remaining;
                  fringe.insert(next_state, est_total_cost);
               }
            }
         }
         */
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
   println!("est_cost_remaining ok");
   assert_eq!(solve(&input), 2);

   panic!("ok");
   // -

   let input = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
   assert_eq!(solve(&input), 12521);
}

fn main() {
   test_example();
   println!("Examples ran clean!");

   let path = Path::new("day23-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve_p2(input) -> {}", solve(&input));
}
