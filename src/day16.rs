use std::collections::{HashMap, VecDeque};

use intbits::*;

use aoc_util::parse_t::*;

#[derive(Default, Debug, Clone)]
struct Valve {
	idx: usize,
	flow: u32,
	connection: Vec<usize>,
}

fn get_id(id: String, names: &mut HashMap<String, usize>, valves: &mut Vec<Valve>) -> usize {
	let next = valves.len();
	let id =*names.entry(id).or_insert(next);
	if id == next {
		valves.push(Valve::default());
	}
	id
}

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day16.txt").unwrap();
	let mut names = HashMap::new();
	let mut valves = Vec::new();
	let mut dist = HashMap::new();
	for l in data.lines() {
		let (id, flow, connect) = parse_t!(l, "Valve ", String, " has flow rate=", u32 ,"; tunnels lead to valves ", Csv<String>,"").or_else(|| {
			parse_t!(l, "Valve ", String, " has flow rate=", u32 ,"; tunnel leads to valve ", Csv<String>,"")
		}).unwrap();
		let idx = get_id(id, &mut names, &mut valves);
		let connect: Vec<_> = connect.into_iter().map(|id| get_id(id, &mut names, &mut valves)).collect();
		valves[idx] = Valve {
			idx, flow, 
			connection: connect,
		};
	}

	for v in valves.iter() {
		let mut res = vec![std::u32::MAX; valves.len()];
		let mut to_check: VecDeque<_> = v.connection.iter().cloned().map(|v| (v, 1)).collect();
		while let Some(cur) = to_check.pop_front() {
			if res[cur.0] > cur.1 {
				res[cur.0] = cur.1;
			} else {
				continue;
			}
			for c in valves[cur.0].connection.iter().copied() {
				if res[c] > cur.1 + 1 {
					to_check.push_back((c, cur.1 + 1));
				}
			}
		}
		dist.insert(v.idx, res);
	}
	let start_open = valves.iter().fold(0, |mut o, v| {
		if v.flow == 0 {
			o.set_bit(v.idx, true);
		}
		o
	});
	let start_from = *names.get(&"AA".to_string()).unwrap();
	let best_flow = valves.iter().fold(0u32, |acc, v| acc + v.flow);

	println!("{}", search(start_from, 30, 0, 0, start_open, &dist, &valves));
	println!("{}", search2([(start_from, 0, false), (start_from, 0, false)], 26, 0, 0, start_open, &dist, &valves, (best_flow, 0)));
}

fn search(at: usize, left: u32, flow: u32, score: u32, open: usize, dist: &HashMap<usize, Vec<u32>>, valves: &Vec<Valve>) -> u32 {
	if left == 0 {
		println!("{:?}", (at, score, open));
		return score;
	}
	// Set best to the score of us just standing here
	let mut best = score + flow * left;
	// println!("at {:?}", (at, left, score, flow));
	for v in valves.iter().cloned() {
		if !open.bit(v.idx) {
			let d = dist[&at][v.idx];
			if d + 1 < left {
				let mut o = open;
				o.set_bit(v.idx, true);
				let f = flow + v.flow;
				let s = score + flow * (d + 1);
				let res = search(v.idx, left - d - 1, f, s, o, dist, valves);
				best = best.max(res);
			}
		}
	}
	// println!("at best {:?}", (at, best));
	best
}

// fn search2(at0: usize, at1: usize, left: u32, flow: u32, score: u32, open: usize, dist: &HashMap<usize, Vec<u32>>, valves: &Vec<Valve>) -> u32 {
// 	// Set best to the score of us just standing here
// 	let mut best = score + flow * left;
// 	best
// }

fn search2(mut cur :[(usize, u32, bool);2], mut left: u32, mut flow: u32, mut score: u32, open: usize, dist: &HashMap<usize, Vec<u32>>, valves: &Vec<Valve>, must_beat: (u32, u32)) -> u32 {
	let wait = cur[0].1.min(cur[1].1).min(left);
	if wait > 0 {
		score += flow * wait;
		left -= wait;

		for (at, w, opening) in cur.iter_mut() {
			*w -= wait;
			if *opening && *w == 0 {
				// println!("open {:?}", (*at, left, score));
				flow += valves[*at].flow;
				*opening = false;
			}
		}
	}
	if left == 0 {
		return score;
	}
	// Set best to the score of us just standing here
	let mut best = score + flow * left;
	if score + must_beat.0 * left < must_beat.1 {
		// Can't beat best
		// println!("Early exit");
		return best;
	}
	// println!("at {:?}", (at, left, score, flow));
	for v in valves.iter().cloned() {
		if !open.bit(v.idx) {
			for (id, (at, w, _)) in cur.iter().enumerate() {
				if *w == 0 {
					let d = dist[at][v.idx];
					if d + 1 < left {
						let mut o = open;
						o.set_bit(v.idx, true);
						let mut nc = cur.clone();
						nc[id] = (v.idx, d + 1, true);
						let res = search2(nc, left, flow, score, o, dist, valves, (must_beat.0, best));
						best = best.max(res);
					}
				}
			}
		}
	}
	// println!("at best {:?}", (at, best));
	best
}