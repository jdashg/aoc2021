use std::fs::File;
use std::io::Read;
use std::path::Path;

struct Player {
   pos: i64,
   score: i64,
}
impl Player {
   fn new(pos: i64) -> Player {
      Player {
         pos: pos,
         score: 0,
      }
   }
}

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

fn parse(input: &str) -> Vec<Player> {
   input.trim().lines().map(|mut line| {
      // "Player 1 starting position: 4"
      line = line.strip_prefix("Player ").unwrap();
      let (_, start_pos) = line.split_once(" starting position: ").unwrap();
      let start_pos: i64 = start_pos.parse().unwrap();
      Player::new(start_pos)
   }).collect()
}

fn solve_p1(input: &str) -> i64 {
   let mut players = parse(input);
   let mut dice = Dice::new();

   let winner = || -> usize {
      loop {
         for (i, p) in players.iter_mut().enumerate() {
            let roll = dice.roll() + dice.roll() + dice.roll();
            p.pos += roll;
            while p.pos >= 11 {
               p.pos -= 10;
            }
            p.score += p.pos;
            if p.score >= 1000 {
               return i;
            }
         }
      }
   }();
   players.remove(winner);
   let loser = &players[0];
   println!("{},{}", loser.score, dice.rolls);
   loser.score * dice.rolls
}

// -

//#[test]
fn test_example() {
   let input = "\
Player 1 starting position: 4
Player 2 starting position: 8
";
   assert_eq!(solve_p1(&input), 739785);
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

   println!("solve_p1(input) -> {}", solve_p1(&input));
}
