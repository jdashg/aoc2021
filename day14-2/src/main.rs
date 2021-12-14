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
fn step(count_by_pair: HashMap<(char,char),usize>,
      rules: &HashMap<(char,char),char>) -> HashMap<(char,char),usize> {
   let mut ret: HashMap<(char,char),usize> = HashMap::new();
   for (pair,count) in count_by_pair {
      let to_insert = rules[&pair];
      [(pair.0, to_insert),
       (to_insert, pair.1)].iter().for_each(|pair| {
          let e = ret.entry(*pair).or_insert(0);
          *e += count;
       });
   }
   ret
}
// "What do you get if you take the quantity of the most common element
//  and subtract the quantity of the least common element [after 10
//  steps]?"
fn solve(input: &str) -> usize {
   let (initial, rules) = parse_inputs(input);
   let mut state: HashMap<(char,char),usize> = HashMap::new();
   initial.iter().zip(initial.iter().skip(1)).for_each(|pair| {
      let e = state.entry((*pair.0, *pair.1)).or_insert(0);
      *e += 1;
   });

   for _ in 0..40 {
      //println!("{}", i);
      state = step(state, &rules);
   }
   let mut freq_by_char = HashMap::new();
   for (pair,count) in state.into_iter() {
      //println!("({},{}): {}", pair.0, pair.1, count);
      [pair.0, pair.1].iter().for_each(|c| {
         let e = freq_by_char.entry(c.clone()).or_insert(0);
         *e += count;
      });
   }
   *freq_by_char.get_mut(initial.first().unwrap()).unwrap() += 1;
   *freq_by_char.get_mut(initial.last().unwrap()).unwrap() += 1;
   for (_,count) in freq_by_char.iter_mut() {
      assert_eq!(*count % 2, 0);
      *count /= 2;
   }
   let (_, max) = freq_by_char.iter().max_by_key(|(_,v)| *v).unwrap();
   let (_, min) = freq_by_char.iter().min_by_key(|(_,v)| *v).unwrap();
   max - min
}
// -

//#[test]
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
   //let (mut state, rules) = parse_inputs(&input);
   //state = step(state, &rules);
   //assert_eq!(state.iter().collect::<String>().as_str(), "NCNBCHB");
   //state = step(state, &rules);
   //assert_eq!(state.iter().collect::<String>().as_str(), "NBCCNBBBCBHCB");
   //state = step(state, &rules);
   //assert_eq!(state.iter().collect::<String>().as_str(), "NBBBCNCCNBBNBNBBCHBHHBCHB");
   //state = step(state, &rules);
   //assert_eq!(state.iter().collect::<String>().as_str(), "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");

   assert_eq!(solve(&input), 2188189693529);
}

fn main() {
   test_example();

   let path = Path::new("day14-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
