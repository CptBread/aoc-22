use aoc_util::parse_t::*;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
enum Resource {
	Ore,
	Clay,
	Obsidian,
	Geode,
	MAX,
}

type RCount = u32;

// #[derive(Debug)]
// struct Bp {
// 	costs: [Vec<(Resource, u8)>; Resource::MAX as usize],
// 	// ore: [RCount; 1],
// 	// clay: [RCount; 1],
// 	// obs: [RCount; 2],
// 	// geode: [RCount; 2],
// }

type Bp = [Vec<(Resource, RCount)>; Resource::MAX as usize];
type State = [(RCount, RCount);4];

fn tick(s: &mut State) {
	for (c, g) in s {
		*c += *g;
	}
}

fn buy_bot(r: Resource, s: &State, bp: &Bp) -> Option<State> {
	let mut res = s.clone();
	for (cr, c) in bp[r as usize].iter().cloned() {
		let d = &mut res[cr as usize];
		d.0 = d.0.checked_sub(c)?;
	}
	res[r as usize].1 += 1;
	Some(res)
}

pub fn solve() {
	let data = std::fs::read_to_string("data/day19.txt").unwrap();
	let mut bp = Vec::new();
	for l in data.lines() {
		// println!("{:?}", parse_t!(l, "Blueprint ", usize, ": Each ore robot costs ", RCount, " ore. Each clay robot costs ", RCount, " ore. Each obsidian robot costs ", RCount, " ore and ", RCount, " clay. Each geode robot costs ", RCount, " ore and ", RCount, " obsidian."));
		let (_, ooc, coc, boc, bcc, goc, gbc) = parse_t!(l, "Blueprint ", usize, ": Each ore robot costs ", RCount, " ore. Each clay robot costs ", RCount, " ore. Each obsidian robot costs ", RCount, " ore and ", RCount, " clay. Each geode robot costs ", RCount, " ore and ", RCount, " obsidian.").unwrap();
		bp.push([
			vec![(Resource::Ore, ooc)],
			vec![(Resource::Ore, coc)],
			vec![(Resource::Ore, boc), (Resource::Clay, bcc)],
			vec![(Resource::Ore, goc), (Resource::Obsidian, gbc)],
		]);
	}
	let mut start: State = [(0,1), (0, 0), (0, 0), (0, 0)];
	tick(&mut start);
	tick(&mut start);
	tick(&mut start);
	tick(&mut start);
	dbg!(buy_bot(Resource::Ore, &start, &bp[0]));
	dbg!(start);
}