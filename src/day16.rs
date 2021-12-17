struct BitFeed {
    data: Vec<bool>,
    pointer: usize
}

impl BitFeed {
    fn new(data: Vec<bool>) -> BitFeed {
        BitFeed {data: data, pointer: 0}
    }

    fn from_hex_str(input: &str) -> BitFeed {
        let mut data = Vec::with_capacity(input.len()*4);
        input.chars().for_each(|c| {
            let val = c.to_digit(16).unwrap();
            // Least cursed thing I could think of
            data.push(val & 8 != 0);
            data.push(val & 4 != 0);
            data.push(val & 2 != 0);
            data.push(val & 1 != 0);
            // Still quite cursed
        });
        BitFeed::new(data)
    }

    /// Returns the requested number of bits as a u64 option
    fn take_bits(&mut self, amount: usize) -> Option<u64> {
        if amount + self.pointer > self.data.len() {
            return None;
        } else {
            let bits = &self.data[self.pointer..(self.pointer+amount)];
            let val = bits.iter().rev().enumerate().fold(0, |acc, (i, b)| acc + ((*b as u64) << i));
            self.pointer += amount;
            return Some(val);
        }
    }
}

fn parse_packet_versions(feed: &mut BitFeed) -> u64 {
    let ver = feed.take_bits(3).unwrap();
    let type_id = feed.take_bits(3).unwrap();

    let mut ver_sum = ver;

    match type_id {
        // 4 - Literal Value
        4 => {
            let mut _sum = 0;
            loop {
                let last_bits = feed.take_bits(5).unwrap();
                _sum += last_bits & 0xF;
                if last_bits & 0x10 == 0 {break}
            }
        },
        // Not 4 - Operator Packet
        _ => {
            if feed.take_bits(1).unwrap() == 0 {
                // Total bit length
                let bit_length = feed.take_bits(15).unwrap() as usize;
                let target_ptr = feed.pointer + bit_length;
                while feed.pointer < target_ptr {
                    let v = parse_packet_versions(feed);
                    ver_sum += v;
                }
            } else {
                // Number of packets
                let num_packets = feed.take_bits(11).unwrap();
                for _ in 0..num_packets {
                    let v = parse_packet_versions(feed);
                    ver_sum += v;
                }
            }

        }
    }
    return ver_sum;
}

#[aoc(day16, part1)]
fn part1(input: &str) -> u64 {
    let mut feed = BitFeed::from_hex_str(input);
    parse_packet_versions(&mut feed)
}

fn parse_packet(feed: &mut BitFeed, depth: usize) -> u64 {
    let _ver = feed.take_bits(3).unwrap();
    let type_id = feed.take_bits(3).unwrap();

    if type_id == 4 {
        let mut sum = 0;
        loop {
            let last_bits = feed.take_bits(5).unwrap();
            sum = sum << 4;
            sum += last_bits & 0xF;
            if last_bits & 0x10 == 0 {break}
        }
        return sum;
    }

    let mut args: Vec<u64> = Vec::new();
    if feed.take_bits(1).unwrap() == 0 {
        // Total bit length
        let bit_length = feed.take_bits(15).unwrap() as usize;
        let target_ptr = feed.pointer + bit_length;
        while feed.pointer < target_ptr {
            args.push(parse_packet(feed, depth+1));
        }
    } else {
        // Number of packets
        let num_packets = feed.take_bits(11).unwrap();
        for _ in 0..num_packets {
            args.push(parse_packet(feed, depth+1));
        }
    }
    
    match type_id {
        0 => args.iter().sum(),
        1 => args.iter().product(),
        2 => *args.iter().min().unwrap(),
        3 => *args.iter().max().unwrap(),
        5 => (args[0] > args[1]) as u64,
        6 => (args[0] < args[1]) as u64,
        7 => (args[0] == args[1]) as u64,
        4 | _ => { panic!("Unexpected type_id: {}", type_id) }
    }
}

#[aoc(day16, part2)]
fn part2(input: &str) -> u64 {
    let mut feed = BitFeed::from_hex_str(input);
    parse_packet(&mut feed, 0)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn bitfeed_test() {
        let mut buff = BitFeed::from_hex_str("5F1");
        assert_eq!(buff.data, vec![false, true, false, true,
                                   true, true, true, true,
                                   false, false, false, true ]);
        assert_eq!(buff.take_bits(3), Some(2));
        assert_eq!(buff.take_bits(4), Some(0xF));
        assert_eq!(buff.take_bits(4), Some(0x8));
        assert_eq!(buff.take_bits(2), None);
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }
    #[test]
    fn part2_test_16() {
        assert_eq!(part2("C200B40A82"), 3);
        assert_eq!(part2("04005AC33890"), 54);
        assert_eq!(part2("880086C3E88112"), 7);
        assert_eq!(part2("CE00C43D881120"), 9);
        assert_eq!(part2("D8005AC2A8F0"), 1);
        assert_eq!(part2("F600BC2D8F"), 0);
        assert_eq!(part2("9C005AC2F8F0"), 0);
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }
    const INPUT:&str = "20546718027401204FE775D747A5AD3C3CCEEB24CC01CA4DFF2593378D645708A56D5BD704CC0110C469BEF2A4929689D1006AF600AC942B0BA0C942B0BA24F9DA8023377E5AC7535084BC6A4020D4C73DB78F005A52BBEEA441255B42995A300AA59C27086618A686E71240005A8C73D4CF0AC40169C739584BE2E40157D0025533770940695FE982486C802DD9DC56F9F07580291C64AAAC402435802E00087C1E8250440010A8C705A3ACA112001AF251B2C9009A92D8EBA6006A0200F4228F50E80010D8A7052280003AD31D658A9231AA34E50FC8010694089F41000C6A73F4EDFB6C9CC3E97AF5C61A10095FE00B80021B13E3D41600042E13C6E8912D4176002BE6B060001F74AE72C7314CEAD3AB14D184DE62EB03880208893C008042C91D8F9801726CEE00BCBDDEE3F18045348F34293E09329B24568014DCADB2DD33AEF66273DA45300567ED827A00B8657B2E42FD3795ECB90BF4C1C0289D0695A6B07F30B93ACB35FBFA6C2A007A01898005CD2801A60058013968048EB010D6803DE000E1C6006B00B9CC028D8008DC401DD9006146005980168009E1801B37E02200C9B0012A998BACB2EC8E3D0FC8262C1009D00008644F8510F0401B825182380803506A12421200CB677011E00AC8C6DA2E918DB454401976802F29AA324A6A8C12B3FD978004EB30076194278BE600C44289B05C8010B8FF1A6239802F3F0FFF7511D0056364B4B18B034BDFB7173004740111007230C5A8B6000874498E30A27BF92B3007A786A51027D7540209A04821279D41AA6B54C15CBB4CC3648E8325B490401CD4DAFE004D932792708F3D4F769E28500BE5AF4949766DC24BB5A2C4DC3FC3B9486A7A0D2008EA7B659A00B4B8ACA8D90056FA00ACBCAA272F2A8A4FB51802929D46A00D58401F8631863700021513219C11200996C01099FBBCE6285106";
    #[test]
    fn part2_problem() {
        let result = part2(INPUT);
        println!("The answer is: {}", result);
        assert_eq!(0,1);
    }
}