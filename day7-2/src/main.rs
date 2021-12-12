use std::fs::File;
use std::io::Read;
use std::path::Path;

fn parse_inputs(input: &str) -> Vec<isize> {
   input.trim().split(",")
                   .map(|x| x.parse().unwrap())
                   .collect()
}

fn move_cost(a: isize, b: isize) -> isize {
   let dist = (a - b).abs();
   // These are triangle numbers
   dist * (dist+1) / 2
}

#[test]
fn test_move_cost() {
   assert_eq!(move_cost(0,0), 0);
   assert_eq!(move_cost(0,1), 1);
   assert_eq!(move_cost(0,2), 3);
   assert_eq!(move_cost(0,3), 6);
}

fn cost_align_at(inputs: &Vec<isize>, x_target: isize) -> isize {
   inputs.iter().map(|x| move_cost(*x, x_target)).sum()
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
   assert_eq!(cost_align_at(&inputs, 2), 206);

   assert_eq!(solve(example_input), 168);
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
