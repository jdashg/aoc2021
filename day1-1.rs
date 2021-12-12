use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
  let path = Path::new("day1-input.txt");

  let mut file = match File::open(&path) {
    Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
    Ok(file) => file,
  };

  let mut str = String::new();
  file.read_to_string(&mut str).unwrap();
  let mut lines = str.split("\n");

  let mut prev = lines.next().unwrap().parse::<i64>().unwrap();
  let mut num_increases = 0;
  for cur_str in lines {
    if cur_str.is_empty() { break; }
    let cur = cur_str.parse::<i64>().unwrap();
    let diff = cur - prev;
    if diff > 0 {
      num_increases += 1;
    }
    prev = cur;
  }

  println!("num_increases: {}", num_increases);
}
