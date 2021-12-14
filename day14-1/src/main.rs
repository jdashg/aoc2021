use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

// -

fn parse_inputs(input: &str) -> (Vec<char>, HashMap<(char,char),char>) {
   let (initial_s, rules_lines) = input.trim().split_once("\n\n").unwrap();
   let initial: Vec<char> = initial_s.chars().collect();
   let rules: HashMap<(char,char),char> = rules_lines.split("\n")
      .map(|line| {
         let (pair_s,res) = line.split_once(" -> ").unwrap();
         ((pair_s.chars().nth(0).unwrap(),
           pair_s.chars().nth(1).unwrap()),
          res.chars().nth(0).unwrap())
      }).collect();
   (initial, rules)
}

// "How many dots are visible after completing just the first fold
//  instruction on your transparent paper?"
fn step(state: Vec<char>, rules: &HashMap<(char,char),char>) -> Vec<char> {
   let a_iter = state.iter();
   let b_iter = state.iter().skip(1);
   let mut new_state = Vec::with_capacity(2*state.len()+1);
   a_iter.zip(b_iter).for_each(|(a,b)| {
      new_state.push(*a);
      new_state.push(rules[&(a.clone(),b.clone())].clone());
   });
   new_state.push(*state.last().unwrap());
   new_state
}
// "What do you get if you take the quantity of the most common element
//  and subtract the quantity of the least common element [after 10
//  steps]?"
fn solve(input: &str) -> usize {
   let (mut state, rules) = parse_inputs(input);
   for _ in 0..10 {
      state = step(state, &rules);
   }
   let mut freq_by_char = HashMap::new();
   for c in state.into_iter() {
      let e = freq_by_char.entry(c).or_insert(0);
      *e += 1;
   }
   let (_, max) = freq_by_char.iter().max_by_key(|(_,v)| *v).unwrap();
   let (_, min) = freq_by_char.iter().min_by_key(|(_,v)| *v).unwrap();
   max - min
}
// -

#[test]
fn test_example() {
   let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
   let (mut state, rules) = parse_inputs(&input);
   state = step(state, &rules);
   assert_eq!(state.iter().collect::<String>().as_str(), "NCNBCHB");
   state = step(state, &rules);
   assert_eq!(state.iter().collect::<String>().as_str(), "NBCCNBBBCBHCB");
   state = step(state, &rules);
   assert_eq!(state.iter().collect::<String>().as_str(), "NBBBCNCCNBBNBNBBCHBHHBCHB");
   state = step(state, &rules);
   assert_eq!(state.iter().collect::<String>().as_str(), "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");

   assert_eq!(solve(&input), 1588);
}

fn main() {
   let path = Path::new("day14-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
