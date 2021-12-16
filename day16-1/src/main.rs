use std::fs::File;
use std::io::Read;
use std::path::Path;
use bitreader::BitReader;

// -

fn parse_inputs(input: &str) -> Vec<u8> {
   let chars = input.trim();
   let mut bytes = vec![0u8; (chars.len()+1) / 2];
   chars.char_indices().for_each(|(ic,c)| {
      let ib = ic / 2;
      let is_low = ic & 1 == 1;
      let mut val = c.to_digit(16).unwrap();
      if !is_low {
         val <<= 4;
      }
      bytes[ib] |= val as u8;
   });
   bytes
}

fn solve(input: &str) -> usize {
   let bytes = parse_inputs(&input);
   let mut reader = BitReader::new(&bytes[..]);
   let root = ElfPacket::from(&mut reader);
   root.version_sum()
}

// -

struct ElfHeader {
   version: u8,
   ptype: u8,
}
impl ElfHeader {
   fn from(reader: &mut BitReader) -> ElfHeader {
      ElfHeader{
         version: reader.read_u8(3).unwrap(),
         ptype: reader.read_u8(3).unwrap(),
      }
   }
}

enum ElfData {
   Literal(usize),
   Operator(Vec<ElfPacket>),
}

struct ElfPacket {
   header: ElfHeader,
   data: ElfData,
}
impl ElfPacket {
   fn from(reader: &mut BitReader) -> ElfPacket {
      let header = ElfHeader::from(reader);
      let data = match header.ptype {
         4 => {
            let mut val: usize = 0;
            loop {
               let group = reader.read_u32(5).unwrap() as usize;
               let has_more = (group & 0b10000) != 0;
               val = val << 4 | group & 0b01111;
               if !has_more { break; }
            }
            ElfData::Literal(val)
         },
         _ => {
            // Operator
            let length_type_id = reader.read_u32(1).unwrap();
            let packets = if length_type_id == 0 {
               let num_bits = reader.read_u32(15).unwrap();
               let read_until = reader.position() + num_bits as u64;
               let mut packets = Vec::new();
               while reader.position() < read_until {
                  packets.push(ElfPacket::from(reader));
               }
               assert_eq!(reader.position(), read_until);
               packets
            } else {
               let num_packets = reader.read_u32(11).unwrap();
               (0..num_packets).map(|_| {
                  ElfPacket::from(reader)
               }).collect()
            };
            ElfData::Operator(packets)
         },
      };
      ElfPacket{
         header: header,
         data: data,
      }
   }

   fn version_sum(&self) -> usize {
      let mut sum = self.header.version as usize;
      match &self.data {
         ElfData::Operator(packets) => {
            sum += packets.iter().map(|p| p.version_sum()).sum::<usize>();
         },
         _ => {},
      }
      sum
   }
}

//#[test]
fn test_example() {
   {
      let bytes = parse_inputs("D2FE28");
      let mut reader = BitReader::new(&bytes[..]);
      let packet = ElfPacket::from(&mut reader);
      assert_eq!(packet.header.version, 6);
      assert_eq!(packet.header.ptype, 4);
      match packet.data {
         ElfData::Literal(val) => {
            assert_eq!(val, 2021);
         },
         _ => panic!(),
      }
   }
   assert_eq!(solve("8A004A801A8002F478"), 16);
   assert_eq!(solve("620080001611562C8802118E34"), 12);
   assert_eq!(solve("C0015000016115A2E0802F182340"), 23);
   assert_eq!(solve("A0016C880162017C3686B18A3D4780"), 31);
   println!("Examples complete.");
}

fn main() {
   test_example();

   let path = Path::new("day16-1/input.txt");
   let mut file = match File::open(&path) {
      Err(why) => panic!("File::open({}) -> Err({})", path.display(), why),
      Ok(file) => file,
   };
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   println!("solve(input) -> {}", solve(&input));
}
