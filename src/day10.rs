

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day10.txt").unwrap();

	fn draw_crt(x: i32, cycle: i32) {
		let cycle = cycle % 40;
		let c = if (x - cycle).abs() < 2 {'#'} else {'.'};
		print!("{}", c);
		if cycle == 39 {
			println!("");
		}
	}

	let mut p0 = 0;
	let mut next = 20;

	let mut reg_x = 1;
	let mut cycle = 0;
	for l in data.lines() {
		let sx = reg_x;
		if l == "noop" {
			draw_crt(sx, cycle);
			cycle += 1;
		}
		else if let Some(v) = l.strip_prefix("addx ") {
			let v: i32 = v.parse().unwrap();
			draw_crt(sx, cycle);
			draw_crt(sx, cycle + 1);
			cycle += 2;
			reg_x += v;
		}

		if cycle >= next {
			p0 += next * sx;
			next += 40;
		}
	}

	println!("{}", p0);
}