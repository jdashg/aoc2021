use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;
use std::ops;

// -

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
struct Vec4 {
   data: [i64; 4],
}
impl Vec4 {
   const fn new(data: [i64; 4]) -> Vec4 {
      Vec4{data:data}
   }
   const fn origin() -> Vec4 {
      Vec4{ data: [0,0,0,1] }
   }
   const fn zero() -> Vec4 {
      Vec4{ data: [0,0,0,0] }
   }
   fn dot(&self, rhs: &Vec4) -> i64 {
      let mut sum = 0;
      for i in 0..4 {
         sum += self.data[i] * rhs.data[i];
      }
      sum
   }
   const fn mul(&self, rhs: i64) -> Vec4 {
      Vec4{ data: [
         self.data[0] * rhs,
         self.data[1] * rhs,
         self.data[2] * rhs,
         self.data[3] * rhs,
      ]}
   }
   const fn neg(&self) -> Vec4 {
      self.mul(-1)
   }
   const fn cross(&self, r: &Vec4) -> Vec4 {
      Vec4 { data: [
         self.data[1]*r.data[2] - self.data[2]*r.data[1],
         self.data[2]*r.data[0] - self.data[0]*r.data[2],
         self.data[0]*r.data[1] - self.data[1]*r.data[0],
         0,
      ]}
   }
}
impl ops::Sub<&Vec4> for &Vec4 {
   type Output = Vec4;

   fn sub(self, rhs: &Vec4) -> Vec4 {
      Vec4{ data: [
         self.data[0] - rhs.data[0],
         self.data[1] - rhs.data[1],
         self.data[2] - rhs.data[2],
         self.data[3] - rhs.data[3],
      ]}
   }
}

#[derive(Clone,Copy)]
struct Mat44 {
   rows: [Vec4; 4],
}
impl Mat44 {
   const fn identity() -> Mat44 {
      Mat44{rows: [Vec4::new([1,0,0,0]),
                   Vec4::new([0,1,0,0]),
                   Vec4::new([0,0,1,0]),
                   Vec4::new([0,0,0,1])],}
   }
   fn transpose(&self) -> Mat44 {
      let mut ret = Mat44::identity();
      for x in 0..4 {
         for y in 0..4 {
            ret.rows[x].data[y] = self.rows[y].data[x];
         }
      }
      ret
   }
}
impl ops::Mul<&Vec4> for &Mat44 {
   type Output = Vec4;

   fn mul(self, rhs: &Vec4) -> Vec4 {
      let mut ret = Vec4::zero();
      for row in 0..4 {
         ret.data[row] = self.rows[row].dot(&rhs);
      }
      ret
   }
}
impl ops::Mul<&Mat44> for &Mat44 {
   type Output = Mat44;
   fn mul(self, rhs: &Mat44) -> Mat44 {
      let rhst = rhs.transpose();
      let mut ret = Mat44::identity();
      for row in 0..4 {
         for col in 0..4 {
            ret.rows[row].data[col] = self.rows[row].dot(&rhst.rows[col]);
         }
      }
      ret
   }
}

// -

struct AdriftScanner {
   id: u64,
   pings: Vec<Vec4>,
}

struct Scanner {
   id: u64,
   visible_beacons: Vec<Vec4>,
}

impl AdriftScanner {
   fn fix(self, rectify_mat: &Mat44) -> Scanner {
      Scanner{
         id: self.id,
         visible_beacons: self.pings.into_iter().map(|p| {
            rectify_mat * &p
         }).collect(),
      }
   }
}

// -

fn parse_scan(mut s: &str) -> AdriftScanner {
   s = s.trim_start();

   // "--- scanner 26 ---"
   s = s.strip_prefix("--- scanner ").unwrap();
   let (id_s, s) = s.split_once(' ').unwrap();
   let id: u64 = id_s.parse().unwrap();
   let (_, s) = s.split_once('\n').unwrap();

   // "-7,43,-97"
   let pings = s.lines().map(|line| {
      let vals: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
      let mut p = Vec4::origin();
      p.data[0] = vals[0];
      p.data[1] = vals[1];
      p.data[2] = vals[2];
      p
   }).collect();

   AdriftScanner{id: id,
      pings: pings,
   }
}

fn parse_input(input: &str) -> Vec<AdriftScanner> {
   input.trim().split("\n\n").map(|single_scan| {
      parse_scan(single_scan)
   }).collect()
}

const fn look_mat(forward: &Vec4, up: &Vec4) -> Mat44 {
   let side = up.cross(forward);
   Mat44 {
      rows: [ *forward,
              side,
              *up,
              Vec4::origin() ]
   }
}

const DIR_MATS: [Mat44; 24] = {
   let axes = [
      Vec4{data: [ 1,0,0,0]},
      Vec4{data: [ 0,1,0,0]},
      Vec4{data: [ 0,0,1,0]},
   ];
   let mut f = 0;
   let mut out = 0;
   let mut ret = [Mat44::identity(); 24];
   while f < 3 {
      let fv = axes[f];
      let fv2 = fv.neg();
      let mut u = 0;
      while u < 3 {
         if f != u {
            let uv = axes[u];
            let uv2 = uv.mul(-1);
            ret[out] = look_mat(&fv, &uv); out += 1;
            ret[out] = look_mat(&fv2, &uv); out += 1;
            ret[out] = look_mat(&fv, &uv2); out += 1;
            ret[out] = look_mat(&fv2, &uv2); out += 1;
         }
         u += 1;
      }
      f += 1;
   }
   ret
};

