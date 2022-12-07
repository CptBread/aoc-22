use std::collections::{HashMap, HashSet};
use uni_path::{Path, PathBuf};

use aoc_util::parse_t::*;

#[derive(Debug)]
enum FType {
	Dir(HashSet<PathBuf>),
	File(u64),
}

impl FType {
	fn add_path(&mut self, path: PathBuf) {
		if let FType::Dir(paths) = self {
			paths.insert(path);
		}
	}

	fn unwrap_dir(&self) -> &HashSet<PathBuf> {
		if let FType::Dir(paths) = self {
			paths
		}
		else {
			panic!()
		}
	}
}

fn calc_sizes(cur_path: &PathBuf, cur_dir: &HashSet<PathBuf>, sizes: &mut HashMap<PathBuf, u64>, fss: &HashMap::<PathBuf, FType>) -> u64 {
	let mut res = 0;
	for p in cur_dir {
		match fss.get(p).unwrap() {
			FType::File(s) => {
				res += s;
			},
			FType::Dir(ref paths) => {
				res += calc_sizes(&p, &paths, sizes, fss);
			}
		}
	}
	sizes.insert(cur_path.clone(), res);
	res
}


#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day7.txt").unwrap();

	let mut fss = HashMap::<PathBuf, FType>::new();
	let mut dir = PathBuf::from("/");
	fss.insert(dir.clone(), FType::Dir(HashSet::new()));

	let root = PathBuf::from("/");

	// println!("{:?}", dir);

	// let mut in_ls = false;
	for l in data.lines() {
		if let Some(cd) = l.strip_prefix("$ cd ") {
			if cd == ".." {
				dir.pop();
			}
			else if cd == "/" {
				dir = root.clone();
			}
			else {
				let add_rotdir = dir == root;
				let new_dir = dir.join(cd);
				fss.entry(dir.clone()).or_insert(FType::Dir(HashSet::new()));
				fss.get_mut(&dir).unwrap().add_path(new_dir.clone());
				dir = new_dir;
			}
		}
		else if !l.starts_with("$") {
			if let Some(d) = l.strip_prefix("dir ") {
				fss.entry(dir.join(d)).or_insert(FType::Dir(HashSet::new()));
				fss.get_mut(&dir).unwrap().add_path(dir.join(d));
			}
			else {
				let (size, name) = parse_t!(l, u64, " ", PassStr, "").unwrap();
				let path = dir.join(name);
				fss.entry(path.clone()).or_insert(FType::File(size));
				fss.get_mut(&dir).unwrap().add_path(path);
			}
		}
	}

	let mut sizes = HashMap::new();
	let root_dir = fss.get(&root).unwrap().unwrap_dir();
	let tot_size = calc_sizes(&root, root_dir, &mut sizes, &fss);
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
	println!("{:?} {}", part1, part2);
}