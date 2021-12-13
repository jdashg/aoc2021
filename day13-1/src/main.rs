use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashSet;

// -

#[derive(PartialEq,Eq,Hash)]
struct Point {
   x: isize,
   y: isize,
}

struct Fold {
   fold_along: String,
   val: isize,
}

fn parse_inputs(input: &str) -> (HashSet<Point>, Vec<Fold>) {
   let (dot_lines, fold_lines) = input.trim().split_once("\n\n").unwrap();
   let dots: HashSet<_> = dot_lines.split("\n")
      .map(|line| {
         let (x,y) = line.split_once(",").unwrap();
         Point{x:x.parse().unwrap(),
               y:y.parse().unwrap()}
      }).collect();
   let folds: Vec<_> = fold_lines.split("\n")
      .map(|line| {
         // "fold along y=7"
         let (fold_along, val) = line.split_once("=").unwrap();
         Fold{fold_along:fold_along.to_string(),
              val: val.parse().unwrap()}
      }).collect();
   (dots, folds)
}

// "How many dots are visible after completing just the first fold
//  instruction on your transparent paper?"
fn solve(input: &str) -> usize {
   let (mut dots, folds) = parse_inputs(input);

   for fold in &folds[..1] {
      println!("{} {}", fold.fold_along, fold.val);
      match fold.fold_along.as_str() {
         "fold along x" => {
            dots = dots.into_iter().map(|mut p| {
               assert_ne!(p.x, fold.val);
               if p.x > fold.val {
                  p.x = fold.val - (p.x - fold.val);
               }
               assert!(p.x >= 0, "{}", p.x);
               p
            }).collect();
         },
         "fold along y" => {
            dots = dots.into_iter().map(|mut p| {
               assert_ne!(p.y, fold.val);
               if p.y > fold.val { // e.g 8 > 7
                  p.y = fold.val - (p.y - fold.val); // -> 7 - (8 - 7)
               }
               assert!(p.y >= 0, "{}", p.y);
               p
            }).collect();
         },
         _ => panic!("{}", fold.fold_along),
      }
   }
   //for (i,p) in dots.iter().enumerate() {
   //   println!("{} {} {}", i, p.x, p.y);
   //}
   dots.len()
}

// -

#[test]
fn test_example() {
   let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
   assert_eq!(solve(&input), 17);
}

fn main() {
   let path = Path::new("day13-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
