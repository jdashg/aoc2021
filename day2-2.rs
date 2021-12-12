use std::fs::File;
use std::io::Read;
use std::path::Path;


fn test_example() {
  let input = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";
  assert_eq!(result(input), 900);
}

fn result(input: &str) -> i64 {
  let inputs = input.split("\n").filter(|x| !x.is_empty());

  let mut x = 0;
  let mut y = 0;
  let mut aim = 0;
  for (cmd, val_str) in inputs.map(|x| x.split_once(' ').unwrap()) {
    let val = val_str.parse::<i64>().unwrap();
    match cmd {
      "down" => aim += val,
      "up" => aim -= val,
      "forward" => {
        x += val;
        y += val * aim;
      },
      _ => panic!("{}", cmd),
    }
  }

  return x * y;
}

fn main() {
  test_example();

  let path = Path::new("day2-input.txt");

  let mut file = match File::open(&path) {
    Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
    Ok(file) => file,
  };

  let mut input = String::new();
  file.read_to_string(&mut input).unwrap();

  println!("-> {}", result(&input));
}
