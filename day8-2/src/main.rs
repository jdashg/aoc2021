use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

struct Case {
   test_pats: Vec<HashSet<char>>,
   out_pats: Vec<HashSet<char>>,
}

impl Case {
   fn solve(&self) -> isize {
      // We have ten unique input patterns
      // 1, 7, 4, and 8 have unique num segments.
      let mut pats_by_len = HashMap::< usize, Vec<&HashSet<char>> >::new();
      for p in self.test_pats.iter() {
         let e = pats_by_len.entry(p.len()).or_insert(Vec::new());
         e.push(&p);
      }
      // 2: 1
      // 3: 7
      // 4: 4
      // 5: 2,5,3
      // 6: 0,6,9
      // 7: 8

      let mut pat_by_val : HashMap<isize,HashSet<char>> = [
         (1, pats_by_len[&2][0].clone()),
         (7, pats_by_len[&3][0].clone()),
         (4, pats_by_len[&4][0].clone()),
         (8, pats_by_len[&7][0].clone()),
      ].iter().cloned().collect();
      assert_eq!(pat_by_val.len(), 4);

      for pat in pats_by_len[&5].iter() {
         let val = match (*pat & &pat_by_val[&4]).len() {
            2 => 2,
            3 => { // 3 or 5
               match (*pat & &pat_by_val[&1]).len() {
                  1 => 5,
                  2 => 3,
                  i => panic!("{}", i),
               }
            },
            i => panic!("{}", i),
         };
         pat_by_val.insert(val, (*pat).clone());
      }
      assert_eq!(pat_by_val.len(), 7);
      pat_by_val.insert(9, (&pat_by_val[&5] | &pat_by_val[&1]).clone());
      assert_eq!(pat_by_val.len(), 8);

      for pat in pats_by_len[&6].iter() {
         let val = match (*pat & &pat_by_val[&5]).len() {
            5 => {
               if *pat == &pat_by_val[&9] {
                  continue;
               } else {
                  6
               }
            },
            4 => 0,
            i => panic!("{}", i),
         };
         pat_by_val.insert(val, (*pat).clone());
      }
      assert_eq!(pat_by_val.len(), 10);

      fn set_to_str(set: &HashSet<char>) -> String {
         let mut chars: Vec<char> = set.into_iter().cloned().collect();
         chars.sort();
         chars.into_iter().collect()
      }

      let val_by_pat: HashMap<_,_> = pat_by_val.into_iter()
         .map(|(val,pat)| (set_to_str(&pat), val)).collect();

      self.out_pats.iter()
         .map(|p| {
            let s = set_to_str(p);
            val_by_pat[&s]
         }).fold(0 as isize, |prev,next| 10*prev + next)
   }
}

fn parse_inputs(input: &str) -> Vec<Case> {
   fn to_pat_sets(s: &str) -> Vec<HashSet<char>> {
      s.split_whitespace()
       .map(|s| s.chars().collect())
       .collect()
   }
   input
      .trim()
      .split("\n")
      .map(|x| {
         let (test_pats, out_pats) = x.split_once(" | ").unwrap();
         Case{test_pats: to_pat_sets(test_pats),
              out_pats: to_pat_sets(out_pats)}
      }).collect()
}

// "For each entry, determine all of the wire/segment connections and
//  decode the four-digit output values. What do you get if you add up
//  all of the output values?"
fn solve(input: &str) -> isize {
   let cases = parse_inputs(input);
   cases.iter().map(|c| c.solve()).sum()
}


#[test]
fn test_example() {
   let small_input = "\
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf
".replace("|\n", "| ");
   assert_eq!(solve(&small_input), 5353);

   let big_input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce
".replace("|\n", "| ");
   assert_eq!(solve(&big_input), 61229);
}

fn main() {
   let path = Path::new("day8-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
