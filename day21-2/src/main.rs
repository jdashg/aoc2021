use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

#[derive(PartialEq,Eq,Hash,Clone)]
struct Player {
   pos: i64,
   score: i64,
}
impl Player {
   fn new(pos: &i64) -> Player {
      Player {
         pos: *pos,
         score: 0,
      }
   }
   fn advance(&mut self, n: &i64) {
      self.pos += *n;
      while self.pos >= 11 {
         self.pos -= 10;
      }
      self.score += self.pos;
   }
}
type Game = Vec<Player>;

struct Dice {
   next_val: i64,
   rolls: i64,
}
impl Dice {
   fn new() -> Dice {
      Dice{next_val: 1,
         rolls: 0}
   }
   fn roll(&mut self) -> i64 {
      let ret = self.next_val;
      self.next_val += 1;
      while self.next_val >= 101 {
         self.next_val -= 100;
      }
      self.rolls += 1;
      ret
   }
}

fn parse(input: &str) -> Game {
   input.trim().lines().map(|mut line| {
      // "Player 1 starting position: 4"
      line = line.strip_prefix("Player ").unwrap();
      let (_, start_pos) = line.split_once(" starting position: ").unwrap();
      let start_pos: i64 = start_pos.parse().unwrap();
      Player::new(&start_pos)
   }).collect()
}

type UniverseCountByPlayerState = HashMap<Player,i64>;



fn solve_p2(input: &str) -> i64 {
   let initial_game_state = parse(input);

   let roll_universes_by_val = {
      let mut all_possibilities: Vec<i64> = vec![0i64];
      for _ in 0..3 { // three rolls
         let mut next: Vec<i64> = Vec::new();
         for prev_rolls in all_possibilities.iter() {
            for maybe_roll in 1..=3 { // 1 through 3
               next.push(prev_rolls + maybe_roll);
            }
         }
         all_possibilities = next;
      }
      assert_eq!(all_possibilities.len(), 27);
      let mut count_by_val = HashMap::<i64,i64>::new();
      all_possibilities.iter().for_each(|val| {
         let e = count_by_val.entry(*val).or_insert(0);
         *e += 1;
      });
      count_by_val
   };

   let mut universe_count_by_game_state = HashMap::<Game,i64>::new();
   universe_count_by_game_state.insert(initial_game_state, 1);

   let wins = {
      let mut wins: Vec<i64> = vec![0i64; 2];
      while !universe_count_by_game_state.is_empty() {
         for cur_player in 0..2 {
            let mut next_universe_count_by_game_state = HashMap::new();

            for (gstate, game_universes) in universe_count_by_game_state.iter() {
               for (roll_val, roll_universes) in roll_universes_by_val.iter() {
                  let mut new_gstate = gstate.clone();
                  let new_pstate = &mut new_gstate[cur_player];
                  new_pstate.advance(roll_val);
                  let new_universes = game_universes * roll_universes;
                  if new_pstate.score >= 21 {
                     wins[cur_player] += new_universes;
                  } else {
                     let e = next_universe_count_by_game_state.entry(new_gstate).or_insert(0);
                     *e += new_universes;
                  }
               }
            }
            universe_count_by_game_state = next_universe_count_by_game_state;
         }
      }
      wins
   };
   println!("{:?}", wins);
   *wins.iter().max().unwrap()
}

// -

//#[test]
fn test_example() {
   let input = "\
Player 1 starting position: 4
Player 2 starting position: 8
";
   assert_eq!(solve_p2(&input), 444356092776315);
}

fn main() {
   test_example();
   println!("Examples ran clean!");

   let path = Path::new("day21-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve_p2(input) -> {}", solve_p2(&input));
}
