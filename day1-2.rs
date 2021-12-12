use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::VecDeque;

fn main() {
  let path = Path::new("day1-input.txt");

  let mut file = match File::open(&path) {
    Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
    Ok(file) => file,
  };

  let mut str = String::new();
  file.read_to_string(&mut str).unwrap();
  let inputs = str.split("\n").filter(|x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap());

  let mut window = VecDeque::<i64>::new();
  let mut prev_sum = None;
  let mut num_increases = 0;

  for input in inputs {
    window.push_back(input);
    if window.len() < 3 { continue; }
    while window.len() > 3 {
      window.pop_front();
    }
    let cur_sum = window.iter().fold(0, |x,y| (x + y));
    match prev_sum {
      Some(prev_sum) => {
        if cur_sum > prev_sum {
          num_increases += 1;
        }
      }
      None => (),
    }
    prev_sum = Some(cur_sum);
  }

  println!("num_increases: {}", num_increases);
}
