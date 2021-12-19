use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;
use std::ops::RangeFrom;
use std::ops::Mul;
use std::ops::Add;

// -

struct NodePool<T> {
   nodes: HashMap<usize,TreeNode<T>>,
   next_id: RangeFrom<usize>,
}
impl<T: Copy+ToString+Mul<i64,Output=T>+Add<Output=T>> NodePool<T> {
   fn new() -> NodePool<T> {
      NodePool{ nodes: HashMap::new(),
        next_id: 1..,
      }
   }
   fn insert(&mut self, node: TreeNode<T>) -> usize {
      let id = self.next_id.next().unwrap();
      self.nodes.insert(id.clone(), node);
      id
   }
   fn add(&mut self, val: T, left_id: &usize, right_id: &usize) -> usize {
      let mut node = TreeNode::new(val);
      node.left = Some(*left_id);
      node.right = Some(*right_id);
      let node_id = self.insert(node);
      let left = self.nodes.get_mut(left_id).unwrap();
      left.parent = Some(node_id);
      let right = self.nodes.get_mut(right_id).unwrap();
      right.parent = Some(node_id);
      node_id
   }
   fn remove_leaf(&mut self, id: &usize) -> TreeNode<T> {
      let mut node = self.nodes.remove(id).unwrap();
      assert!(node.left == None && node.right == None);
      if let Some(parent_id) = node.parent {
         let parent = self.nodes.get_mut(&parent_id).unwrap();
         if parent.left == Some(*id) {
            parent.left = None;
         } else if parent.right == Some(*id) {
            parent.right = None;
         }
      }
      node.parent = None;
      node
   }

   fn next<F: FnMut(&TreeNode<T>) -> Option<usize> >(&self, id: &usize, mut f: F) -> Option<usize> {
      f(self.nodes.get(id).unwrap())
   }
   fn next_leaf(&self, start_id: &usize) -> Option<usize> {
      let mut id = start_id.clone();

      // Go up until we've gone right
      {
         let node = self.nodes.get(&id).unwrap();
         // Leaves have no left or right
         assert!(node.left == None && node.right == None);

         let mut gone_right = false;
         while let Some(parent_id) = self.next(&id, |n| n.parent) {
            let prev = id;
            id = parent_id;
            let parent = self.nodes.get(&id).unwrap();
            if parent.left == Some(prev) { // Came from left
               gone_right = true;
               break;
            }
         }
         if !gone_right {
            return None;
         }
      }

      // Go right once
      id = self.next(&id, |n| n.right).unwrap(); // Go right once
      id = self.seek_left(&id); // Seek left
      assert_ne!(id, *start_id);
      Some(id)
   }
   fn prev_leaf(&self, start_id: &usize) -> Option<usize> {
      let mut id = start_id.clone();

      // Go up until we've gone left
      {
         let node = self.nodes.get(&id).unwrap();
         // Leaves have no left or right
         assert!(node.left == None && node.right == None);

         let mut gone_left = false;
         while let Some(parent_id) = self.next(&id, |n| n.parent) {
            let prev = id;
            id = parent_id;
            let parent = self.nodes.get(&id).unwrap();
            if parent.right == Some(prev) { // Came from right
               gone_left = true;
               break;
            }
         }
         if !gone_left {
            return None;
         }
      }
      id = self.next(&id, |n| n.left).unwrap(); // Go left once
      id = self.seek_right(&id); // Seek right
      assert_ne!(id, *start_id);
      Some(id)
   }

   fn traverse<F: FnMut(&TreeNode<T>) -> Option<usize> >(&self, start_id: &usize, mut f: F) -> usize {
      let mut id = start_id.clone();
      //while let Some(id2) = self.next(&id, f) {
      while let Some(id2) = f(self.nodes.get(&id).unwrap()) {
         id = id2;
      }
      id
   }
   fn _seek_root(&self, id: &usize) -> usize {
      self.traverse(id, |n| n.parent)
   }
   fn seek_left(&self, id: &usize) -> usize {
      self.traverse(id, |n| n.left)
   }
   fn seek_right(&self, id: &usize) -> usize {
      self.traverse(id, |n| n.right)
   }

