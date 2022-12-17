use std::{collections::{HashSet}};
use vek::{Vec2};

use aoc_util::parse_t::*;

type Pos = Vec2<i32>;

fn move_tail(head: &Pos, tail: &mut Pos) {
	let diff = *head - *tail;
	if diff.x.abs() > 1 {
		tail.x += diff.x.signum();
		if diff.y.abs() > 0 {
			tail.y += diff.y.signum();
		}
	}
	else if diff.y.abs() > 1 {
		tail.y += diff.y.signum();
		if diff.x.abs() > 0 {
			tail.x += diff.x.signum();
		}
	}
}

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day9.txt").unwrap();
	let mut visited = HashSet::new();
	let mut visited2 = HashSet::new();
	let mut head = Pos::zero();
	let mut tail = Pos::zero();
	let mut tail_cont = vec![tail; 8];
	visited.insert(tail);
	visited2.insert(tail);

	for l in data.lines() {
		let (dir, steps) = parse_t!(l, char, " ", i32, "").unwrap();
		let dir = match dir {
			'R' => Pos::new(1, 0),
			'L' => Pos::new(-1, 0),
			'U' => Pos::new(0, 1),
			'D' => Pos::new(0, -1),
			_ => panic!(),
		};
		for _ in 0..steps {
			head += dir;
			move_tail(&head, &mut tail);
			visited.insert(tail);

			move_tail(&tail, &mut tail_cont[0]);
			for i in 1..tail_cont.len() {
				let h = tail_cont[i - 1].clone();
				move_tail(&h, &mut tail_cont[i]);
			}
			visited2.insert(*tail_cont.last().unwrap());
		}

		// let max = tail_cont.iter().chain(std::iter::once(&head)).fold(Pos::zero(), |acc, v| Pos::max(*v, acc));
		// let min = tail_cont.iter().chain(std::iter::once(&head)).fold(Pos::zero(), |acc, v| Pos::min(*v, acc));
		// for y in min.y..=max.y {
		// 	for x in min.x..=max.x {
		// 		let at = Pos::new(x, y);
		// 		let mut c = '.';
		// 		if tail_cont.contains(&at) {
		// 			c = '#';
		// 		}
		// 		if head == at {
		// 			c = 'H';
		// 		}
		// 		print!("{}", c);
		// 	}
		// 	println!("");
		// }
		// println!("");
	}
	// let max = visited2.iter().fold(Pos::zero(), |acc, v| Pos::max(*v, acc));
	// let min = visited2.iter().fold(Pos::zero(), |acc, v| Pos::min(*v, acc));
	// for y in min.y..=max.y {
	// 	for x in min.x..=max.x {
	// 		let at = Pos::new(x, y);
	// 		let mut c = '.';
	// 		if tail_cont.contains(&at) {
	// 			c = '#';
	// 		}
	// 		if head == at {
	// 			c = 'H';
	// 		}
	// 		print!("{}", c);
	// 	}
	// 	println!("");
	// }

	// 6266, 2369
	println!("{} {}", visited.len(), visited2.len());
}