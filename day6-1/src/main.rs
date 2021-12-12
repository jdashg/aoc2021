use std::fs::File;
use std::io::Read;
use std::path::Path;

fn solve(input: &str) -> usize {
   let mut inputs = input.split("\n")
                     .filter(|x| !x.is_empty());
   let mut fish_list: Vec<usize> = inputs.next().unwrap()
                               .split(',')
                               .map(|x| x.parse::<usize>().unwrap())
                               .collect();
   for _ in 0..80 {
      let mut num_new_fish = 0;
      for f in fish_list.iter_mut() {
         if *f == 0 {
            num_new_fish += 1;
            *f = 6;
         } else {
            *f -= 1;
         }
      }
      for _ in 0..num_new_fish {
         fish_list.push(8);
      }
   }
   fish_list.len()
}

#[test]
fn example() {
   let example_input = "\
3,4,3,1,2
";
   assert_eq!(solve(example_input), 5934);
}

fn main() {
   let path = Path::new("day6-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
