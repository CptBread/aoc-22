use aoc_util::array2d::*;

fn prop_vis_step(start: Pos, dx: isize, dy: isize, arr: &mut Array2D<(u8, bool)>) {
	let mut at = start;
	let v = arr.get_mut(at).unwrap();
	v.1 = true;

	let mut h = v.0;
	while let Some(pos) = arr.pos_offset(at, dx, dy) {
		at = pos;
		let v = arr.get_mut(at).unwrap();
		if v.0 > h {
			v.1 = true;
			h = v.0;
		}
	}
}

fn prop_vis(arr: &mut Array2D<(u8, bool)>) {
	let h = arr.height;
	let w = arr.width;
	for n in 0..h {
		prop_vis_step([0, n].into(), 1, 0, arr);
		prop_vis_step([w - 1, n].into(), -1, 0, arr);
	}
	for n in 0..w {
		prop_vis_step([n, 0].into(), 0, 1, arr);
		prop_vis_step([n, h - 1].into(), 0, -1, arr);
	}
}

fn eval(idx: usize, arr: &Array2D<(u8, bool)>) -> usize {
	let at = arr.idx_to_pos(idx);
	let mut score = 1;
	let tree_h = arr.data[idx].0;
	let mut n  = 0;
	for x in (0..at.x).into_iter().rev() {
		let mut at = at;
		at.x = x;
		n += 1;

		let t = arr.get(at).unwrap();
		if t.0 >= tree_h {
			break;
		}
	}
	score *= n;
	n = 0;
	for x in (at.x + 1)..arr.width {
		let mut at = at;
		at.x = x;
		n += 1;

		let t = arr.get(at).unwrap();
		if t.0 >= tree_h {
			break;
		}
	}
	score *= n;
	n = 0;
	for y in (0..at.y).into_iter().rev() {
		let mut at = at;
		at.y = y;
		n += 1;

		let t = arr.get(at).unwrap();
		if t.0 >= tree_h {
			break;
		}
	}
	score *= n;
	n = 0;
	for y in (at.y + 1)..arr.height {
		let mut at = at;
		at.y = y;
		n += 1;

		let t = arr.get(at).unwrap();
		if t.0 >= tree_h {
			break;
		}
	}
	score * n
}

#[allow(dead_code)]
pub fn solve()
{
	let mut arr = Array2D::load_file("data/day8.txt", |c| (c as u8 - b'0', false));
	prop_vis(&mut arr);
	// arr.print(|v| (v.0 + b'0') as char );
	// arr.print(|v| if v.1 {'#'} else {'.'} );

	let mut max = 0;
	for idx in 0..arr.data.len() {
		let s = eval(idx, &arr);
		if s > max {
			println!("{:?}", (arr.idx_to_pos(idx), arr.data[idx], s));
		}
		max = max.max(s);
	}

	let vis = arr.data.iter().filter(|v| v.1).count();
	// 1717 321975
	println!("{} {}", vis, max);
}