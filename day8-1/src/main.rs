use std::fs::File;
use std::io::Read;
use std::path::Path;

struct Case<'a> {
   test_pats: Vec<&'a str>,
   out_pats: Vec<&'a str>,
}

fn parse_inputs(input: &str) -> Vec<Case> {
   input
      .trim()
      .split("\n")
      .map(|x| {
         let (test_pats, out_pats) = x.split_once(" | ").unwrap();
         Case{test_pats: test_pats.split_whitespace().collect(),
              out_pats: out_pats.split_whitespace().collect()}
      }).collect()
}

// "In the output values, how many times do digits 1, 4, 7, or 8 appear?"
fn solve(input: &str) -> isize {
   let cases = parse_inputs(input);
   const UNIQUE_SEGMENT_COUNTS: [usize;4] = [2, 3, 4, 7]; // 1, 7, 4, 8 respectively
   cases.iter().map(|c| {
      c.out_pats.iter().map(|p| {
         UNIQUE_SEGMENT_COUNTS.contains(&p.len()) as isize
      }).sum::<isize>()
   }).sum()
}


#[test]
fn test_example() {
   let small_input = "\
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf
".replace("|\n", "| ");
   assert_eq!(solve(&small_input), 0);

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
   assert_eq!(solve(&big_input), 26);
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
