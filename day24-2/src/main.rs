use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::{thread, time};

#[derive(PartialEq,Eq,Clone,Debug,Hash,PartialOrd,Ord)]
struct AluState {
   register_by_name: BTreeMap<char,i64>,
}
impl AluState {
   fn new() -> AluState {
      AluState {
         register_by_name: [
            ('w', 0),
            ('x', 0),
            ('y', 0),
            ('z', 0),
         ].iter().cloned().collect(),
      }
   }
   fn get(&self, reg: char) -> i64 {
      *self.register_by_name.get(&reg).unwrap()
   }
   fn set(mut self, reg: char, val: i64) -> Self {
      let reg = self.register_by_name.get_mut(&reg).unwrap();
      *reg = val;
      self
   }
   fn to_string(&self) -> String {
      let regs: Vec<String> = self.register_by_name.iter().map(|(reg,val)| {
         format!("{}: {}", reg, val)
      }).collect();
      regs.join("\n")
   }
}
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
enum AluInstArg {
   Reg(char),
   Lit(i64),
}
impl AluInstArg {
   fn to_string(&self) -> String {
      match *self {
         Self::Reg(reg) => reg.to_string(),
         Self::Lit(lit) => lit.to_string(),
      }
   }
}


#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct AluInst {
   name: String,
   args: Vec<AluInstArg>,
}
impl AluInst {
   fn parse(line: &str) -> AluInst {
      let (name, args) = line.split_once(' ').unwrap();
      let (reg, args) = line.split_once(' ').unwrap();
      AluInst {
         name: name.to_string(),
         args: args.split(' ').map(|arg| {
            if let Ok(arg) = arg.parse::<i64>() {
               AluInstArg::Lit(arg)
            } else if arg.len() == 1 {
               AluInstArg::Reg(arg.chars().next().unwrap())
            } else {
               panic!("{}", arg);
            }
         }).collect(),
      }
   }

   fn run<'a>(&self, mut state: AluState, mut input: &'a str) -> (AluState, &'a str) {
      let right: i64 = if self.name == "inp" {
         let right: i64 = input[0..1].parse().unwrap();
         input = &input[1..];
         right
      } else  {
         match self.args[1] {
            AluInstArg::Reg(reg) => {
               *state.register_by_name.get(&reg).unwrap()
            },
            AluInstArg::Lit(val) => val,
         }
      };

      let op_fn: fn(i64, i64)->i64 = match self.name.as_str() {
         "add" => |a,b| a + b,
         "mul" => |a,b| a * b,
         "div" => |a,b| a / b,
         "mod" => |a,b| a % b,
         "eql" => |a,b| (a == b) as i64,
         "inp" => |_,b| b,
         _ => panic!("{}", self.name),
      };

      let left = match self.args[0] {
         AluInstArg::Reg(reg) => reg,
         _ => panic!("{:?}", self.args[0]),
      };
      let left = state.register_by_name.get_mut(&left).unwrap();
      *left = op_fn(*left, right);
      (state, input)
   }

   fn to_string(&self) -> String {
      let args: Vec<String> = self.args.iter()
            .map(|x| x.to_string()).collect();
      format!("{} {}", self.name, args.join(" "))
   }
}

fn parse(input: &str) -> Vec<AluInst> {
   input.trim().lines().map(|line| {
      AluInst::parse(line)
   }).collect()
}

#[derive(Clone,PartialEq,Eq,Hash)]
struct AluProg(Vec<AluInst>);
impl AluProg {
   fn to_string(&self) -> String {
      let lines: Vec<String> = self.0.iter().enumerate()
         .map(|(i,x)| format!("[{:4}] {}", i, x.to_string())).collect();
      lines.join("\n")
   }
   fn run<'a>(&self, mut state: AluState, mut input: &'a str)
            -> (AluState, &'a str) {
      for inst in self.0.iter() {
         let (state2,input2) = inst.run(state, input);
         state = state2; input = input2;
      }
      (state, input)
   }
}


fn run(program: &Vec<AluInst>, mut input: &str) -> AluState {
   println!("input: {}", input);
   let mut state = AluState::new();
   for inst in program.iter() {
      let (new_state, new_input) = inst.run(state, input);
      state = new_state;
      input = new_input;
   }
   assert_eq!(input.len(), 0);
   println!("end state:\n{}\n", state.to_string());
   state
}