fn find_mat(known: &Scanner, adrift: &AdriftScanner) -> Option<Mat44> {
   for dir_mat in DIR_MATS.iter() {
      let mut hits_by_offset: HashMap<Vec4,usize> = HashMap::new();
      let mut max_hits = 0;
      for (i, p_ping) in adrift.pings.iter().enumerate() {
         let remaining = adrift.pings.len() - i;
         //if max_hits + remaining < 12 {
         //   // Cannot possibly make it to 12 hits
         //   return None;
         //}

         let p_guess = dir_mat * p_ping;
         for p_known in known.visible_beacons.iter() {
            let v_to_known = p_known - &p_guess;
            //println!("v_to_known {:?}", v_to_known);
            let e = hits_by_offset.entry(v_to_known).or_insert(0);
            *e += 1;
            //println!("{}", *e);
            max_hits = cmp::max(max_hits, *e);
            if *e >= 12 {
               let mut offset_mat = Mat44::identity();
               offset_mat.rows[0].data[3] = v_to_known.data[0];
               offset_mat.rows[1].data[3] = v_to_known.data[1];
               offset_mat.rows[2].data[3] = v_to_known.data[2];
               let rectify_mat = &offset_mat * dir_mat;
               return Some(rectify_mat);
            }
         }
      }
   }
   None
}

// -

fn solve_p1(input: &str) -> usize {
   let mut adrift_by_id: HashMap<u64,_> = parse_input(input)
      .into_iter().map(|x| (x.id, x)).collect();
   let mut known_by_id: HashMap<u64,_> = HashMap::new();

   // 0 is known-good
   let fixed = adrift_by_id.remove(&0).unwrap().fix(&Mat44::identity());
   known_by_id.insert(0, fixed);

   let mut pairs_to_try: Vec<(u64,u64)> = Vec::new();
   for (i,_) in adrift_by_id.iter() {
      pairs_to_try.push((0 as u64,*i));
   }

   while let Some((known_id,adrift_id)) = pairs_to_try.pop() {
      println!("Trying {},{}", known_id, adrift_id);
      if let Some(adrift) = adrift_by_id.get(&adrift_id) {
         let known = known_by_id.get(&known_id).unwrap();
         if let Some(rectify_mat) = find_mat(known, adrift) {
            println!("  hit!");

            let fixed = adrift_by_id.remove(&adrift_id).unwrap().fix(&rectify_mat);
            known_by_id.insert(adrift_id, fixed);

            for (i,_) in adrift_by_id.iter() {
               pairs_to_try.push((adrift_id,*i));
            }
         }
      }
   }
   assert!(adrift_by_id.is_empty());

   let mut all_beacons = HashSet::new();
   for (_,s) in known_by_id.iter() {
      all_beacons.extend(s.visible_beacons.iter().cloned());
   }

   all_beacons.len()
}

//#[test]
fn test_example() {
   let input = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";
   let actual_beacon_list = "\
-892,524,684
-876,649,763
-838,591,734
-789,900,-551
-739,-1745,668
-706,-3180,-659
-697,-3072,-689
-689,845,-530
-687,-1600,576
-661,-816,-575
-654,-3158,-753
-635,-1737,486
-631,-672,1502
-624,-1620,1868
-620,-3212,371
-618,-824,-621
-612,-1695,1788
-601,-1648,-643
-584,868,-557
-537,-823,-458
-532,-1715,1894
-518,-1681,-600
-499,-1607,-770
-485,-357,347
-470,-3283,303
-456,-621,1527
-447,-329,318
-430,-3130,366
-413,-627,1469
-345,-311,381
-36,-1284,1171
-27,-1108,-65
7,-33,-71
12,-2351,-103
26,-1119,1091
346,-2985,342
366,-3059,397
377,-2827,367
390,-675,-793
396,-1931,-563
404,-588,-901
408,-1815,803
423,-701,434
432,-2009,850
443,580,662
455,729,728
456,-540,1869
459,-707,401
465,-695,1988
474,580,667
496,-1584,1900
497,-1838,-617
527,-524,1933
528,-643,409
534,-1912,768
544,-627,-890
553,345,-567
564,392,-477
568,-2007,-577
605,-1665,1952
612,-1593,1893
630,319,-379
686,-3108,-505
776,-3184,-501
846,-3110,-434
1135,-1161,1235
1243,-1093,1063
1660,-552,429
1693,-557,386
1735,-437,1738
1749,-1800,1813
1772,-405,1572
1776,-675,371
1779,-442,1789
1780,-1548,337
1786,-1538,337
1847,-1591,415
1889,-1729,1762
1994,-1805,1792
";
   assert_eq!(solve_p1(&input), 79);

   println!("Examples complete.");
}

fn main() {
   test_example();

   let path = Path::new("day19-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve_p1(input) -> {:?}", solve_p1(&input));
//   println!("solve_p2(input) -> {:?}", solve_p2(&input));
}
