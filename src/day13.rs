use std::cmp::Ordering;

use aoc_util::parse_t::*;

#[derive(Debug, Clone)]
enum Packet {
	List(Vec<Packet>),
	Num(i32),
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> Ordering {
		use Packet::*;
		match (self, other) {
			(Num(s), Num(o)) => s.cmp(o),
			(List(s), List(o)) => {
				let mut sit = s.iter();
				let mut oit = o.iter();
				while let (s, o) = (sit.next(), oit.next()) {
					match (s, o) {
						(Some(s), Some(o)) => {
							let ord = s.cmp(o);
							if ord != Ordering::Equal {
								return ord;
							}
						},
						(Some(_), None) => return Ordering::Greater,
						(None, Some(_)) => return Ordering::Less,
						(None, None) => return Ordering::Equal,
					};
				}
				Ordering::Equal
			},
			(List(s), Num(_)) => {
				if s.len() < 1 {
					Ordering::Less
				} else if s.len() > 1 {
					if s[0].cmp(other) != Ordering::Less {
						Ordering::Greater
					} else {
						Ordering::Less
					}
				} else {
					s[0].cmp(other)
				}
			},
			_ => {
				other.cmp(self).reverse()
			}
		}
	}
}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Packet {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
	}
}

impl Eq for Packet {}

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day13.txt").unwrap();
	let mut packets = Vec::new();
	let mut last: Option<Packet> = None;
	let mut n = 0;
	let mut p0 = 0;
	for l in data.lines() {
		if let Some(mut s) = l.strip_prefix("[") {
			// println!("{}", l);
			let mut stack = Vec::new();
			let mut cur_list = Vec::new();
			loop {
				if let Some(ss) = s.strip_prefix("[") {
					s = ss;
					let mut list = Vec::new();
					std::mem::swap(&mut cur_list, &mut list);
					stack.push(list);
				} else if s == "]" {
					break;
				} else if let Some(ss) = s.strip_prefix("]") {
					s = ss;
					let mut other = stack.pop().unwrap();
					std::mem::swap(&mut cur_list, &mut other);
					cur_list.push(Packet::List(other));
				} else if let Some(ss) = s.strip_prefix(",") {
					s = ss;
				} else {
					// println!("v {}", s);
					let end = s.find(|c: char| !c.is_ascii_digit()).unwrap();
					let (v, rest) = s.split_at(end);
					s = rest;
					let v = v.parse::<i32>().unwrap();
					cur_list.push(Packet::Num(v));
				}
			}
			let cur = Packet::List(cur_list);
			packets.push(cur.clone());
			if let Some(l) = last {
				n += 1;
				// println!("{:?} {:?}", cur, l);
				// println!("{:?}", l.cmp(&cur));
				if l.cmp(&cur) == Ordering::Less {
					// println!("{:?} {:?} {}", p0, n, p0 + n);
					p0 += n;
				}
				last = None;
			} else {
				last = Some(cur);
			}
		}
	}
	let div2 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
	let div6 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);
	packets.push(div2.clone());
	packets.push(div6.clone());
	packets.sort();
	let f2 = packets.iter().position(|v| *v == div2).unwrap() + 1;
	let f6 = packets.iter().position(|v| *v == div6).unwrap() + 1;
	println!("{} {}", p0, f2 * f6);
}