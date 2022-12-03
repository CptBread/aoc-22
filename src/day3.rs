
fn to_score(c: char) -> usize {
	let r = match c {
		'a'..='z' => {c as u8 - b'a' + 1}
		'A'..='Z' => {c as u8 - b'A' + 27}
		_ => panic!()
	};
	r as usize
}

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day3.txt").unwrap();
	let mut tot = 0;
	let mut g_tot = 0;
	let mut group = Vec::new();
	for s in data.split("\r\n") {
		group.push(s);
		let (p0, p1) = s.split_at(s.len() / 2);
		for i in p0.chars() {
			if p1.contains(i) {
				tot += to_score(i);
				break;
			}
		}
		if group.len() == 3 {
			let (e0, e1, e2) = (group[0], group[1], group[2]);
			for i in e0.chars() {
				if e1.contains(i) && e2.contains(i) {
					g_tot += to_score(i);
					break;
				}
			}
			group.truncate(0);
		}
	}

	println!("{} {}", tot, g_tot);
}