use std::fs::File;
use std::io::Read;
use std::path::Path;

fn result(input: &str) -> u64 {
   let inputs = input.split("\n").filter(|x| !x.is_empty());
   let bit_width = inputs.clone().next().unwrap().len();

   let mut more_list: Vec<Vec<char>> = inputs.map(|x| x.chars().collect()).collect();
   let mut fewer_list = more_list.clone();

   let mut with_ones = Vec::<Vec<char>>::new();
   let mut with_zeros = with_ones.clone();
   for i in 0..bit_width {
      // more_list
      if more_list.len() != 1 {
         with_ones.clear();
         with_zeros.clear();
         for bits in more_list {
            if bits[i] == '1' {
               with_ones.push(bits.clone());
            } else {
               with_zeros.push(bits.clone());
            }
         }
         if with_ones.len() >= with_zeros.len() {
            more_list = with_ones.clone();
         } else {
            more_list = with_zeros.clone();
         }
      }

      if fewer_list.len() != 1 {
         // fewer_list
         with_ones.clear();
         with_zeros.clear();
         for bits in fewer_list {
            if bits[i] == '1' {
               with_ones.push(bits.clone());
            } else {
               with_zeros.push(bits.clone());
            }
         }
         if with_ones.len() >= with_zeros.len() {
            fewer_list = with_zeros.clone();
         } else {
            fewer_list = with_ones.clone();
         }
      }
   }
   assert_eq!(more_list.len(), 1);
   assert_eq!(fewer_list.len(), 1);

   let o2_gen_rating = u64::from_str_radix(more_list[0].iter().collect::<String>().as_str(), 2).unwrap();
   let co2_scrubber_rating = u64::from_str_radix(fewer_list[0].iter().collect::<String>().as_str(), 2).unwrap();
   o2_gen_rating * co2_scrubber_rating
}

fn main() {
   let example = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";
   assert_eq!(result(example), 230);

   // -

   let path = Path::new("day3-input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("-> {}", result(&input));
}
