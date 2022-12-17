use aoc_util::parse_t::*;


#[derive(Debug, Clone)]
enum Arg {
	Num(i64),
	Old,
}

impl Arg {
	fn get(&self, old: i64) -> i64 {
		match self {
			Arg::Num(i) => *i,
			Arg::Old => old
		}
	}
}

impl<'a> ParseTUtil<'a> for Arg {
	type Res = Self;
	fn parse(s: &str) -> Option<Self> {
		if s == "old" {
			Some(Arg::Old)
		}
		else {
			s.parse::<i64>().map(|n| Arg::Num(n)).ok()
		}
	}
}

#[derive(Debug, Clone)]
enum Op {
	Mul,
	Add,
}

impl Op {
	fn exec(&self, arg0: i64, arg1: i64) -> i64 {
		match self {
			Op::Mul => arg0 * arg1,
			Op::Add => arg0 + arg1,
		}
	}
}

impl<'a> ParseTUtil<'a> for Op {
	type Res = Self;
	fn parse(s: &str) -> Option<Self> {
		Some(match s {
			"*" => Op::Mul,
			"+" => Op::Add,
			_ => return None,
		})
	}
}

#[derive(Debug, Clone)]
struct Monkey {
	id: usize,
	args: (Arg, Arg),
	op: Op,
	items: Vec<i64>,
	test: i64,
	if_t: usize,
	if_f: usize,
}

impl Monkey {
	fn turn(&mut self, cmd: &mut Vec<(usize, i64)>) {
		while let Some(i) = self.items.pop() {
			let w = self.op.exec(self.args.0.get(i), self.args.1.get(i)) / 3;
			let t = if w % self.test == 0 {self.if_t} else {self.if_f};
			cmd.push((t, w));
		}
	}

	fn turn2(&mut self, cmd: &mut Vec<(usize, i64)>, mod_by: i64) {
		while let Some(i) = self.items.pop() {
			let w = self.op.exec(self.args.0.get(i), self.args.1.get(i)) % mod_by;
			let t = if w % self.test == 0 {self.if_t} else {self.if_f};
			cmd.push((t, w));
		}
	}
}

impl<'a> ParseTUtil<'a> for Monkey {
	type Res = Self;
	fn parse(s: &str) -> Option<Self> {
		let (id, items, a0, op, a1, test, if_t, if_f) = parse_t!(s,
			"Monkey ", usize, ":\r\n  Starting items: ", Csv<i64>,
			"\r\n  Operation: new = ", Arg, " ", Op, " ", Arg,
			"\r\n  Test: divisible by ", i64,
			"\r\n    If true: throw to monkey ", usize,
			"\r\n    If false: throw to monkey ", usize, ""
		)?;
		Some(Self {
			id, items, args: (a0, a1), op, test, if_t, if_f
		})
	}
}

fn distribute(cmd: &mut Vec<(usize, i64)>, monk: &mut Vec<Monkey>) {
	for (t, i) in cmd.iter() {
		// println!("{} goes to {}", i, t);
		monk[*t].items.push(*i)
	}
	cmd.clear();
}

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day11.txt").unwrap();
	let mut monk = Vec::new();
	for l in data.split("\r\n\r\n") {
		let m = Monkey::parse(l).unwrap();
		assert_eq!(m.id, monk.len());
		println!("{:?}", m);
		monk.push(m);
	}
	let mut inspections = vec![0; monk.len()];
	let mut cmd = Vec::new();
	let monk_c = monk.clone();
	for _n in 0..20 {
		for id in 0..monk.len() {
			inspections[id] += monk[id].items.len();
			monk[id].turn(&mut cmd);
			distribute(&mut cmd, &mut monk);
		}
		// println!("{:?}", _n);
		// for m in monk.iter() {
		// 	println!("{:?}", m.items);
		// }
	}

	inspections.sort_unstable();
	let w0 = inspections.pop().unwrap();
	let w1 = inspections.pop().unwrap();
	println!("{:?}", w0 * w1);


	monk = monk_c;
	let mod_by = monk.iter().fold(1, |a, v| a * v.test);
	let mut inspections = vec![0; monk.len()];
	for _n in 0..10000 {
		for id in 0..monk.len() {
			inspections[id] += monk[id].items.len();
			monk[id].turn2(&mut cmd, mod_by);
			distribute(&mut cmd, &mut monk);
		}
		// println!("{:?}", _n);
		// for m in monk.iter() {
		// 	println!("{:?}", m.items);
		// }
	}
	
	// println!("{:?}", inspections);

	// for m in monk.iter() {
	// 	println!("{:?}", m.items);
	// }
	inspections.sort_unstable();
	let w0 = inspections.pop().unwrap();
	let w1 = inspections.pop().unwrap();
	
	println!("{:?}", w0 * w1);
	// 64032
	// 12729522272
}