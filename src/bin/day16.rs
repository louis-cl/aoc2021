#[derive(Debug, PartialEq, Clone)]
struct Header {
    v: u8,
    t: u8,
}
#[derive(Debug, PartialEq, Clone)]
struct Length {
    packet_mode: bool,
    l: usize
}

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    ValueP(Header, u64),
    OperatorP(Header, u8, Length, Vec<Packet>),
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("?"),
    }
}

fn from_bin(bin: &[u8]) -> u64 {
    bin.iter().fold(0, |x, b| (x << 1) | *b as u64)
}

fn parse_value(bin: &[u8]) -> (usize, u64) {
    let mut v = 0;
    let mut i = 0;
    let mut cont = true;
    while cont {
        v <<= 4;
        cont = bin[i] == 1;
        v |= from_bin(&bin[i+1..i+5]);
        i += 5;
    }
    (i, v)
}

fn parse(bin: &[u8]) -> (usize, Packet) {
    // println!("parsing from {:?}", bin);
    let mut i = 0;
    let v = from_bin(&bin[0..3]) as u8;
    let t = from_bin(&bin[3..6]) as u8;
    i += 6;
    let header = Header { v, t};
    match t {
        4 => {
            let (n, v) = parse_value(&bin[i..]);
            (n+i, Packet::ValueP(header, v))
        },
        id => {
            // println!("to parse as op {:?}", &bin[i..i+16]);
            let packet_mode = bin[i] == 1;
            i += 1;
            let to_read = if !packet_mode { 15 } else { 11 };
            let l= from_bin(&bin[i..i+to_read]) as usize;
            i += to_read;

            let length = Length { packet_mode, l};
            // println!("{:?}", length);

            let mut packets = Vec::new();
            if packet_mode {  // num of sub packets
                for _ in 0..l {
                    let (n, p) = parse(&bin[i..]);
                    // println!("sub packet {:?} of size {}", p, n);
                    i += n;
                    packets.push(p);
                }
            } else { // length in bits
                let mut j = 0;
                while j < l {
                    let (n, p) = parse(&bin[i+j..]);
                    // println!("sub packet {:?} of size {}", p, n);
                    j += n;
                    packets.push(p);
                }
                assert_eq!(j, l);
                i += j;
            }
            (i, Packet::OperatorP(header,  id,length, packets))
        }
    }
}

fn decode(s: &str) -> Vec<u8> {
    s.chars().map(|c| to_binary(c).chars()).flatten()
        .map(|c| c.to_digit(2).unwrap() as u8).collect()
}

fn pparse(s: &str) -> Packet {
    parse(&decode(s)).1
}

#[cfg(test)]
mod parse_test {
    use super::*;

    #[test]
    fn value() {
        let p = pparse("D2FE28");
        assert_eq!(p, Packet::ValueP(Header { v: 6, t: 4}, 2021));
    }

    #[test]
    fn operator1() {
        let p = pparse("38006F45291200");
        assert_eq!(p, Packet::OperatorP(
            Header { v: 1, t: 6},
            6,
            Length { packet_mode: false, l: 27},
            [Packet::ValueP(Header { v: 6, t: 4}, 10),
                Packet::ValueP(Header { v: 2, t: 4}, 20)].to_vec()
        ));
    }

    #[test]
    fn operator2() {
        let p = pparse("EE00D40C823060");
        assert_eq!(p, Packet::OperatorP(
            Header { v: 7, t:3},
            3,
            Length { packet_mode: true, l: 3},
            [Packet::ValueP(Header { v: 2, t: 4}, 1),
                Packet::ValueP(Header { v: 4, t: 4}, 2),
                Packet::ValueP(Header { v: 1, t: 4}, 3)].to_vec()
        ));
    }
}

fn sum_version(packet: &Packet) -> u64 {
    return match packet {
        Packet::ValueP(h, _) => h.v as u64,
        Packet::OperatorP(h, _, _, ps) => {
            ps.iter().map(sum_version).sum::<u64>() + h.v as u64
        }
    }
}

fn value(packet: &Packet) -> u64 {
    match packet {
        Packet::ValueP(_, v) => *v,
        Packet::OperatorP(_, id, _, ps) => {
            let mut sub = ps.iter().map(value);
            match id {
                0 => sub.sum::<u64>(),
                1 => sub.product::<u64>(),
                2 => sub.min().unwrap(),
                3 => sub.max().unwrap(),
                5 => if sub.next().unwrap() > sub.next().unwrap() { 1 } else { 0 },
                6 => if sub.next().unwrap() < sub.next().unwrap() { 1 } else { 0 },
                7 => if sub.next().unwrap() == sub.next().unwrap() { 1 } else { 0 },
                _ => panic!("unknown")
            }
        }
    }
}

fn solve(inp: &str) -> (u64, u64) {
    let packet = pparse(inp);
    println!("{:?}", packet);
    let p1 = sum_version(&packet);

    let p2 = value(&packet);
    (p1,p2)
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (p1, p2) = solve("8A004A801A8002F478");
        assert_eq!(p1, 16);
        // assert_eq!(p2, 315);
    }
    #[test]
    fn test2() {
        let (p1, p2) = solve("620080001611562C8802118E34");
        assert_eq!(p1, 12);
        // assert_eq!(p2, 315);
    }
    #[test]
    fn test3() {
        let (p1, p2) = solve("C0015000016115A2E0802F182340");
        assert_eq!(p1, 23);
        // assert_eq!(p2, 315);
    }
    #[test]
    fn test4() {
        let (p1, p2) = solve("A0016C880162017C3686B18A3D4780");
        assert_eq!(p1, 31);
        // assert_eq!(p2, 315);
    }
    #[test]
    fn test5() {
        assert_eq!(solve("C200B40A82").1, 3);
        assert_eq!(solve("04005AC33890").1, 54);
        assert_eq!(solve("880086C3E88112").1, 7);
        assert_eq!(solve("CE00C43D881120").1, 9);
        assert_eq!(solve("D8005AC2A8F0").1, 1);
        assert_eq!(solve("F600BC2D8F").1, 0);
        assert_eq!(solve("9C005AC2F8F0").1, 0);
        assert_eq!(solve("9C0141080250320F1802104A08").1, 1);

    }
}

fn main() {
    let (p1, p2) = solve(include_str!("../../input/day16").lines().next().unwrap());
    println!("p1 = {}", p1);
    println!("p2 = {}", p2);
}