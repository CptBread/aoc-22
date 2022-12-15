use std::collections::HashMap;
use vek::vec::Vec2;

use aoc_util::parse_t::*;

fn dist(v0: Vec2<i32>, v1: Vec2<i32>) -> i32 {
	(v0 - v1).reduce(|v0, v1| v0.abs() + v1.abs())
}

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day15.txt").unwrap();
	let mut map = HashMap::new();
	let mut pairs = Vec::new();
	let mut max = Vec2::zero();
	let mut min = max;
	for l in data.lines() {
		let (sx, sy, bx, by) = parse_t!(l, "Sensor at x=", i32, ", y=", i32, ": closest beacon is at x=", i32, ", y=", i32, "").unwrap();
		// let t = parse_t!(l, "Sensor at x=", i32, ", y=", i32, ": closest beacon is at x=", i32, ", y=", i32, "").unwrap();
		let s = Vec2::new(sx, sy);
		let b = Vec2::new(bx, by);
		map.insert(s, 'S');
		map.insert(b, 'B');
		let dist = dist(s, b);
		pairs.push((s, b, dist));
		max = Vec2::max(max, s + Vec2::new(dist, dist));
		min = Vec2::min(min, s + Vec2::new(-dist, -dist));
	}

	// Sort by the lowest y
	pairs.sort_by(|(l, _, ld), (r, _, rd)| (l.y - ld).cmp(&(r.y - rd)));

	let row = 2000000;
	let mut p0 = 0;
	let mut search = pairs.as_slice();
	while let Some(((s, _, range), rest)) = search.split_first() {
		if s.y + range < row || s.y - range > row {
			search = rest;
		} else {
			break;
		}
	}
	for x in min.x..=max.x {
		let at = Vec2::new(x, row);
		for (s, _, range) in pairs.iter().copied() {
			let dist = dist(at, s);
			let d = range - dist;
			if d >= 0 {
				if let Some('B') = map.get(&at).copied() {}
				else {
					p0 += 1;
				}
				break;
			}
		}
	}

	let mut search = pairs.as_slice();
	let mut at = Vec2::zero();
	'fy: for y in 0..=4000000 {
		while let Some(((s, _, range), rest)) = search.split_first() {
			if s.y + range < y {
				search = rest;
			} else {
				break;
			}
		}

		let mut x = 0;
		'fx: while x <= 4000000 {
			at = Vec2::new(x, y);
			x += 1;

			for (s, _, range) in search.iter().copied() {
				let dist = dist(at, s);
				if range >= dist {
					// Found something. Check next pos
					let skip = range - dist - 1;
					if skip > 0 {
						x += skip;
					}
					continue 'fx;
				}
			}
			// i.e. found nothing
			break 'fy;
		}
	}

	// 4811413 13171855019123
	// println!("{:?}", p0);
	println!("{:?} {:?}", p0, (at.x as i64) * 4000000 + at.y as i64);
	// println!("{:?} {:?}", (at.x as i64) * 4000000 + at.y as i64 , at);
}