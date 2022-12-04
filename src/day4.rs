use aoc_util::parse_f::*;

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day4.txt").unwrap();

	let mut part0 = 0;
	let mut part1 = 0;
	for s in data.split("\r\n") {
		let (s0, e0, s1, e1, _) = parse_f!(s, (from_str::<i32>, "-"), (from_str::<i32>, ","), (from_str, "-"), (from_str, "")).unwrap();
		if (s0 <= s1 && e0 >= e1) || (s1 <= s0 && e1 >= e0) {
			part0 += 1;
		}

		let r0 = s0..=e0;
		let r1 = s1..=e1;
		if r0.contains(&s1) || r0.contains(&e1) || r1.contains(&s0) || r1.contains(&e0) {
			part1 += 1;
		}
	}
	println!("{} {}", part0, part1);
}