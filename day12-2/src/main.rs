use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

// -

struct Graph {
   neighbors: HashMap<String,Vec<String>>,
}
impl Graph {
   fn add_directed_edge(&mut self, from: String, to: String) {
      let e = self.neighbors.entry(from).or_insert(Vec::new());
      e.push(to);
   }
}

fn parse_inputs(input: &str) -> Graph {
   let mut graph = Graph{
      neighbors: HashMap::new()
   };

   for line in input.trim().split("\n") {
      let (left, right) = line.split_once("-").unwrap();
      let mut left_to_right = true;
      let mut right_to_left = true;
      if left == "start" {
         right_to_left = false;
      }
      if right == "start" {
         left_to_right = false;
      }
      if left == "end" {
         left_to_right = false;
      }
      if right == "end" {
         right_to_left = false;
      }
      if left_to_right {
         graph.add_directed_edge(left.to_string(), right.to_string());
      }
      if right_to_left {
         graph.add_directed_edge(right.to_string(), left.to_string());
      }
   }
   graph
}

fn is_small(s: &String) -> bool {
   let lower = s.to_lowercase();
   *s == lower
}

fn enumerate_paths(graph: &Graph, did_small_cave_twice: bool, path_stack: &mut Vec<String>) -> Vec<Vec<String>> {
   let cur = path_stack.last().unwrap().clone();
   if cur == "end" { return vec![path_stack.clone()]; }
   let mut ret = Vec::new();
   for next in graph.neighbors[&cur].iter() {
      let mut next_did_small_cave_twice = did_small_cave_twice;
      if is_small(&next) {
         let count = path_stack.iter().filter(|x| *x == next).count();
         let limit = if did_small_cave_twice { 0 } else { 1 };
         if count > limit {
            continue;
         }
         next_did_small_cave_twice |= count == 1;
      }
      path_stack.push(next.clone());
      ret.extend(enumerate_paths(graph, next_did_small_cave_twice, path_stack));
      path_stack.pop();
   }
   ret
}

// "How many paths through this cave system are there that visit small
//  caves at most once?"
fn solve(input: &str) -> usize {
   let graph = parse_inputs(input);

   let mut path_stack = vec!["start".to_string()];
   let paths = enumerate_paths(&graph, false, &mut path_stack);
   paths.len()
}

// -

#[test]
fn test_example() {
   let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
   assert_eq!(solve(&input), 36);
}
#[test]
fn test_example2() {
   let input = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
   assert_eq!(solve(&input), 103);
}
#[test]
fn test_example3() {
   let input = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";
   assert_eq!(solve(&input), 3509);
}

fn main() {
   let path = Path::new("day12-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