fn enumerate_run_states(prog: &AluProg, mut state: AluState) -> Vec<AluState> {
   let mut ret = Vec::new();
   for inst in prog.0.iter() {
      let (state2, _) = inst.run(state, "");
      state = state2;
      ret.push(state.clone())
   }
   ret
}

// -

//#[test]
fn test_example() {
   let prog = "\
inp x
mul x -1
";
   println!("\nExample:\n{}", prog.trim());
   let prog = parse(prog);
   assert_eq!(run(&prog, "0").get('x'), 0);
   assert_eq!(run(&prog, "1").get('x'), -1);
   assert_eq!(run(&prog, "2").get('x'), -2);

   let prog = "\
inp z
inp x
mul z 3
eql z x
";
   println!("\nExample:\n{}", prog.trim());
   let prog = parse(prog);
   assert_eq!(run(&prog, "00").get('z'), 1);
   assert_eq!(run(&prog, "01").get('z'), 0);
   assert_eq!(run(&prog, "11").get('z'), 0);
   assert_eq!(run(&prog, "13").get('z'), 1);
   assert_eq!(run(&prog, "26").get('z'), 1);
}

fn read_input_file(path: &str) -> String {
   let path = Path::new(path);
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();
   input
}

fn enum_possible_lines(progs: &Vec<AluProg>)
   -> BTreeSet<(usize, String)>
{
   let mut ret = BTreeSet::new();
   for prog in progs.iter() {
      for (i,inst) in prog.0.iter().enumerate() {
         ret.insert((i, inst.to_string()));
      }
   }
   ret
}

fn crack(prog: &AluProg, zmod_out: i64) -> Vec<AluState> {
   let mut poss_eff_state = Vec::new();
   for w in 1..=9 {
      for z in -1000..1000 {
         let state = AluState::new()
            .set('w', w)
            .set('z', z);
         poss_eff_state.push(state);
      }
   }

   // -

   let mut in_state_by_zmod_out: HashMap<i64, Vec<AluState>> = HashMap::new();
   println!("\ncracking:\n{}", prog.to_string());
   for state in poss_eff_state.iter() {
      let states = enumerate_run_states(prog, state.clone());
      if states[7].get('x') != 0 {
         continue;
      }
      let end_state = states.last().unwrap();
      let z = end_state.get('z') % 26;
      let e = in_state_by_zmod_out.entry(z).or_insert(Vec::new());
      e.push(end_state.clone());
   }
   for (i,(zmod,vec)) in in_state_by_zmod_out.iter().enumerate() {
      println!("zmod[{}]: {} in {} states", i, zmod, vec.len());
   }
   in_state_by_zmod_out.get(&zmod_out).unwrap().clone()
}

