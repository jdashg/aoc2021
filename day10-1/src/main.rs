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
   lines.iter()
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
                     return points_by_illegal_char(c);
                  }
               },
            };
         }
         0
      }).sum()
}

fn points_by_illegal_char(c: char) -> usize {
   match c {
      ')' => 3,
      ']' => 57,
      '}' => 1197,
      '>' => 25137,
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
   assert_eq!(solve(&input), 26397);
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
