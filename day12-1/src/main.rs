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
   fn add_edge(&mut self, a: String, b: String) {
      self.add_directed_edge(a.clone(), b.clone());
      self.add_directed_edge(b, a);
   }
}

fn parse_inputs(input: &str) -> Graph {
   let mut graph = Graph{
      neighbors: HashMap::new()
   };

   for line in input.trim().split("\n") {
      let (left, right) = line.split_once("-").unwrap();
      graph.add_edge(left.to_string(), right.to_string());
   }
   graph
}

fn is_small(s: &String) -> bool {
   let lower = s.to_lowercase();
   *s == lower
}

fn enumerate_paths(graph: &Graph, path_stack: &mut Vec<String>) -> Vec<Vec<String>> {
   let cur = path_stack.last().unwrap().clone();
   if cur == "end" { return vec![path_stack.clone()]; }
   let mut ret = Vec::new();
   for next in graph.neighbors[&cur].iter() {
      if is_small(&next) && path_stack.contains(&next) {
         continue;
      }
      path_stack.push(next.clone());
      ret.extend(enumerate_paths(graph, path_stack));
      path_stack.pop();
   }
   ret
}

// "How many paths through this cave system are there that visit small
//  caves at most once?"
fn solve(input: &str) -> usize {
   let graph = parse_inputs(input);

   let mut path_stack = vec!["start".to_string()];
   let paths = enumerate_paths(&graph, &mut path_stack);
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
   assert_eq!(solve(&input), 10);
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
   assert_eq!(solve(&input), 19);
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
   assert_eq!(solve(&input), 226);
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