   fn depth(&self, start_id: &usize) -> usize {
      let mut depth = 0;
      self.traverse(start_id, |n| {
         if n.parent != None {
            depth += 1
         }
         n.parent
      });
      depth
   }

   fn magnitude(&self, id: &usize) -> T {
      let node = self.nodes.get(id).unwrap();
      if node.left != None {
         self.magnitude(&node.left.unwrap())*3 + self.magnitude(&node.right.unwrap())*2
      } else {
         node.val
      }
   }

   fn to_string2(&self, id: &usize, with_ids: &bool) -> String {
      let node = self.nodes.get(id).unwrap();
      let mut ret = String::new();
      if *with_ids {
         ret.push_str(&id.to_string());
         ret.push(':');
      }
      if node.left != None {
         ret.push('[');
         ret.push_str(&self.to_string2(&node.left.unwrap(), with_ids));
         ret.push(',');
         ret.push_str(&self.to_string2(&node.right.unwrap(), with_ids));
         ret.push(']');
      } else {
         ret.push_str(&node.val.to_string())
      }
      ret
   }
   fn to_string(&self, id: &usize) -> String {
      self.to_string2(id, &false)
   }
}
#[derive(Clone)]
struct TreeNode<T> {
   val: T,
   parent: Option<usize>,
   left: Option<usize>,
   right: Option<usize>,
}
impl<T> TreeNode<T> {
   fn new(val: T) -> TreeNode<T> {
      TreeNode{val: val,
         parent:None,
         left:None,
         right:None}
   }
}

// "[[1,2],3]"
fn parse_node<'a>(mut s: &'a str, pool: &mut NodePool<i64>) -> (usize, &'a str) {
   let node_id = if &s[0..1] == "[" {
      // Pair
      let (left_id, s2) = parse_node(&s[1..], pool);
      assert_eq!(&s2[0..1], ",");
      let (right_id, s3) = parse_node(&s2[1..], pool);
      assert_eq!(&s3[0..1], "]");
      s = &s3[1..];
      pool.add(-1, &left_id, &right_id)
   } else {
      // Regular (always one char)
      let val = s[0..1].parse::<i64>().unwrap();
      s = &s[1..];
      pool.insert(TreeNode::new(val))
   };
   (node_id, s)
}

fn parse_tree(s: &str, pool: &mut NodePool<i64>) -> usize {
   assert_eq!(&s[0..1], "[");
   let (root_id, extra) = parse_node(s, pool);
   assert_eq!(extra, "");
   root_id
}

fn parse_inputs(mut input: &str) -> (NodePool<i64>, Vec<usize>) {
   let mut pool = NodePool::new();
   input = input.trim();
   let roots: Vec<_> = input.lines()
      .map(|s| parse_tree(s, &mut pool)).collect();
   (pool, roots)
}

// To explode a pair, the pair's left value is added to the first
// regular number to the left of the exploding pair (if any), and the
// pair's right value is added to the first regular number to the right
// of the exploding pair (if any). Exploding pairs will always consist
// of two regular numbers. Then, the entire exploding pair is replaced
// with the regular number 0.
fn explode(pool: &mut NodePool<i64>, pair_id: &usize) {
   //println!("explode({})", *pair_id);
   let node = pool.nodes.get_mut(pair_id).unwrap();
   node.val = 0;
   let node2 = node.clone();
   let was_left = pool.remove_leaf(&node2.left.unwrap());
   let was_right = pool.remove_leaf(&node2.right.unwrap());

   if let Some(next_left) = pool.prev_leaf(pair_id) {
      pool.nodes.get_mut(&next_left).unwrap().val += was_left.val;
   }
   if let Some(next_right) = pool.next_leaf(pair_id) {
      pool.nodes.get_mut(&next_right).unwrap().val += was_right.val;
   }
}

fn split(pool: &mut NodePool<i64>, id: &usize) {
   //println!("split({})", *id);
   let val = pool.nodes.get(id).unwrap().val;
   let lval = val / 2;
   let rval = val - lval;
   let mut left = TreeNode::new(lval);
   let mut right = TreeNode::new(rval);
   left.parent = Some(*id);
   right.parent = Some(*id);
   let left_id = pool.insert(left);
   let right_id = pool.insert(right);
   let node = pool.nodes.get_mut(id).unwrap();
   node.left = Some(left_id);
   node.right = Some(right_id);
   node.val = -1;
}

