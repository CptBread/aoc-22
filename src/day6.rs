use std::collections::HashSet;

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day6.txt").unwrap();
	let data: Vec<_> = data.chars().collect();
	for (n, sub) in data.windows(4).enumerate() {
		let set: HashSet<_> = sub.iter().collect();
		if set.len() == 4 {
			println!("Part1 {}", n + 4);
			break;
		}
	}

	for (n, sub) in data.windows(14).enumerate() {
		let set: HashSet<_> = sub.iter().collect();
		if set.len() == 14 {
			println!("Part2 {}", n + 14);
			break;
		}
	}
}