fn main() {
   test_example();
   println!("Examples ran clean!");
   thread::sleep(time::Duration::from_millis(1000));

   let prog = read_input_file("day24-1/input.txt");
   println!("\nMONAD:\n{}", prog.trim());
   let prog = parse(&prog);
   let input = "13579246899999";
   println!("MONAD({}) -> {:?}", input, run(&prog, &input).get('z') == 0);

   // Split into subprogs,
   // skipping inp which we'll assign to w manually.
   let mut subprogs: Vec<AluProg> = Vec::new();
   {
      let mut subprog: AluProg = AluProg(Vec::new());
      for inst in prog.iter() {
         if inst.name == "inp" {
            if subprog.0.len() > 0 {
               subprogs.push(subprog.clone());
               subprog.0.clear();
            }
         } else {
            subprog.0.push(inst.clone());
         }
      }
      subprogs.push(subprog);
   }
   assert_eq!(subprogs.len(), 14);
   println!("\nenum_possible_lines:");
   for (i,line) in enum_possible_lines(&subprogs).iter() {
      println!("[{:4}] {}", i, line);
   }
   let deduped: HashSet<AluProg> = subprogs.iter().cloned().collect();
   println!("\n{} -> {} after deduping", subprogs.len(), deduped.len());
   //for (i,sub) in inp_subprogs.iter().enumerate() {
   //   println!("input[{}]:\n{}", i, sub.to_string("  "));
   //}
/*
   let mut poss_inputs: Vec<String> = vec!["".to_string()];
   for _ in 0..1 {
      let mut next = Vec::new();
      for x in poss_inputs {
         for i in 1..=9 {
            let s = x.to_string() + &i.to_string();
            next.push(s);
         }
      }
      poss_inputs = next;
   }
   let mut poss_eff_state = Vec::new();
   for w in 1..=9 {
      for zmod26 in 0..26 {
         let state = AluState::new()
            .set('w', w)
            .set('z', zmod26);
         poss_eff_state.push(state);
      }
   }

   let mut inputs_by_output: HashMap<i64, Vec<String>> = HashMap::new();

   let mut all_progs = subprogs.len();
   for i in 0..all_progs {
      let mut in_state_by_zmod_out: HashMap<i64, Vec<AluState>> = HashMap::new();
      let subprog = &subprogs[i];
      println!("input[{}]:\n{}", i, subprog.to_string());
      for state in poss_eff_state.iter() {
         let (state2, _) = subprog.run(state.clone(), "");
         let zmod = state2.get('z') % 26;
         let e = in_state_by_zmod_out.entry(zmod).or_insert(Vec::new());
         e.push(state2);
      }
      for (i,(zmod,vec)) in in_state_by_zmod_out.iter().enumerate() {
         println!("zmod[{}]: {} in {} states", i, zmod, vec.len());
      }
   }

*/
   // -
   /*
      A  ,   B  ,  C
 0:   1  ,  13  ,  5
 1:   1  ,  15  , 14
 2:   1  ,  15  , 15
 3:   1  ,  11  , 16
 4:  26  , -16  ,  8
 5:  26  , -11  ,  9
 6:  26  , - 6  ,  2
 7:   1  ,  11  , 13
 8:   1  ,  10  , 16
 9:  26  , -10  ,  6
10:  26  , - 8  ,  6
11:  26  , -11  ,  9
12:   1  ,  12  , 11
13:  26  , -15  ,  5
   */

   let mut states: Vec<(String,AluState)> = Vec::new();
   states.push((String::new(), AluState::new()));
   for (i,prog) in subprogs.iter().enumerate() {
      println!("i={}: {} in play", i, states.len());
      let a = &prog.0[3];
      let b = &prog.0[4];
      let c = &prog.0[14];
      assert!(a.to_string().starts_with("div z"));
      assert!(b.to_string().starts_with("add x"));
      assert!(c.to_string().starts_with("add y"));
      let a = if let AluInstArg::Lit(lit) = a.args[1] { lit }
              else { panic!("{:?}", a) };
      let b = if let AluInstArg::Lit(lit) = b.args[1] { lit }
              else { panic!("{:?}", b) };
      let c = if let AluInstArg::Lit(lit) = c.args[1] { lit }
              else { panic!("{:?}", c) };
      assert_eq!(a == 1, b >= 10);
      assert_eq!(a == 26, b <= -6);
      let mut new_states = Vec::new();
      for (input, state) in states.iter() {
         let w_list = {
            let ideal_w = state.get('z') % 26 + b;// Wi = z%26 + Bi
            if 1 <= ideal_w && ideal_w <= 9 {
               //println!("Picking ideal i[{}]: {}", i, ideal_w);
               vec![ideal_w]
            } else if b < 0 {
               // We blew it, give up?
               vec![]
            } else {
               // Muddle through?
               (1..=9).collect()
            }
         };

         for w in w_list {
            let (new_state,_) = prog.run(state.clone().set('w', w), "");
            let new_input = input.clone() + &w.to_string();
            new_states.push((new_input,new_state));
         }
      }
      states = new_states;
   }
   println!("{} states remain!", states.len());

   let highest = states.iter().max().unwrap();
   println!("highest {}-digit: {}\n{}",
   highest.0.len(), highest.0, highest.1.to_string());

   let lowest = states.iter().map(|(input,_)| input).min().unwrap();
   println!("lowest {}-digit: {}", lowest.len(), lowest);

   for (input,state) in states.iter() {
      assert_eq!(state.get('z'), 0, "{}", state.to_string());
   }


   //crack(&subprogs[subprogs.len()-1], 0);

   /*
   for poss_input in poss_inputs.into_iter() {
      let mut state = AluState::new();
      let mut input: &str = &poss_input;
      for i in 0..2 {
         let subprog = &inp_subprogs[i];
         println!("input[{}]:\n{}", i, subprog.to_string("  "));
         let res = subprog.run(state, input);
         state = res.0; input = res.1;
      }
      assert_eq!(input.len(), 0);
      let zmod26 = state.get('z') % 26;
      let e = inputs_by_output.entry(zmod26).or_insert(Vec::new());
      e.push(poss_input);
   }
   println!("inputs_by_output: {}: {:?}", inputs_by_output.len(),
         inputs_by_output);
   */
}