fn reduce_once(pool: &mut NodePool<i64>, root_id: &usize) -> bool {
   // 1. If any pair is nested inside four pairs, the leftmost such
   //    pair explodes.
   {
      let mut leaf_id = pool.seek_left(root_id);
      loop {
         let depth = pool.depth(&leaf_id);
         assert!(depth <= 5);
         if depth == 5 {
            let pair_id = pool.next(&leaf_id, |n| n.parent).unwrap();
            explode(pool, &pair_id);
            return true;
         }
         if let Some(next_leaf_id) = pool.next_leaf(&leaf_id) {
            leaf_id = next_leaf_id;
            continue;
         }
         break;
      }
   }

   // 2. If any regular number is 10 or greater, the leftmost such
   //    regular number splits.
   {
      let mut leaf_id = pool.seek_left(&root_id);
      loop {
         let val = &pool.nodes.get_mut(&leaf_id).unwrap().val;
         if *val >= 10 {
            split(pool, &leaf_id);
            return true;
         }
         if let Some(next_leaf_id) = pool.next_leaf(&leaf_id) {
            leaf_id = next_leaf_id;
            continue;
         }
         break;
      }
   }
   return false;
}

fn reduce(pool: &mut NodePool<i64>, root_id: &usize) {
   //println!("reduce({})", pool.to_string2(root_id,&false));
   while reduce_once(pool, root_id) {
      //println!("reduce_once -> {}", pool.to_string2(root_id,&false));
   }
}

fn add_inputs(input: &str) -> (NodePool<i64>, usize) {
   let (mut pool, root_ids) = parse_inputs(input);
   let mut iter = root_ids.iter();
   let init = iter.next().unwrap().clone();
   let root_id = iter.fold(init, |acc, next| {
      let root_id = pool.add(-1, &acc, next);
      reduce(&mut pool, &root_id);
      root_id
   });
   (pool, root_id)
}

// -

fn assert_adds_to(input: &str, expected: &str) {
   let (pool, root_id) = add_inputs(input);
   let was = pool.to_string(&root_id);
   //println!("Expect: {}", expected);
   //println!("   Was: {}", was);
   assert_eq!(was, expected);
}

//#[test]
fn test_example() {
   assert_adds_to("\
[1,1]
[2,2]
[3,3]
[4,4]
", "[[[[1,1],[2,2]],[3,3]],[4,4]]");

   assert_adds_to("\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
", "[[[[3,0],[5,3]],[4,4]],[5,5]]");

   assert_adds_to("\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
", "[[[[5,0],[7,4]],[5,5]],[6,6]]");
   assert_adds_to("\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
", "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");


   {
      let (pool, root_id) = add_inputs("[[1,2],[[3,4],5]]");
      assert_eq!(pool.magnitude(&root_id), 143);
   }
   {
      let (pool, root_id) = add_inputs("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
      assert_eq!(pool.magnitude(&root_id), 1384);
   }
   {
      let (pool, root_id) = add_inputs("[[[[1,1],[2,2]],[3,3]],[4,4]]");
      assert_eq!(pool.magnitude(&root_id), 445);
   }
   {
      let (pool, root_id) = add_inputs("[[[[3,0],[5,3]],[4,4]],[5,5]]");
      assert_eq!(pool.magnitude(&root_id), 791);
   }
   {
      let (pool, root_id) = add_inputs("[[[[5,0],[7,4]],[5,5]],[6,6]]");
      assert_eq!(pool.magnitude(&root_id), 1137);
   }
   {
      let (pool, root_id) = add_inputs("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
      assert_eq!(pool.magnitude(&root_id), 3488);
   }
   let example = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";
   let (pool, root_id) = add_inputs(&example);
   assert_eq!(pool.to_string(&root_id), "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
   assert_eq!(pool.magnitude(&root_id), 4140);

   println!("Examples complete.");
}

fn main() {
   test_example();

   let path = Path::new("day18-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   let (pool, root_id) = add_inputs(&input);
   println!("solve(input) -> {}", pool.magnitude(&root_id));
}
