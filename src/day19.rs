use std::rc::Rc;

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

impl Resource {
	pub fn iter() -> impl Iterator<Item = Resource> {
		[
			Resource::Ore,
			Resource::Clay,
			Resource::Obsidian,
			Resource::Geode,
		].iter().copied()
	}
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

struct BotBp {
	res: Resource,
	cost: Vec<(Resource, RCount)>,
}

type Bp = [BotBp; Resource::MAX as usize];

#[derive(Debug, Clone, Copy)]
struct State([(RCount, RCount);4]);

impl State {
	fn tick(&mut self) {
		for (c, g) in &mut self.0 {
			*c += *g;
		}
	}
}


fn buy_bot(r: Resource, s: &State, bp: &Bp) -> Option<State> {
	let mut res = s.clone();
	for (cr, c) in bp[r as usize].cost.iter().cloned() {
		let d = &mut res.0[cr as usize];
		d.0 = d.0.checked_sub(c)?;
	}
	res.tick();
	res.0[r as usize].1 += 1;
	Some(res)
}

pub fn solve() {
	let data = std::fs::read_to_string("data/day19.txt").unwrap();
	let mut bps = Vec::new();
	for l in data.lines() {
		// println!("{:?}", parse_t!(l, "Blueprint ", usize, ": Each ore robot costs ", RCount, " ore. Each clay robot costs ", RCount, " ore. Each obsidian robot costs ", RCount, " ore and ", RCount, " clay. Each geode robot costs ", RCount, " ore and ", RCount, " obsidian."));
		let (_, ooc, coc, boc, bcc, goc, gbc) = parse_t!(l, "Blueprint ", usize, ": Each ore robot costs ", RCount, " ore. Each clay robot costs ", RCount, " ore. Each obsidian robot costs ", RCount, " ore and ", RCount, " clay. Each geode robot costs ", RCount, " ore and ", RCount, " obsidian.").unwrap();
		bps.push([
			BotBp{ res: Resource::Ore, cost: vec![(Resource::Ore, ooc)]},
			BotBp{ res: Resource::Clay, cost: vec![(Resource::Ore, coc)]},
			BotBp{ res: Resource::Obsidian, cost: vec![(Resource::Ore, boc), (Resource::Clay, bcc)]},
			BotBp{ res: Resource::Geode, cost: vec![(Resource::Ore, goc), (Resource::Obsidian, gbc)]},
		]);
	}
	let bp: &Bp = &bps[0];
	// let mut start = State([(0,1), (0, 0), (0, 0), (0, 0)]);
	dbg!(start_search(24, bp));
}

fn start_search(mut left: u32, bp: &Bp) -> RCount {
	let mut state = State([(0,1), (0, 0), (0, 0), (0, 0)]);
	let skip = bp.iter().fold(99, |acc, b| acc.min(b.cost[0].1));
	left -= skip;
	for _ in 0..skip {
		state.tick();
	}
	search(left, state, bp)
}

fn search(left: u32, mut state: State, bp: &Bp) -> RCount {
	if left <= 0 {
		// dbg!(state);
		return state.0[Resource::Geode as usize].0;
	}
	let mut best = 0;
	for r in Resource::iter() {
		if let Some(res) = buy_bot(r, &mut state, bp) {
			best = best.max(search(left - 1, res, bp));
		}
	}
	state.tick();
	best = best.max(search(left - 1, state, bp));
	best
}