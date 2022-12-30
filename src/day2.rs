use aoc_util::parse_f::*;

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day2.txt").unwrap();

	let parser = |s: &str| -> Option<(usize, usize)> {
		let mut it = s.chars();
		let v = it.next()? as usize - 'A' as usize;
		it.next()?;
		Some((v, it.next()? as usize - 'X' as usize))
	};

	let scorer = [1, 2, 3];
	let outcomes = [(1, 2), (2, 0), (0, 1)]; // (wins, loses) idx what they played

	let (strat, _) = parse_f!(data, (seperated_f("\r\n", parser), "")).unwrap();
	let mut tot = 0;
	let mut tot2 = 0;
	for (p, r) in strat.iter() {
		let mut score = scorer[*r];
		let (win, loss) = outcomes[*p];
		let draw = *p;
		if *r == win {
			score += 6;
		}
		else if *r == draw {
			score += 3;
		}
		tot += score;

		match *r {
			0 => {
				tot2 += scorer[loss];
			},
			1 => {
				tot2 += scorer[draw] + 3;
			},
			2 => {
				tot2 += scorer[win] + 6;
			}
			_ => panic!()
		}
	}

	println!("{} {}", tot, tot2);
}