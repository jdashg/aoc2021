use std::fs::File;
use std::io::Read;
use std::path::Path;

fn parse_inputs(input: &str) -> Vec<isize> {
   input.trim().split(",")
                   .map(|x| x.parse().unwrap())
                   .collect()
}

fn cost_align_at(inputs: &Vec<isize>, x_target: isize) -> isize {
   inputs.iter().map(|x| (x - x_target).abs()).sum()
}

fn solve(input: &str) -> isize {
   let inputs = parse_inputs(input);

   fn min_cost(inputs: &Vec<isize>, begin: isize, end: isize) -> isize {
      if end - begin <= 4 {
         (begin..end).map(|x| cost_align_at(inputs, x)).min().unwrap()
      } else {
         let x_first_right = (begin + end) / 2; // (0+4)/2 = 2
         let x_last_left = x_first_right - 1;

         let cost_last_left = cost_align_at(&inputs, x_last_left);
         let cost_first_right = cost_align_at(&inputs, x_first_right);

         if cost_last_left < cost_first_right {
            min_cost(&inputs, begin, x_first_right)
         } else {
            min_cost(&inputs, x_first_right, end)
         }
      }
   }
   min_cost(&inputs, 0, inputs.len() as isize)
}

#[test]
fn test_example() {
   let example_input = "\
16,1,2,0,4,2,7,1,2,14
";
   let inputs = parse_inputs(example_input);
   assert_eq!(cost_align_at(&inputs, 1), 41);
   assert_eq!(cost_align_at(&inputs, 3), 39);
   assert_eq!(cost_align_at(&inputs, 10), 71);

   assert_eq!(solve(example_input), 37);
}

fn main() {
   let path = Path::new("day7-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
