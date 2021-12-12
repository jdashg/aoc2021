use std::fs::File;
use std::io::Read;
use std::path::Path;
use memoise::memoise;

fn solve(input: &str, days: usize) -> usize {
   let mut inputs = input.split("\n")
                     .filter(|x| !x.is_empty());
   let fish_list: Vec<usize> = inputs.next().unwrap()
                               .split(',')
                               .map(|x| x.parse::<usize>().unwrap())
                               .collect();
   fish_list.into_iter().map(|f| fish_after_n(f, days)).sum()
}

#[memoise(timer <= 8, days <= 256)]
fn fish_after_n(timer: usize, days: usize) -> usize {
   if timer >= days { return 1; }
   let days_left_at_first_spawn = days - timer - 1;
   return fish_after_n(6, days_left_at_first_spawn) +
          fish_after_n(8, days_left_at_first_spawn);
}

#[test]
fn test_fish_after_n() {
   assert_eq!(fish_after_n(1, 1), 1);
   assert_eq!(fish_after_n(0, 1), 2);
   assert_eq!(fish_after_n(0, 2), 2);
   assert_eq!(fish_after_n(0, 3), 2);
   assert_eq!(fish_after_n(0, 4), 2);
   assert_eq!(fish_after_n(0, 5), 2);
   assert_eq!(fish_after_n(0, 6), 2);
   assert_eq!(fish_after_n(0, 7), 2);
   assert_eq!(fish_after_n(0, 8), 3);
}
#[test]
fn example() {
   let example_input = "\
3,4,3,1,2
";
   assert_eq!(solve(example_input, 18), 26);
   assert_eq!(solve(example_input, 80), 5934);
   assert_eq!(solve(example_input, 256), 26984457539);
}

fn main() {
   let path = Path::new("day6-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input, 256));
}
