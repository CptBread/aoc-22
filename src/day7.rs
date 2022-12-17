use std::collections::{HashMap};
use uni_path::{PathBuf};

use aoc_util::parse_t::*;

#[derive(Debug)]
enum FType {
	Dir,
	File(u64),
}

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day7.txt").unwrap();

	let mut fss = HashMap::<PathBuf, FType>::new();
	let mut dir = PathBuf::from("/");
	fss.insert(dir.clone(), FType::Dir);

	let root = PathBuf::from("/");
	for l in data.lines() {
		if let Some(cd) = l.strip_prefix("$ cd ") {
			if cd == ".." {
				dir.pop();
			}
			else if cd == "/" {
				dir = root.clone();
			}
			else {
				let new_dir = dir.join(cd);
				fss.entry(dir.clone()).or_insert(FType::Dir);
				dir = new_dir;
			}
		}
		else if !l.starts_with("$") {
			if let Some(d) = l.strip_prefix("dir ") {
				fss.entry(dir.join(d)).or_insert(FType::Dir);
			}
			else {
				let (size, name) = parse_t!(l, u64, " ", PassStr, "").unwrap();
				let path = dir.join(name);
				fss.entry(path.clone()).or_insert(FType::File(size));
			}
		}
	}

	let mut sizes = HashMap::new();
	let mut tot_size = 0;

	for (p, f) in fss.iter() {
		if let FType::File(size) = f {
			tot_size += size;
			for p in p.ancestors().skip(1) {
				*sizes.entry(p).or_default() += size;
			}
		}
	}

	let to_free = 30000000 - (70000000 - tot_size);

	let mut part1 = 0;
	let mut part2 = u64::MAX;
	for (_, s) in sizes.iter() {
		if *s <= 100000 {
			part1 += *s;
		}
		if *s > to_free && *s < part2 {
			part2 = *s;
		}
	}
	// 1517599 2481982
	println!("{:?} {}", part1, part2);
}