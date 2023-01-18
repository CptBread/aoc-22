use std::rc::Rc;

use aoc_util::parse_t::*;
use vek::num_traits::{CheckedSub, Saturating};

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
	ore: RCount,
	extra: (Resource, RCount),
	max: RCount,
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

	fn tick_n(&mut self, n: u32) {
		for (c, g) in &mut self.0 {
			*c += *g * n;
		}
	}
}

fn try_buy_bot(r: Resource, time: &mut u32, s: &State, bp: &Bp) -> Option<State> {
	let bp = &bp[r as usize];
	if s.0[bp.extra.0 as usize].1 == 0 {
		return None;
	}
	if s.0[bp.res as usize].1 >= bp.max {
		return None;
	}

	let ore = s.0[0];
	let mut takes = if bp.ore > ore.0 {
		let left = bp.ore - ore.0;
		(left + ore.1 - 1) / ore.1
	} else {
		0
	};

	let ex =  bp.extra;
	let ex_s = s.0[ex.0 as usize];
	if ex.1 > ex_s.0 {
		let left = ex.1 - ex_s.0;
		takes = takes.max((left + ex_s.1 - 1) / ex_s.1);
	}

	if takes >= *time {
		return None;
	}
	*time -= takes + 1;

	let mut res = s.clone();
	res.tick_n(takes + 1);
	res.0[0].0 -= bp.ore;
	res.0[ex.0 as usize].0 -= ex.1;

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
			BotBp{ res: Resource::Ore, ore: ooc, extra: (Resource::Ore, 0), max: [ooc, coc, boc, goc].iter().copied().max().unwrap()},
			BotBp{ res: Resource::Clay, ore: coc, extra: (Resource::Ore, 0), max: bcc},
			BotBp{ res: Resource::Obsidian, ore: boc, extra: (Resource::Clay, bcc), max: gbc},
			BotBp{ res: Resource::Geode, ore: goc, extra: (Resource::Obsidian, gbc), max: !0},
		]);
	}
	// let bp: &Bp = &bps[0];

	let mut tot = 0;
	for (id, bp) in bps.iter().enumerate() {
		let res = dbg!(start_search(24, bp));
		tot += res * (id as u32 + 1);
	}
	// let mut start = State([(0,1), (0, 0), (0, 0), (0, 0)]);
	// dbg!(start_search(24, bp));
	println!("{}", tot);

	let mut tot = 1;
	for bp in bps[0..3].iter() {
		let res = dbg!(start_search(32, bp));
		tot *= res;
	}
	println!("{}", tot);
}

fn start_search(mut left: u32, bp: &Bp) -> RCount {
	let mut state = State([(0,1), (0, 0), (0, 0), (0, 0)]);
	let skip = bp.iter().fold(99, |acc, b| acc.min(b.ore));
	left -= skip;
	state.tick_n(skip);
	search(left, state, bp, 0)
}

fn search(left: u32, mut state: State, bp: &Bp, to_beat: u32) -> RCount {
	if left <= 0 {
		// dbg!(state);
		return state.0[Resource::Geode as usize].0;
	}
	let geode = state.0[Resource::Geode as usize];
	let best_possible = geode.0 + left * geode.1 + (left / 2) * (left - 1);
	if best_possible < to_beat {
		return to_beat;
	}

	let geode_bp = &bp[Resource::Geode as usize];
	if state.0[Resource::Obsidian as usize].1 >= geode_bp.extra.1 && state.0[0].1 >= geode_bp.ore {
		return best_possible;
	}

	let mut best = 0;
	let mut missed = 0;
	for r in Resource::iter() {
		let mut t = left;
		if let Some(res) = try_buy_bot(r, &mut t, &mut state, bp) {
			best = best.max(search(t, res, bp, best));
		}
		else {
			missed += 1;
		}
	}
	if missed > 0 {
		state.tick_n(left);
		best = best.max(search(0, state, bp, best));
	}
	best
}
