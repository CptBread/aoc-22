use std::collections::{HashSet, HashMap, VecDeque};
use itertools::Itertools;
use vek::Vec3;
type Pos = Vec3<i32>;

pub fn solve() {
	let data = std::fs::read_to_string("data/day18.txt").unwrap();
	let mut map = HashMap::new();
	let mut max = Pos::zero();
	let mut min = Pos::zero();
	for l in data.lines() {
		let v: (i32, i32, i32) = l.split(",").map(|s| s.parse().unwrap()).next_tuple().unwrap();
		let p = Pos::new(v.0, v.1, v.2);
		max = Pos::max(max, p);
		min = Pos::min(min, p);
		map.insert(p, false);
	}
	max += Pos::one();
	min -= Pos::one();
	let offsets = [
		Pos::new(1, 0, 0),
		Pos::new(-1, 0, 0),
		Pos::new(0, 1, 0),
		Pos::new(0, -1, 0),
		Pos::new(0, 0, 1),
		Pos::new(0, 0, -1),
	];
	let mut sides = 0;
	for (p, not_out) in map.iter() {
		if *not_out {
			continue;
		}
		for off in offsets.iter() {
			let o = p + off;
			if map.get(&o).copied().unwrap_or(true) {
				sides += 1;
			}
		}
	}
	let part0 = sides;

	let mut to_check = VecDeque::new();
	to_check.push_back(max);

	while let Some(at) = to_check.pop_front() {
		for off in offsets {
			let p = at + off;
			if !map.contains_key(&p) && Pos::min(min, p) == min && Pos::max(max, p) == max {
				map.insert(p, true);
				to_check.push_back(p);
			}
		}
	}

	let mut sides = 0;
	for (p, not_out) in map.iter() {
		if *not_out {
			continue;
		}
		for off in offsets.iter() {
			let o = p + off;
			if map.get(&o).copied().unwrap_or(false) {
				sides += 1;
			}
		}
	}
	println!("{} {}", part0, sides);
}