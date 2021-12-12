use std::fs::File;
use std::io::Read;
use std::path::Path;

fn result(input: &str) -> u64 {
   let inputs = input.split("\n").filter(|x| !x.is_empty());

   let mut one_counts = Vec::new();
   {
      let first = inputs.clone().next().unwrap();
      one_counts.resize(first.len(), 0);
   }

   let mut input_count = 0;
   for cur_bits in inputs {
      input_count += 1;
      for (one_count, cur_bit) in one_counts.iter_mut().zip(cur_bits.chars()) {
         if cur_bit == '1' {
            *one_count += 1;
         }
      }
   }

   let more_threshold = input_count / 2;
   let gamma = one_counts.iter().fold(0u64, |mut gamma, one_count| {
      gamma <<= 1;
      if one_count > &more_threshold {
         gamma |= 1;
      }
      gamma
   });
   let epsilon = !gamma & ((1 << one_counts.len())-1);
   println!("{} {}", gamma, epsilon);


   return gamma * epsilon;
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
   assert_eq!(result(example), 198);

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
