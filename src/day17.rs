use std::collections::{HashSet};

use vek::Vec2;
type Pos = Vec2<i32>;

fn can_fit(decs: &(Pos, &[bool]), at: Pos, map: &HashSet<Pos>) -> bool {
	let size = decs.0;
	if at.x + size.x > 7 || at.x < 0 {
		return false;
	}
	for y in 0..size.y {
		let s = y * size.x;
		for x in 0..size.x {
			if decs.1[(s + x) as usize] {
				let p = Pos::new(x, y) + at;
				if map.contains(&p) {
					return false;
				}
			}

		}
	}
	true
}

pub fn solve() {
	let data = std::fs::read_to_string("data/day17.txt").unwrap();
	let moves: Vec<_> = data.chars().map(|c|
		match c {
			'>' => Pos::new(1, 0),
			'<' => Pos::new(-1, 0),
			_ => panic!(),
		}
	).collect();
	let pieces = vec![
		(Pos::new(4,1), &[true;4][..]),
		(Pos::new(3,3), &[false, true, false, true, true, true, false, true, false][..]),
		(Pos::new(3,3), &[true, true, true, false, false, true, false, false, true][..]),
		(Pos::new(1,4), &[true;4][..]),
		(Pos::new(2,2), &[true;4][..]),
	];
	let mut move_it = moves.iter().cycle();
	let mut piece_it = pieces.iter().cycle();
	let mut map = HashSet::new();
	let mut max = 0;
	let mut p0 = 0;
	let mut last = 0;
	let cycle = pieces.len() * moves.len();
	let mut diffs = Vec::new();
	let mut end_at = None;
	let mut calc_h = 0;
	for n in 0..1_000_000_000_000u64 {
		let mut at = Pos::new(2, max + 3);
		let piece = piece_it.next().unwrap();
		if n == 2022 {
			p0 = max;
		} else if let Some(end) = end_at {
			calc_h += (max - last) as u64;
			last = max;
			if end == n {
				break;
			}
		} else if n % (cycle as u64) == 0 {
			println!("{:?} {:?} {:?}", max - last, max, n);
			if max != 0 {
				diffs.push(max - last);
			}
			last = max;
			if diffs.len() > 40 {
				let d = diffs.len();
				for size in 4..(d / 3) {
					let (front, pat) = diffs.split_at(d - size);
					let (front,sec) = front.split_at(d - size - size);
					let (_,third) = front.split_at(d - size - size - size);
					// println!("{:?}", (pat, sec));
					if pat == sec && sec == third {
						let v: i32 = pat.iter().copied().sum();
						println!("{:?}", (size, n, v, max));
						let size = cycle * size;

						let left = 1_000_000_000_000u64 - n;

						let fill = left / size as u64;
						calc_h = max as u64 + (fill * v as u64);
						let l = left % size as u64;
						end_at = Some(l + n);
						println!("{:?}", (calc_h, end_at, l, fill, left));
						// return;
					}
					// if
				}
			}
		}

		// println!("{:?}", piece);
		loop {
			let m = move_it.next().unwrap();
			let p = at + m;
			// println!("move {:?} {:?}", m, (at, can_fit(piece, p, &map)));
			if can_fit(piece, p, &map) {at = p;}
			if at.y == 0 {
				break;
			}
			let p = at + Pos::new(0, -1);
			if !can_fit(piece, p, &map) {
				// println!("hit rock {:?}", p);
				break;
			}
			at = p;
		}

		let size = piece.0;
		for y in 0..size.y {
			let s = y * size.x;
			for x in 0..size.x {
				if piece.1[(s + x) as usize] {
					let p = Pos::new(x, y) + at;
					map.insert(p);
				}
			}
		}
		max = max.max(at.y + size.y);

		// for y in (0..=max).rev() {
		// 	let s = y * size.x;
		// 	for x in 0..7 {
		// 		if map.contains(&Pos::new(x, y)) {
		// 			print!("#");
		// 		} else {
		// 			print!(".");
		// 		}
		// 	}
		// 	println!("");
		// } 
	}
	println!("{} {}", p0, calc_h);
}