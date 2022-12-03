
#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day1.txt").unwrap();

	let mut totals = Vec::new();
	for s in data.split("\r\n\r\n") {
		let s = s.trim();
		let mut tot = 0;
		for s in s.split("\r\n") {
			let v = s.parse::<u32>().unwrap();
			tot += v;
		}
		totals.push(tot);
	}
	totals.sort();
	let top3 = &totals[(totals.len() - 3)..];
	let top3: u32 = top3.iter().sum();
	println!("{:?} {:?}", totals.last(), top3);
}