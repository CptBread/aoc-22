use std::collections::HashMap;
use vek::vec::Vec2;

use aoc_util::parse_t::*;

fn read_pos(s: &str) -> Vec2<i32> {
	let (x, y) = parse_t!(s, i32, ",", i32, "").unwrap();
	Vec2::new(x, y)
}

enum Item {
	Source,
	Sand,
	Stone,
}
use Item::*;

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day14.txt").unwrap();
	let mut map = HashMap::new();
	map.insert(Vec2::new(500, 0), Source);
	let source_pos = Vec2::new(500, 0);

	let _print_map = |m: &HashMap<Vec2<i32>, Item>| {
		let (min, max) = m.keys().fold((Vec2::new(500, 0), Vec2::new(500, 0)), |(min, max), v| (Vec2::min(min, *v), Vec2::max(max, *v)));
		for y in min.y..=max.y {
			for x in min.x..=max.x {
				let c = match m.get(&Vec2::new(x, y)) {
					Some(Source) => '+',
					Some(Sand) => 'o',
					Some(Stone) => '#',
					None => '.',
				};
				print!("{}", c);
			}
			println!("");
		}
	};

	for l in data.lines() {
		let mut prev = None;
		for p in l.split(" -> ").map(read_pos) {
			if let Some(prev) = prev {
				let step: Vec2<i32> = p - prev;
				let step = step.map(|v| v.signum());
				let mut at = prev;
				loop {
					map.insert(at, Stone);
					if at == p {
						break;
					}
					at += step;
				}
			}
			prev = Some(p);
		}
	}

	let (min, max) = map.keys().fold((Vec2::new(500, 0), Vec2::new(500, 0)), |(min, max), v| (Vec2::min(min, *v), Vec2::max(max, *v)));
	let down = Vec2::new(0, 1);
	let dl = Vec2::new(-1, 1);
	let dr = Vec2::new(1, 1);
	let moves = [down, dl, dr];
	let bottom = max.y + 1;
	let mut sands = 0;
	let mut p0 = 0;
	// _print_map(&map);
	'_sand: loop{
		let mut at = source_pos;
		'_place: loop {
			if p0 == 0 && (!(min.x..=max.x).contains(&at.x) || !(min.y..=max.y).contains(&at.y)) {
				p0 = sands;
			}
			let mut stop = true;
			if at.y != bottom {
				for m in moves.iter() {
					let next = at + m;
					if map.get(&next).is_none() {
						at = next;
						stop = false;
						break;
					}
				}
			}
			if stop {
				map.insert(at, Sand);
				sands += 1;
				if at == source_pos {
					break '_sand;
				}
				// _print_map(&map);
				break;
			}
		}
	}
	// _print_map(&map);
	println!("{} {}", p0, sands);
}