use std::fs::File;
use std::io::Read;
use std::path::Path;

fn parse_inputs(input: &str) -> Vec<&str> {
   input.trim().split("\n").collect()
}

// "What do you get if you multiply together the sizes of the three
//  largest basins?"
fn solve(input: &str) -> usize {
   let lines = parse_inputs(input);
   let mut scores: Vec<_> = lines.iter()
      .map(|line| {
         let mut stack = Vec::new();
         for c in line.chars() {
            match c {
               '(' => stack.push(')'),
               '[' => stack.push(']'),
               '{' => stack.push('}'),
               '<' => stack.push('>'),
               _ => {
                  if c != stack.pop().unwrap_or('x') {
                     return 0;
                  }
               },
            };
         }
         stack.reverse();
         stack.into_iter().map(points_by_char)
            .fold(0, |prev,next| 5*prev + next)
      }).filter(|x| *x != 0).collect();
   scores.sort();
   scores[scores.len()/2]
}

fn points_by_char(c: char) -> usize {
   match c {
      ')' => 1,
      ']' => 2,
      '}' => 3,
      '>' => 4,
      _ => panic!("{}", c),
   }
}

#[test]
fn test_example() {
   let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
   assert_eq!(solve(&input), 288957);
}

fn main() {
   let path = Path::new("day10-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
