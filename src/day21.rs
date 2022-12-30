use std::cell::Cell;
use std::collections::HashMap;

use aoc_util::parse_f::*;
use aoc_util::parse_t::*;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
enum Op {
	Num(i64),
	Add(usize, usize),
	Sub(usize, usize),
	Mul(usize, usize),
	Div(usize, usize),
	Eqc(usize, usize),
}
use Op::*;

impl Op {
	fn params(&self) -> Option<(usize, usize)> {
		match self {
			Num(_) => None,
			Add(p0, p1) => Some((*p0, *p1)),
			Sub(p0, p1) => Some((*p0, *p1)),
			Mul(p0, p1) => Some((*p0, *p1)),
			Div(p0, p1) => Some((*p0, *p1)),
			Eqc(p0, p1) => Some((*p0, *p1)),
		}
	}
}

fn parse_op(s: &str, names: &mut HashMap<String, usize>) -> Option<Op> {
	if let Some(n) = i64::parse(s) {
		return Some(Num(n));
	}

	let (p0, op, p1) = parse_t!(s, String, " ", char, " ", String, "")?;
	let p0 = name_to_id(p0, names);
	let p1 = name_to_id(p1, names);
	match op {
		'+' => Some(Add(p0, p1)),
		'-' => Some(Sub(p0, p1)),
		'*' => Some(Mul(p0, p1)),
		'/' => Some(Div(p0, p1)),
		_ => None,
	}
}

fn name_to_id(name: String, names: &mut HashMap<String, usize>) -> usize {
	let next_id = names.len();
	*names.entry(name).or_insert(next_id)
}

fn resize_to_fit_idx<T: Default + Clone>(v: &mut Vec<T>, idx: usize) -> &mut T {
	if v.len() <= idx {
		v.resize(idx + 1, T::default());
	}
	v.get_mut(idx).unwrap()
}

thread_local! {
	static rems: Cell<usize> = Cell::new(0);
}

fn resolve(id: usize, nodes: &Vec<Op>) -> i64 {
	match nodes[id] {
		Num(n) => n,
		Add(p0, p1) => resolve(p0, nodes) + resolve(p1, nodes),
		Sub(p0, p1) => resolve(p0, nodes) - resolve(p1, nodes),
		Mul(p0, p1) => resolve(p0, nodes) * resolve(p1, nodes),
		Div(p0, p1) => {
			let p0 = resolve(p0, nodes);
			let p1 = resolve(p1, nodes);
			if p0 % p1 != 0 {
				rems.with(|v| v.set(v.get() + 1));
			}
			p0 / p1
		},
		Eqc(p0, p1) => {
			let p0 = resolve(p0, nodes);
			let p1 = resolve(p1, nodes);
			println!("p0 {:?}", p0);
			println!("p1 {:?}", p1);
			if p0 == p1 {1} else {0}
		},
	}
}

pub fn solve() {
	let data = std::fs::read_to_string("data/day21.txt").unwrap();
	let mut names = HashMap::new();
	let mut nodes = Vec::new();
	for l in data.lines() {
		// println!("{:?}", parse_t!(l, "Blueprint ", usize, ": Each ore robot costs ", RCount, " ore. Each clay robot costs ", RCount, " ore. Each obsidian robot costs ", RCount, " ore and ", RCount, " clay. Each geode robot costs ", RCount, " ore and ", RCount, " obsidian."));
		let (name, op, _) = parse_f!(l, (from_str, ": "), (|s| parse_op(s, &mut names), "")).unwrap();
		let id = name_to_id(name, &mut names);
		*resize_to_fit_idx(&mut nodes, id) = Some(op);
	}
	let mut nodes: Vec<_> = nodes.into_iter().map(|v| v.unwrap()).collect();
	let root = names["root"];
	let me = names["humn"];
	println!("{}", resolve(root, &nodes));
	let mut id0 = 0;
	let mut id1 = 0;
	if let Some((p0, p1)) = nodes[root].params() {
		id0 = p0;
		id1 = p1;
		nodes[root] = Eqc(p0, p1);
	}
	println!("{}", resolve(root, &nodes));
	let t = resolve(id1, &nodes);
	let mut at = 0;
	let mut step = i64::MAX / 100;
	loop {
		let n = at + step;
		nodes[me] = Num(n);
		let v = resolve(id0, &nodes);
		if v > t {
			at = n;
		} else if v == t {
			at = n;
			println!("TARGET {}", n);
			break;
		} else {
			println!("Reduce step to {}. {:?}", step, (n, v));
			step /= 2;
		}
	}
	let mut min = 0;
	let mut max = 0;
	for n in 0.. {
		let nn = at + n;
		nodes[me] = Num(nn);
		let v = resolve(id0, &nodes);
		if v != t {
			println!("MAX {} {}", at + n - 1, n - 1);
			max = at + n - 1;
			break;
		}
	}
	for n in 0.. {
		let nn = at - n;
		nodes[me] = Num(nn);
		let v = resolve(id0, &nodes);
		if v != t {
			println!("MIN {} {}", at - n + 1, 1 - n);
			min = at - n + 1;
			break;
		}
	}
	for n in min..=max {
		rems.with(|r| r.set(0));
		nodes[me] = Num(n);
		resolve(id0, &nodes);
		if rems.with(|r| r.get() == 0) {
			println!("RESULT {}", n);
			break;
		}
	}

	nodes[me] = Num(at);
	// println!("{}", resolve(id0, &nodes));
	println!("{}", resolve(id0, &nodes));
	println!("{}", resolve(id1, &nodes));
	println!("{}", resolve(root, &nodes));
}