
use aoc_util::array2d::*;

use std::collections::VecDeque;

fn dijk(start: usize, target: usize, arr: &mut Array2D<(u8, i32)>) -> i32 {
	let mut to_check = VecDeque::new();
	to_check.push_back(start);
	arr.data[start].1 = 0;

	while let Some(idx) = to_check.pop_front() {
		let cur = arr.data[idx];
		let at = arr.idx_to_pos(idx);
		for np in arr.neighbours(at).iter().filter_map(|v| *v) {
			let n = arr.get_mut(np).unwrap();
			if n.1 == !0 && cur.0 <= n.0 + 1 {
				n.1 = cur.1 + 1;
				let np = arr.pos_to_idx(np).unwrap();
				if np == target {
					break;
				}
				to_check.push_back(np);
			}
		}
	}
	arr.data[target].1
}

#[allow(dead_code)]
pub fn solve()
{
	let mut start = 0;
	let mut target = 0;
	let mut n = 0;
	let mut arr = Array2D::load_file("data/day12.txt", |c| {
		let res = match c {
			'S' => {
				start = n;
				0
			},
			'E' => {
				target = n;
				b'z' - b'a'
			},
			c => c as u8 - b'a',
		};
		n += 1;
		(res, !0)
	});

	let steps = dijk(target, start, &mut arr);
	let best = arr.data.iter().fold(std::i32::MAX, |best, c| {
		if c.0 == 0 && c.1 != !0{
			best.min(c.1)
		}
		else {
			best
		}
	});

	println!("{:?} {:?}", steps, best);
}