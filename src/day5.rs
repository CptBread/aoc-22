use aoc_util::parse_t::*;

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day5.txt").unwrap();

	let (init, moves) = data.split_once("\r\n\r\n").unwrap();

	let mut stacks = Vec::new();
	for s in init.lines() {
		let mut n = 0;
		for c in s.chars().skip(1).step_by(4) {
			if stacks.len() <= n {
				stacks.push(Vec::new());
			}
			if c.is_ascii_alphabetic() {
				stacks[n].insert(0, c);
			}
			n += 1;
		}
	}
	println!("{:?}", stacks);

	let mut stacks2 = stacks.clone();

	for s in moves.lines() {
		let (num, from, to) = parse_t!(s, "move ", usize, " from ", usize, " to ", usize, "").unwrap();
		let from = from - 1;
		let to = to - 1;
		for _ in 0..num {
			let c = stacks[from].pop().expect("tried to move from empty stack!");
			stacks[to].push(c);
		}

		let mut moving = stacks2[from].rchunks(num).take(1).next().unwrap().to_vec();
		let f_len = stacks2[from].len();
		stacks2[from].truncate(f_len - num);
		stacks2[to].append(&mut moving);
	}

	for s in stacks.iter() {
		print!("{}", s.last().unwrap());
	}
	println!("");
	for s in stacks2.iter() {
		print!("{}", s.last().unwrap());
	}
}